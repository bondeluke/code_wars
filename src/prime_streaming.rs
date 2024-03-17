// https://www.codewars.com/kata/5519a584a73e70fa570005f5

use itertools::Itertools;

pub fn stream() -> impl Iterator<Item=u32> {
    let n_to_i = |n: u32| -> usize {
        (match n % 6 {
            1 => (n - 1) / 3 - 1,
            5 => (n - 5) / 3,
            _ => panic!("Unexpected input {n}")
        }) as usize
    };

    let limit = 20_000_003;
    let mut sieve = vec![true; n_to_i(limit) + 1];

    [2, 3].into_iter().chain(
        (1_i32..).interleave(1_i32..)
            .zip([-1, 1_i32].into_iter().cycle())
            .enumerate()
            .filter_map(move |(i, (k, c))| {
                let n = (k * 6 + c) as u32;
                let is_prime = sieve[i];

                if is_prime {
                    if n < 65536 {
                        for x in (n..limit / 6)
                            .step_by((n * 2) as usize)
                            .filter(|&x| x % 6 == 1 || x % 6 == 5) {
                            sieve[n_to_i(x)] = false
                        }
                    }
                }

                match is_prime {
                    true => Some(n),
                    false => None
                }
            }))
}


#[cfg(test)]
mod tests {
    use super::*;

    fn test_segment(start: u32, numbers: [u32; 10]) {
        let mut prime_iterator = stream();
        for _ in 0..start {
            prime_iterator.next();
        }
        for i in numbers {
            assert_eq!(Some(i), prime_iterator.next(),
                       "\nYour result (left) did not match the expected output (right)");
        }
    }

    #[test]
    fn tests() {
        println!("testing segment from 0");
        test_segment(0, [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);

        println!("testing segment from 10");
        test_segment(10, [31, 37, 41, 43, 47, 53, 59, 61, 67, 71]);

        println!("testing segment from 100");
        test_segment(100, [547, 557, 563, 569, 571, 577, 587, 593, 599, 601]);

        println!("testing segment from 1,000");
        test_segment(1_000, [7927, 7933, 7937, 7949, 7951, 7963, 7993, 8009, 8011, 8017]);
    }
}
