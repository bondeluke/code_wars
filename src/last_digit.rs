// https://www.codewars.com/kata/5511b2f550906349a70004e1
//
// For this problem, let's use the fact that the last digits for a given number repeat in a cycle.
// Number  |  Last digits that repeat in cycle
//   0     |  0
//   1     |  1
//   2     |  4, 8, 6, 2
//   3     |  9, 7, 1, 3
//   4     |  6, 4
//   5     |  5
//   6     |  6
//   7     |  9, 3, 1, 7
//   8     |  4, 2, 6, 8
//   9     |  1, 9

#[allow(dead_code)]
fn last_digit(base: &str, exp: &str) -> i32 {
    if exp == "0" { return 1; }

    let exp_mod_100 = match exp.len() {
        1 => exp,
        len => &exp[len - 2..]
    }.parse::<u32>().unwrap();

    let base_last_digit = base[base.len() - 1..].parse::<i32>().unwrap();

    match exp_mod_100 % 4 {
        0 => base_last_digit.pow(4) % 10,
        r => base_last_digit.pow(r) % 10
    }
}

#[test]
fn returns_expected() {
    assert_eq!(last_digit("5", "0"), 1);
    assert_eq!(last_digit("4", "1"), 4);
    assert_eq!(last_digit("4", "2"), 6);
    assert_eq!(last_digit("9", "7"), 9);
    assert_eq!(last_digit("10", "10000000000"), 0);
    assert_eq!(last_digit("1606938044258990275541962092341162602522202993782792835301376", "2037035976334486086268445688409378161051468393665936250636140449354381299763336706183397376"), 6);
    assert_eq!(last_digit("3715290469715693021198967285016729344580685479654510946723", "68819615221552997273737174557165657483427362207517952651"), 7);
    assert_eq!(last_digit("4", "284946833419820080521186068249490821653595131279057042914995121813550469451189995289000"), 6);
}