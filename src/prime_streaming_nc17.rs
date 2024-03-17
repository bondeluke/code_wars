// https://www.codewars.com/kata/59122604e5bc240817000016

pub fn stream() -> impl Iterator<Item=u32> {
    PrimeIterator::new()
}

struct PrimeIterator {
    index: usize,
    pi: usize,
    primes: Vec<u32>,
}

impl PrimeIterator {
    fn new() -> Self {
        Self {
            index: 0,
            pi: 0,
            primes: vec![2, 3, 5, 7, 11, 13, 17, 19, 23]
        }
    }

    fn expand(&mut self) {
        const RANGE: usize = 8;
        let p1 = self.primes[self.pi];
        let p2 = self.primes[self.pi + RANGE];
        let min = (p1 * p1 + 2) as usize;
        let max = (p2 * p2) as usize;

        let mut sieve = vec![true; max - min + 1];
        for &prime in &self.primes {
            let prime = prime as usize;
            let bot = ((min - 1) / prime + 1) * prime;
            for i in (bot..=max).step_by(prime) {
                sieve[i - min] = false;
            }
        }

        let len = self.primes.len();
        self.primes.extend(
            sieve.iter().enumerate()
                .filter_map(|(i, &is_prime)|
                    match is_prime {
                        true => Some((i + min) as u32),
                        false => None
                    }
                )
        );
        let len2 = self.primes.len();
        self.pi += RANGE;

        println!("{:>5} primes found in range {:>8} - {:<8} ({:>6} numbers checked)", len2 - len, min, max, max - min);
    }
}

impl Iterator for PrimeIterator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.primes.len() {
            self.expand();
        }

        let result = Some(self.primes[self.index]);
        self.index += 1;
        return result;
    }
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
