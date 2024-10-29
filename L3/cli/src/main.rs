mod commands;
mod user;

use std::io::{self, Write};

fn main() {
    let mut input_buffer: String = String::new();
    // let mut user = User::new();

    loop {
        print!("\n[RUST-CLI] {} $ ", commands::get_homedir());
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input_buffer).unwrap();
        input_buffer.clear();
    }
}
