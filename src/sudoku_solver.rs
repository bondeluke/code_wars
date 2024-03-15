// https://www.codewars.com/kata/5296bc77afba8baa690002d7
use itertools::iproduct;

#[allow(dead_code)]
fn sudoku(puzzle: &mut [[u8; 9]; 9]) {
    // options[y][x][k] tells us whether value k + 1 can be at position (x, y)
    let mut options = [[[false; 9]; 9]; 9];
    for (y, x, k) in iproduct!(0..9, 0..9, 0..9) {
        options[y][x][k] = puzzle[y][x] == 0 || puzzle[y][x] == ((k + 1) as u8)
    }

    // Calculate cell groups: squares, rows, columns
    let cell_groups = (0..9).map(|a| (0..9).map(|b| ((a % 3) * 3 + b % 3, (a / 3) * 3 + b / 3)).collect())
        .chain((0..9).map(|y| (0..9).map(|x| (x, y)).collect()))
        .chain((0..9).map(|x| (0..9).map(|y| (x, y)).collect()))
        .collect::<Vec<Vec<(usize, usize)>>>();

    while puzzle.iter().flat_map(|r| r.iter()).filter(|&&v| v == 0).count() > 0 {
        // Cross out options by cell group
        for group in cell_groups.as_slice() {
            for &(x, y) in group {
                let cell_value = puzzle[y][x];
                if cell_value != 0 {
                    for &(x2, y2) in group {
                        options[y2][x2][cell_value as usize - 1] = (x, y) == (x2, y2);
                    }
                }
            }
        }

        // Update puzzle[y][x] for which there is only 1 option
        for (x, y) in iproduct!(0..9, 0..9) {
            if let [value] = options[y][x].iter().enumerate()
                .filter_map(|(i, &x)| if x { Some(i) } else { None })
                .collect::<Vec<usize>>().as_slice() {
                puzzle[y][x] = (value + 1) as u8;
            }
        }
    }
}

#[cfg(test)]
mod sample_tests {
    use super::sudoku;

    #[test]
    fn puzzle_1() {
        let mut puzzle = [
            [6, 0, 5, 7, 2, 0, 0, 3, 9],
            [4, 0, 0, 0, 0, 5, 1, 0, 0],
            [0, 2, 0, 1, 0, 0, 0, 0, 4],
            [0, 9, 0, 0, 3, 0, 7, 0, 6],
            [1, 0, 0, 8, 0, 9, 0, 0, 5],
            [2, 0, 4, 0, 5, 0, 0, 8, 0],
            [8, 0, 0, 0, 0, 3, 0, 2, 0],
            [0, 0, 2, 9, 0, 0, 0, 0, 1],
            [3, 5, 0, 0, 6, 7, 4, 0, 8],
        ];
        let solution = [
            [6, 1, 5, 7, 2, 4, 8, 3, 9],
            [4, 8, 7, 3, 9, 5, 1, 6, 2],
            [9, 2, 3, 1, 8, 6, 5, 7, 4],
            [5, 9, 8, 4, 3, 2, 7, 1, 6],
            [1, 3, 6, 8, 7, 9, 2, 4, 5],
            [2, 7, 4, 6, 5, 1, 9, 8, 3],
            [8, 4, 9, 5, 1, 3, 6, 2, 7],
            [7, 6, 2, 9, 4, 8, 3, 5, 1],
            [3, 5, 1, 2, 6, 7, 4, 9, 8],
        ];

        sudoku(&mut puzzle);
        assert_eq!(puzzle, solution, "\nYour solution (left) did not match the correct solution (right)");
    }

    #[test]
    fn puzzle_2() {
        let mut puzzle = [
            [0, 0, 8, 0, 3, 0, 5, 4, 0],
            [3, 0, 0, 4, 0, 7, 9, 0, 0],
            [4, 1, 0, 0, 0, 8, 0, 0, 2],
            [0, 4, 3, 5, 0, 2, 0, 6, 0],
            [5, 0, 0, 0, 0, 0, 0, 0, 8],
            [0, 6, 0, 3, 0, 9, 4, 1, 0],
            [1, 0, 0, 8, 0, 0, 0, 2, 7],
            [0, 0, 5, 6, 0, 3, 0, 0, 4],
            [0, 2, 9, 0, 7, 0, 8, 0, 0],
        ];
        let solution = [
            [9, 7, 8, 2, 3, 1, 5, 4, 6],
            [3, 5, 2, 4, 6, 7, 9, 8, 1],
            [4, 1, 6, 9, 5, 8, 3, 7, 2],
            [8, 4, 3, 5, 1, 2, 7, 6, 9],
            [5, 9, 1, 7, 4, 6, 2, 3, 8],
            [2, 6, 7, 3, 8, 9, 4, 1, 5],
            [1, 3, 4, 8, 9, 5, 6, 2, 7],
            [7, 8, 5, 6, 2, 3, 1, 9, 4],
            [6, 2, 9, 1, 7, 4, 8, 5, 3],
        ];

        sudoku(&mut puzzle);
        assert_eq!(puzzle, solution, "\nYour solution (left) did not match the correct solution (right)");
    }
}
