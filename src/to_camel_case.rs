// https://www.codewars.com/kata/517abf86da9663f1d2000003

#[allow(dead_code)]
fn to_camel_case(text: &str) -> String {
    text.split(['-', '_'])
        .enumerate()
        .map(|(i, word)| match i {
            0 => word.to_string(),
            _ => word[..1].to_uppercase() + &word[1..],
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::to_camel_case;

    fn do_test(text: &str, expected: &str) {
        let result = to_camel_case(text);
        assert_eq!(result, expected, "Result \"{result}\" did not match expected output \"{expected}\"")
    }

    #[test]
    fn fixed_tests() {
        do_test("", "");
        do_test("the_stealth_warrior", "theStealthWarrior");
        do_test("The-Stealth-Warrior", "TheStealthWarrior");
        do_test("A-B-C", "ABC");
    }
}