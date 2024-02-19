// https://www.codewars.com/kata/513e08acc600c94f01000001

#[allow(dead_code)]
fn rgb(r: i32, g: i32, b: i32) -> String {
    format!("{:02X}{:02X}{:02X}", r.clamp(0, 255), g.clamp(0, 255), b.clamp(0, 255))
}

#[cfg(test)]
mod sample_tests {
    use self::super::*;

    #[test]
    fn tests() {
        assert_eq!(rgb(0, 0, 0), "000000");
        assert_eq!(rgb(1, 2, 3), "010203");
        assert_eq!(rgb(255, 255, 255), "FFFFFF");
        assert_eq!(rgb(254, 253, 252), "FEFDFC");
        assert_eq!(rgb(-20, 275, 125), "00FF7D");
    }
}
