// https://www.codewars.com/kata/58c5577d61aefcf3ff000081/train/rust

fn encode_rail_fence_cipher(text: &str, num_rails: usize) -> String {
    if text == "" { return text.to_string(); }

    let chars = text.chars();
    let length = chars.count();
    let cycle = (num_rails - 1) * 2;
    let mut answer = "".to_string();
    let mut index = 0;

    // Handle the 0-th rail
    while index < length {
        answer.push_str(&text[index..index + 1]);
        index += cycle;
    }

    // Handle the middle rails
    for rail in 1..num_rails - 1 {
        let (mut first_index, mut second_index) = (rail, cycle - rail);
        loop {
            if first_index < length {
                answer.push_str(&text[first_index..first_index + 1]);
                first_index += cycle;
            } else { break; }
            if second_index < length {
                answer.push_str(&text[second_index..second_index + 1]);
                second_index += cycle;
            } else { break; }
        }
    }

    // Handle the last rail
    index = cycle / 2;
    while index < length {
        answer.push_str(&text[index..index + 1]);
        index += cycle;
    }

    return answer;
}

fn decode_rail_fence_cipher(text: &str, num_rails: usize) -> String {
    todo!()
}

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
