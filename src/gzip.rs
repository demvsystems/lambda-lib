pub fn decode(bytes: Vec<u8>) -> Option<String> {
    use flate2::read::GzDecoder;
    use std::io::Read;

    let mut decoder = GzDecoder::new(&bytes[..]);
    let mut xml = String::new();

    match decoder.read_to_string(&mut xml) {
        Ok(_) => Some(xml),
        Err(_) => None,
    }
}

pub fn encode(content: &str) -> Option<Vec<u8>> {
    use flate2::read::GzEncoder;
    use flate2::Compression;
    use std::io::Read;

    let mut encoder = GzEncoder::new(content.as_bytes(), Compression::best());
    let mut bytes = Vec::new();

    match encoder.read_to_end(&mut bytes) {
        Ok(_) => Some(bytes),
        Err(_) => None,
    }
}
