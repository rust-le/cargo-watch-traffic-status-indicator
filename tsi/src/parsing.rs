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
impl From<Color> for usize {
    fn from(color: Color) -> Self {
        //  red = 1, yellow = 2, green = 4
        let mut sum = 0;
        if color.red {
            sum += 1
        };
        if color.yellow {
            sum += 2
        };
        if color.green {
            sum += 4
        };
        sum
    }
}
pub fn parse_text(text: &str) -> Result<Option<Color>, Box<dyn Error>> {
    let blank: Color = Color::new();
    let yellow = Regex::new(r"^\[Running.+\]$")?;
    let red = Regex::new(r"^\[Finished running. Exit status: \d{1,3}\]$")?;
    if yellow.is_match(text) {
        Ok(Some(Color {
            yellow: true,
            ..blank
        }))
    } else if text == "[Finished running. Exit status: 0]" {
        Ok(Some(Color {
            green: true,
            ..blank
        }))
    } else if red.is_match(text) {
        Ok(Some(Color { red: true, ..blank }))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_impl_from_color_for_usize() {
        // arrange
        let blank: Color = Color::new();
        let rainbow: Color = Color {
            red: true,
            green: true,
            yellow: true,
        };
        // act
        // assert
        assert_eq!(usize::from(blank), 0);
        assert_eq!(usize::from(rainbow), 7);
    }
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
            ("[Finished running. Exit status: 0]", Some(green)),
            ("[Finished running. Exit status: 101]", Some(red)),
            ("[Running 'cargo test && cargo build']", Some(yellow)),
            ("[Some text]", None),
        ];
        // act
        messages.into_iter().for_each(|(message, color)| {
            // assert
            assert!(parse_text(message).is_ok());
            if let Ok(c) = parse_text(message) {
                assert_eq!(c, color);
            }
        });
    }
}
