// Write a function called sum_intervals that accepts an array of intervals,
// and returns the sum of all the interval lengths.
// Overlapping intervals should only be counted once.

fn can_merge(a: &(i32, i32), b: &(i32, i32)) -> bool {
    let (smaller, larger) = if a.0 <= b.0 { (a, b) } else { (b, a) };
    return smaller.1 >= larger.0;  // i.e. not SSLL
}

// merge assumes the parameters a and b can be merged
fn merge(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    let (smaller, larger) = if a.0 <= b.0 { (a, b) } else { (b, a) };
    if larger.1 <= smaller.1 {
        smaller // SLLS
    } else {
        (smaller.0, larger.1) // SLSL
    }
}

fn attempt_merge(intervals: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    intervals.into_iter()
        .fold(Vec::new(), |mut merged, interval| {
            match merged.iter_mut().find(|e| can_merge(*e, &interval)) {
                Some(element) => *element = merge(*element, interval),
                None => merged.push(interval),
            }
            merged
        })
}

pub fn sum_intervals(intervals: &[(i32, i32)]) -> i32 {
    let mut merged = intervals.to_vec();

    while {
        let prev_len = merged.len();
        merged = attempt_merge(merged);
        prev_len != merged.len()
    } {}

    merged.into_iter()
        .map(|(left, right)| { right - left })
        .sum()
}

#[cfg(test)]
mod sample_tests {
    use super::sum_intervals;

    const ERR_MSG: &str = "\nYour result (left) did not match expected output (right).";

    #[test]
    fn non_overlapping_intervals() {
        assert_eq!(sum_intervals(&[(1, 5)]), 4, "{}", ERR_MSG);
        assert_eq!(sum_intervals(&[(1, 5), (6, 10)]), 8, "{}", ERR_MSG);
    }

    #[test]
    fn overlapping_intervals() {
        assert_eq!(sum_intervals(&[(1, 5), (1, 5)]), 4, "{}", ERR_MSG);
        assert_eq!(sum_intervals(&[(1, 4), (7, 10), (3, 5)]), 7, "{}", ERR_MSG);
    }

    #[test]
    fn large_intervals() {
        assert_eq!(sum_intervals(&[(-1_000_000_000, 1_000_000_000)]), 2_000_000_000, "{}", ERR_MSG);
        assert_eq!(sum_intervals(&[(0, 20), (-100_000_000, 10), (30, 40)]), 100_000_030, "{}", ERR_MSG);
    }
}