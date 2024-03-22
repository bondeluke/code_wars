// https://www.codewars.com/kata/59122604e5bc240817000016

use std::iter::once;

pub fn stream() -> impl Iterator<Item=u32> {
    PrimeIterator::new()
}

#[derive(Clone)]
struct Wheel {
    basis: Vec<u32>,
    spokes: Vec<u32>,
}

impl Wheel {
    fn circumference(&self) -> u32 {
        self.basis.iter().product()
    }
}

fn next_wheel(wheel: Wheel) -> Wheel {
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

fn get_wheel(basis_size: u32) -> Wheel {
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
            wheel: get_wheel(6),
        }
    }

    fn initialize(&mut self) {
        println!("Basis: {:?}", self.wheel.basis);
        println!("Initializing primes up to {}...", self.wheel.circumference());
        let basis_size = self.wheel.basis.len();
        let p_squared = self.wheel.basis[basis_size - 1].pow(2);

        // (1) Start with the basis
        self.primes.extend(&self.wheel.basis);

        // (2) Spokes up to p_squared are guaranteed prime
        let mut i = 1;
        while self.wheel.spokes[i] < p_squared {
            self.primes.push(self.wheel.spokes[i]);
            i += 1;
        }

        // (3) Remaining spokes need to be checked for primality
        'spoke_loop: for &spoke in &self.wheel.spokes[i..] {
            for &prime in &self.primes[basis_size..] {
                if prime * prime > spoke {
                    self.primes.push(spoke);
                    break;
                }
                if spoke % prime == 0 {
                    continue 'spoke_loop;
                }
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
