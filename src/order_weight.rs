// https://www.codewars.com/kata/55c6126177c9441a570000cc

#[allow(dead_code)]
fn order_weight(s: &str) -> String {
    let mut weights: Vec<(&str, u32)> = s.split_whitespace()
        .map(|x| (x, x
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .sum()))
        .collect();

    weights.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0)));
    weights.iter().map(|x| x.0).collect::<Vec<&str>>().join(" ")
}

fn testing(s: &str, exp: &str) -> () {
    assert_eq!(order_weight(s), exp)
}

#[test]
fn basics_order_weight() {
    testing("103 123 4444 99 2000", "2000 103 123 4444 99");
    testing("2000 10003 1234000 44444444 9999 11 11 22 123",
            "11 11 2000 10003 22 123 1234000 44444444 9999");
}
