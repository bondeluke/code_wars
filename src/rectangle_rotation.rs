// https://www.codewars.com/kata/5886e082a836a691340000c3

use std::cmp::{max, min};

#[allow(dead_code)]
fn rectangle_rotation(a: i32, b: i32) -> i32 {
    let a_intercept = a as f64 * 2.0_f64.sqrt() / 2.0;
    let b_intercept = b as f64 * 2.0_f64.sqrt() / 2.0;

    let in_rect = |x: f64, y: f64| -> bool {
        -x - a_intercept < y && y < -x + a_intercept
            && x - b_intercept < y && y < x + b_intercept
    };

    let x = (a_intercept - b_intercept) / 2.0;
    let (xf, xc) = (x.floor(), x.ceil());
    let y = (x + b_intercept).floor();

    let points: Vec<(i32, i32)> = [
        (xf, y),
        (xc, y),
        (xf, y - 1.0),
        (xc, y - 1.0)
    ]
        .iter()
        .filter_map(|&(x, y)| match in_rect(x, y) {
            true => Some((x as i32, y as i32)),
            false => None
        })
        .collect();

    let (x, y) = points[0];
    let (w, h) = (x + y, max(x, y) - min(x, y));
    let mut count = w * h + (w + 1) * (h + 1);

    if points.len() > 1 && points[0].1 == points[1].1 {
        let (x, y) = points[1];
        count += (max(x, y) - min(x, y) + 1) * 2
    }

    count
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
