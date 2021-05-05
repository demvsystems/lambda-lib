pub fn decode(bytes: Vec<u8>) -> Result<String, String> {
    use flate2::read::GzDecoder;
    use std::io::Read;

    let mut decoder = GzDecoder::new(&bytes[..]);
    let mut xml = String::new();

    match decoder.read_to_string(&mut xml) {
        Ok(_) => Ok(xml),
        Err(error) => Err(error.to_string()),
    }
}

pub fn encode(content: &str) -> Result<Vec<u8>, String> {
    use flate2::read::GzEncoder;
    use flate2::Compression;
    use std::io::Read;

    let mut encoder = GzEncoder::new(content.as_bytes(), Compression::best());
    let mut bytes = Vec::new();

    match encoder.read_to_end(&mut bytes) {
        Ok(_) => Ok(bytes),
        Err(error) => Err(error.to_string()),
    }
}
