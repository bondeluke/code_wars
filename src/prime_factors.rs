// https://www.codewars.com/kata/54d512e62a5e54c96200019e

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