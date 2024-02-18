// https://www.codewars.com/kata/550f22f4d758534c1100025a

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    West,
    South,
}

impl Direction {
    fn opposite(&self, other: &Direction) -> bool {
        match self {
            Direction::North => *other == Direction::South,
            Direction::East => *other == Direction::West,
            Direction::West => *other == Direction::East,
            Direction::South => *other == Direction::North
        }
    }
}

#[allow(dead_code)]
fn dir_reduc(arr: &[Direction]) -> Vec<Direction> {
    let mut stack = vec![];
    for &dir in arr.iter() {
        match stack.is_empty() || !dir.opposite(stack.last().unwrap()) {
            true => { stack.push(dir); }
            false => { stack.pop(); }
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