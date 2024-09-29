// L2.5
// https://linux.die.net/man/1/grep
// cargo run --bin t5 -- -n -C 1 --regexp "\d{12}" --file test/grep_input.txt
// cargo run --bin t5 -- -n -i -C 1 --regexp pellentesque --file test/grep_input.txt

/*

Usage: t5.exe [OPTIONS] --regexp <PATTERN> --file <FILE>

Options:
  -e, --regexp <PATTERN>
  -f, --file <FILE>
  -B, --before <BEFORE>    печатать -N строк (до совпадения)
  -A, --after <AFTER>      печатать +N строк (после совпадения)
  -C, --context <CONTEXT>  (A+B) печатать ±N строк (вокруг совпадения)
  -c, --count              напечатать количество строк
  -i, --ignore-case        игнорировать регистр
  -v, --invert             вместо совпадения, исключать
  -F, --fixed              точное совпадение со строкой, не паттерн
  -n, --line-num           напечатать номера строк
  -h, --help               Print help

*/

use std::{cmp, fs};

use clap::Parser;
use regex::RegexBuilder;

#[derive(Parser)]
struct Args {
    #[clap(short = 'e', long = "regexp")]
    pattern: String,

    #[clap(short = 'f', long)]
    file: String,

    /// печатать -N строк (до совпадения)
    #[clap(short = 'B', long)]
    before: Option<usize>,

    /// печатать +N строк (после совпадения)
    #[clap(short = 'A', long)]
    after: Option<usize>,

    /// (A+B) печатать ±N строк (вокруг совпадения)
    #[clap(short = 'C', long)]
    context: Option<usize>,

    /// напечатать количество строк
    #[clap(short = 'c', long)]
    count: bool,

    /// игнорировать регистр
    #[clap(short = 'i', long)]
    ignore_case: bool,

    /// вместо совпадения, исключать
    #[clap(short = 'v', long)]
    invert: bool,

    /// точное совпадение со строкой, не паттерн
    #[clap(short = 'F', long)]
    fixed: bool,

    /// напечатать номера строк
    #[clap(short = 'n', long)]
    line_num: bool,
}

fn main() {
    // Предварительная обработка аргументов
    let args = Args::parse();
    let content = fs::read_to_string(&args.file).expect("Couldn't read the file");
    let re = RegexBuilder::new(&args.pattern)
        .case_insensitive(args.ignore_case)
        .build()
        .expect("Couldn't compile regular expression");

    // Обработка ключей контекста (-C или -A + -B)
    let before = args.context.unwrap_or(args.before.unwrap_or_default());
    let after = args.context.unwrap_or(args.after.unwrap_or_default());

    // Поиск
    let lines: Vec<&str> = content.lines().collect();
    let mut result = vec![];
    for (index, line) in lines.iter().enumerate() {
        let find = if args.fixed {
            // Точное совпадение со строкой
            if args.ignore_case {
                line.to_lowercase() == args.pattern.to_lowercase()
            } else {
                line.to_string() == args.pattern
            }
        } else {
            // Содержит паттерн
            re.find(line).is_some()
        };

        // XOR invert
        if find ^ args.invert {
            // + контекст
            let start = cmp::max(index - before, 0);
            let finish = cmp::min(lines.len() - 1, index + after);
            for i in start..=finish {
                if !result.contains(&i) {
                    result.push(i);
                }
            }
        }
    }

    // Вывод
    if args.count {
        println!("{}", result.len())
    } else {
        for index in result {
            // + номера строк
            if args.line_num {
                print!("{} ", index + 1);
            }

            println!("{}", lines[index]);
        }
    }
}
