use std::{collections::HashMap, fs, path::PathBuf};

use crate::command;

#[derive(Default)]
pub struct User {
    workdir: String,
    history: String,
    env: HashMap<String, String>,
}

impl User {

    pub fn get_workdir(&self) -> Result<String, String> {
        Ok(self.workdir.to_string())
    }

    pub fn change_workdir(&mut self, directory: &str) -> Result<String, String> {

        let path = match self.relative_to_absolute(directory) {
            Ok(p) => p,
            Err(e) => return Err(e),
        };

        if !PathBuf::from(&path).is_dir() {
            return Err(format!("Target is not directory: {path}"));
        }

        self.workdir.clear();
        self.workdir.push_str(&path);
        Ok("".to_string())
    }

    pub fn update_history(&mut self, command: &str) {
        if !self.history.is_empty() {
            self.history.push('\n');
        }
        self.history.push_str(command);
    }

    pub fn get_history(&self) -> Result<String, String> {
        Ok(self.history.to_string())
    }

    pub fn update_env(&mut self, argument: &str) -> Result<String, String> {
        let (key, value) = match argument.split_once("=") {
            Some((k, v)) => (k, v),
            None => return Err(format!("Failed to create environment variable. Example: key=value")),
        };

        self.env.insert(key.to_string(), value.to_string());
        Ok("".to_string())
    }

    pub fn process_env(&self, input: &str) -> String {
        let mut output = String::from(input);
        for (key, value) in self.env.iter() {
            output = output.replace(format!("${key}").as_str(), value);
        };
        output
    }

    pub fn read_file(&self, file: &str) -> Result<String, String> {
        let path = match self.relative_to_absolute(file) {
            Ok(p) => p,
            Err(e) => return Err(e),
        };

        match fs::read_to_string(path) {
            Ok(content) => Ok(content),
            Err(e) => Err(format!("{e}")),
        }
    }

    pub fn write_file(&self, file: &str, content: &str) -> Result<String, String> {
        let path = match self.relative_to_absolute(file) {
            Ok(p) => p,
            Err(e) => return Err(e),
        };

        match fs::write(path, content) {
            Ok(_) => Ok(format!("")),
            Err(e) => Err(format!("{e}")),
        }
    }

    pub fn append_file(&self, file: &str, content: &str) -> Result<String, String> {
        let old_content = match self.read_file(file) {
            Ok(content) => content,
            Err(e) => return Err(e),
        };

        let new_content = old_content + content;
        
        match self.write_file(file, &new_content) {
            Ok(_) => Ok(format!("")),
            Err(e) => Err(e),
        }
    }

    pub fn open_file(&self, file: &str) -> Result<String, String> {
        let command = command::get_opencmd();

        let path = match self.relative_to_absolute(file) {
            Ok(p) => p,
            Err(e) => return Err(e),
        };

        match std::process::Command::new(command).arg(path).output() {
            Ok(output) => if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string())
            } else {
                Err(String::from_utf8_lossy(&output.stderr).to_string())
            }
            Err(e) => Err(format!("Failed to open the file: {e}"))
        }
    }

    pub fn list_directory(&self, directory: &str) -> Result<String, String> {

        let path = match self.relative_to_absolute(directory) {
            Ok(p) => p,
            Err(e) => return Err(e),
        };

        let directory = match fs::read_dir(path) {
            Ok(d) => d,
            Err(e) => return Err(format!("{e}")),
        };

        let mut output = String::new();
        for entry in directory {
            match entry {
                Err(e) => return Err(format!("{e}")),

                Ok(entity) => {
                    // Параметры файла
                    let fname = entity.file_name();
                    let fname = fname.to_str().unwrap();
                    let ftype = entity.file_type().unwrap();
                    let fsize = entity.metadata().unwrap().len();

                    // https://en.wikipedia.org/wiki/File-system_permissions
                    let fperm = if entity.metadata().unwrap().permissions().readonly() {
                        "r--"
                    } else {
                        "rw-"
                    };

                    // https://en.wikipedia.org/wiki/Unix_file_types
                    let ftype = if ftype.is_file() {
                        '-'
                    } else if ftype.is_dir() {
                        'd'
                    } else if ftype.is_symlink() {
                        'l'
                    } else {
                        '?'
                    };

                    // Формирование вывода
                    let file = format!("{ftype}{fperm} {fsize:<8} {fname:20}\n");
                    output += file.as_str();
                },
            }
        }

        Ok(output)
    }
    
    fn relative_to_absolute(&self, input: &str) -> Result<String, String> {
        let path = PathBuf::from(input);
        if path.is_absolute() {
            return Ok(input.to_string());
        }
        let mut workdir = PathBuf::from(&self.workdir);
        workdir.push(path);
        match workdir.canonicalize() {
            Ok(p) => Ok(p.to_str().unwrap().to_string()),
            Err(e) => Err(format!("Failed to canonicalize path: {e}")),
        }
    }
}