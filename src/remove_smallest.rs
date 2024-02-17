// https://www.codewars.com/kata/563cf89eb4747c5fb100001b

fn remove_smallest(numbers: &[u32]) -> Vec<u32> {
    let mut numbers = numbers.to_vec();
    match numbers.iter().enumerate().min_by_key(|(_, value)| **value) {
        Some((min_index, _)) => {
            numbers.remove(min_index);
            numbers
        }
        None => numbers,
    }
}

#[cfg(test)]
mod tests {
    use super::remove_smallest;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn do_test(a: &[u32], expected: &[u32]) {
        assert_eq!(remove_smallest(a), expected, "{ERR_MSG} with numbers = {a:?}")
    }

    #[test]
    fn fixed_tests() {
        do_test(&[1, 2, 3, 4, 5], &[2, 3, 4, 5]);
        do_test(&[1, 2, 3, 4], &[2, 3, 4]);
        do_test(&[5, 3, 2, 1, 4], &[5, 3, 2, 4]);
        do_test(&[1, 2, 3, 1, 1], &[2, 3, 1, 1]);
    }
}

