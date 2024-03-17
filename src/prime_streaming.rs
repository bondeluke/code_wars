// https://www.codewars.com/kata/5519a584a73e70fa570005f5

#[allow(dead_code)]
fn stream() -> impl Iterator<Item=u32> {
    PrimeIterator::new()
}

struct PrimeIterator {
    i: usize,
    pi: usize,
    primes: Vec<u32>,
}

impl PrimeIterator {
    fn new() -> Self {
        Self {
            i: 0,
            pi: 0,
            primes: vec![2, 3, 5, 7, 11, 13, 17, 19, 23],
        }
    }

    fn expand(&mut self) {
        const RANGE : usize = 8;
        let p1 = self.primes[self.pi];
        let p2 = self.primes[self.pi + RANGE];
        let min = (p1 * p1 + 2) as usize;
        let max = (p2 * p2) as usize;
        // println!("Looking for primes in range {} - {}", min, max);
        let mut sieve = vec![true; max - min + 1];
        for &prime in &self.primes {
            let low = ((min - 1) / prime as usize + 1) * prime as usize;
            for j in (low..=max).step_by(prime as usize) {
                sieve[j - min] = false;
            }
        }
        for (i, &is_prime) in sieve.iter().enumerate() {
            if is_prime {
                self.primes.push((i + min) as u32);
            }
        }
        self.pi += RANGE;
    }
}

impl Iterator for PrimeIterator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i == self.primes.len() {
            self.expand();
        }

        let result = Some(self.primes[self.i]);
        self.i += 1;
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
