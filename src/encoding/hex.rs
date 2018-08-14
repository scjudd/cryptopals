use super::errors::DecodeError;
use super::errors::DecodeErrorKind::*;

pub fn decode(hex: &str) -> Result<Vec<u8>, DecodeError> {
    // two hex digits make up a single byte
    if hex.len() % 2 != 0 {
        return Err(DecodeError {
            offset: 0,
            kind: OddLen,
        });
    }

    // two hex digits make up a single byte
    let mut data = Vec::with_capacity(hex.len() / 2);
    let hex_radix = 16;

    for pairs in hex.chars().enumerate().collect::<Vec<_>>().chunks(2) {
        for (offset, ch) in pairs {
            if !ch.is_digit(hex_radix) {
                return Err(DecodeError {
                    offset: *offset,
                    kind: IllegalChar(*ch),
                });
            }
        }

        let chars = pairs.into_iter().map(|pair| pair.1);
        let enc: String = chars.collect();
        let byte = u8::from_str_radix(&enc, hex_radix).unwrap();
        data.push(byte);
    }

    Ok(data)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn decode_works() {
        assert!(decode("49").is_ok());
    }

    #[test]
    fn decode_fails_with_odd_length() {
        assert!(decode("9").is_err());
        assert!(decode("999").is_err());
    }

    #[test]
    fn decode_fails_with_invalid_hex() {
        assert!(decode("-1").is_err());
        assert!(decode("+1").is_err());
        assert!(decode("zz").is_err());
    }
}
