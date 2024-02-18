// https://www.codewars.com/kata/530e15517bc88ac656000716

fn rotate(c: char, lb: u8, rot: u8) -> char {
    (lb + (c as u8 - lb + rot) % 26) as char
}

#[allow(dead_code)]
fn rot13(message: &str) -> String {
    const ROT: u8 = 13;
    message.chars().map(|c| match c {
        'a'..='z' => rotate(c, b'a', ROT),
        'A'..='Z' => rotate(c, b'A', ROT),
        _ => c
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::rot13;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn do_test(s: &str, expected: &str) {
        assert_eq!(rot13(s), expected, "{ERR_MSG} with message = \"{s}\"")
    }

    #[test]
    fn sample_tests() {
        do_test("test", "grfg");
        do_test("Test", "Grfg");
    }
}