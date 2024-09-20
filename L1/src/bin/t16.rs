// L1.16

use std::cmp::Ordering;

fn main() {
    let test = vec![-10, 1, 5, 10, 12, 50, 101, 200, 5005, 12345];

    assert_eq!(binary_search(&test, 10), Some(3));
    assert_eq!(binary_search(&test, 200), Some(7));
    assert_eq!(binary_search(&test, 1), Some(1));
    assert_eq!(binary_search(&test, -10), Some(0));
    assert_eq!(binary_search(&test, -9999), None);
}

fn binary_search<T: Ord>(array: &[T], target: T) -> Option<usize> {
    let (mut start, mut end) = (0, array.len());

    while start < end {
        let middle = (start + end) / 2;
        match array[middle].cmp(&target) {
            Ordering::Equal => return Some(middle),
            Ordering::Less => start = middle,
            Ordering::Greater => end = middle,
        }
    }

    None
}
