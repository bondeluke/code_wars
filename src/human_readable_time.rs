// https://www.codewars.com/kata/52685f7382004e774f0001f7/train/rust

struct DivisionResult {
    quotient: u32,
    remainder: u32,
}

fn divide(dividend: u32, divisor: u32) -> DivisionResult {
    let remainder = dividend % divisor;
    let quotient = (dividend - remainder) / divisor;
    DivisionResult {
        remainder,
        quotient,
    }
}

fn make_readable(seconds: u32) -> String {
    let hours = divide(seconds, 60 * 60);
    let minutes = divide(hours.remainder, 60);
    let h = hours.quotient;
    let m = minutes.quotient;
    let s = minutes.remainder;
    format!("{h:02}:{m:02}:{s:02}")
}

#[cfg(test)]
mod tests {
    use super::make_readable;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn do_test(s: u32, expected: &str) {
        assert_eq!(make_readable(s), expected, "{ERR_MSG} with seconds = {s}")
    }

    #[test]
    fn fixed_tests() {
        do_test(0, "00:00:00");
        do_test(59, "00:00:59");
        do_test(60, "00:01:00");
        do_test(3599, "00:59:59");
        do_test(3600, "01:00:00");
        do_test(86399, "23:59:59");
        do_test(86400, "24:00:00");
        do_test(359999, "99:59:59");
    }
}