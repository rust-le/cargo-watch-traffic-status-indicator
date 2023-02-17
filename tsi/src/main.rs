use std::io;

use crate::parsing::parse_text;
mod parsing;

fn main() {
    let mut text: String = String::new();
    while io::stdin().read_line(&mut text).is_ok() {
        let trimmed = text.trim_end();
        if let Ok(Some(value)) = parse_text(trimmed) {
            println!("{:?}", value);
        }
        println!("Parsed text: {}", trimmed);
        text.clear();
    }
}
