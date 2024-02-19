// https://www.codewars.com/kata/5518a860a73e708c0a000027

#[allow(dead_code)]
fn last_digit(lst: &[u64]) -> u64 {
    let f = |x, m| std::cmp::min(x % m + m, x);

    lst.into_iter()
        .rev()
        .fold(1u64, |acc, &next| {
            let exp = f(acc, 4);     // Because a^k = a^(4n+k) [mod 10]
            let base = f(next, 20);  // Still not quite sure why base pattern repeats every 20
            base.pow(exp as u32)
        }) % 10
}

#[cfg(test)]
mod tests {
    use super::last_digit;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn do_test(v: &[u64], expected: u64) {
        assert_eq!(last_digit(v), expected, "{ERR_MSG} with list = {v:?}")
    }

    #[test]
    fn fixed_tests() {
        for (a, b) in [
            (vec![], 1),
            (vec![0, 0], 1),
            (vec![0, 1], 0),
            (vec![1, 2], 1),
            (vec![0, 0, 0], 0),
            (vec![1, 2], 1),
            (vec![3, 4, 5], 1),
            (vec![4, 3, 6], 4),
            (vec![7, 6, 21], 1),
            (vec![12, 30, 21], 6),
            (vec![2, 2, 2, 0], 4),
            (vec![2, 2, 101, 2], 6),
            (vec![937640, 767456, 981242], 0),
            (vec![123232, 694022, 140249], 6),
            (vec![499942, 898102, 846073], 6),
            (vec![0, 0, 0, 0, 1, 1, 0, 0, 0, 1], 1),
            (vec![0, 0, 0, 0, 1, 0, 2, 2, 1, 0], 1),
            (vec![2, 2, 1, 1, 2, 1, 1, 0, 2, 0], 4),
            (vec![652072, 185919, 902109, 569251, 333285, 196192, 998532, 761616, 308860, 718276], 8),
            (vec![448253, 700211, 553808, 566110, 732971, 36894, 393888, 334711, 153213, 433768], 3),
        ] {
            do_test(&a, b);
        }
    }
}
