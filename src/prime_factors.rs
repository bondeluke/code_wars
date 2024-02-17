// Given a positive number n > 1 find the prime factor decomposition of n.
// The result will be a string with the following form : "(p1**n1)(p2**n2)...(pk**nk)"
// with the p(i) in increasing order and n(i) empty if n(i) is 1.
// Example: n = 86240 should return "(2**5)(5)(7**2)(11)"

pub fn prime_factors(n: i64) -> String {
    let mut factors: Vec<(u64, u8)> = Vec::new();
    let mut number: u64 = n as u64;
    let sqrt = (n as f64).sqrt() as u64;

    for divisor in 2..=sqrt {
        let mut exp = 0;
        while number % divisor == 0 {
            exp += 1;
            number /= divisor
        }
        if exp > 0 {
            factors.push((divisor, exp))
        }
        if number == 1 { break; }
    }

    // Handle the case where one of the factors is > sqrt
    if number != 1 {
        factors.push((number, 1))
    }

    factors.into_iter()
        .map(|(factor, exp)| {
            match exp {
                1 => format!("({factor})"),
                _ => format!("({factor}**{exp})")
            }
        })
        .collect()
}

fn testing(n: i64, exp: &str) -> () {
    assert_eq!(&prime_factors(n), exp)
}

#[test]
fn basics_prime_factors() {
    testing(7775460, "(2**2)(3**3)(5)(7)(11**2)(17)");
    testing(17 * 17 * 93 * 677, "(3)(17**2)(31)(677)");
    testing(2 * 2 * 3 * 13, "(2**2)(3)(13)");
    testing(7537 * 123863, "(7537)(123863)");
    testing(29 * 29, "(29**2)");
}