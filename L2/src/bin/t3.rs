// L2.3
// https://linux.die.net/man/1/sort
// cargo run --bin t3 -- -u -H -k 2 -o test/sort_output.txt test/sort_input.txt

/*

Usage: t3.exe [OPTIONS] <input_file>

Arguments:
  <input_file>

Options:
  -o, --output <OUTPUT>         [default: output.txt]
  -k <COLUMN>                   Указание колонки для сортировки [default: 1]
  -n, --numeric-sort            Сортировать по числовому значению
  -H, --human-numeric-sort      Сортировать по числовому значению с учетом суффиксов
  -M, --month-sort              Сортировать по названию месяца
  -r, --reverse                 Вывод в обратном порядке
  -u, --unique                  Удалить дубликаты строк
  -c, --check-sorted            Проверить, отсортированы ли данные
  -b, --ignore-trailing-spaces  Игнорировать хвостовые пробелы
  -h, --help                    Print help

*/

use clap::Parser;
use std::{cmp::Ordering, collections::HashSet, fs};

#[derive(Parser)]
struct Args {
    #[clap(id = "input_file")]
    input: String,

    #[clap(short, long, default_value = "output.txt")]
    output: String,

    /// Указание колонки для сортировки
    #[clap(short = 'k', default_value = "1")]
    column: usize,

    /// Сортировать по числовому значению
    #[clap(short = 'n', long)]
    numeric_sort: bool,

    /// Сортировать по числовому значению с учетом суффиксов
    #[clap(short = 'H', long)] // -h занят для --help
    human_numeric_sort: bool,

    /// Сортировать по названию месяца
    #[clap(short = 'M', long)]
    month_sort: bool,

    /// Вывод в обратном порядке
    #[clap(short = 'r', long)]
    reverse: bool,

    /// Удалить дубликаты строк
    #[clap(short = 'u', long)]
    unique: bool,

    /// Проверить, отсортированы ли данные
    #[clap(short = 'c', long)]
    check_sorted: bool,

    /// Игнорировать хвостовые пробелы
    #[clap(short = 'b', long)]
    ignore_trailing_spaces: bool,
}

fn main() {
    // Подготовка
    let args = Args::parse();
    let content = fs::read_to_string(&args.input).expect("Couldn't read the file");
    let mut lines: Vec<&str> = content.lines().collect();

    //---------------------------------------
    // Предобработка

    // Игнорирование пробелов в приоритете
    if args.ignore_trailing_spaces {
        lines = lines.iter().map(|line| line.trim_end()).collect()
    }

    // Проверка сортировки обязательно до фильтрации дубликатов
    if args.check_sorted {
        if check_sorted(&lines, &args) {
            println!("File \'{}\' sorted", args.input);
        } else {
            println!("File \'{}\' NOT sorted", args.input);
        }
        return;
    }

    // Фильтрация дубликатов
    if args.unique {
        let mut uniques = HashSet::with_capacity(lines.len());
        lines.retain(|line| uniques.insert(line.to_string()));
    }

    //---------------------------------------
    // Сортировка

    lines.sort_by(|a, b| {
        let a_key = get_value_on_column(a, args.column);
        let b_key = get_value_on_column(b, args.column);

        let result = if args.human_numeric_sort {
            comparator_human_numeric(&a_key, &b_key)
        } else if args.month_sort {
            comparator_month(&a_key, &b_key)
        } else if args.numeric_sort {
            comparator_numeric(&a_key, &b_key)
        } else {
            a_key.cmp(&b_key)
        };

        if args.reverse {
            result.reverse()
        } else {
            result
        }
    });

    //---------------------------------------
    // Результат

    fs::write(&args.output, lines.join("\n")).expect("Couldn't write to the file");
}

fn get_value_on_column<'a>(line: &'a str, mut nth: usize) -> &'a str {
    if line.len() == 0 {
        return line;
    }

    let words: Vec<&str> = line.split_whitespace().collect();
    if nth < 1 {
        nth = 1;
    }
    if nth > words.len() {
        nth = words.len()
    }
    words[nth - 1]
}

fn check_sorted(lines: &[&str], args: &Args) -> bool {
    for i in 1..lines.len() {
        let a_key = get_value_on_column(lines[i - 1], args.column);
        let b_key = get_value_on_column(lines[i], args.column);

        let result = if args.human_numeric_sort {
            comparator_human_numeric(&a_key, &b_key)
        } else if args.month_sort {
            comparator_month(&a_key, &b_key)
        } else if args.numeric_sort {
            comparator_numeric(&a_key, &b_key)
        } else {
            a_key.cmp(&b_key)
        };

        if result == Ordering::Greater && !args.reverse {
            return false;
        } else if result == Ordering::Less && args.reverse {
            return false;
        }
    }

    true
}

/// (unknown) < -FLOAT ... -1 < 0 < 1 ... < FLOAT
fn comparator_numeric(a: &str, b: &str) -> Ordering {
    let res_number_a = a.parse::<f64>();
    let res_number_b = b.parse::<f64>();

    match (res_number_a, res_number_b) {
        (Ok(number_a), Ok(number_b)) => {
            if number_a < number_b {
                Ordering::Less
            } else if number_a == number_b {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        }
        (Ok(_), Err(_)) => Ordering::Greater,
        (Err(_), Ok(_)) => Ordering::Less,
        (Err(_), Err(_)) => a.cmp(b),
    }
}

/// (unknown) < 'JAN' < 'FEB' < ... < 'DEC'
fn comparator_month(a: &str, b: &str) -> Ordering {
    static MONTHS: [&str; 12] = [
        "jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec",
    ];

    let opt_idx_a = MONTHS.iter().position(|&el| el == a.to_lowercase());
    let opt_idx_b = MONTHS.iter().position(|&el| el == b.to_lowercase());

    match (opt_idx_a, opt_idx_b) {
        (Some(index_a), Some(index_b)) => index_a.cmp(&index_b),
        (None, Some(_)) => Ordering::Less,
        (Some(_), None) => Ordering::Greater,
        (None, None) => a.cmp(b),
    }
}

/// (unknown) 1.0 < 1.0K < 1.0M < 1.0G < ... < 1.0Q
fn comparator_human_numeric(a: &str, b: &str) -> Ordering {
    // https://en.wikipedia.org/wiki/Metric_prefix
    static SUFFIXES: [(&str, u128); 10] = [
        ("K", u128::pow(10, 3)),
        ("M", u128::pow(10, 6)),
        ("G", u128::pow(10, 9)),
        ("T", u128::pow(10, 12)),
        ("P", u128::pow(10, 15)),
        ("E", u128::pow(10, 18)),
        ("Z", u128::pow(10, 21)),
        ("Y", u128::pow(10, 24)),
        ("R", u128::pow(10, 27)),
        ("Q", u128::pow(10, 30)),
    ];

    let (object_a, suffix_a) = if a.len() > 1 {
        match a.split_at_checked(a.len() - 1) {
            Some((object, suffix)) => (object, suffix),
            None => (a, ""),
        }
    } else {
        (a, "")
    };

    let (object_b, suffix_b) = if b.len() > 1 {
        match b.split_at_checked(b.len() - 1) {
            Some((object, suffix)) => (object, suffix),
            None => (b, ""),
        }
    } else {
        (b, "")
    };

    // Проверка числовых значений на валидность
    let res_number_a = object_a.parse::<f64>();
    let res_number_b = object_b.parse::<f64>();
    let (number_a, number_b) = match (res_number_a, res_number_b) {
        (Ok(number_a), Ok(number_b)) => (number_a, number_b),
        (Ok(_), Err(_)) => return Ordering::Greater,
        (Err(_), Ok(_)) => return Ordering::Less,
        (Err(_), Err(_)) => return a.cmp(&b),
    };

    // Проверка суффикса на валидность
    let opt_idx_suffix_a = SUFFIXES.iter().position(|&(el, _)| el == suffix_a);
    let opt_idx_suffix_b = SUFFIXES.iter().position(|&(el, _)| el == suffix_b);
    let (idx_suffix_a, idx_suffix_b) = match (opt_idx_suffix_a, opt_idx_suffix_b) {
        (Some(idx_a), Some(idx_b)) => (idx_a, idx_b),
        (Some(_), None) => return Ordering::Greater,
        (None, Some(_)) => return Ordering::Less,
        (None, None) => return comparator_numeric(a, b),
    };

    // Сравнение валидных чисел
    let full_number_a = number_a * SUFFIXES[idx_suffix_a].1 as f64;
    let full_number_b = number_b * SUFFIXES[idx_suffix_b].1 as f64;
    if full_number_a < full_number_b {
        Ordering::Less
    } else if full_number_a == number_b {
        Ordering::Equal
    } else {
        Ordering::Greater
    }
}
