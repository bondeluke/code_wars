fn last_2_digits(base: &str, exp: &str) -> u64 {
    if exp == "0" { return 1; }

    let exp_mod_100 = match exp.len() {
        1 => exp,
        len => &exp[len - 2..]
    }.parse::<u64>().unwrap();

    let base_last_digit = base[base.len() - 1..].parse::<u64>().unwrap();

    if base_last_digit == 0 {
        return 20;
    }
    if base_last_digit == 1 {
        return if exp_mod_100 == 1 {
            1
        } else { 21 }
    }
    if base_last_digit == 5 {
        return 5;
    }

    let the_mod = match base_last_digit {
        6 => { 5 }
        7 => { 4 }
        4 | 9 => { 10 }
        // 0 | 1 | 5 => { return base_last_digit; }
        2 | 3 | 8 => { 20 }
        _ => panic!("Unexpected.")
    };

    if exp_mod_100 < the_mod {
        return base_last_digit.pow(exp_mod_100 as u32) % 100;
    }

    match exp_mod_100 % the_mod {
        0 => base_last_digit.pow(the_mod as u32) % 100,
        1 => base_last_digit.pow((the_mod + 1) as u32) % 100,
        2 => base_last_digit.pow((the_mod + 2) as u32) % 100,
        3 => base_last_digit.pow((the_mod + 3) as u32) % 100,
        4 => base_last_digit.pow((the_mod + 4) as u32) % 100,
        r => base_last_digit.pow(r as u32) % 100
    }
}

// #[allow(dead_code)]
pub fn last_digit(list: &[u64]) -> u64 {
    if list.is_empty() {
        return 1;
    }
    if list.len() == 1 {
        return list[0];
    }

    let mut l = list.to_vec();

    let mut exp = l.pop().unwrap();
    let mut base = l.pop().unwrap();
    let mut last2 = last_2_digits(&base.to_string(), &exp.to_string());
    while !l.is_empty() {
        exp = last2;
        base = l.pop().unwrap();
        last2 = last_2_digits(&base.to_string(), &exp.to_string());
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
            (vec![499942, 898102, 846073], 6)
        ] {
            do_test(&a, b);
        }
    }
}
