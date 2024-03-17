// https://www.codewars.com/kata/5519a584a73e70fa570005f5

fn expand(primes: &mut Vec<u32>, s: &mut usize) {
    *s += 8;
    let p = primes[*s];
    let max = (p * p) as usize;
    let min = (primes.last().unwrap() + 1) as usize;
    println!("Looking for primes in range {} - {}", min, max);
    let mut sieve = vec![true; max - min + 1];
    for &mut prime in primes.iter_mut() {
        let low = ((min - 1) / prime as usize + 1) * prime as usize;
        for j in (low..=max).step_by(prime as usize) {
            sieve[j - min] = false;
        }
    }
    for (i, &is_prime) in sieve.iter().enumerate() {
        if is_prime {
            primes.push((i + min) as u32);
        }
    }
}

pub fn stream() -> impl Iterator<Item=u32> {
    let mut primes: Vec<u32> = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
    let mut s = 0_usize;
    expand(&mut primes, &mut s);

    (0..).map(move |i| {
        if i == primes.len() {
            expand(&mut primes, &mut s);
        }

        primes[i]
    })
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
