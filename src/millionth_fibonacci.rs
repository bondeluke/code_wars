use num::bigint::BigInt;
use num::{One, Zero};

fn fib(n: i32) -> BigInt {
    match n {
        0 | 1 => BigInt::from(n),
        _ if n > 0 => {
            let mut fi = BigInt::zero();
            let mut fi_1 = BigInt::one();
            let mut fi_2 = BigInt::zero();

            for _ in 2..=n {
                fi = fi_1.clone() + fi_2.clone();
                fi_2 = fi_1;
                fi_1 = fi.clone();
            }
            fi
        }
        _ => {
            let mut fi = BigInt::one();
            let mut fi_1 = BigInt::zero();
            let mut fi_2 = BigInt::one();

            for _ in (n..=-1).rev() {
                fi = fi_2.clone() - fi_1.clone();
                fi_2 = fi_1;
                fi_1 = fi.clone();
            }
            fi
        }
    }
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

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
