pub mod encoding;
pub mod english;
mod score;
use score::Score;

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
    pub score: Score,
}

pub fn crack_single_byte_xor(bytes: &[u8]) -> CrackResult {
    (0..128u8)
        .map(|byte| std::iter::repeat(byte).take(bytes.len()).collect::<Vec<_>>())
        .map(|key| {
            let plaintext = fixed_xor(bytes, &key).unwrap();
            let score = english::score(&plaintext);
            CrackResult {
                plaintext,
                key,
                score,
            }
        })
        .max_by_key(|result| result.score)
        .unwrap()
}
