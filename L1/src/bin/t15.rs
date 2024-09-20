// L1.15

fn main() {
    let mut array = vec![50, 10, 40, 33, 55, 10, 90, 66];
    println!("array: {:?}", array);

    qsort(&mut array);
    println!("qsort: {:?}", array);

    assert_eq!(array, [10, 10, 33, 40, 50, 55, 66, 90]);
}

fn qsort(array: &mut [impl Ord + Copy]) {
    if array.len() <= 1 {
        return;
    }

    let pivot_index = partition(array);
    let (left, right) = array.split_at_mut(pivot_index);

    qsort(&mut left[..pivot_index]);
    qsort(&mut right[1..]);
}

fn partition(array: &mut [impl Ord + Copy]) -> usize {
    let len = array.len();
    let pivot_index = len / 2;
    let pivot = array[pivot_index];
    let mut store_index = 0;

    array.swap(pivot_index, len - 1);

    for i in 0..len - 1 {
        if array[i] < pivot {
            array.swap(i, store_index);
            store_index += 1;
        }
    }

    array.swap(store_index, len - 1);
    store_index
}
