// https://www.codewars.com/kata/585894545a8a07255e0002f1

fn to_xy(n: i8) -> (i8, i8) {
    (n % 3, n / 3)
}

fn to_n(xy: (i8, i8)) -> i8 {
    xy.1 * 3 + xy.0 % 3
}

fn swap_xy(do_it: bool, xy: (i8, i8)) -> (i8, i8) {
    if do_it {
        (xy.1, xy.0)
    } else {
        xy
    }
}

fn corner_options(state: &[bool; 9], i: i8) -> Vec<i8> {
    let (x, y) = to_xy(i);
    let xs = if x == 0 { 1 } else { -1 };
    let ys = if y == 0 { 1 } else { -1 };
    let n1 = to_n((x + xs, y));
    let n2 = to_n((x + xs * 2, y));
    let n3 = to_n((x, y + ys));
    let n4 = to_n((x, y + ys * 2));
    let n5 = to_n((x + xs, y + ys));
    let n6 = to_n((x + xs * 2, y + ys * 2));
    let n7 = to_n((x + xs * 2, y + ys));
    let n8 = to_n((x + xs, y + ys * 2));
    let mut result = vec![n1, n3, n5, n7, n8];
    if state[n1 as usize] { result.push(n2) }
    if state[n3 as usize] { result.push(n4) }
    if state[n5 as usize] { result.push(n6) }

    result.sort();
    result.into_iter().filter(|&k| !state[k as usize]).collect()
}


fn edge_options(visited: &[bool; 9], i: i8) -> Vec<i8> {
    let (mut x, mut y) = to_xy(i);
    let s = if x == 0 || y == 0 { 1 } else { -1 };
    let swap = y == 1;
    if swap {
        (x, y) = (y, x);
    }
    let n1 = to_n(swap_xy(swap, (x, y + s)));
    let n2 = to_n(swap_xy(swap, (x, y + s * 2)));
    let n3 = to_n(swap_xy(swap, (x - 1, y)));
    let n4 = to_n(swap_xy(swap, (x - 1, y + s)));
    let n5 = to_n(swap_xy(swap, (x - 1, y + s * 2)));
    let n6 = to_n(swap_xy(swap, (x + 1, y)));
    let n7 = to_n(swap_xy(swap, (x + 1, y + s)));
    let n8 = to_n(swap_xy(swap, (x + 1, y + s * 2)));
    let mut result = [n1, n3, n4, n5, n6, n7, n8].to_vec();
    if visited[n1 as usize] {
        result.push(n2)
    }
    result.sort();
    result.into_iter().filter(|&k| !visited[k as usize]).collect()
}

fn center_options(visited: &[bool; 9]) -> Vec<i8> {
    visited.into_iter().enumerate()
        .filter(|&(i, p)| *p)
        .map(|(i, p)| i as i8).
        collect()
}

fn count_patterns(from: char, length: u8) -> u64 {
    let mut visited: [bool; 9] = [false; 9];
    let mut current = (from as usize) - ('A' as usize);
    visited[current] = true;

    while visited.into_iter().filter(|&v| v).count() < length as usize {
        let options = match current {
            0 | 2 | 6 | 8 => corner_options(&visited, current as i8),
            1 | 3 | 5 | 7 => edge_options(&visited, current as i8),
            4 => center_options(&visited),
            _ => panic!("Unexpected current value '{current}'!")
        };
        current = *(options.first().unwrap()) as usize;
        visited[current] = true;
    }

    println!("hasdfdello");
    0
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
