fn last_2_digits(base: u64, exp: u64) -> u64 {
    let b_m = base % 10;
    let e_em = exp % 100;
    if exp == 0 { return 1; }

    let the_mod = match b_m {
        1 => { return if e_em == 1 { 1 } else { 21 }; }
        0 => { return if base == 0 { 0 } else { 20 }; }
        5 => { return 5; }
        6 => { 5 }
        7 => { 4 }
        4 | 9 => { 10 }
        2 | 3 | 8 => { 20 }
        _r => panic!("Unexpected. {_r}")
    };

    if e_em < the_mod {
        return b_m.pow(e_em as u32) % 100;
    }

    match e_em % the_mod {
        0 => b_m.pow(the_mod as u32) % 100,
        1 => b_m.pow((the_mod + 1) as u32) % 100,
        2 => b_m.pow((the_mod + 2) as u32) % 100,
        3 => b_m.pow((the_mod + 3) as u32) % 100,
        r => b_m.pow(r as u32) % 100
    }
}

// #[allow(dead_code)]
pub fn last_digit(list: &[u64]) -> u64 {
    if list.is_empty() {
        return 1;
    }

    let mut l = list.to_vec();

    let mut exp = l.pop().unwrap();
    let mut base = l.pop().unwrap();
    let mut last2 = last_2_digits(base, exp);
    while !l.is_empty() {
        exp = last2;
        base = l.pop().unwrap();
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
        ] {
            do_test(&a, b);
        }
    }
}
