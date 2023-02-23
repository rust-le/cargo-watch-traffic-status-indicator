use crate::parsing::parse_text;
use std::io;
mod parsing;
use serialport;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let ports = serialport::available_ports()?;
    //for p in ports {
        //println!("{}", p.port_name);
    //}
    let mut port = serialport::new("/dev/ttyACM1", 9600)
        .timeout(Duration::from_millis(10))
        .open()?;

    let mut text: String = String::new();
    while io::stdin().read_line(&mut text).is_ok() {
        let trimmed = text.trim_end();
        if let Ok(Some(value)) = parse_text(trimmed) {
            println!("{:?}", value);
            let number: usize = usize::from(value);
            println!("{}", number);
            port.write(number.to_string().as_bytes())?;

            let mut serial_buf: Vec<u8> = vec![0; 1];
            port.read(serial_buf.as_mut_slice())
                .expect("Found no data!");
            let s = String::from_utf8(serial_buf).expect("Found invalid UTF-8");
            println!("{:?}", s);
        }
        text.clear();
    }
    Ok(())
}
