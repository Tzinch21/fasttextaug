use super::token::{Token, TokenType};
use super::token_handler::TokenHandler;
use std::sync::Arc;
const RESERVE_CAPACITY_TO_INSERT_OPERATIONS: usize = 5;

/// Doc, main struct to store each token and perform operations on them
pub struct Doc {
    tokens: Vec<TokenHandler>,
    changed_count: usize,
}

impl Doc {
    pub fn new(input: &String) -> Self {
        let tokens = Doc::tokenize(input);
        let changed_count = 0;
        Doc {
            tokens,
            changed_count,
        }
    }

    pub fn from_arc(input: Arc<String>) -> Self {
        let tokens = Doc::tokenize(input.as_ref());
        let changed_count = 0;
        Doc {
            tokens,
            changed_count,
        }
    }

    /// Split input string to vector of different tokens
    ///
    /// If it alphanumeric -> WordToken, if it's space -> SpaceToken, else SpecSymbolToken
    fn tokenize(text: &String) -> Vec<TokenHandler> {
        let mut pos: usize = 0;
        let mut res: Vec<TokenHandler> =
            Vec::with_capacity(text.len() + RESERVE_CAPACITY_TO_INSERT_OPERATIONS);
        for (idx, c) in text.match_indices(|c| !char::is_alphanumeric(c)) {
            let previous_word = text.get(pos..idx);
            if let Some(word) = previous_word {
                if word.len() > 0 {
                    res.push(TokenHandler::new(TokenType::WordToken, String::from(word)));
                    pos = idx;
                }
            }
            if c.len() > 0 {
                let flag = c.chars().next().unwrap().is_whitespace();
                let kind = match flag {
                    true => TokenType::SpaceToken,
                    false => TokenType::SpecSymbolToken,
                };
                res.push(TokenHandler::new(kind, String::from(c)));
                pos += c.len();
            }
        }
        let remains = text.get(pos..);
        if let Some(value) = remains {
            if value.len() > 0 {
                res.push(TokenHandler::new(TokenType::WordToken, String::from(value)));
            }
        }
        res.shrink_to(res.len() + RESERVE_CAPACITY_TO_INSERT_OPERATIONS);
        res
    }

    /// Create string from tokens
    fn concatenate_tokens(vec_tokens: Vec<&Token>) -> String {
        let total_tokens_len = vec_tokens.iter().map(|t| t.byte_len()).sum();
        let mut concated_str = String::with_capacity(total_tokens_len);
        vec_tokens
            .iter()
            .for_each(|t| concated_str.push_str(&t.token()));
        concated_str
    }

    /// Filter and get only WordTokens with their original indexes
    pub fn get_word_tokens_with_indexes(
        &mut self,
        include_special_char: bool,
    ) -> Vec<(usize, &mut TokenHandler)> {
        let mut word_tokens = Vec::with_capacity(self.tokens.len());
        for (idx, token) in self.tokens.iter_mut().enumerate() {
            let token_type = token.get_original().kind();
            match (token_type, include_special_char) {
                (TokenType::WordToken, _) => word_tokens.push((idx, token)),
                (TokenType::SpecSymbolToken, true) => word_tokens.push((idx, token)),
                (_, _) => (),
            }
        }
        word_tokens.shrink_to_fit();
        word_tokens
    }

    /// Get only WordTokens original indexes
    pub fn get_word_indexes(&mut self, include_special_char: bool) -> Vec<usize> {
        self.get_word_tokens_with_indexes(include_special_char)
            .into_iter()
            .map(|x| x.0)
            .collect()
    }

    /// Swap two tokens by their indexes
    pub fn perform_swap_by_idx(&mut self, idx_a: usize, idx_b: usize) {
        let tokens_len = self.tokens.len();
        if (idx_a < tokens_len) & (idx_b < tokens_len) {
            self.tokens.swap(idx_a, idx_b)
        }
    }

    /// Calculate number of word tokens
    pub fn get_word_tokens_count(&self, include_special_char: bool) -> usize {
        let mut count = 0;
        for token in self.tokens.iter() {
            let token_type = token.get_original().kind();
            match (token_type, include_special_char) {
                (TokenType::WordToken, _) => count += 1,
                (TokenType::SpecSymbolToken, true) => count += 1,
                (_, _) => (),
            }
        }
        count
    }

    /// Get original tokens (before augmentation)
    pub fn get_original_tokens(&self) -> Vec<&Token> {
        self.tokens.iter().map(|ch| ch.get_original()).collect()
    }

    /// Get latest tokens (after augmentation, if it was)
    pub fn get_augmented_tokens(&self) -> Vec<&Token> {
        self.tokens.iter().map(|ch| ch.get_latest()).collect()
    }

    /// Create string from augmented tokens
    pub fn get_augmented_string(&self) -> String {
        Doc::concatenate_tokens(self.get_augmented_tokens())
    }

    /// Get number of changes
    pub fn get_changed_count(&self) -> usize {
        self.changed_count
    }

    /// Set number of changes
    pub fn set_change_count(&mut self, value: usize) -> () {
        self.changed_count = value
    }

    /// Clear all changes
    pub fn set_to_original(&mut self) -> () {
        for token in self.tokens.iter_mut() {
            token.set_to_original();
        }
        self.changed_count = 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Doc {
        fn from_handlers(handlers: Vec<TokenHandler>) -> Self {
            Doc {
                tokens: handlers,
                changed_count: 0,
            }
        }
    }

    #[test]
    fn test_tokenize() {
        let input_str = String::from("My example sentence?!. ");
        let expected_handlers = vec![
            TokenHandler::new(TokenType::WordToken, String::from("My")),
            TokenHandler::new(TokenType::SpaceToken, String::from(" ")),
            TokenHandler::new(TokenType::WordToken, String::from("example")),
            TokenHandler::new(TokenType::SpaceToken, String::from(" ")),
            TokenHandler::new(TokenType::WordToken, String::from("sentence")),
            TokenHandler::new(TokenType::SpecSymbolToken, String::from("?")),
            TokenHandler::new(TokenType::SpecSymbolToken, String::from("!")),
            TokenHandler::new(TokenType::SpecSymbolToken, String::from(".")),
            TokenHandler::new(TokenType::SpaceToken, String::from(" ")),
        ];
        assert_eq!(Doc::tokenize(&input_str), expected_handlers);

        let mut doc = Doc::new(&input_str);
        assert_eq!(doc.tokens, expected_handlers);
        let word_token_len = doc.get_word_tokens_with_indexes(false).len();
        assert_eq!(word_token_len, 3);
        let word_spec_token_len = doc.get_word_tokens_with_indexes(true).len();
        assert_eq!(word_spec_token_len, 6);
    }

    #[test]
    fn test_tokenize_cyrillic() {
        let input_str = String::from("Пр0веряем раб0ту с кирилицей! .");
        let expected_handlers = vec![
            TokenHandler::new(TokenType::WordToken, String::from("Пр0веряем")),
            TokenHandler::new(TokenType::SpaceToken, String::from(" ")),
            TokenHandler::new(TokenType::WordToken, String::from("раб0ту")),
            TokenHandler::new(TokenType::SpaceToken, String::from(" ")),
            TokenHandler::new(TokenType::WordToken, String::from("с")),
            TokenHandler::new(TokenType::SpaceToken, String::from(" ")),
            TokenHandler::new(TokenType::WordToken, String::from("кирилицей")),
            TokenHandler::new(TokenType::SpecSymbolToken, String::from("!")),
            TokenHandler::new(TokenType::SpaceToken, String::from(" ")),
            TokenHandler::new(TokenType::SpecSymbolToken, String::from(".")),
        ];
        assert_eq!(Doc::tokenize(&input_str), expected_handlers);

        let mut doc = Doc::new(&input_str);
        assert_eq!(doc.tokens, expected_handlers);
        let word_token_len = doc.get_word_tokens_with_indexes(false).len();
        assert_eq!(word_token_len, 4);
        let word_spec_token_len = doc.get_word_tokens_with_indexes(true).len();
        assert_eq!(word_spec_token_len, 6);
    }

    #[test]
    fn test_tokenize_one_more() {
        let input_str = String::from("Create tokens");
        let expected_handlers = vec![
            TokenHandler::new(TokenType::WordToken, String::from("Create")),
            TokenHandler::new(TokenType::SpaceToken, String::from(" ")),
            TokenHandler::new(TokenType::WordToken, String::from("tokens")),
        ];
        assert_eq!(Doc::tokenize(&input_str), expected_handlers);

        let mut doc = Doc::new(&input_str);
        assert_eq!(doc.tokens, expected_handlers);
        let word_token_len = doc.get_word_tokens_with_indexes(false).len();
        assert_eq!(word_token_len, 2);
        let word_spec_token_len = doc.get_word_tokens_with_indexes(true).len();
        assert_eq!(word_spec_token_len, 2);
    }

    #[test]
    fn test_tokenize_one_more_cyrillic() {
        let input_str = String::from("Делаем токены");
        let expected_handlers = vec![
            TokenHandler::new(TokenType::WordToken, String::from("Делаем")),
            TokenHandler::new(TokenType::SpaceToken, String::from(" ")),
            TokenHandler::new(TokenType::WordToken, String::from("токены")),
        ];
        assert_eq!(Doc::tokenize(&input_str), expected_handlers);

        let mut doc = Doc::new(&input_str);
        assert_eq!(doc.tokens, expected_handlers);
        let word_token_len = doc.get_word_tokens_with_indexes(false).len();
        assert_eq!(word_token_len, 2);
        let word_spec_token_len = doc.get_word_tokens_with_indexes(true).len();
        assert_eq!(word_spec_token_len, 2);
    }

    #[test]
    fn test_tokenize_one_word() {
        let input_str = String::from("example");
        let expected_handlers = vec![TokenHandler::new(
            TokenType::WordToken,
            String::from("example"),
        )];
        assert_eq!(Doc::tokenize(&input_str), expected_handlers);

        let mut doc = Doc::new(&input_str);
        assert_eq!(doc.tokens, expected_handlers);
        let word_token_len = doc.get_word_tokens_with_indexes(false).len();
        assert_eq!(word_token_len, 1);
        let word_spec_token_len = doc.get_word_tokens_with_indexes(true).len();
        assert_eq!(word_spec_token_len, 1);
    }

    #[test]
    fn test_tokenize_only_spec() {
        let input_str = String::from("!@#");
        let expected_handlers = vec![
            TokenHandler::new(TokenType::SpecSymbolToken, String::from("!")),
            TokenHandler::new(TokenType::SpecSymbolToken, String::from("@")),
            TokenHandler::new(TokenType::SpecSymbolToken, String::from("#")),
        ];
        assert_eq!(Doc::tokenize(&input_str), expected_handlers);

        let mut doc = Doc::new(&input_str);
        assert_eq!(doc.tokens, expected_handlers);
        let word_token_len = doc.get_word_tokens_with_indexes(false).len();
        assert_eq!(word_token_len, 0);
        let word_spec_token_len = doc.get_word_tokens_with_indexes(true).len();
        assert_eq!(word_spec_token_len, 3);
    }

    #[test]
    fn test_tokenize_spaces() {
        let input_str = String::from("   ");
        let expected_handlers = vec![
            TokenHandler::new(TokenType::SpaceToken, String::from(" ")),
            TokenHandler::new(TokenType::SpaceToken, String::from(" ")),
            TokenHandler::new(TokenType::SpaceToken, String::from(" ")),
        ];
        assert_eq!(Doc::tokenize(&input_str), expected_handlers);

        let mut doc = Doc::new(&input_str);
        assert_eq!(doc.tokens, expected_handlers);
        let word_token_len = doc.get_word_tokens_with_indexes(false).len();
        assert_eq!(word_token_len, 0);
        let word_spec_token_len = doc.get_word_tokens_with_indexes(true).len();
        assert_eq!(word_spec_token_len, 0);
    }

    #[test]
    fn test_tokenize_empty() {
        let input_str = String::from("");
        let expected_handlers: Vec<TokenHandler> = vec![];
        assert_eq!(Doc::tokenize(&input_str), expected_handlers);

        let mut doc = Doc::new(&input_str);
        assert_eq!(doc.tokens, expected_handlers);
        let word_token_len = doc.get_word_tokens_with_indexes(false).len();
        assert_eq!(word_token_len, 0);
        let word_spec_token_len = doc.get_word_tokens_with_indexes(true).len();
        assert_eq!(word_spec_token_len, 0);
    }

    #[test]
    fn test_concatenate_tokens() {
        let input_handlers = vec![
            TokenHandler::new(TokenType::WordToken, String::from("My")),
            TokenHandler::new(TokenType::SpaceToken, String::from(" ")),
            TokenHandler::new(TokenType::WordToken, String::from("example")),
            TokenHandler::new(TokenType::SpaceToken, String::from(" ")),
            TokenHandler::new(TokenType::WordToken, String::from("sentence")),
            TokenHandler::new(TokenType::SpecSymbolToken, String::from("?")),
            TokenHandler::new(TokenType::SpecSymbolToken, String::from("!")),
            TokenHandler::new(TokenType::SpecSymbolToken, String::from(".")),
            TokenHandler::new(TokenType::SpaceToken, String::from(" ")),
        ];
        let expected_str = String::from("My example sentence?!. ");
        let result = Doc::from_handlers(input_handlers).get_augmented_string();
        assert_eq!(result, expected_str);
    }

    #[test]
    fn test_concatenate_one_token() {
        let input_handlers = vec![TokenHandler::new(
            TokenType::WordToken,
            String::from("example"),
        )];
        let expected_str = String::from("example");
        assert_eq!(
            Doc::from_handlers(input_handlers).get_augmented_string(),
            expected_str
        );
    }

    #[test]
    fn test_concatenate_one_space_token() {
        let input_handlers = vec![TokenHandler::new(TokenType::SpaceToken, String::from(" "))];
        let expected_str = String::from(" ");
        assert_eq!(
            Doc::from_handlers(input_handlers).get_augmented_string(),
            expected_str
        );
    }

    #[test]
    fn test_concatenate_zero_token() {
        let input_handlers = vec![TokenHandler::new(TokenType::SpaceToken, String::from(""))];
        let expected_str = String::from("");
        assert_eq!(
            Doc::from_handlers(input_handlers).get_augmented_string(),
            expected_str
        );
    }

    #[test]
    fn test_concatenate_empty_vec() {
        let input_handlers = vec![];
        let expected_str = String::from("");
        assert_eq!(
            Doc::from_handlers(input_handlers).get_augmented_string(),
            expected_str
        );
    }

    #[test]
    fn test_concatenate_tokens_cyrillic() {
        let input_handlers = vec![
            TokenHandler::new(TokenType::WordToken, String::from("Этот")),
            TokenHandler::new(TokenType::SpaceToken, String::from(" ")),
            TokenHandler::new(TokenType::WordToken, String::from("пр1мер")),
            TokenHandler::new(TokenType::SpaceToken, String::from(" ")),
            TokenHandler::new(TokenType::WordToken, String::from("раб0тает")),
            TokenHandler::new(TokenType::SpecSymbolToken, String::from("?")),
            TokenHandler::new(TokenType::SpecSymbolToken, String::from("!")),
        ];
        let expected_str = String::from("Этот пр1мер раб0тает?!");
        let result = Doc::from_handlers(input_handlers).get_augmented_string();
        assert_eq!(result, expected_str);
    }

    #[test]
    fn test_dont_add_change_in_token_handler() {
        let doc = Doc::new(&String::from("Test example!"));
        assert_eq!(doc.changed_count, 0);
        assert_eq!(doc.get_augmented_string(), String::from("Test example!"))
    }
}
