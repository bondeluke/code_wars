// https://www.codewars.com/kata/541c8630095125aba6000c00

fn digital_root(n: i64) -> i64 {
    match n.to_string().chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>() {
        r if r < 10 => r as i64,
        r => digital_root(r as i64),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(digital_root(16), 7);
    }
}
