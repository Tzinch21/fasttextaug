use crate::utils;

/// Three token types
///
/// WordToken -> for every continuous sequence of alphanumeric chars
/// SpaceToken -> Any Space token
/// Non Space & non alphanumeric chars, like #$~, etc. (Useful for Keyboard Model)
#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    WordToken,
    SpecSymbolToken,
    SpaceToken,
}

/// Struct that stores token, it's type and it's lexicographic length
///
/// For example, Russian 'ф' - not one utf-8 char -> it takes 2 bytes
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

    /// Get lexicographic length
    pub fn utf8_len(&self) -> usize {
        self.token_len
    }

    /// Get bytes lenght
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
