// https://www.codewars.com/kata/54d496788776e49e6b00052f/train/rust

fn sum_of_divided(l: Vec<i64>) -> Vec<(i64, i64)> {
    let mut result: Vec<(i64, i64)> = Vec::new();
    if l.is_empty() {
        return result;
    }

    let largest = l.iter()
        .max_by_key(|x| x.abs()).unwrap()
        .abs() as usize + 1;

    let mut is_prime = vec![true; largest];

    for p in 2..largest {
        if !is_prime[p] { continue; }

        for x in (p..largest).step_by(p) {
            is_prime[x] = false;
        }

        let mut sum = 0;
        let mut flip = false;
        for n in &l {
            if n.abs() % (p as i64) == 0 {
                flip = true;
                sum += n
            }
        }
        if flip {
            result.push((p as i64, sum))
        }
    }

    result
}

fn testing(l: Vec<i64>, exp: Vec<(i64, i64)>) -> () {
    assert_eq!(sum_of_divided(l), exp)
}

#[test]
fn basics_sum_of_divided() {
    testing(vec![12, 15], vec![(2, 12), (3, 27), (5, 15)]);
    testing(vec![15, 21, 24, 30, 45], vec![(2, 54), (3, 135), (5, 90), (7, 21)]);
}