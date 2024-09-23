// L2.1
// cargo run --bin t1 -- [-c|-l|-w] <filename>
// t1.exe [-c|-l|-w] <filename>

use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let (flag, file) = parse_arguments(&args);
    let content = fs::read_to_string(file).expect("Couldn't open the file");

    match flag {
        Flag::C => println!("{}", content.chars().count()),
        Flag::L => println!("{}", content.lines().count()),
        Flag::W => println!("{}", content.split_whitespace().count()),
    };
}

// Используем перечисление для точной обработки и удобства расширения
// При добавлении новых флагов, компилятор сообщит о необходимости обработать новые флаги в ветках match
enum Flag {
    C,
    L,
    W,
}

// Парсинг и валидация аргументов командной строки
fn parse_arguments<'a>(args: &'a [String]) -> (Flag, &'a str) {
    if args.len() < 2 || 3 < args.len() {
        eprintln!("Wrong arguments. Usage: [-c|-l|-w] <filename>");
        std::process::exit(1);
    }

    let flag;
    let file;

    if args.len() == 2 {
        flag = Flag::C;
        file = &args[1];
    } else {
        flag = match args[1].as_str() {
            "-c" => Flag::C,
            "-l" => Flag::L,
            "-w" => Flag::W,
            _ => {
                eprintln!("Unknown flag. Awailable flags: '-c', '-w', '-l'");
                std::process::exit(1);
            }
        };
        file = &args[2];
    }

    (flag, file)
}
