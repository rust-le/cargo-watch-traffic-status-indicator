use std::io;
mod parsing;

fn main() {
    let mut text: String = String::new();
    while io::stdin().read_line(&mut text).is_ok() {
        let trimmed = text.trim_end();
        let value: usize = parsing::parse_text(&trimmed).unwrap();
        println!("Parsed text: {}", trimmed);
        println!("{}", value);
        text.clear();
    }
}
