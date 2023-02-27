use crate::parsing::parse_text;
use std::io;
mod parsing;
use serialport;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut port = serialport::new("/dev/ttyACM1", 9600)
        .timeout(Duration::from_millis(10))
        .open()?;
    let mut text: String = String::new();
    while io::stdin().read_line(&mut text).is_ok() {
        let trimmed = text.trim_end();
        if let Ok(Some(value)) = parse_text(trimmed) {
            let number: usize = usize::from(value);
            port.write(number.to_string().as_bytes())?;
        }
        text.clear();
    }
    Ok(())
}
