pub mod encoding;
pub mod english;

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

#[derive(Debug)]
pub struct CrackResult {
    pub plaintext: Vec<u8>,
    pub key: Vec<u8>,
    pub score: f64,
}

pub fn crack_single_byte_xor(bytes: &[u8]) -> CrackResult {
    let mut best = CrackResult {
        plaintext: Vec::from(bytes),
        key: vec![0x00],
        score: english::score(&bytes),
    };

    for key in std::u8::MIN..std::u8::MAX {
        let full_key = std::iter::repeat(key)
            .take(bytes.len())
            .collect::<Vec<u8>>();

        let plaintext = fixed_xor(bytes, &full_key).unwrap();
        let score = english::score(&plaintext);

        if score > best.score {
            best = CrackResult {
                plaintext,
                key: vec![key],
                score,
            }
        }
    }

    best
}
