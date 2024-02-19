use num_bigint::BigInt;
use num_traits::{Zero};

type Matrix = [[BigInt; 2]; 2];

fn parse(m: [[i32; 2]; 2]) -> Matrix {
    [
        [BigInt::from(m[0][0]), BigInt::from(m[0][1])],
        [BigInt::from(m[1][0]), BigInt::from(m[1][1])]
    ]
}

fn multiply(a: &Matrix, b: &Matrix) -> Matrix {
    let mut result = parse([[0, 0], [0, 0]]);
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                result[i][j] += &a[i][k] * &b[k][j];
            }
        }
    }
    result
}

// Compute the power of a matrix
fn matrix_power(mut m: Matrix, mut exp: i32) -> Matrix {
    let mut result = parse([[1, 0], [1, 0]]);
    while exp > 0 {
        if exp % 2 == 1 {
            result = multiply(&result, &m);
        }
        m = multiply(&m, &m);
        exp /= 2;
    }
    result
}

// Compute the n-th Fibonacci number using matrix exponentiation
// https://rosettacode.org/wiki/Fibonacci_matrix-exponentiation
pub fn fib(n: i32) -> BigInt {
    if n == 0 { return BigInt::zero(); }

    let result = match n > 0 {
        true => { matrix_power(parse([[1, 1], [1, 0]]), n) }
        false => { matrix_power(parse([[0, 1], [1, -1]]), -n) }
    };

    result[0][1].clone()
}

#[cfg(test)]
mod sample_tests {
    use super::fib;
    use num::bigint::BigInt;
    use num::traits::{Zero, One};
    use std::str::FromStr;

    fn do_test(n: i32, expected: BigInt) {
        let actual = fib(n);
        assert_eq!(actual, expected, "Test failed with n = {n}\nExpected \"{expected:?}\"\nBut got \"{actual:?}\"")
    }

    #[test]
    fn small_positive_numbers() {
        do_test(0, BigInt::zero());
        do_test(1, BigInt::one());
        do_test(2, BigInt::one());
        do_test(3, BigInt::from(2));
        do_test(4, BigInt::from(3));
        do_test(5, BigInt::from(5));
    }

    #[test]
    fn small_negative_numbers() {
        do_test(-1, BigInt::from(1));
        do_test(-6, BigInt::from(-8));
        do_test(-96, BigInt::from_str("-51680708854858323072").unwrap());
    }

    #[test]
    fn large_numbers() {
        do_test(
            -500,
            BigInt::from_str("-139423224561697880139724382870407283950070256587697307264108962948325571622863290691557658876222521294125")
                .unwrap(),
        );

        do_test(
            1000,
            BigInt::from_str("43466557686937456435688527675040625802564660517371780402481729089536555417949051890403879840079255169295922593080322634775209689623239873322471161642996440906533187938298969649928516003704476137795166849228875")
                .unwrap(),
        );
    }
}
