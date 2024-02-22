// https://www.codewars.com/kata/585894545a8a07255e0002f1

fn to_xy(n: i8) -> (i8, i8) {
    (n % 3, n / 3)
}

fn to_n(x: i8, y: i8) -> i8 {
    y * 3 + x % 3
}

fn filter(visited: &[bool; 9], blocked: Vec<i8>) -> Vec<u8> {
    (0..9_i8)
        .filter_map(|i| match !blocked.contains(&i) && !visited[i as usize] {
            true => Some(i as u8),
            false => None
        })
        .collect()
}

fn corner_options(visited: &[bool; 9], current: u8) -> Vec<u8> {
    let (x, y) = to_xy(current as i8);
    let xs = if x == 0 { 1 } else { -1 };
    let ys = if y == 0 { 1 } else { -1 };

    let blocked: Vec<i8> = [
        (to_n(x + xs, y), to_n(x + xs * 2, y)),
        (to_n(x, y + ys), to_n(x, y + ys * 2)),
        (to_n(x + xs, y + ys), to_n(x + xs * 2, y + ys * 2)),
    ].iter().filter_map(|&(blocker, blocked)| {
        match !visited[blocker as usize] {
            true => Some(blocked),
            false => None,
        }
    }).collect();

    filter(visited, blocked)
}


fn edge_options(visited: &[bool; 9], current: u8) -> Vec<u8> {
    let (x, y) = to_xy(current as i8);
    let s = if x == 0 || y == 0 { 1 } else { -1 };
    let mut blocked: Vec<i8> = Vec::new();

    if x == 1 && !visited[to_n(x, y + s) as usize] {
        blocked.push(to_n(x, y + s * 2));
    }
    if y == 1 && !visited[to_n(x + s, y) as usize] {
        blocked.push(to_n(x + s * 2, y));
    }

    filter(visited, blocked)
}

fn dfs(visited: &mut [bool; 9], current: u8, length: u8) -> u64 {
    visited[current as usize] = true;

    let count = if (visited.iter().filter(|v| **v).count() as u8) == length {
        1
    } else {
        match current {
            0 | 2 | 6 | 8 => corner_options(visited, current),
            1 | 3 | 5 | 7 => edge_options(visited, current),
            4 => filter(visited, Vec::new()),
            _ => panic!("Unexpected current value '{current}'!")
        }.iter()
            .fold(0u64, |sum, &option| {
                sum + dfs(visited, option, length)
            })
    };

    visited[current as usize] = false;
    count
}

fn count_patterns(from: char, length: u8) -> u64 {
    let mut visited: [bool; 9] = [false; 9];
    let current = (from as u8) - ('A' as u8);

    if length == 0 {
        return 0;
    }

    dfs(&mut visited, current, length)
}

#[cfg(test)]
mod tests {
    use super::count_patterns;

    #[test]
    fn basic_tests() {
        assert_eq!(count_patterns('A', 0), 0);
        assert_eq!(count_patterns('A', 10), 0);
        assert_eq!(count_patterns('B', 1), 1);
        assert_eq!(count_patterns('C', 2), 5);
        assert_eq!(count_patterns('D', 3), 37);
        assert_eq!(count_patterns('E', 4), 256);
        assert_eq!(count_patterns('E', 8), 23280);
    }
}
