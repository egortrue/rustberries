// L2.2

fn unpack(input: &str) -> Result<String, &'static str> {
    let mut result = String::with_capacity(input.len() * 10);
    let mut string_buffer = String::with_capacity(1); // Всегда один символ
    let mut number_buffer = String::with_capacity(input.len()); // Любое кол-во подряд идущих цифр
    let mut escaping = false;

    // Обновляет результат и очищает буферы
    fn flush(string_buffer: &mut String, number_buffer: &mut String, result: &mut String) {
        if string_buffer.is_empty() {
            return;
        }
        let mut repeat = 1;
        if !number_buffer.is_empty() {
            repeat = number_buffer.parse::<usize>().unwrap();
            number_buffer.clear();
        }
        result.push_str(string_buffer.repeat(repeat).as_str());
        string_buffer.clear();
    }

    for cur_char in input.chars() {
        // Начинается escape-последовательность
        if cur_char == '\\' && escaping == false {
            escaping = true;
            continue;
        }

        // Обрабатываем число как мильтипликатор
        if cur_char.is_ascii_digit() && escaping == false {
            if string_buffer.is_empty() {
                return Err("Invalid multiplicator: nothing to multiply");
            }
            if number_buffer.is_empty() && cur_char == '0' {
                return Err("Invalid multiplicator: begins with zero");
            }
            number_buffer.push(cur_char);
            continue;
        }

        // Обновляем буферы
        flush(&mut string_buffer, &mut number_buffer, &mut result);
        string_buffer.push(cur_char);
        escaping = false
    }

    // Если '\' - был последним
    if escaping {
        return Err("Invalid escaping");
    }

    // Обработка последней последовательности в буфере
    flush(&mut string_buffer, &mut number_buffer, &mut result);

    Ok(result)
}

fn main() {
    assert_eq!(unpack(r"😍10").unwrap(), r"😍😍😍😍😍😍😍😍😍😍");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unpack_correct_strings() {
        assert_eq!(unpack(r"").unwrap(), r"");
        assert_eq!(unpack(r"abcd").unwrap(), r"abcd");
        assert_eq!(unpack(r"a2").unwrap(), r"aa");
        assert_eq!(unpack(r"a4bc2d5e").unwrap(), r"aaaabccddddde");
        assert_eq!(unpack(r"a20").unwrap(), r"aaaaaaaaaaaaaaaaaaaa");
        assert_eq!(unpack(r"a16b10c").unwrap(), r"aaaaaaaaaaaaaaaabbbbbbbbbbc");
        assert_eq!(unpack(r"東2京12").unwrap(), r"東東京京京京京京京京京京京京");
        assert_eq!(unpack(r"😍10").unwrap(), r"😍😍😍😍😍😍😍😍😍😍");
    }

    #[test]
    fn test_unpack_incorrect_strings() {
        assert_eq!(
            unpack(r"45"),
            Err("Invalid multiplicator: nothing to multiply")
        );
        assert_eq!(
            unpack(r"a012"),
            Err("Invalid multiplicator: begins with zero")
        );
    }

    #[test]
    fn test_unpack_correct_escape_strings() {
        assert_eq!(unpack(r"qwe\4\5").unwrap(), r"qwe45");
        assert_eq!(unpack(r"qwe\45").unwrap(), r"qwe44444");
        assert_eq!(unpack(r"qwe\\5").unwrap(), r"qwe\\\\\");

        // Предпологаем, что можно ескейпить все, а не только цифры и слеш
        assert_eq!(unpack(r"\ab").unwrap(), r"ab");
        assert_eq!(unpack(r"\😍b").unwrap(), r"😍b");
    }

    #[test]
    fn test_unpack_incorrect_escape_strings() {
        assert_eq!(unpack(r"\"), Err("Invalid escaping"));
        assert_eq!(unpack(r"abc\"), Err("Invalid escaping"));
    }
}
