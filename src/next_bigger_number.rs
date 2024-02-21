// https://www.codewars.com/kata/55983863da40caa2c900004e

#[allow(dead_code)]
fn next_bigger_number(n: u64) -> Option<u64> {
    let mut digits: Vec<u64> = n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect();

    for i in (1..digits.len()).rev() {
        if digits[i - 1] < digits[i] {
            // The max index will be the smallest number greater than the number at index i - i
            let max_index = (i..digits.len())
                .filter(|&j| digits[i - 1] < digits[j])
                .max().unwrap();
            digits.swap(i - 1, max_index);
            digits[i..].sort();
            return Some(digits.iter().fold(0, |acc, &d| acc * 10 + d));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::next_bigger_number;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected result (right).";

    #[test]
    fn sample_tests() {
        // assert_eq!(next_bigger_number(9), None, "{ERR_MSG}");
        assert_eq!(next_bigger_number(12), Some(21), "{ERR_MSG}");
        assert_eq!(next_bigger_number(513), Some(531), "{ERR_MSG}");
        assert_eq!(next_bigger_number(2017), Some(2071), "{ERR_MSG}");
        assert_eq!(next_bigger_number(414), Some(441), "{ERR_MSG}");
        assert_eq!(next_bigger_number(144), Some(414), "{ERR_MSG}");
        assert_eq!(next_bigger_number(9069180804445176296), Some(9069180804445176629), "{ERR_MSG}");
    }
}
