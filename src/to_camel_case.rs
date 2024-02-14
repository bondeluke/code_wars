fn to_camel_case(text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();
    let mut camel_case: Vec<char> = Vec::new();
    let mut i = 0;
    while i < text.len() {
        let char_to_push = if chars[i] == '_' || chars[i] == '-' {
            i = i + 1;
            chars[i].to_ascii_uppercase()
        } else {
            chars[i]
        };
        camel_case.push(char_to_push);
        i = i + 1
    }
    camel_case.into_iter().collect()
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