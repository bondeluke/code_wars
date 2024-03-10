// https://www.codewars.com/kata/58c5577d61aefcf3ff000081/train/rust

fn encode_rail_fence_cipher(text: &str, num_rails: usize) -> String {
    let length = text.len();
    let cycle_length = (num_rails - 1) * 2;
    let mut encoded_text = String::new();

    for rail in 0..num_rails {
        for index in (rail..length).step_by(cycle_length) {
            encoded_text.push_str(&text[index..=index]);
            if rail != 0 && rail != num_rails - 1 {
                let index2 = index + (cycle_length - 2 * rail);
                if index2 < length {
                    encoded_text.push_str(&text[index2..=index2]);
                }
            }
        }
    }

    return encoded_text;
}

// fn decode_rail_fence_cipher(text: &str, num_rails: usize) -> String {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(encode_rail_fence_cipher("WEAREDISCOVEREDFLEEATONCE", 3), "WECRLTEERDSOEEFEAOCAIVDEN");
        // assert_eq!(decode_rail_fence_cipher("WECRLTEERDSOEEFEAOCAIVDEN", 3), "WEAREDISCOVEREDFLEEATONCE");
        assert_eq!(encode_rail_fence_cipher("Hello, World!", 3), "Hoo!el,Wrdl l");
        // assert_eq!(decode_rail_fence_cipher("Hoo!el,Wrdl l", 3), "Hello, World!");
    }
}
