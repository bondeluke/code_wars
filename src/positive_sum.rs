// https://www.codewars.com/kata/5715eaedb436cf5606000381/train/rust

fn positive_sum(slice: &[i32]) -> i32 {
    slice.iter()
        .filter(|n| n.is_positive())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn some_examples() {
        assert_eq!(positive_sum(&[1,2,3,4,5]), 15);
        assert_eq!(positive_sum(&[1,-2,3,4,5]), 13);
        assert_eq!(positive_sum(&[-1,2,3,4,-5]), 9);
    }

    #[test]
    fn empty_list() {
        assert_eq!(positive_sum(&[]), 0);
    }

    #[test]
    fn all_negative() {
        assert_eq!(positive_sum(&[-1,-2,-3,-4,-5]), 0);
    }
}