mod command;
mod user;

use std::io::{self, Write};
use command::Command;
use user::User;

fn main() {
    let mut input_buffer: String = String::new();
    let mut output_buffer: String = String::new();

    let mut user = User::default();
    user.change_workdir(command::get_rootdir()).expect("Root directory is not accessable");

    loop {
        input_buffer.clear();
        output_buffer.clear();

        // Приглашение к вводу
        print!("[RUST-CLI][{}]$ ", user.get_workdir().expect("Should never failed"));
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
        let output = match command::parse(input) {
            None => Err("Command not found"),
            Some((command, argument)) => {

                // Запуск команд
                match command {

                    // Базовые команды
                    Command::HELP => command::get_help(),
                    Command::ECHO => Ok(argument),
                    Command::EXIT => break,

                    // Команды, использующие состояние пользователя
                    Command::PWD => user.get_workdir(),
                    Command::CD => user.change_workdir(argument),
                    Command::HISTORY => user.get_history(),
                    Command::LS => {
                        match user.list_directory(argument) {
                            Ok(stdout) => {
                                output_buffer.push_str(&stdout);
                                Ok(output_buffer.as_str())
                            },
                            Err(e) => Err(e),
                        }
                    },

                }

            },
        };

        // Вывод
        match output {
            Ok(stdout) => if !stdout.is_empty() {
                println!("{}", stdout)
            },
            Err(stderror) => eprintln!("ERROR: {}", stderror),
        }
        println!();
    }
}
