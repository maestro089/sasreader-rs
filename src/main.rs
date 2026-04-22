
use std::fs::File;
use std::io::{BufReader, Read};
use std::io::Result;

fn main() -> Result<()> {
    let file = File::open("cars.sas7bdat")?;
    let mut reader = BufReader::new(file);
    let mut header_byte = [0u8; 288];

    reader.read_exact(&mut header_byte)?;

    // let header = SasReadHeaderFile{ file_byte: header_byte };
    let mut header = SasReadHeaderFile::new(header_byte);
    header.parse_metadata();

    Ok(())
}

#[derive(Debug)]
struct SASProperties{
    encode: String,
}

struct SasReadHeaderFile{
    file_byte: [u8; 288],
    properties: SASProperties,
}

impl SasReadHeaderFile{
    fn new(file_byte: [u8; 288]) -> Self{
        Self {
            file_byte, 
            properties: SASProperties {
                encode: "UTF-8".to_string(),
            }
        }
    }

    fn parse_metadata(&mut self){
        let _encoding_code = u16::from_le_bytes([self.file_byte[70], self.file_byte[71]]);
        self.properties.encode = "wqeqwe".to_string();
        
        println!("{:?}", self.properties);
    }
}