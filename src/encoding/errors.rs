#[derive(Debug)]
pub struct DecodeError {
    pub offset: usize,
    pub kind: DecodeErrorKind,
}

#[derive(Debug)]
pub enum DecodeErrorKind {
    IllegalChar(char),
    InvalidLength,
}
