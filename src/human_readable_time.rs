// https://www.codewars.com/kata/52685f7382004e774f0001f7/train/rust

#[allow(dead_code)]
fn make_readable(seconds_total: u32) -> String {
    let s = seconds_total % 60;
    let m_s = (seconds_total - s) % 3600;
    let h_s = seconds_total - s - m_s;
    format!("{:02}:{:02}:{:02}", h_s / 3600, m_s / 60, s)
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