// https://www.codewars.com/kata/59122604e5bc240817000016

use std::iter::once;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

pub fn stream(basis_size: usize) -> impl Iterator<Item=u32> {
    PrimeIterator::new(basis_size)
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
    sieve: Vec<bool>,
    next_spoke: Arc<Vec<usize>>,
    wheel: Arc<Wheel>,
    primes: Vec<usize>,
    arc_primes: Arc<Vec<usize>>,
    segment: usize,
    cursor: i32,
}

impl PrimeIterator {
    fn new(basis_size: usize) -> Self {
        let wheel = get_wheel(basis_size);
        Self {
            sieve: PrimeIterator::initialize_sieve(&wheel),
            next_spoke: Arc::new(PrimeIterator::initialize_next_spoke(&wheel)),
            wheel: Arc::new(wheel),
            primes: vec![],
            segment: 1,
            cursor: -1,
            arc_primes: Arc::new(vec![]),
        }
    }

    fn initialize_sieve(wheel: &Wheel) -> Vec<bool> {
        let mut sieve = vec![false; wheel.circumference() / 3];
        for &spoke in &wheel.spokes {
            sieve[spoke / 3] = true;
        }
        sieve
    }

    fn initialize_next_spoke(wheel: &Wheel) -> Vec<usize> {
        let circ = wheel.circumference();

        let mut next_spoke = vec![0; circ];
        let mut spoke_index = 0;
        for i in 0..circ {
            let spoke = wheel.spokes[spoke_index];
            next_spoke[i] = spoke_index;
            if i == spoke {
                spoke_index += 1;
            }
        }
        next_spoke
    }

    fn initialize_primes(&mut self) {
        let spokes = &self.wheel.spokes;
        let upper_limit = self.wheel.circumference();
        let basis_size = self.wheel.basis.len();

        //println!("Expanding primes in range 0 - {}...", upper_limit);

        // (0) Start with the basis and the first non-1 spoke
        self.primes.extend(&self.wheel.basis);
        self.primes.push(spokes[1]);

        // (1) Create a sieve to track spoke primality
        let mut sieve = self.sieve.clone();

        // (2) Cross out composites using spoke multiples
        let mut spoke_index = 2;
        for i in basis_size.. {
            let prime = self.primes[i];
            let p_squared = prime * prime;
            if p_squared > upper_limit { break; }

            for &spoke in &spokes[self.next_spoke[prime]..] {
                let n = prime * spoke;
                if n > upper_limit { break; }

                //println!("Crossing out {} x {} = {} at index {}", prime, spoke, n, n / 3);
                sieve[n / 3] = false;
            }

            while spokes[spoke_index] < p_squared {
                if sieve[spokes[spoke_index] / 3] {
                    //println!("Adding {spoke} as a prime");
                    self.primes.push(spokes[spoke_index]);
                }
                spoke_index += 1;
            }
        }

        // (3) Add remaining spokes that have not been crossed out
        for &spoke in &spokes[spoke_index..] {
            if sieve[spoke / 3] {
                //println!("Adding {spoke} as a prime (leftover)");
                self.primes.push(spoke);
            }
        }

        self.arc_primes = Arc::new(self.primes.clone());
    }

    fn extend(
        segment: usize,
        mut sieve: Vec<bool>,
        wheel: Arc<Wheel>,
        primes: Arc<Vec<usize>>,
        next_spoke: Arc<Vec<usize>>,
    ) -> Vec<usize> {
        let spokes = &wheel.spokes;
        let circ = wheel.circumference();
        let lower_limit = segment * circ;
        let upper_limit = (segment + 1) * circ;
        let basis_size = wheel.basis.len();

        //println!("Expanding primes in range {} - {}...", lower_limit, upper_limit);

        for &prime in &primes[basis_size..] {
            let p_squared = prime * prime;
            if p_squared > upper_limit { break; }

            let lowest_factor = lower_limit / prime + 1;
            let spoke_approx = lowest_factor % circ;
            let k = lowest_factor / circ;
            for &spoke in &spokes[next_spoke[spoke_approx]..] {
                let n = prime * (k * circ + spoke);
                if n > upper_limit { break; }

                //println!("Crossing out {prime} x ({k} * {circ} + {spoke}) = {n} at index {}", n - lower_limit);
                sieve[(n - lower_limit) / 3] = false;
            }
        }

        let mut new_primes: Vec<usize> = Vec::with_capacity(spokes.len());

        // (3) Add spokes which have not been crossed out
        for &spoke in spokes {
            if sieve[spoke / 3] {
                //println!("Adding {spoke} as a prime");
                new_primes.push(lower_limit + spoke);
            }
        }

        new_primes
    }
}

impl Iterator for PrimeIterator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.cursor += 1;

        if self.cursor as usize == self.primes.len() {
            if self.cursor == 0 {
                self.initialize_primes();
            } else {
                const THREAD_COUNT: usize = 8;

                let handles = (0..THREAD_COUNT).map(|i| {
                    let segment = self.segment;
                    let sieve = self.sieve.clone();
                    let wheel = Arc::clone(&self.wheel);
                    let primes = Arc::clone(&self.arc_primes);
                    let next_spoke = Arc::clone(&self.next_spoke);
                    thread::spawn(move || {
                        PrimeIterator::extend(segment + i, sieve, wheel, primes, next_spoke)
                    })
                }).collect::<Vec<JoinHandle<Vec<usize>>>>();

                for handle in handles {
                    self.primes.extend(handle.join().unwrap());
                }

                self.segment += THREAD_COUNT;
            }
        }

        Some(self.primes[self.cursor as usize] as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::stream;

    fn test_segment(start: u32, numbers: [u32; 10]) {
        let mut prime_iterator = stream(7);
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
