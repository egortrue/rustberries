use std::env;

/// Список доступных команд
pub fn get_help() -> Result<String, String> {
    Ok("Available commands:\n  \
        help             - Print this helper\n  \
        pwd              - Print current working directory\n  \
        echo <argument>  - Display a line of text\n  \
        cd <argument>    - Change working directory\n  \
        ls <argument>    - List content of current working directory \n \
        history          - List of all commands that were inputed
        quit             - Exit the shell"
        .to_string())
}

/// Корневой путь системы
pub fn get_rootdir() -> String {
    if cfg!(windows) {
        r"C:\".to_string()
    } else {
        "/".to_string()
    }
}

/// Путь до домашнего каталога
pub fn get_homedir() -> String {
    if cfg!(windows) {
        env::var("UserProfile").unwrap_or(r"C:\Users".to_string())
    } else {
        env::var("HOME").unwrap_or(r"/home".to_string())
    }
}

pub fn get_username() -> String {
    if cfg!(windows) {
        env::var("UserName").unwrap()
    } else {
        env::var("USERNAME").unwrap()
    }
}
