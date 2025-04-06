use anyhow::Result;
use base64::prelude::*;
use flate2::read::ZlibDecoder;
use std::io::prelude::*;

pub fn main() -> Result<()> {
    let mut decoder = ZlibDecoder::new(std::fs::File::open("string-1743786954011.z")?);
    let mut buffer = String::new();

    decoder.read_to_string(&mut buffer)?;

    println!("{buffer}");

    // let encoded_shot = "$;</)/V($I*#3.]0-0?-$$3'5&###[BXB0;;?A$*#";
    // let decoded = dbg!(BASE64_STANDARD_NO_PAD.decode(encoded_shot)?);

    // let mut decoder = ZlibDecoder::new(decoded.as_slice());
    // // let mut shot_data = String::new();
    // let mut buf = Vec::new();
    // decoder.read_to_end(&mut buf)?;
    // // decoder.read_to_string(&mut shot_data)?;
    // dbg!(&buf);

    Ok(())
}
