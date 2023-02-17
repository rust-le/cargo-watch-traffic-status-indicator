use regex::Regex;
use std::error::Error;
#[derive(Debug, PartialEq)]
pub struct Color {
    pub red: bool,
    pub green: bool,
    pub yellow: bool,
}
impl Color {
    fn new() -> Self {
        Color {
            red: false,
            green: false,
            yellow: false,
        }
    }
}
pub fn parse_text(text: &str) -> Result<Option<Color>, Box<dyn Error>> {
    let re = Regex::new(r"^\[(Running|Finished running).+]$").unwrap();
    if re.is_match(text) {
        let c = match text {
            "[Finished running. Exit status: 0]" => Some(Color::new()),
            _ => Some(Color::new()),
        };
        Ok(c)
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse_text() {
        // arrange
        let blank: Color = Color::new();
        let red: Color = Color { red: true, ..blank };
        let yellow: Color = Color {
            yellow: true,
            ..blank
        };
        let green: Color = Color {
            green: true,
            ..blank
        };
        let messages = vec![
            ("[Finished running. Exit status: 0]", &green),
            ("[Running 'cargo check']", &yellow),
            ("[Finished running. Exit status: 101]", &red),
            ("[Running 'cargo test && cargo build']", &yellow),
        ];
        // act
        messages.into_iter().for_each(|(message, color)| {
            // assert
            assert!(parse_text(message).is_ok());
            if let Ok(Some(c)) = parse_text(message) {
                assert_eq!(&c, color);
            }
        });
    }
}
