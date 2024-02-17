// Write an algorithm that takes an array and moves all of the zeros to the end, preserving the order of the other elements.

pub fn move_zeros(arr: &[u8]) -> Vec<u8> {
    let mut zero_vec: Vec<u8> = Vec::new();
    let mut new_vec: Vec<u8> = Vec::new();
    for number in arr {
        match number {
            0 => zero_vec.push(0),
            n => new_vec.push(*n)
        }
    }
    new_vec.extend(zero_vec);
    new_vec
}

#[cfg(test)]
mod tests {
    use super::move_zeros;

    fn do_test(a: &[u8], expected: &[u8]) {
        let actual = move_zeros(a);
        assert_eq!(actual, expected, "With arr = {a:?}\nExpected {expected:?} but got {actual:?}")
    }

    #[test]
    fn sample_tests() {
        do_test(&[1, 2, 0, 1, 0, 1, 0, 3, 0, 1], &[1, 2, 1, 1, 3, 1, 0, 0, 0, 0]);
        do_test(&[9, 0, 0, 9, 1, 2, 0, 1, 0, 1, 0, 3, 0, 1, 9, 0, 0, 0, 0, 9], &[9, 9, 1, 2, 1, 1, 3, 1, 9, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        do_test(&[0, 0], &[0, 0]);
        do_test(&[0], &[0]);
        do_test(&[], &[]);
    }
}
