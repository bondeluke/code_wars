// https://www.codewars.com/kata/54da5a58ea159efa38000836

use std::collections::HashMap;

pub fn find_odd(arr: &[i32]) -> i32 {
    let mut histogram : HashMap<i32, usize> = HashMap::new();
    for value in arr {
        *histogram.entry(*value).or_insert(0) += 1;
    }
    *histogram.iter()
        .filter(|&(_key, value)| value % 2 == 1)
        .next()
        .expect("I was promised one integer to appear an odd number of times.")
        .0
}

#[cfg(test)]
mod tests {
    use super::find_odd;

    #[test]
    fn basic_tests() {
        assert_eq!(find_odd(&vec![20,1,-1,2,-2,3,3,5,5,1,2,4,20,4,-1,-2,5]), 5);
        assert_eq!(find_odd(&vec![1,1,2,-2,5,2,4,4,-1,-2,5]), -1);
        assert_eq!(find_odd(&vec![20,1,1,2,2,3,3,5,5,4,20,4,5]), 5);
        assert_eq!(find_odd(&vec![10]), 10);
        assert_eq!(find_odd(&vec![1,1,1,1,1,1,10,1,1,1,1]), 10);
        assert_eq!(find_odd(&vec![5,4,3,2,1,5,4,3,2,10,10]), 1);
    }
}