mod search;

fn main() {
    let numbers: Vec<u32> = Vec::from([2, 4, 5, 7, 10, 14, 16, 20, 32, 35, 40]);
    search::binary_search(35, numbers);
}
