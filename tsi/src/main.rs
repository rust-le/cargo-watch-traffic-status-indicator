use std::io;
extern crate hidapi;
use crate::parsing::parse_text;
use hidapi::HidApi;
mod parsing;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = HidApi::new()?;
    for device in api.device_list() {
        println!("{:04x}:{:04x}", device.vendor_id(),device.product_id());
    }

    let mut text: String = String::new();
    while io::stdin().read_line(&mut text).is_ok() {
        let trimmed = text.trim_end();
        if let Ok(Some(value)) = parse_text(trimmed) {
            println!("{:?}", value);
            let number: usize = usize::from(value);
            println!("{}", number);
        }
        text.clear();
    }
    Ok(())
}
