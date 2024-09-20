// L1.22

fn my_remove(array: &mut Vec<impl Ord>, index: usize) {
    if array.len() < index {
        panic!();
    }

    // По очереди свапаем все влево
    for i in index..array.len() - 1 {
        array.swap(i, i + 1);
    }

    // Обрезаем конец...
    array.truncate(array.len() - 1);
}

fn main() {
    let mut array = vec![0, 2, 5, 7, 10, 23, 51, 6];
    let index = 2;

    // O(n)
    array.remove(index);
    assert_eq!(array, vec![0, 2, 7, 10, 23, 51, 6]);

    // O(1) без сохранения порядка
    array = vec![0, 2, 5, 7, 10, 23, 51, 6];
    array.swap_remove(index);
    assert_eq!(array, vec![0, 2, 6, 7, 10, 23, 51]);

    // Моё O(n)
    array = vec![0, 2, 5, 7, 10, 23, 51, 6];
    my_remove(&mut array, index);
    assert_eq!(array, vec![0, 2, 7, 10, 23, 51, 6]);
}
