// https://www.codewars.com/kata/59122604e5bc240817000016

use std::cmp::{max};
use std::iter::once;

pub fn stream17() -> impl Iterator<Item=u32> {
    PrimeIterator::new()
}

#[derive(Clone)]
struct Wheel {
    pub basis: Vec<usize>,
    pub spokes: Vec<usize>,
}

impl Wheel {
    pub fn circumference(&self) -> usize {
        self.basis.iter().product()
    }
}

fn next_wheel(wheel: Wheel) -> Wheel {
    let p = if wheel.spokes.len() > 1 { wheel.spokes[1] } else { 3 };
    let circ = wheel.circumference();
    let basis = wheel.basis.iter().copied().chain(once(p)).collect::<Vec<usize>>();
    let mut spokes: Vec<usize> = vec![];
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

fn get_wheel(basis_size: usize) -> Wheel {
    let mut wheel = Wheel { basis: vec![2], spokes: vec![1] };
    for _ in 1..basis_size {
        wheel = next_wheel(wheel);
    }
    wheel
}

struct PrimeIterator {
    cursor: i32,
    primes: Vec<usize>,
    wheel: Wheel,
    segment: usize,
    next_spoke: Vec<usize>,
    sieve: Vec<bool>,
}

impl PrimeIterator {
    fn new() -> Self {
        Self {
            wheel: get_wheel(4),
            sieve: vec![],
            next_spoke: vec![],
            primes: vec![],
            segment: 1,
            cursor: -1,
        }
    }

    fn initialize_caches(&mut self) {
        let circ = self.wheel.circumference();

        // Next spoke lookup
        self.next_spoke = vec![0; circ];
        let mut spoke_index = 0;
        for i in 0..circ {
            let spoke = self.wheel.spokes[spoke_index];
            self.next_spoke[i] = spoke_index;
            if i == spoke {
                spoke_index += 1;
            }
        }

        // Copyable sieve
        self.sieve = vec![false; circ / 3];
        for &spoke in &self.wheel.spokes {
            self.sieve[spoke / 3] = true;
        }
    }

    fn initialize_primes(&mut self) {
        let spokes = &self.wheel.spokes;
        let circ = self.wheel.circumference();
        let limit = self.wheel.circumference();
        let basis_size = self.wheel.basis.len();
        let spokes_len = spokes.len();

        println!("Initializing primes up to {}...", circ);

        // (1) Start with the basis and the first non-1 spoke
        self.primes.extend(&self.wheel.basis);
        self.primes.push(spokes[1]);

        // (2) Create a sieve to track spoke primality
        let mut sieve = self.sieve.clone();

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

                sieve[product / 3] = false;
                //println!("Crossing out {} x {} = {} at index {}", prime, spoke, product, product / 3);
            }

            while spokes[spoke_index] < p_squared {
                if sieve[spokes[spoke_index] / 3] {
                    //println!("Adding {spoke} as a prime");
                    self.primes.push(spokes[spoke_index]);
                }
                spoke_index += 1;
            }

            j_lb += 1;
        }

        // (4) Add remaining spokes that have not been crossed out
        for &spoke in &spokes[spoke_index..spokes_len] {
            if sieve[spoke / 3] {
                //println!("Adding {spoke} as a prime (leftover)");
                self.primes.push(spoke);
            }
        }

        println!("Initialized {} primes. Last prime added was {}", self.primes.len(), self.primes[self.primes.len() - 1])
    }

    fn extend(&mut self) {
        let spokes = &self.wheel.spokes;
        let circ = self.wheel.circumference();
        let lower_limit = self.segment * circ;
        let upper_limit = (self.segment + 1) * circ;
        let basis_size = self.wheel.basis.len();

        println!("Expanding primes in range {} - {}...", lower_limit, upper_limit);

        // (1) Create a sieve to track spoke primality
        let mut sieve = self.sieve.clone();

        // (2) Cross out composites using spoke multiples
        for &prime in &self.primes[basis_size..] {
            let p_squared = prime * prime;
            if p_squared > upper_limit { break; }

            // prime * (k * circ + s_prime) > lower_limit
            let lowest_factor = max((lower_limit - 1) / prime + 1, prime);
            let s_prime = lowest_factor % circ;
            let mut k = lowest_factor / circ;
            let mut spoke_index = self.next_spoke[s_prime];
            loop {
                let spoke = spokes[spoke_index];
                let n = prime * (k * circ + spoke);
                if n > upper_limit { break; }

                println!("Crossing out {prime} x ({k} * {circ} + {spoke}) = {n} at index {}", n - lower_limit);
                sieve[(n - lower_limit) / 3] = false;
                if spoke_index < spokes.len() - 1 {
                    spoke_index += 1
                } else {
                    println!("This actually happened!");
                    k += 1;
                    spoke_index = 0;
                }
            }
        }

        // (3) Add primes for the spokes which have not been crossed out
        for &spoke in spokes {
            if sieve[spoke / 3] {
                //println!("Adding {spoke} as a prime");
                self.primes.push(lower_limit + spoke);
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
                self.initialize_caches();
                self.initialize_primes();
            } else {
                self.extend();
            }
        }

        Some(self.primes[self.cursor as usize] as u32)
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
