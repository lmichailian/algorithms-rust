use std::fmt::{Debug, Display};

fn merge_sort<T: PartialOrd + Copy + Debug + Display>(arr: &[T]) -> Vec<T> {
    let collection: Vec<T> = arr.to_vec();
    let middle = collection.len() / 2;
    let mut left: Vec<T> = collection[..middle].to_vec();
    let mut right: Vec<T> = collection[middle..].to_vec();

    order_sides(&mut left);
    order_sides(&mut right);

    println!("{:?} {:?}", &left, &right);

    collection
}

fn order_sides<T: PartialOrd + Copy + Debug + Display>(arr: &mut [T]) {
    for item in 0..arr.len() {
        if item != arr.len() - 1 {
            //println!("{:?} {:?}", &arr[item + 1], &arr[item]);
            if arr[item + 1] < arr[item] {
                arr.swap(item, item + 1);
            }
        }
    }
}

fn main() {
    let numbers: Vec<u32> = vec![7, 3, 2, 16, 24, 4, 11, 1];
    assert_eq!(merge_sort(&numbers), vec![1, 2, 3, 4, 7, 11, 16, 24]);
}
