// https://www.codewars.com/kata/5886e082a836a691340000c3

pub fn rectangle_rotation(a: i32, b: i32) -> i32 {
    let a_intercept = a as f64 * 2.0_f64.sqrt() / 2.0;
    let b_intercept = b as f64 * 2.0_f64.sqrt() / 2.0;

    let in_rectangle = |x: f64, y: f64| -> bool {
        (-x - a_intercept < y && y < -x + a_intercept)
            && (x - b_intercept < y && y < x + b_intercept)
    };

    let bound = ((a_intercept + b_intercept) / 2.0).ceil() as i32;

    (-bound..=bound).map(|x| {
        (-bound..=bound).filter(|&y| in_rectangle(x as f64, y as f64)).count()
    }).sum::<usize>() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(rectangle_rotation(6, 4), 23);
        assert_eq!(rectangle_rotation(30, 2), 65);
        assert_eq!(rectangle_rotation(8, 6), 49);
        assert_eq!(rectangle_rotation(16, 20), 333);
    }
}
