use regex::Regex;
use std::error::Error;
#[derive(Debug,PartialEq)]
pub struct Color {
    pub red: bool,
    pub green: bool,
    pub yellow: bool,
}
pub fn parse_text(text: &str) -> Result<Color, Box<dyn Error>> {
    unimplemented!()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_green_messages() {
        // arrange
        let messages = ["[Finished running. Exit status: 0]"];
        let should_be_green: Color = Color {
            red: false,
            green: true,
            yellow: false,
        };
        // act
        // assert
        messages.into_iter().for_each(|message| {
            assert!(parse_text(message).is_ok());
            assert_eq!(parse_text(message).unwrap(), should_be_green);
        });
    }
}
