// https://www.codewars.com/kata/57cebe1dc6fdc20c57000ac9

#[allow(dead_code)]
pub fn find_short(s: &str) -> u32 {
    s.split(' ')
        .map(|word| word.len())
        .min()
        .expect("There was no minimum") as u32
}

#[cfg(test)]
mod tests {
    use super::find_short;

    fn do_test(s: &str, expected: u32) {
        let actual = find_short(s);
        assert_eq!(actual, expected, "With s = \"{s}\"\nExpected {expected} but got {actual}")
    }

    #[test]
    fn sample_tests() {
        do_test("bitcoin take over the world maybe who knows perhaps", 3);
        do_test("turns out random test cases are easier than writing out basic ones", 3);
        do_test("lets talk about javascript the best language", 3);
        do_test("i want to travel the world writing code one day", 1);
        do_test("Lets all go on holiday somewhere very cold", 2);
        do_test("Let's travel abroad shall we", 2);
    }
}