pub fn binary_search(niddle: u32, haystack: Vec<u32>) -> u32 {
    let mut found = false;
    let mut position = 0;
    let mut sliced: Vec<u32> = haystack;

    while !found {
        let middle = sliced.len() / 2;

        if sliced[middle] == niddle {
            found = true;
            position = middle;
        } else if sliced[middle] < niddle {
            sliced = sliced[middle..].to_vec();
        } else {
            sliced = sliced[..middle].to_vec();
        }
    }

    sliced[position]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_find_a_number() {
        let numbers: Vec<u32> = vec![2, 4, 5, 7, 10, 14, 16, 20, 32, 35, 40];
        assert_eq!(35, binary_search(35, numbers.clone()));
        assert_eq!(2, binary_search(2, numbers.clone()));
        assert_eq!(5, binary_search(5, numbers.clone()));
        assert_eq!(14, binary_search(14, numbers.clone()));
        assert_eq!(40, binary_search(40, numbers.clone()));
    }
}
