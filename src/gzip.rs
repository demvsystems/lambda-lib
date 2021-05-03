pub fn decode(bytes: Vec<u8>) -> Result<String, String> {
    use flate2::read::GzDecoder;
    use std::io::Read;

    let mut decoder = GzDecoder::new(&bytes[..]);
    let mut xml = String::new();

    match decoder.read_to_string(&mut xml) {
        Ok(_) => Ok(xml),
        Err(error) => Err(error.to_string())
    }
}

pub fn encode(content: &str) -> Result<String, String> {
    use flate2::read::GzDecoder;
    use std::io::Read;

    let mut decoder = GzDecoder::new(&bytes[..]);
    let mut xml = String::new();

    match decoder.read_to_string(&mut xml) {
        Ok(_) => Ok(xml),
        Err(error) => Err(error.to_string()),
    }
}
