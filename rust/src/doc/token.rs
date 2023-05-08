#[derive(Debug, PartialEq)]
pub enum TokenType {
    WordToken,
    SpecSymbolToken,
    SpaceToken,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    kind: TokenType,
    token: String,
    change_seq: usize,
}

impl Token {
    pub fn new(kind: TokenType, token: String, change_seq: usize) -> Self {
        Token {
            kind,
            token,
            change_seq,
        }
    }

    pub fn kind(&self) -> &TokenType {
        &self.kind
    }

    pub fn token(&self) -> &String {
        &self.token
    }

    pub fn len(&self) -> usize {
        self.token.len()
    }
}
