// https://www.codewars.com/kata/550f22f4d758534c1100025a

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    West,
    South,
}

fn are_opposites(a: Direction, b: Direction) -> bool {
    match a {
        Direction::North => { b == Direction::South }
        Direction::East => { b == Direction::West }
        Direction::West => { b == Direction::East }
        Direction::South => { b == Direction::North }
    }
}

#[allow(dead_code)]
fn dir_reduc(arr: &[Direction]) -> Vec<Direction> {
    let mut stack = vec![];
    for &direction in arr.iter() {
        if stack.is_empty() || !are_opposites(direction, *stack.last().unwrap()) {
            stack.push(direction);
        } else {
            stack.pop();
        }
    }
    stack
}

#[cfg(test)]
mod tests {
    use super::{dir_reduc, Direction::{*}};

    #[test]
    fn basic() {
        let a = [North, South, South, East, West, North, West];
        assert_eq!(dir_reduc(&a), [West]);

        let a = [North, West, South, East];
        assert_eq!(dir_reduc(&a), [North, West, South, East]);
    }
}