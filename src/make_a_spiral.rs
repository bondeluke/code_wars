// https://www.codewars.com/kata/521c2db8ddc89b9b7a0000c1

#[allow(dead_code)]
fn spiralize(size: usize) -> Vec<Vec<i8>> {
    let mut matrix: Vec<Vec<i8>> = (0..size)
        .map(|_| vec![0i8; size])
        .collect();

    let iterations = (size as f32 / 4.0).ceil() as usize;

    for i in 0..iterations {
        let top = 2 * i;
        let bot = size - 2 * i - 1;
        let right = size - 2 * i - 1;
        let left = 2 * i;

        // top
        for x in left..=right {
            matrix[top][x] = 1;
        }
        // right
        for y in top + 1..=bot {
            matrix[y][right] = 1;
        }
        if (bot - top) <= 1 {
            break;
        }
        // bot
        for x in (left..right).rev() {
            matrix[bot][x] = 1;
        }
        // left
        for y in (top + 2..bot).rev() {
            matrix[y][left] = 1;
        }

        if i != iterations - 1 {
            matrix[top + 2][left + 1] = 1;
        }
    }

    matrix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test5() {
        assert_eq!(
            spiralize(5),
            [
                [1, 1, 1, 1, 1],
                [0, 0, 0, 0, 1],
                [1, 1, 1, 0, 1],
                [1, 0, 0, 0, 1],
                [1, 1, 1, 1, 1],
            ],
        );
    }

    #[test]
    fn test6() {
        assert_eq!(
            spiralize(6),
            [
                [1, 1, 1, 1, 1, 1],
                [0, 0, 0, 0, 0, 1],
                [1, 1, 1, 1, 0, 1],
                [1, 0, 0, 1, 0, 1],
                [1, 0, 0, 0, 0, 1],
                [1, 1, 1, 1, 1, 1]
            ]
        );
    }

    #[test]
    fn test8() {
        assert_eq!(
            spiralize(8),
            [
                [1, 1, 1, 1, 1, 1, 1, 1],
                [0, 0, 0, 0, 0, 0, 0, 1],
                [1, 1, 1, 1, 1, 1, 0, 1],
                [1, 0, 0, 0, 0, 1, 0, 1],
                [1, 0, 1, 0, 0, 1, 0, 1],
                [1, 0, 1, 1, 1, 1, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 1],
                [1, 1, 1, 1, 1, 1, 1, 1],
            ],
        );
    }
}
