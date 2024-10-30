mod command;
mod user;

use std::io::{self, Write};
use command::{CommandType, FileType};
use user::User;

fn main() {
    let mut input_buffer: String = String::new();
    let mut stdout_buffer: String = String::new();
    let mut stderr_buffer: String = String::new();

    let mut user = User::default();
    user.change_workdir(command::get_rootdir()).expect("Root directory is not accessable");

    println!("\nWelcome to RUST-CLI (@Egor Trukhin)\n");
    loop {
        input_buffer.clear();
        stdout_buffer.clear();
        stderr_buffer.clear();

        // Приглашение к вводу
        print!("[RUST-CLI][{}]$ ", user.get_workdir().expect("Should never fail"));
        io::stdout().flush().expect("Error write to stdout");

        // Чтение с клавиатуры
        io::stdin().read_line(&mut input_buffer).expect("Error reading the input");
        let input = input_buffer.trim();
        if input.len() == 0 {
            continue;
        }

        // Обновление пользовательской истории
        user.update_history(input);

        // Парсинг
        let status = match command::parse_pipe(input) {
            Err(e) => {
                stderr_buffer.clear();
                stderr_buffer.push_str(&e);
                Err(stderr_buffer.as_str())
            },
            Ok(commands) => { 

                // Запуск пайплайна
                let mut last_result = None;
                for command in commands {

                    // Обработка входного файла (при наличии)
                    let input_file = if command.have_file_to(FileType::READ) {
                        match user.read_file(&command.get_filename().unwrap()) {
                            Ok(content) => Some(content),
                            Err(stderr) => {                            
                                stderr_buffer.clear();
                                stderr_buffer.push_str(&stderr);
                                break; // Остановка на ошибке чтение входного файл при его наличии
                            },
                        }
                    } else {
                        None
                    };

                    // Постпроцессинг аргументов (приоритет: пайпа > файл > аргументы)
                    let arguments = {  
                        
                        // Вывод предыдущей команды
                        if last_result.is_some() {
                            user.process_env(&stdout_buffer)
                        }
                        
                        // Чтение из файла
                        else if input_file.is_some() {
                            user.process_env(&input_file.unwrap())
                        } 
                        
                        // Аргумент командной строки
                        else {
                            user.process_env(&command.arguments)
                        }
                    };

                    // Запуск команды
                    let result = match command.ctype {
                        CommandType::EXIT => return,
                        CommandType::HELP => Ok(command::get_help().to_string()),
                        CommandType::ECHO => Ok(arguments),
                        CommandType::PWD => user.get_workdir(),
                        CommandType::HISTORY => user.get_history(),
                        CommandType::CD => user.change_workdir(&arguments),
                        CommandType::EXPORT => user.update_env(&arguments),
                        CommandType::LS => user.list_directory(&arguments),
                        CommandType::OPEN => user.open_file(&arguments),
                    };

                    // Обновление буфферов
                    match &result {
                        Ok(stdout) => {
                            stdout_buffer.clear();
                            stdout_buffer.push_str(stdout);

                            // Запись результата в файл
                            let result_write = if command.have_file_to(FileType::WRITE) {
                                user.write_file(&command.get_filename().unwrap(), &stdout_buffer)
                            } else if command.have_file_to(FileType::APPEND) {
                                user.append_file(&command.get_filename().unwrap(), &stdout_buffer)
                            } else {
                                Ok("".to_string())
                            };

                            match result_write {
                                Ok(_) => (),
                                Err(stderr) => {                            
                                    stderr_buffer.clear();
                                    stderr_buffer.push_str(&stderr);
                                    break; // Остановка на ошибке записи успешной команды в файл
                                }
                            }
                        },
                        Err(stderr) => {
                            stderr_buffer.clear();
                            stderr_buffer.push_str(stderr);
                            break; // Остановка на первой упавшей команде в пайпе
                        },
                    };

                    last_result = Some(result);
                };

                if let Some(result) = last_result {
                    match result {
                        Ok(_) => Ok(stdout_buffer.as_str()),
                        Err(_) => Err(stderr_buffer.as_str()),
                    }
                } else {
                    Err(stderr_buffer.as_str())
                }
            },
        };

        // Вывод
        match status {
            Ok(stdout) => {
                println!("{}", stdout);
                // Дополнительный отступ
                if !stdout.is_empty() && !stdout.ends_with('\n') {
                    println!();
                }
            },
            Err(stderror) => eprintln!("ERROR: {}\n", stderror),
        }
    }
}
