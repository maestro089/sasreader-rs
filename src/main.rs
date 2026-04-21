
use std::fs::File;
use std::io::{BufReader, Read};

fn main() -> std::io::Result<()> {
    let file = File::open("cars.sas7bdat")?;
    let mut reader = BufReader::new(file);
    let mut header = [0u8; 288];

    reader.read_exact(&mut header)?;

    let encoding_code = u16::from_le_bytes([header[70], header[71]]);

    println!("Код кодировки: {}", encoding_code);

    Ok(())
}