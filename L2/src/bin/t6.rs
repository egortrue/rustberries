// L2.6
// https://linux.die.net/man/1/cut
// echo "a|b|c|d|e|f|g|h|i|j|k" | cargo run --bin t6 -- -d "|" -f "1,5-8"

/*

Usage: t6.exe [OPTIONS] --fields <FIELDS>

Options:
  -d, --delimiter <DELIMITER>  Использовать собственный разделитель вместо табуляции [default: "\t"]
  -f, --fields <FIELDS>        Выводит перечисленные через запятую столбцы (номер, диапазон). Например, "-2,4,6-8,10-"
  -s, --separated              Только строки с указанным разделителем
  -h, --help                   Print help

*/

use clap::Parser;
use std::{
    cmp,
    io::{self, BufRead},
};

#[derive(Parser)]
struct Args {
    /// Использовать собственный разделитель вместо табуляции
    #[clap(short, long, default_value = "\t")]
    delimiter: String,

    /// Выводит перечисленные через запятую столбцы (номер, диапазон).
    /// Например, "-2,4,6-8,10-"
    #[clap(
        short,
        long,
        value_delimiter = ',', // автоматически резделяет аргумент
        allow_hyphen_values = true, // позволяет использовать минус в начале значений
        required = true
    )]
    fields: Vec<String>,

    /// Только строки с указанным разделителем
    #[clap(short, long)]
    separated: bool,
}

fn main() {
    let args = Args::parse();

    // Обработка указанный полей
    // Перевод из строчных диапазонов в цифровые значения
    // "-2,4,6-8,10-" -> [(1, 2), (4, 4), (6, 8), (10, 18446744073709551615)]
    let mut fields = vec![];
    for range in args.fields {
        let range_first;
        let range_last;

        // "-10" -> (1, 10)
        if range.starts_with('-') {
            let number = &range[1..];
            range_first = 1;
            range_last = number.parse::<usize>().expect("Couldn't parse field");
        }
        // "10-" -> (10, INF)
        else if range.ends_with('-') {
            let number = &range[..range.len() - 1];
            range_first = number.parse::<usize>().expect("Couldn't parse field");
            range_last = usize::MAX;
        }
        // "5-10" -> (5, 10)
        else if range.contains('-') {
            let numbers: Vec<&str> = range.split("-").collect();
            range_first = numbers[0].parse::<usize>().expect("Couldn't parse field");
            range_last = numbers[1].parse::<usize>().expect("Couldn't parse field");
        }
        // "10" -> (10, 10)
        else {
            range_first = range.parse::<usize>().expect("Couldn't parse field");
            range_last = range_first;
        }

        fields.push((range_first, range_last));
    }

    // Чтение и вывод
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();

        // Проверка на наличие сепаратора
        if args.separated && !line.contains(&args.delimiter) {
            continue;
        }

        // Вывод нужных полей
        let columns: Vec<&str> = line.split(&args.delimiter).collect();
        for (range_start, range_finish) in fields.iter() {
            let range_finish = cmp::min(columns.len(), *range_finish);
            for i in *range_start..=range_finish {
                if let Some(value) = columns.get(i - 1) {
                    print!("{value}\t");
                }
            }
        }
        println!();
    }
}
