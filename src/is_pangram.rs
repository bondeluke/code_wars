// A pangram is a sentence that contains every single letter of the alphabet at least once.
// For example, the sentence "The quick brown fox jumps over the lazy dog" is a pangram,
// because it uses the letters A-Z at least once (case is irrelevant).
//
// Given a string, detect whether it is a pangram. Return True if it is, False if not.
// Ignore numbers and punctuation.

fn is_pangram(s: &str) -> bool {
    let word = s.to_lowercase();
    ('a'..='z').all(|c| word.contains(c))
}

#[cfg(test)]
mod tests {
    use super::is_pangram;

    fn do_test(s: &str, expected: bool) {
        let actual = is_pangram(s);
        assert_eq!(actual, expected, "Test failed with s = \"{s}\"\nExpected {expected} but got {actual}")
    }

    #[test]
    fn sample_tests() {
        do_test("The quick, brown fox jumps over the lazy dog!", true);
        do_test("Cwm fjord bank glyphs vext quiz", true);
        do_test("Pack my box with five dozen liquor jugs.", true);
        do_test("How quickly daft jumping zebras vex.", true);
        do_test("ABCD45EFGH,IJK,LMNOPQR56STUVW3XYZ", true);
        do_test("This isn't a pangram!", false);
        do_test("abcdefghijklmopqrstuvwxyz", false);
        do_test("Aacdefghijklmnopqrstuvwxyz", false);
    }
}