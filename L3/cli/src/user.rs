use std::{collections::HashMap, fs, path::PathBuf};

#[derive(Default)]
pub struct User {
    workdir: String,
    history: String,
    env: HashMap<String, String>,
}

impl User {

    pub fn get_workdir(&self) -> Result<&str, &str> {
        Ok(&self.workdir)
    }

    pub fn change_workdir(&mut self, directory: &str) -> Result<&str, &str> {
        let path = PathBuf::from(&directory);
        if !path.exists() {
            return Err("Target doesn't exists");
        }
        if !path.is_dir() {
            return Err("Target is not directory");
        }

        self.workdir.clear();
        self.workdir.push_str(directory);
        Ok("")
    }

    pub fn update_history(&mut self, command: &str) {
        if !self.history.is_empty() {
            self.history.push('\n');
        }
        self.history.push_str(command);
    }

    pub fn get_history(&self) -> Result<&str, &str> {
        Ok(self.history.as_str())
    }

    pub fn update_env(&mut self, key: &str, value: &str) {
        self.env.insert(key.to_string(), value.to_string());
    }

    pub fn get_env(&self, key: &str) -> Result<&str, &str> {
        match self.env.get(key) {
            Some(value) => Ok(&value),
            None => Err("Environment variable not found"),
        }
    }

    pub fn list_directory(&self, directory: &str) -> Result<String, &str> {
        let mut output = String::new();
        let directory = if directory.is_empty() {
            self.workdir.as_str()
        } else {
            directory
        };

        if let Ok(directory) = fs::read_dir(directory) {
            for entry in directory {
                if let Ok(entity) = entry {

                    let fname = entity.file_name();
                    let fname = fname.to_str().unwrap();
                    let ftype = entity.file_type().unwrap();
                    let fsize = entity.metadata().unwrap().len();

                    // https://en.wikipedia.org/wiki/Unix_file_types
                    let fperm = if entity.metadata().unwrap().permissions().readonly() {
                        "r--"
                    } else {
                        "rw-"
                    };

                    let ftype = if ftype.is_file() {
                        'f'
                    } else if ftype.is_dir() {
                        'd'
                    } else if ftype.is_symlink() {
                        'l'
                    } else {
                        '-'
                    };

                    let file = format!("{ftype}{fperm} {fsize:<8} {fname:20}\n");
                    output += file.as_str();
                } else {
                    return Err("Directory is not accessable")
                }
            }
        } else {
            return Err("Directory is not accessable")
        }


        Ok(output)
    }
    
}