extern crate cryptopals;
use cryptopals::encoding::hex;

#[test]
fn set1_challenge3() {
    let ciphertext = hex::decode(
        &"1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736",
    ).unwrap();
    let result = cryptopals::crack_single_byte_xor(&ciphertext);
    assert_eq!(
        result.plaintext,
        "Cooking MC\'s like a pound of bacon".as_bytes()
    );
}
