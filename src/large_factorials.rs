//https://www.codewars.com/kata/557f6437bf8dcdd135000010/train/rust

fn get_base(x: u64) -> (u64, usize) {
    let limit: u64 = 18446744073709551615 / (x + 1);

    let mut base = 10_u64;
    let mut exp = 1;

    while base * 10 < limit {
        base *= 10;
        exp += 1
    }

    (base, exp)
}

#[allow(dead_code)]
fn factorial(x: u32) -> String {
    let x = x as u64;
    let (base, exp) = get_base(x);
    let mut result = vec![1u64];

    for factor in 2..=x {
        for part in result.iter_mut() {
            *part *= factor;
        }
        for i in 0..result.len() {
            while result[i] >= base {
                result[i] -= base;
                if i == result.len() - 1 {
                    result.push(0)
                }
                result[i + 1] += 1;
            }
        }
    }

    result.iter().rev().enumerate().map(|(i, &p)| match p {
        0 => "0".repeat(exp),
        _ => if i == 0 { p.to_string() } else { format!("{p:0exp$}") }
    }).collect::<Vec<String>>().join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(factorial(1), String::from("1"));
        assert_eq!(factorial(5), String::from("120"));
        assert_eq!(factorial(9), String::from("362880"));
        assert_eq!(factorial(15), String::from("1307674368000"));
        assert_eq!(
            factorial(100),
            "93326215443944152681699238856266700490715968264381621468592963895217599993229915608941463976156518286253697920827223758251185210916864000000000000000000000000"
        );
    }
}
