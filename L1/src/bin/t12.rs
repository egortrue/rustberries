// L1.12

use std::{collections::HashMap, hash::Hash, ops::AddAssign};

fn intersection<T: Eq + Hash>(arr1: Vec<T>, arr2: Vec<T>) -> Vec<T> {
    let mut dict = HashMap::new();

    for el in arr1 {
        dict.insert(el, 1);
    }

    for el in arr2 {
        if dict.contains_key(&el) {
            dict.get_mut(&el).unwrap().add_assign(1);
        }
    }

    let mut result = vec![];
    for (key, value) in dict {
        if value > 1 {
            result.push(key);
        }
    }

    result
}

fn main() {
    let arr1 = vec!["aa", "bb", "cc"];
    let arr2 = vec!["bb", "cc", "dd"];
    let result = intersection(arr1, arr2);

    println!("{result:?}");

    let arr1 = vec![1, 112450, 5234, 23452345, 1235, 123456];
    let arr2 = vec![14245, 5234, 1, 3124, 123476, 1, 1235];
    let result = intersection(arr1, arr2);

    println!("{result:?}");
}
