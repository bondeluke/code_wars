// https://www.codewars.com/kata/53d40c1e2f13e331fc000c26

use num::bigint::BigInt;
use num::traits::{Zero};

type Matrix = [[BigInt; 2]; 2];

// Maybe turn this into a macro once I understand how to write them
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

// Compute the power of a matrix using binary exponentiation algorithm
fn matrix_power(mut m: Matrix, mut exp: u32) -> Matrix {
    let mut result = parse([[1, 0], [0, 1]]);
    loop {
        if exp & 1 != 0 {
            result = multiply(&result, &m)
        }
        exp >>= 1;
        if exp == 0 {
            break;
        }
        m = multiply(&m, &m);
    }
    result
}

// Compute the n-th Fibonacci number using matrix exponentiation
// https://rosettacode.org/wiki/Fibonacci_matrix-exponentiation
#[allow(dead_code)]
fn fib(n: i32) -> BigInt {
    if n == 0 { return BigInt::zero(); }

    let result = match n > 0 {
        true => { matrix_power(parse([[1, 1], [1, 0]]), n as u32) }
        false => { matrix_power(parse([[0, 1], [1, -1]]), -n as u32) }
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
