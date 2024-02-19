use num_bigint::BigInt;

// Cycle lengths of last two digits for any bass mod 10
// 0 | 1 | 5 => { 1 }
// 7 => { 4 }
// 6 => { 5 }
// 4 | 9 => { 10 }
// 2 | 3 | 8 => { 20 }
fn last_2_digits(base: u64, exp: u64) -> u64 {
    if exp == 0 { return 1; }
    if exp == 1 { return base; }
    if base == 0 { return 0; }
    if base == 1 { return 1; }
    if base % 10 == 0 { return 20; }

    let cycle = match base % 10 {
        0 | 1 | 5 => { 1 }
        7 => { 4 }
        6 => { 5 }
        4 | 9 => { 10 }
        2 | 3 | 8 => { 20 }
        _ => panic!("Uh oh!")
    };

    let mut b_m = base % 20;
    let mut e_em = exp % cycle;

    if b_m == 1 {
        b_m += 20;
    }

    if e_em == 0 || e_em == 1 {
        e_em += cycle;
    }

    return (BigInt::from(b_m).pow(e_em as u32) % BigInt::from(100))
        .to_string()
        .parse::<u64>()
        .unwrap();
}

// #[allow(dead_code)]
pub fn last_digit(list: &[u64]) -> u64 {
    if list.is_empty() {
        return 1;
    }
    let mut stack = list.to_vec();
    let mut exp = stack.pop().unwrap();
    let mut base = stack.pop().unwrap();
    let mut last2 = last_2_digits(base, exp);
    while !stack.is_empty() {
        exp = last2;
        base = stack.pop().unwrap();
        last2 = last_2_digits(base, exp);
    }
    last2 % 10
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
        ] {
            do_test(&a, b);
        }
    }
}
