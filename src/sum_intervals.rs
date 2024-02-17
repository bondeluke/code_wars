fn merge(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    let (smaller, larger) = if a.0 <= b.0 { (a, b) } else { (b, a) };
    if larger.1 <= smaller.1 {
        smaller // SLLS
    } else {
        (smaller.0, larger.1) // SLSL
    }
}

fn can_merge(a: (i32, i32), b: (i32, i32)) -> bool {
    let (smaller, larger) = if a.0 <= b.0 { (a, b) } else { (b, a) };
    return smaller.1 >= larger.0;  // i.e. not SSLL
}

fn smash(intervals: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut new_vec = Vec::new();

    for interval in intervals {
        let mut merge_happened = false;
        for i in 0..new_vec.len() {
            if can_merge(new_vec[i], interval) {
                new_vec[i] = merge(new_vec[i], interval);
                merge_happened = true;
                break;
            }
        }
        if !merge_happened {
            new_vec.push(interval);
        }
    }

    new_vec
}

fn sum_intervals(intervals: &[(i32, i32)]) -> i32 {
    let mut combined: Vec<(i32, i32)> = intervals.to_vec();
    let mut length = combined.len();

    loop {
        combined = smash(combined);
        if combined.len() == length {
            break;
        }
        length = combined.len()
    }

    combined.iter()
        .map(|&(left, right)| { right - left })
        .sum()
}

#[cfg(test)]
mod sample_tests {
    use super::*;

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