extern crate cryptopals;
use cryptopals::encoding::hex;

#[test]
fn set1_challenge2() {
    let buff1 = hex::decode(&"1c0111001f010100061a024b53535009181c").unwrap();
    let buff2 = hex::decode(&"686974207468652062756c6c277320657965").unwrap();
    let output = hex::decode(&"746865206b696420646f6e277420706c6179").unwrap();
    assert_eq!(cryptopals::fixed_xor(&buff1, &buff2).unwrap(), output);
}
