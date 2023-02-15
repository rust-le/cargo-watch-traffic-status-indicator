use std::error::Error;
pub fn parse_text(text: String) -> Result<usize, Box<dyn Error>> {
    Ok(text.len())
}
