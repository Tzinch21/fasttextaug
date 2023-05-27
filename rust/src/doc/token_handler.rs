use super::token::{Token, TokenType};

/// Struct, that stores original version of token & changed version
#[derive(Debug, PartialEq)]
pub struct TokenHandler {
    original_token: Token,
    changed_token: Option<Token>,
}

impl TokenHandler {
    pub fn new(kind: TokenType, token_str: String) -> Self {
        TokenHandler {
            original_token: Token::new(kind, token_str),
            changed_token: None,
        }
    }

    /// Set new changed version of this token
    pub fn change(&mut self, kind: TokenType, new_token_str: String) {
        self.changed_token = Some(Token::new(kind, new_token_str));
    }

    pub fn is_changed(&self) -> bool {
        if let Some(_) = &self.changed_token {
            return true;
        }
        false
    }

    pub fn get_original(&self) -> &Token {
        &self.original_token
    }

    /// If it has any change -> returns latest version, else -> original
    pub fn get_latest(&self) -> &Token {
        if let Some(token) = &self.changed_token {
            return token;
        }
        &self.original_token
    }

    /// Clear all changes
    pub fn set_to_original(&mut self) -> () {
        self.changed_token = None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_handler_on_create() {
        let th = TokenHandler::new(TokenType::WordToken, String::from("old"));
        assert!(!th.is_changed());
        assert_eq!(th.get_latest().token(), "old");
    }

    #[test]
    fn test_token_handler_on_change() {
        let mut th = TokenHandler::new(TokenType::WordToken, String::from("old"));
        th.change(TokenType::WordToken, String::from("new"));
        assert!(th.is_changed());
        assert_eq!(th.get_latest().token(), "new");
    }
}
