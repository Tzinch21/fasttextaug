use super::token::{Token, TokenType};

#[derive(Debug, PartialEq)]
pub struct ChangeLog {
    original_token: Token,
    change_logs: Vec<Token>,
    is_changed: bool,
}

impl ChangeLog {
    pub fn new(kind: TokenType, token_str: String, change_seq: Option<usize>) -> Self {
        let token = match change_seq {
            Some(val) => Token::new(kind, token_str, val),
            None => Token::new(kind, token_str, 0),
        };
        ChangeLog {
            original_token: token,
            change_logs: Vec::with_capacity(3),
            is_changed: false,
        }
    }

    pub fn add_change(&mut self, kind: TokenType, new_token_str: String, change_seq: usize) {
        self.change_logs
            .push(Token::new(kind, new_token_str, change_seq));
        self.is_changed = true;
    }

    pub fn is_changed(&self) -> bool {
        self.is_changed
    }

    pub fn get_original(&self) -> &Token {
        &self.original_token
    }

    pub fn get_logs(&self) -> &Vec<Token> {
        &self.change_logs
    }

    pub fn get_latest(&self) -> &Token {
        if let Some(token) = self.change_logs.last() {
            return token;
        }
        &self.original_token
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change_log_on_create() {
        let log = ChangeLog::new(TokenType::WordToken, String::from("old"), None);
        assert!(!log.is_changed);
        assert_eq!(log.get_logs().len(), 0);
        assert_eq!(log.get_latest().token(), "old");
    }

    #[test]
    fn test_change_log_on_change() {
        let mut log = ChangeLog::new(TokenType::WordToken, String::from("old"), None);
        log.add_change(TokenType::WordToken, String::from("new"), 1);
        assert!(log.is_changed);
        assert_eq!(log.get_logs().len(), 1);
        assert_eq!(log.get_latest().token(), "new");
    }
}
