pub fn merge_sort<T: PartialOrd + Copy>(arr: &[T]) -> Vec<T> {
    if arr.len() <= 1 {
        return arr.to_vec();
    }

    let mid = arr.len() / 2;
    let (left, right) = arr.split_at(mid);

    let left = merge_sort(&left);
    let right = merge_sort(&right);

    merge(&left, &right)
}

fn merge<T: PartialOrd + Copy>(left: &[T], right: &[T]) -> Vec<T> {
    let mut result = vec![];
    let mut left_index = 0;
    let mut right_index = 0;

    while left_index < left.len() && right_index < right.len() {
        if left[left_index] < right[right_index] {
            result.push(left[left_index]);
            left_index += 1;
        } else {
            result.push(right[right_index]);
            right_index += 1;
        }
    }

    while left_index < left.len() {
        result.push(left[left_index]);
        left_index += 1;
    }

    while right_index < right.len() {
        result.push(right[right_index]);
        right_index += 1;
    }

    result  
}
