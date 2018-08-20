const ETAOIN: &'static [u8] = b"ETAOIN SHRDLU";

fn etaoin_count(bytes: &[u8]) -> u32 {
    bytes.into_iter().fold(0, |score, byte| {
        match ETAOIN.contains(&byte.to_ascii_uppercase()) {
            true => score + 1,
            false => score,
        }
    })
}

pub fn score(bytes: &[u8]) -> f64 {
    etaoin_count(bytes) as f64 / bytes.len() as f64
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn etaoin_count_works() {
        assert_eq!(etaoin_count(b"test"), 4);
        assert_eq!(etaoin_count(b"tezz"), 2);
        assert_eq!(etaoin_count(&[0x00]), 0);
    }

    #[test]
    fn score_works() {
        assert_eq!(score(b"test"), 1.0);
        assert_eq!(score(b"tezz"), 0.5);
        assert_eq!(score(&[0x00]), 0.0);
    }
}
