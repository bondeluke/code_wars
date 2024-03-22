// https://www.codewars.com/kata/59122604e5bc240817000016

use std::iter::once;

pub fn stream17() -> impl Iterator<Item=u32> {
    PrimeIterator::new()
}

#[derive(Clone)]
pub struct Wheel {
    pub basis: Vec<u32>,
    pub spokes: Vec<u32>,
}

impl Wheel {
    pub fn circumference(&self) -> u32 {
        self.basis.iter().product()
    }
}

pub fn next_wheel(wheel: Wheel) -> Wheel {
    let p = if wheel.spokes.len() > 1 { wheel.spokes[1] } else { 3 };
    let circ = wheel.circumference();
    let basis = wheel.basis.iter().copied().chain(once(p)).collect::<Vec<u32>>();
    let mut spokes: Vec<u32> = vec![];
    for k in 0..p {
        for s in &wheel.spokes {
            let spoke = k * circ + s;
            if spoke % p != 0 {
                spokes.push(spoke);
            }
        }
    }
    Wheel { basis, spokes }
}

pub fn get_wheel(basis_size: u32) -> Wheel {
    let mut wheel = Wheel { basis: vec![2], spokes: vec![1] };
    for _ in 1..basis_size {
        wheel = next_wheel(wheel);
    }
    wheel
}

struct PrimeIterator {
    cursor: i32,
    segment: usize,
    primes: Vec<u32>,
    wheel: Wheel,
}

impl PrimeIterator {
    fn new() -> Self {
        Self {
            cursor: -1,
            segment: 1,
            primes: vec![],
            wheel: get_wheel(7),
        }
    }

    fn initialize(&mut self) {
        println!("Initializing primes up to {}...", self.wheel.circumference());
        let basis_size = self.wheel.basis.len();
        let spokes_len = self.wheel.spokes.len();
        let p_squared = self.wheel.spokes[1].pow(2);

        // (1) Start with the basis
        self.primes.extend(&self.wheel.basis);

        // (2) Spokes up to p_squared are guaranteed prime
        let mut cutoff_index = 1;
        while cutoff_index < spokes_len && self.wheel.spokes[cutoff_index] < p_squared {
            self.primes.push(self.wheel.spokes[cutoff_index]);
            cutoff_index += 1;
        }

        let mut sieve = vec![true; spokes_len];
        let mut lookup: Vec<Option<usize>> = (0..self.wheel.circumference()).map(|x| None).collect();
        for i in 0..self.wheel.spokes.len() {
            lookup[self.wheel.spokes[i] as usize] = Some(i);
        }

        let circ = self.wheel.circumference();
        for &prime in &self.primes[basis_size..] {
            if prime * prime > circ { break; }

            for &spoke in &self.wheel.spokes[1..] {
                if spoke < prime { continue; }

                let product = prime * spoke;
                if product > circ { break; }

                if let Some(index) = lookup[product as usize] {
                    //println!("crossing out {} x {} = {} at index {}", prime, spoke, product, index);
                    sieve[index] = false;
                } else {
                    //println!("lookup not found {} x {} = {}", prime, spoke, product)
                }
            }
        }

        for i in cutoff_index..spokes_len {
            let spoke = self.wheel.spokes[i];
            if sieve[lookup[spoke as usize].unwrap()] {
                self.primes.push(spoke);
            }
        }

        println!("Initialized {} primes. Last prime added was {}", self.primes.len(), self.primes[self.primes.len() - 1])
    }

    fn extend(&mut self) {}
}

impl Iterator for PrimeIterator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.cursor += 1;

        if self.cursor as usize == self.primes.len() {
            if self.primes.is_empty() {
                self.initialize()
            } else {
                self.extend();
            }
        }

        Some(self.primes[self.cursor as usize])
    }
}

#[cfg(test)]
mod tests {
    use super::stream17;

    fn test_segment(start: u32, numbers: [u32; 10]) {
        let mut prime_iterator = stream17();
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
