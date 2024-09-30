// L2.8
// Пример: echo ../../../ | cd - | pwd

use core::str;
use std::{
    env::{self},
    io::{self, Write},
    process::{self},
};

#[derive(Debug)]
enum Command {
    HELP,
    QUIT,
    PWD,
    CD,
    ECHO,
    PS,
    KILL,
}

fn main() {
    loop {
        // Приглашение к вводу
        print!("\n(L2.8) $ ");
        io::stdout().flush().unwrap();

        // Чтение и парсинг ввода
        let mut buffer: String = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        let commands = parse(&buffer.trim());

        // Выполнение списка команд (пайплайна)
        let mut last_result: Option<Result<String, String>> = None;
        for (command, mut argument) in commands {
            // Получение вывода предыдущей команды
            let last_stdout;
            if let Some(last_output) = last_result.clone() {
                last_stdout = last_output.unwrap();
                argument = last_stdout.as_str();
            }

            // Обрабатываем специальный аргумент (stdout -> stdin)
            if argument == "-" {
                last_result = Some(Err(format!(
                    "Command \"{:?}\" waited for input from stdin, but it was not provided",
                    command
                )));
                break;
            }

            // Выполнение команды
            let result = match command {
                Command::QUIT => return,
                Command::HELP => print_help(),
                Command::PWD => print_workdir(),
                Command::PS => print_processes(),
                Command::ECHO => print_echo(argument),
                Command::CD => change_directory(argument),
                Command::KILL => kill_process(argument),
            };
            last_result = Some(result);
        }

        // Вывод
        if let Some(result) = last_result {
            match result {
                Ok(stdout) => println!("{stdout}"),
                Err(stderr) => eprintln!("{stderr}"),
            }
        }
    }
}

/// Поддержка пайплайнов
fn parse(input: &str) -> Vec<(Command, &str)> {
    // Разделение по пайпам
    let input_commands = input.split("|");
    let mut commands = vec![];

    // Парсинг каждой команды по-отдельности
    for input_command in input_commands {
        if let Some(command) = parse_command(&input_command.trim()) {
            commands.push(command);
        }
        // Если команду не удалось распарсить - выводим только помощь
        else {
            commands.clear();
            commands.push((Command::HELP, ""));
            break;
        }
    }

    commands
}

/// Парсинг простейших команд
fn parse_command(input: &str) -> Option<(Command, &str)> {
    // Простейшие команды
    match input {
        "quit" => return Some((Command::QUIT, "")),
        "pwd" => return Some((Command::PWD, "")),
        "ps" => return Some((Command::PS, "")),
        "help" => return Some((Command::HELP, "")),
        _ => (),
    };

    // Параметризированные команды
    let (command, argument) = input.split_once(" ").unwrap_or_default();
    match command {
        "echo" => return Some((Command::ECHO, argument)),
        "cd" => return Some((Command::CD, argument)),
        "kill" => return Some((Command::KILL, argument)),
        _ => (),
    }

    eprintln!("Wrong command: \"{}\"", input);
    None
}

/// Список доступных команд
fn print_help() -> Result<String, String> {
    Ok("Available commands:\n  \
       help             - Print this helper\n  \
       pwd              - Print current working directory\n  \
       ps               - Print running processes\n  \
       echo <argument>  - Display a line of text\n  \
       cd <argument>    - Change working directory\n  \
       kill <argument>  - Kill a process\n  \
       quit             - Exit the current shell"
        .to_string())
}

/// Показать путь до текущего каталога
fn print_workdir() -> Result<String, String> {
    match env::current_dir() {
        Ok(path) => Ok(path.display().to_string()),
        Err(err) => Err(err.to_string()),
    }
}

/// Вывод аргумента в STDOUT
fn print_echo(input: &str) -> Result<String, String> {
    Ok(input.to_string())
}

/// Смена директории (в качестве аргумента могут быть то-то и то)
fn change_directory(path: &str) -> Result<String, String> {
    match env::set_current_dir(path) {
        Ok(_) => Ok("".to_string()),
        Err(err) => Err(err.to_string()),
    }
}

/// Выводит общую информацию по запущенным процессам в формате id процесса, название, время работы в мсек.
fn print_processes() -> Result<String, String> {
    if cfg!(windows) {
        start_subprocess("tasklist", &[]) // Нет аргументов для фильтрации
    } else {
        start_subprocess("ps", &["-eo", "pid,comm,etime"])
    }
}

/// Убить процесс, переданный в качестве аргумента
fn kill_process(id_str: &str) -> Result<String, String> {
    if let Err(_) = id_str.parse::<usize>() {
        return Err(format!("Invalid pid: \"{id_str}\""));
    }

    if cfg!(windows) {
        start_subprocess("taskkill", &["/F", "/pid", id_str])
    } else {
        start_subprocess("kill", &[id_str])
    }
}

/// Функционал fork/exec-команд
fn start_subprocess(command: &str, args: &[&str]) -> Result<String, String> {
    match process::Command::new(command).args(args).output() {
        Ok(result) => {
            if result.status.success() {
                Ok(String::from_utf8_lossy(&result.stdout).to_string())
            } else {
                Err(String::from_utf8_lossy(&result.stderr).to_string())
            }
        }
        Err(err) => Err(err.to_string()),
    }
}
