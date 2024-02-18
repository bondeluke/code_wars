// https://www.codewars.com/kata/521c2db8ddc89b9b7a0000c1

#[allow(dead_code)]
fn snail(matrix: &[Vec<i32>]) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    if matrix.is_empty() || matrix[0].is_empty() {
        return result;
    }

    let n = matrix.len();
    let iterations = (n as f32 / 2.0).ceil() as usize;

    for i in 0..iterations {
        let top = i;
        let bot = n - i - 1;
        let right = n - i - 1;
        let left = i;

        // top
        for x in left..=right {
            result.push(matrix[top][x])
        }
        // right
        for y in top + 1..=bot {
            result.push(matrix[y][right])
        }
        // bot
        for x in (left..right).rev() {
            result.push(matrix[bot][x])
        }
        // left
        for y in (top + 1..bot).rev() {
            result.push(matrix[y][left])
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test1() {
        let square = &[
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        let expected = vec![1, 2, 3, 6, 9, 8, 7, 4, 5];
        assert_eq!(snail(square), expected);
    }

    #[test]
    fn sample_test2() {
        let square = &[
            vec![1, 2, 3],
            vec![8, 9, 4],
            vec![7, 6, 5],
        ];
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(snail(square), expected);
    }

    #[test]
    fn sample_test3() {
        let square: &[Vec<i32>; 1] = &[Vec::new()];
        let expected = Vec::new();
        assert_eq!(snail(square), expected, "Failed with empty input");
    }

    #[test]
    fn sample_test4() {
        let square = &[vec![1]];
        let expected = vec![1];
        assert_eq!(snail(square), expected);
    }
}
