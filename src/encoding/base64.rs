use super::errors::DecodeError;
use super::errors::DecodeErrorKind::*;

const ALPHABET: &'static [u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn encode(data: &[u8]) -> String {
    let mut base64 = String::new();

    for bytes in data.chunks(3) {
        let mut chunk = [0; 4];

        // aaaaaaaa bbbbbbbb cccccccc
        // 00aaaaaa 00aabbbb 00bbbbcc 00cccccc
        for (n, byte) in bytes.into_iter().enumerate() {
            match n {
                0 => {
                    chunk[0] |= byte >> 2u8;
                    chunk[1] |= (byte & 0x3) << 4u8;
                }
                1 => {
                    chunk[1] |= byte >> 4u8;
                    chunk[2] |= (byte & 0xf) << 2u8;
                }
                2 => {
                    chunk[2] |= byte >> 6u8;
                    chunk[3] |= byte & 0x3f;
                }
                _ => panic!("shouldn't have more than 3 bytes"),
            }
        }

        let chunk_length = bytes.len() + 1;

        for n in 0..chunk_length {
            let idx = chunk[n as usize] as usize;
            base64.push(char::from(ALPHABET[idx]));
        }

        for _ in 0..4 - chunk_length {
            base64.push('=');
        }
    }

    base64
}

pub fn decode(base64: &str) -> Result<Vec<u8>, DecodeError> {
    let unpadded_length = base64.chars().filter(|&ch| ch != '=').count();

    if unpadded_length % 4 == 1 {
        return Err(DecodeError {
            offset: 0,
            kind: InvalidLength,
        });
    }

    let mut buf = Vec::new();
    let mut padding = false;
    let mut num_padding = 0;
    let needed_padding = match unpadded_length % 4 {
        0 => 0,
        n => 4 - n,
    };

    for (offset, ch) in base64.chars().enumerate() {
        if ch == '=' {
            padding = true;
            num_padding += 1;
            if num_padding > needed_padding {
                return Err(DecodeError {
                    offset: offset,
                    kind: IllegalChar(ch),
                });
            }
            continue;
        }

        if padding {
            return Err(DecodeError {
                offset: offset,
                kind: IllegalChar(ch),
            });
        }

        match ALPHABET.iter().position(|&c| char::from(c) == ch) {
            None => {
                return Err(DecodeError {
                    offset: offset,
                    kind: IllegalChar(ch),
                });
            }

            Some(idx) => {
                buf.push(idx as u8);
            }
        }
    }

    let mut data = Vec::with_capacity(buf.len());

    for bytes in buf.chunks(4) {
        let mut chunk = [0; 3];

        // 00aaaaaa 00aabbbb 00bbbbcc 00cccccc
        // aaaaaaaa bbbbbbbb cccccccc
        for (n, byte) in bytes.into_iter().enumerate() {
            match n {
                0 => chunk[0] |= byte << 2u8,
                1 => {
                    chunk[0] |= (byte & 0x30) >> 4u8;
                    chunk[1] |= (byte & 0x0f) << 4u8;
                }
                2 => {
                    chunk[1] |= (byte & 0x3c) >> 2u8;
                    chunk[2] |= (byte & 0x03) << 6u8;
                }
                3 => chunk[2] |= byte,
                _ => panic!("shouldn't have more than 4 bytes"),
            }
        }

        let chunk_length = bytes.len() - 1;

        for n in 0..chunk_length {
            data.push(chunk[n as usize]);
        }
    }

    Ok(data)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn encode_works() {
        let base64 = encode(&[0xb0, 0x1d, 0xfa, 0xce]);
        assert_eq!(base64, "sB36zg==");
    }

    #[test]
    fn decode_works() {
        {
            let data = decode("sB36zg==").unwrap();
            assert_eq!(data, &[0xb0, 0x1d, 0xfa, 0xce]);
        }

        {
            let data = decode("sB36zg").unwrap();
            assert_eq!(data, &[0xb0, 0x1d, 0xfa, 0xce]);
        }
    }

    #[test]
    fn decode_fails_with_invalid_length() {
        let err = decode("f===").err().unwrap();
        assert_eq!(err.kind, InvalidLength);
    }

    #[test]
    fn decode_fails_with_illegal_char() {
        {
            let err = decode("foobarb_").err().unwrap();
            assert_eq!(err.offset, 7);
            assert_eq!(err.kind, IllegalChar('_'));
        }

        {
            // Data following padding
            let err = decode("fo=o").err().unwrap();
            assert_eq!(err.offset, 3);
            assert_eq!(err.kind, IllegalChar('o'));
        }

        {
            // No padding is needed for this string
            let err = decode("fo==o=o=").err().unwrap();
            assert_eq!(err.offset, 2);
            assert_eq!(err.kind, IllegalChar('='));
        }

        {
            // One too many padding characters
            let err = decode("fo===").err().unwrap();
            assert_eq!(err.offset, 4);
            assert_eq!(err.kind, IllegalChar('='));
        }
    }
}
