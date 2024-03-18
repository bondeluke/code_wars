// https://www.codewars.com/kata/59122604e5bc240817000016

use std::cmp::{max, min};

pub fn stream() -> impl Iterator<Item=u32> {
    PrimeIterator::new()
}

struct PrimeIterator {
    cursor: i32,
    p1_index: usize,
    primes: Vec<u32>,
    range: usize,
}

impl PrimeIterator {
    fn new() -> Self {
        Self {
            cursor: -1,
            p1_index: 1,
            primes: vec![2, 3, 5, 7],
            range: 8,
        }
    }

    fn extend(&mut self) {
        let p1 = self.primes[self.p1_index];
        let p2_index = min(self.p1_index + self.range, self.primes.len() - 1);
        let p2 = self.primes[p2_index];
        let lb = (p1 * p1) as usize;
        let ub = (p2 * p2) as usize;

        let mut sieve = vec![true; (ub - lb) / 2];
        //println!("extending primes: min={}, max={}, sieve.len={}", lb, ub, sieve.len());
        for &p in &self.primes[1..] {
            if p == p2 { break; }

            let prime = p as usize;
            let low = ((lb - 1) / prime + 1) * prime;
            let odd_low = if low % 2 == 0 { low + prime } else { low };
            let start = max(odd_low, prime * prime);

            //println!("prime={}, low={}, odd_low={}, p^2={}, ({} - {}),", prime, low, odd_low, prime * prime, lb, ub);
            for n in (start..ub).step_by(prime * 2) {
                //println!("crossing out {:>4}={:>2}*{:>2} at index {}", n, prime, n / prime, (n - lb) / 2);
                sieve[(n - lb) / 2] = false;
            }
        }

        self.primes.extend(
            sieve.iter().enumerate()
                .filter_map(|(i, &is_prime)|
                    match is_prime {
                        true => Some((i * 2 + lb) as u32),
                        false => None
                    }
                )
        );

        // println!("extending:({:>3}, {:>3}) ({:>4}, {:>4}) {:>8} - {:<8} ({:>7}, {:>6}) {:>7} primes", self.p1_index, p2_index, p1, p2, lb, ub, ub - lb, sieve.len(), self.primes.len());
        self.p1_index = p2_index;
    }
}

impl Iterator for PrimeIterator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.cursor += 1;

        if self.cursor as usize == self.primes.len() {
            self.extend();
        }

        Some(self.primes[self.cursor as usize])
    }
}

#[cfg(test)]
mod tests {
    use super::stream;

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
