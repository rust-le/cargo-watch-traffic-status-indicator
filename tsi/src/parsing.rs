use regex::Regex;
use std::error::Error;
struct Color {
    red: bool,
    green: bool,
    yellow: bool,
}
pub fn parse_text(text: &str) -> Result<usize, Box<dyn Error>> {
    Ok(text.len())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse_text() {
        // arrange
        let some_text: &str = "one two";
        // act
        // assert
        assert!(parse_text(&some_text).is_ok());
        assert_eq!(parse_text(&some_text).unwrap(), 7);
    }
}
