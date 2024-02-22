// https://www.codewars.com/kata/585894545a8a07255e0002f1

fn to_xy(n: i8) -> (i8, i8) {
    (n % 3, n / 3)
}

fn to_n(x: i8, y: i8) -> i8 {
    y * 3 + x % 3
}

fn corner_options(visited: &[bool; 9], current: i8) -> Vec<u8> {
    let (x, y) = to_xy(current);
    let xs = if x == 0 { 1 } else { -1 };
    let ys = if y == 0 { 1 } else { -1 };
    let n1 = to_n(x + xs, y);
    let n2 = to_n(x + xs * 2, y);
    let n3 = to_n(x, y + ys);
    let n4 = to_n(x, y + ys * 2);
    let n5 = to_n(x + xs, y + ys);
    let n6 = to_n(x + xs * 2, y + ys * 2);
    let n7 = to_n(x + xs * 2, y + ys);
    let n8 = to_n(x + xs, y + ys * 2);
    let mut result = vec![n1, n3, n5, n7, n8];
    if visited[n1 as usize] { result.push(n2) }
    if visited[n3 as usize] { result.push(n4) }
    if visited[n5 as usize] { result.push(n6) }

    result.sort();
    result.iter()
        .filter(|&&i| !visited[i as usize])
        .map(|&i| i as u8)
        .collect()
}


fn edge_options(visited: &[bool; 9], current: i8) -> Vec<u8> {
    let (x, y) = to_xy(current);
    let s = if x == 0 || y == 0 { 1 } else { -1 };
    let mut result: Vec<i8> = (0..9_i8).collect();

    if x == 1 && !visited[to_n(x, y + s) as usize] {
        let remove_me = to_n(x, y + s * 2);
        result.retain(|&k| k != remove_me);
    }
    if y == 1 && !visited[to_n(x + s, y) as usize] {
        let remove_me = to_n(x + s * 2, y);
        result.retain(|&k| k != remove_me);
    }

    result.iter()
        .filter(|&&i| !visited[i as usize])
        .map(|&i| i as u8)
        .collect()
}

fn center_options(visited: &[bool; 9]) -> Vec<u8> {
    (0..9_i8)
        .filter(|&i| !visited[i as usize])
        .map(|i| i as u8)
        .collect()
}

fn dfs(visited: &mut [bool; 9], current: u8, length: u8, count: &mut u64) {
    visited[current as usize] = true;

    if (visited.into_iter().filter(|v| **v).count() as u8) == length {
        *count += 1
    } else {
        let options = match current {
            0 | 2 | 6 | 8 => corner_options(&visited, current as i8),
            1 | 3 | 5 | 7 => edge_options(&visited, current as i8),
            4 => center_options(&visited),
            _ => panic!("Unexpected current value '{current}'!")
        };
        for option in options {
            dfs(visited, option, length, count)
        }
    }

    visited[current as usize] = false;
}

fn count_patterns(from: char, length: u8) -> u64 {
    let mut visited: [bool; 9] = [false; 9];
    let mut count: u64 = 0;
    let current = (from as u8) - ('A' as u8);

    if length == 0 || length >= 10 {
        return 0;
    }

    dfs(&mut visited, current, length, &mut count);

    count
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
