
/*

Схематично: [cmd arg > file | cmd arg < file | cmd arg >> file]

*/

#[derive(Default, Debug)]
pub enum CommandType {
    #[default]
    HELP,
    PWD,
    ECHO,
    CD,
    LS,
    EXPORT,
    OPEN,
    HISTORY,
    EXIT
}

#[derive(PartialEq, Eq, Debug)]
pub enum FileType {
    READ,
    WRITE,
    APPEND,
}

#[derive(Default, Debug)]
pub struct Command {
    pub ctype: CommandType,
    pub arguments: String,
    file: Option<(String, FileType)>,
}

impl Command {
    pub fn get_filename(&self) -> Option<String> {
        match &self.file {
            Some((name, _)) => Some(name.clone()),
            None => None,
        }
    }

    pub fn have_file_to(&self, ftype: FileType) -> bool {
        match &self.file {
            Some((_, lftype)) => ftype == *lftype,
            None => false,
        }
    }
}

// Базовый парсинг команд
fn parse_base(input: &str) -> Result<Command, String> {

    let (raw_command, arguments) = match input.split_once(" ") {
        Some((c, a)) => (c.trim(), a.trim().to_string()),
        None => (input.trim(), "".to_string()),
    };

    let ctype = match raw_command {
        "exit" => CommandType::EXIT,
        "help" => CommandType::HELP,
        "pwd" => CommandType::PWD,
        "history" => CommandType::HISTORY,
        "ls" => CommandType::LS,
        "echo" => CommandType::ECHO,
        "cd" => CommandType::CD,
        "export" => CommandType::EXPORT,
        "open" => CommandType::OPEN,
        _ => return Err(format!("Command not found: {input}")),
    };

    let mut command = Command::default();
    command.ctype = ctype;
    command.arguments = arguments;
    Ok(command)
}

// Парсинг использования ввода/вывода файлов
fn parse_file(input: &str) -> Result<Command, String> {    
    let raw_command;
    let file;

    if input.contains(" < ") {
        let data = input.split_once("<").unwrap();
        raw_command = data.0;
        file = Some((data.1.trim().to_string(), FileType::READ));
    } else if input.contains(" > ") {
        let data = input.split_once(">").unwrap();
        raw_command = data.0;
        file = Some((data.1.trim().to_string(), FileType::WRITE));
    } else if input.contains(" >> ")  {
        let data = input.split_once(">>").unwrap();
        raw_command = data.0;
        file = Some((data.1.trim().to_string(), FileType::APPEND));
    } else {
        raw_command = input;
        file = None;
    }

    let mut command = match parse_base(raw_command.trim()) {
        Ok(c) => c,
        Err(e) => return Err(e),
    };
    command.file = file;
    Ok(command)
}

// Парсинг каждой команды по-отдельности в пайплайне
pub fn parse_pipe(input: &str) -> Result<Vec<Command>, String> {
    let raw_commands = input.split("|");
    let mut commands = vec![];
    for raw_command in raw_commands {
        match parse_file(raw_command.trim()) {
            Ok(command) => commands.push(command),
            Err(e) => return Err(e),
        }
    }
    Ok(commands)
}

/// Список доступных команд
pub fn get_help() -> &'static str {
    "Available commands:\n  \
    help             - Print this help text\n  \
    pwd              - Print current working directory\n  \
    history          - List of all CommandTypes that were inputed \n  \
    echo <text>      - Output a text\n  \
    cd <dir>         - Change working directory\n  \
    ls <dir>         - List content of directory (default, working directory) \n  \
    export <key=val> - Add new environment variable (example: export key=value) \n  \
    open <file>      - Open a file or run an executable \n  \
    exit             - Exit the shell"
}

/// Корневой путь системы
pub fn get_rootdir() -> &'static str {
    if cfg!(windows) {
        r"C:\"
    } else {
        "/"
    }
}

pub fn get_opencmd() -> &'static str {
    if cfg!(windows) {
        "start"
    } else {
        "open"
    }
}


