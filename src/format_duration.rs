// https://chat.openai.com/c/05bda066-aa51-466d-8fc1-b8cb8fe7dde4

fn format_duration(mut seconds: u64) -> String {
    let mut counts: Vec<(&str, u64)> = Vec::new();
    for (label, duration) in [
        ("year", 365 * 24 * 60 * 60),
        ("day", 24 * 60 * 60),
        ("hour", 60 * 60),
        ("minute", 60),
        ("second", 1)
    ] {
        let count = seconds / duration;
        if count > 0 {
            counts.push((label, count));
            seconds %= duration;
        }
    }

    match counts.iter()
        .map(|&(l, d)| format!("{} {}{}", d, l, if d > 1 { "s" } else { "" }))
        .collect::<Vec<String>>()
        .as_slice()
    {
        [] => String::from("now"),
        [only] => only.clone(),
        [init @ .., last] => format!("{} and {}", init.join(", "), last)
    }
}

#[cfg(test)]
mod tests {
    use super::format_duration;

    #[test]
    fn test_basic() {
        assert_eq!(format_duration(1), "1 second");
        assert_eq!(format_duration(62), "1 minute and 2 seconds");
        assert_eq!(format_duration(120), "2 minutes");
        assert_eq!(format_duration(3600), "1 hour");
        assert_eq!(format_duration(3662), "1 hour, 1 minute and 2 seconds");
    }
}
