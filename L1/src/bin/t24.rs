// L1.24

use std::collections::HashSet;

fn check_unique(string: &str) -> bool {
    let mut set = HashSet::new();

    for char in string.to_lowercase().chars() {
        if !set.insert(char) {
            return false;
        }
    }

    true
}

fn main() {
    assert!(check_unique("string"));
    assert!(!check_unique("AAAAA"));
    assert!(check_unique("abcd!@#"));
    assert!(!check_unique("!ABCD!"));
}
