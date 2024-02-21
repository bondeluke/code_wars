// https://chat.openai.com/c/05bda066-aa51-466d-8fc1-b8cb8fe7dde4

fn format_duration(seconds: u64) -> String {
    match [
        ("year", 60 * 60 * 24 * 365, 100000),
        ("day", 60 * 60 * 24, 365),
        ("hour", 60 * 60, 24),
        ("minute", 60, 60),
        ("second", 1, 60),
    ].iter()
        .filter_map(|(unit, duration, modulo)| match seconds / duration % modulo {
            0 => None,
            1 => Some(format!("1 {unit}")),
            c => Some(format!("{c} {unit}s")),
        })
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
