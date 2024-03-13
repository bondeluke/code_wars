//https://www.codewars.com/kata/557f6437bf8dcdd135000010/train/rust

#[allow(dead_code)]
pub fn factorial(x: u32) -> String {
    if x == 0 { return String::from("1"); }

    // Pick the largest base such that x * base < u64::MAX
    // to ensure that no products overflow and carry < base
    let x = x as u64;
    let exp = ((u64::MAX / x) as f64).log10() as usize;
    let base = 10_u64.pow(exp as u32);

    let mut result = vec![1];
    for factor in 2..=x {
        let mut carry = 0;

        for part in result.iter_mut() {
            let product = *part * factor + carry;
            *part = product % base;
            carry = product / base;
        }

        if carry > 0 {
            result.push(carry);
        }
    }

    result.iter().rev().enumerate().map(|x| match x {
        (_, 0) => "0".repeat(exp),
        (0, p) => p.to_string(),
        (_, p) => format!("{p:0exp$}")
    }).collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(factorial(0), String::from("1"));
        assert_eq!(factorial(1), String::from("1"));
        assert_eq!(factorial(2), String::from("2"));
        assert_eq!(factorial(3), String::from("6"));
        assert_eq!(factorial(4), String::from("24"));
        assert_eq!(factorial(5), String::from("120"));
        assert_eq!(factorial(9), String::from("362880"));
        assert_eq!(factorial(15), String::from("1307674368000"));
        assert_eq!(
            factorial(100),
            "93326215443944152681699238856266700490715968264381621468592963895217599993229915608941463976156518286253697920827223758251185210916864000000000000000000000000"
        );
    }
}
