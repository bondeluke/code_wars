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

fn try_reduce(directions: Vec<Direction>) -> Vec<Direction> {
    if directions.len() <= 1 {
        return directions;
    }
    for i in 0..directions.len() - 1 {
        if are_opposites(directions[i], directions[i + 1]) {
            return [&directions[..i], &directions[i + 2..]].concat();
        }
    }
    directions
}

#[allow(dead_code)]
fn dir_reduc(arr: &[Direction]) -> Vec<Direction> {
    let mut directions = arr.to_vec();
    loop {
        let prev_len = directions.len();
        directions = try_reduce(directions);
        if prev_len == directions.len() {
            break;
        }
    }
    directions
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