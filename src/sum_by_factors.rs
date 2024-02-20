// https://www.codewars.com/kata/54d496788776e49e6b00052f/train/rust

fn prime_factors(mut n: u64) -> Vec<i64> {
    let mut pf: Vec<i64> = Vec::new();
    let mut d = 2;
    while n > 1 {
        if n % d == 0 { pf.push(d as i64); }
        while n % d == 0 { n /= d; }
        d += 1;
    }
    pf
}

fn sum_of_divided(l: Vec<i64>) -> Vec<(i64, i64)> {
    let mut primes: Vec<i64> = l.iter()
        .flat_map(|&n| prime_factors(n.abs() as u64))
        .collect();

    primes.sort();
    primes.dedup();

    primes.iter()
        .map(|&p| (p, l.iter().filter(|&n| n % p == 0).sum()))
        .collect()
}

fn testing(l: Vec<i64>, exp: Vec<(i64, i64)>) -> () {
    assert_eq!(sum_of_divided(l), exp)
}

#[test]
fn basics_sum_of_divided() {
    testing(vec![12, 15], vec![(2, 12), (3, 27), (5, 15)]);
    testing(vec![15, 21, 24, 30, 45], vec![(2, 54), (3, 135), (5, 90), (7, 21)]);
}