pub mod encoding;

#[derive(Debug)]
pub enum FixedXorError {
    DiffLens,
}

pub fn fixed_xor(buf1: &[u8], buf2: &[u8]) -> Result<Vec<u8>, FixedXorError> {
    if buf1.len() != buf2.len() {
        return Err(FixedXorError::DiffLens);
    }

    let mut data = Vec::with_capacity(buf1.len());

    for (b1, b2) in buf1.into_iter().zip(buf2) {
        data.push(b1 ^ b2);
    }

    Ok(data)
}
