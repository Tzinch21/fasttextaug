use crate::utils;

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
    token_len: usize,
}

impl Token {
    pub fn new(kind: TokenType, token: String) -> Self {
        let token_len = utils::get_chars_len(&token);
        Token {
            kind,
            token,
            token_len,
        }
    }

    pub fn kind(&self) -> &TokenType {
        &self.kind
    }

    pub fn token(&self) -> &String {
        &self.token
    }

    pub fn utf8_len(&self) -> usize {
        self.token_len
    }

    pub fn byte_len(&self) -> usize {
        self.token.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_len_en() {
        let token = Token::new(TokenType::WordToken, String::from("hello"));
        assert_eq!(token.utf8_len(), 5);
        assert_eq!(token.byte_len(), 5);
    }

    #[test]
    fn test_char_len_ru() {
        let token = Token::new(TokenType::WordToken, String::from("привет"));
        assert_eq!(token.utf8_len(), 6);
        assert_eq!(token.byte_len(), 12);
    }
}
