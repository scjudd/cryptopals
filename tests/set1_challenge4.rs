extern crate cryptopals;
use cryptopals::encoding::hex;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[test]
fn set1_challenge4() {
    let file = File::open("tests/data/set1_challenge4.txt").unwrap();

    let result = BufReader::new(file)
        .lines()
        .map(|line| hex::decode(&line.unwrap()).unwrap())
        .map(|bytes| cryptopals::crack_single_byte_xor(&bytes))
        .max_by_key(|result| result.score)
        .unwrap();

    assert_eq!(
        result.plaintext,
        "Now that the party is jumping\n".as_bytes()
    );
}
