// https://www.codewars.com/kata/58c5577d61aefcf3ff000081/train/rust

fn encode_rail_fence_cipher(text: &str, num_rails: usize) -> String {
    let length = text.len();
    let cycle_length = (num_rails - 1) * 2;
    let mut encoded_text = String::new();

    for rail in 0..num_rails {
        for index in (rail..length).step_by(cycle_length) {
            encoded_text.push_str(&text[index..=index]);
            if rail != 0 && rail != num_rails - 1 {
                let index2 = index + cycle_length - 2 * rail;
                if index2 < length {
                    encoded_text.push_str(&text[index2..=index2]);
                }
            }
        }
    }

    return encoded_text;
}

fn decode_rail_fence_cipher(text: &str, num_rails: usize) -> String {
    let length = text.len();
    let half_cycle_len = num_rails - 1;
    let cycle_len = half_cycle_len * 2;
    let half_cycles = length / half_cycle_len;

    let rail_len: Vec<usize> = (0..num_rails).map(|rail| {
        match rail {
            0 => (length + cycle_len) / cycle_len,
            _ if rail == num_rails - 1 => (length + half_cycle_len) / cycle_len,
            _ => {
                let step = match half_cycles % 2 == 0 {
                    true => half_cycle_len - rail - 1,
                    false => half_cycle_len + rail - num_rails - 1,
                };
                (length + step) / half_cycle_len
            }
        }
    }).collect();

    let mut sections = vec![0_usize; num_rails];
    let mut decoded_text = String::new();

    let mut rail_index = 0_i8;
    let mut step = 1_i8;
    while decoded_text.len() < length {
        let index = sections[rail_index as usize];
        decoded_text.push_str(&text[index..=index]);
        sections[rail_index as usize] += 1;
        if rail_index == 0 {
            step = 1;
        }
        if rail_index == (num_rails - 1) as i8 {
            step = -1;
        }
        rail_index += step;
    }
    println!();

    return decoded_text;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(encode_rail_fence_cipher("WEAREDISCOVEREDFLEEATONCE", 3), "WECRLTEERDSOEEFEAOCAIVDEN");
        assert_eq!(decode_rail_fence_cipher("WECRLTEERDSOEEFEAOCAIVDEN", 3), "WEAREDISCOVEREDFLEEATONCE");
        assert_eq!(encode_rail_fence_cipher("Hello, World!", 3), "Hoo!el,Wrdl l");
        assert_eq!(decode_rail_fence_cipher("Hoo!el,Wrdl l", 3), "Hello, World!");
    }
}
