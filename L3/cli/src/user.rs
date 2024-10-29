use std::collections::HashMap;

pub struct User {
    name: String,
    home: String,
    work_dir: String,
    history: Vec<String>,
    env: HashMap<String, String>,
}

impl User {}
