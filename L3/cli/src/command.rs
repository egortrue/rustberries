
pub enum Command {
    HELP,
    PWD,
    ECHO,
    CD,
    LS,
    HISTORY,
    EXIT
}

/// Парсинг входящих команд
pub fn parse(input: &str) -> Option<(Command, &str)> {
    // Простейшие команды
    match input {
        "exit" => return Some((Command::EXIT, "")),
        "help" => return Some((Command::HELP, "")),
        "pwd" => return Some((Command::PWD, "")),
        "history" => return Some((Command::HISTORY, "")),
        "ls" => return Some((Command::LS, "")),
        _ => (),
    };

    // Параметризированные команды
    let (command, argument) = input.split_once(" ").unwrap_or_default();
    match command {
        "echo" => return Some((Command::ECHO, argument)),
        "cd" => return Some((Command::CD, argument)),
        "ls" => return Some((Command::LS, argument)),
        _ => (),
    }

    None
}

/// Список доступных команд
pub fn get_help() -> Result<&'static str, &'static str> {
    Ok("Available commands:\n  \
        help             - Print this helper\n  \
        pwd              - Print current working directory\n  \
        echo <argument>  - Display a line of text\n  \
        cd <argument>    - Change working directory\n  \
        ls <argument>    - List content of current working directory \n  \
        history          - List of all commands that were inputed \n  \
        exit             - Exit the shell")
}

/// Корневой путь системы
pub fn get_rootdir() -> &'static str {
    if cfg!(windows) {
        r"C:\"
    } else {
        "/"
    }
}

