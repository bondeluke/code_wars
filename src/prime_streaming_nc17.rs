// https://www.codewars.com/kata/59122604e5bc240817000016

use num::integer::sqrt;

pub fn stream() -> impl Iterator<Item=u32> {
    PrimeIterator::new()
}

struct PrimeIterator {
    cursor: i32,
    pi: usize,
    primes: Vec<u32>,
    range: usize,
}

impl PrimeIterator {
    fn new() -> Self {
        Self {
            cursor: -1,
            pi: 1,
            primes: vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37],
            range: 10,
        }
    }

    fn extend(&mut self) {
        let p1 = self.primes[self.pi];
        let p2 = self.primes[self.pi + self.range];
        let min = (p1 * p1) as usize;
        let max = (p2 * p2) as usize;

        let mut sieve = vec![true; (max - min) / 2 + 1];
        // println!("extending primes: min={}, max={}, sieve.len={}", min, max, sieve.len());
        for &prime in &self.primes[1..] {
            let prime = prime as usize;
            if prime > sqrt(max) { break; }
            let low = ((min - 1) / prime + 1) * prime;
            let odd_low = if low % 2 == 0 { low + prime } else { low };
            // println!("prime={}, low={}, odd_low={}", prime, low, odd_low);
            if odd_low % 2 == 0 { panic!("nope! we expected odd_low to be odd, but it's {odd_low}") }
            for n in (odd_low..=max).step_by(prime * 2) {
                //println!("crossing out {} at index {}", n, (n - min) / 2);
                if n % 2 == 0 { panic!("we do not expect to cross out even numbers like {n}") }
                sieve[(n - min) / 2] = false;
            }
        }

        let len = self.primes.len();
        self.primes.extend(
            sieve.iter().enumerate()
                .filter_map(|(i, &is_prime)|
                    match is_prime {
                        true => Some((i * 2 + min) as u32),
                        false => None
                    }
                )
        );
        let len2 = self.primes.len();
        self.pi += self.range;

        println!("extending:({:>3}, {:>3}) ({:>4}, {:>4}) {:>8} - {:<8} ({:>7}, {:>6}) {:>5} added, {:>7} total", self.pi, self.pi + self.range, p1, p2, min, max, max - min, sieve.len(), len2 - len, self.primes.len());

        if self.range < 50 {
            self.range = 50;
        }
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
