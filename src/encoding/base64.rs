pub fn encode(data: &[u8]) -> String {
    let alphabet = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
        'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1',
        '2', '3', '4', '5', '6', '7', '8', '9', '+', '/',
    ];

    let mut base64_string = String::new();

    for chunks in data.chunks(3) {
        let mut buf = [0; 4];

        // aaaaaaaa bbbbbbbb cccccccc
        // 00aaaaaa 00aabbbb 00bbbbcc 00cccccc
        for (n, chunk) in chunks.into_iter().enumerate() {
            match n {
                0 => {
                    buf[0] = chunk >> 2u8;
                    buf[1] = (chunk & 0x3) << 4u8;
                }
                1 => {
                    buf[1] |= chunk >> 4u8;
                    buf[2] = (chunk & 0xf) << 2u8;
                }
                2 => {
                    buf[2] |= chunk >> 6u8;
                    buf[3] = chunk & 0x3f;
                }
                _ => panic!("shouldn't have more than 3 chunks"),
            }
        }

        for n in 0..chunks.len() + 1 {
            let idx = buf[n as usize] as usize;
            base64_string.push(alphabet[idx]);
        }

        for _ in 0..3 - chunks.len() {
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
        assert_eq!(encode(&[0xb0, 0x1d, 0xfa, 0xce]), "sB36zg==");
    }
}
