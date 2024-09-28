// L2.4
// Функция возвращает ссылки на исходные объекты
// НО, замечание по требованию: "Все слова должны быть приведены к нижнему регистру."
// Невозможно вернуть ссылки на слова, приведя их к нижнему регистру
// т.к. на вход подаются immutable + to_lowercase() возвращает новую строку
// Нижний регистр используется только для поиска анаграмм

use std::collections::HashMap;

// O(n + n)
fn are_anagrams(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    let mut char_count: HashMap<char, usize> = HashMap::new();
    for c in s1.chars() {
        *char_count.entry(c).or_default() += 1;
    }
    for c in s2.chars() {
        match char_count.get(&c) {
            Some(v) => {
                if *v == 0 {
                    return false;
                } else {
                    *char_count.get_mut(&c).unwrap() -= 1;
                }
            }
            None => return false,
        };
    }
    true
}

fn find_anagrams<'a>(words: &[&'a str]) -> HashMap<&'a str, Vec<&'a str>> {
    let mut anagrams: HashMap<&'a str, Vec<&'a str>> = HashMap::with_capacity(words.len());

    // Поиск
    for &word in words {
        let mut push_to_key: Option<&str> = None;

        for key in anagrams.keys() {
            if are_anagrams(key.to_lowercase().as_str(), word.to_lowercase().as_str()) {
                if !anagrams[key].contains(&word) {
                    push_to_key = Some(*key);
                }
                break;
            }
        }

        if let Some(key) = push_to_key {
            anagrams.get_mut(key).unwrap().push(word);
        } else {
            anagrams.insert(word, Vec::new());
        }
    }

    // Очистка
    let mut to_remove = vec![];
    let mut to_sort = vec![];
    for &key in anagrams.keys() {
        if anagrams[key].len() > 0 {
            to_sort.push(key);
        } else {
            to_remove.push(key);
        }
    }
    for key in to_remove {
        anagrams.remove(key);
    }
    for key in to_sort {
        anagrams.get_mut(key).unwrap().sort();
    }

    anagrams
}

fn main() {
    let input_arr = ["abc", "cab", "cbb", "aac", "aca"];
    let groups = find_anagrams(&input_arr);
    println!("{groups:#?}");

    /* Output

    {
        "aac": [
            "aca",
        ],
        "abc": [
            "cab",
        ],
    }

    */

    let input_arr = vec!["dzx", "zxD", "dxz", "", "czd", "xzd"];
    let groups = find_anagrams(&input_arr);
    println!("{groups:#?}");

    /* Output

    {
        "dzx": [
            "dxz",
            "xzd",
            "zxD",
        ],
    }

    */
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_anagram_true() {
        assert!(are_anagrams("rat", "tar"));
        assert!(are_anagrams("abcdef", "cefdab"));
    }

    #[test]
    fn test_is_anagram_false() {
        assert!(!are_anagrams("rat", "cat"));
        assert!(!are_anagrams("hello", "world"));
    }
}
