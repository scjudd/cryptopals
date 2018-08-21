const ALPHABET: &'static [u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn encode(data: &[u8]) -> String {
    let mut base64_string = String::new();

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

        for n in 0..bytes.len() + 1 {
            let idx = chunk[n as usize] as usize;
            base64_string.push(char::from(ALPHABET[idx]));
        }

        for _ in 0..3 - bytes.len() {
            base64_string.push('=');
        }
    }

    base64_string
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn encode_works() {
        let base64 = encode(&[0xb0, 0x1d, 0xfa, 0xce]);
        assert_eq!(base64, "sB36zg==");
    }
}
