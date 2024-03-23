// https://www.codewars.com/kata/59122604e5bc240817000016

use std::cmp::max;
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
    primes: Vec<u32>,
    wheel: Wheel,
    segment: usize,
}

impl PrimeIterator {
    fn new() -> Self {
        Self {
            cursor: -1,
            primes: vec![],
            wheel: get_wheel(7),
            segment: 1,
        }
    }

    fn initialize(&mut self) {
        let spokes = &self.wheel.spokes;
        let limit = self.wheel.circumference();
        let basis_size = self.wheel.basis.len();
        let spokes_len = spokes.len();

        println!("Initializing primes up to {}...", self.wheel.circumference());

        // (1) Start with the basis and the first non-1 spoke
        self.primes.extend(&self.wheel.basis);
        self.primes.push(spokes[1]);

        // (2) Create a sieve to track spoke primality
        let mut sieve: Vec<bool> = vec![false; (self.wheel.circumference() / 3) as usize];
        for &spoke in &self.wheel.spokes {
            sieve[(spoke / 3) as usize] = true;
        }

        // (3) Cross out composites using spoke multiples
        let mut j_lb = 1;
        let mut spoke_index = 2;
        for i in basis_size.. {
            let prime = self.primes[i];
            let p_squared = prime * prime;
            if p_squared > limit { break; }

            for j in j_lb..spokes_len {
                let spoke = spokes[j];
                if spoke < prime {
                    j_lb = j;
                    continue;
                }

                let product = prime * spoke;
                if product > limit { break; }

                sieve[(product / 3) as usize] = false;
                //println!("Crossing out {} x {} = {} at index {}", prime, spoke, product, product / 3);
            }

            while spokes[spoke_index] < p_squared {
                if sieve[(spokes[spoke_index] / 3) as usize] {
                    //println!("Adding {spoke} as a prime");
                    self.primes.push(spokes[spoke_index]);
                }
                spoke_index += 1;
            }

            j_lb += 1;
        }

        // (4) Add remaining spokes that have not been crossed out
        for &spoke in &spokes[spoke_index..spokes_len] {
            if sieve[(spoke / 3) as usize] {
                //println!("Adding {spoke} as a prime (leftover)");
                self.primes.push(spoke);
            }
        }

        //println!("Initialized {} primes. Last prime added was {}", self.primes.len(), self.primes[self.primes.len() - 1])
    }

    fn extend(&mut self) {
        let spokes = &self.wheel.spokes;
        let circ = self.wheel.circumference() as usize;
        let lower_limit = self.segment * circ;
        let upper_limit = (self.segment + 1) * circ;
        let basis_size = self.wheel.basis.len();

        //println!("Expanding primes in range {} - {}...", lower_limit, upper_limit);

        // (2) Create a sieve to track spoke primality
        let mut sieve: Vec<bool> = vec![false; circ / 2];
        for &spoke in &self.wheel.spokes {
            sieve[(spoke / 2) as usize] = true;
        }

        // (3) Cross out composites using spoke multiples
        for i in basis_size.. {
            let prime = self.primes[i] as usize;
            let p_squared = prime * prime;
            if p_squared > upper_limit { break; }

            let low = ((lower_limit - 1) / prime + 1) * prime;
            let odd_low = if low % 2 == 0 { low + prime } else { low };
            let start = max(odd_low, p_squared);
            for n in (start..upper_limit).step_by(prime * 2) {
                // println!("Crossing out {} x {} = {} at index {}", prime, n / prime, n, n - lower_limit);
                sieve[(n - lower_limit) / 2] = false;
            }
        }

        // (4) Add remaining spokes that have not been crossed out
        for &spoke in spokes {
            if sieve[(spoke / 2) as usize] {
                //println!("Adding {spoke} as a prime (leftover)");
                self.primes.push((lower_limit as u32) + spoke);
            }
        }
        self.segment += 1;
    }
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
