// Given a list and a number, create a new list that contains each number of list at most N times, without reordering.
use std::collections::HashMap;

pub fn delete_nth(list: &[u8], n: usize) -> Vec<u8> {
    let mut histogram: HashMap<u8, usize> = HashMap::new();
    let mut new_list: Vec<u8> = Vec::new();

    for number in list {
        match histogram.get(number) {
            Some(value) => histogram.insert(*number, value + 1),
            None => histogram.insert(*number, 1),
        };
        if histogram[number] <= n {
            new_list.push(*number)
        }
    }
    new_list
}

#[cfg(test)]
mod tests {
    use super::delete_nth;

    #[test]
    fn test_basic() {
        assert_eq!(delete_nth(&[20, 37, 20, 21], 1), vec![20, 37, 21]);
        assert_eq!(delete_nth(&[1, 1, 3, 3, 7, 2, 2, 2, 2], 3), vec![1, 1, 3, 3, 7, 2, 2, 2]);
    }
}