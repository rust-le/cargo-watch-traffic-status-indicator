use std::io;
mod parsing;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    println!("Start");
    let mut text: String = String::new();
    io::stdin().read_line(&mut text)?;

    println!("{}", text);
    let value: usize = parsing::parse_text(text).unwrap();
    println!("{}", value);
    Ok(())
}
