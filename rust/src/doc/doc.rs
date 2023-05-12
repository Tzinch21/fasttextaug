use super::changelog::ChangeLog;
use super::token::{Token, TokenType};

const RESERVE_CAPACITY_TO_INSERT_OPERATIONS: usize = 5;

pub struct Doc {
    input: String,
    tokens: Vec<ChangeLog>,
    changed_count: usize,
}

impl Doc {
    pub fn new(input: String) -> Self {
        let tokens = Doc::tokenize(&input);
        let changed_count = 0;
        Doc {
            input,
            tokens,
            changed_count,
        }
    }

    fn tokenize(text: &String) -> Vec<ChangeLog> {
        let mut pos: usize = 0;
        let mut res: Vec<ChangeLog> =
            Vec::with_capacity(text.len() + RESERVE_CAPACITY_TO_INSERT_OPERATIONS);
        for (idx, c) in text.match_indices(|c| !char::is_alphanumeric(c)) {
            let previous_word = text.get(pos..idx);
            if let Some(word) = previous_word {
                if word.len() > 0 {
                    res.push(ChangeLog::new(
                        TokenType::WordToken,
                        String::from(word),
                        None,
                    ));
                    pos = idx;
                }
            }
            if c.len() > 0 {
                let flag = c.chars().next().unwrap().is_whitespace();
                let kind = match flag {
                    true => TokenType::SpaceToken,
                    false => TokenType::SpecSymbolToken,
                };
                res.push(ChangeLog::new(kind, String::from(c), None));
                pos += c.len();
            }
        }
        let remains = text.get(pos..);
        if let Some(value) = remains {
            if value.len() > 0 {
                res.push(ChangeLog::new(
                    TokenType::WordToken,
                    String::from(value),
                    None,
                ));
            }
        }
        res.shrink_to(res.len() + RESERVE_CAPACITY_TO_INSERT_OPERATIONS);
        res
    }

    fn concatenate_tokens(vec_tokens: Vec<&Token>) -> String {
        let total_tokens_len = vec_tokens.iter().map(|t| t.len()).sum();
        let mut concated_str = String::with_capacity(total_tokens_len);
        vec_tokens
            .iter()
            .for_each(|t| concated_str.push_str(&t.token()));
        concated_str
    }

    pub fn get_word_tokens(&mut self, include_special_char: bool) -> Vec<&mut ChangeLog> {
        let mut word_tokens = Vec::with_capacity(self.tokens.len());
        for token in self.tokens.iter_mut() {
            let token_type = token.get_original().kind();
            match (token_type, include_special_char) {
                (TokenType::WordToken, _) => word_tokens.push(token),
                (TokenType::SpecSymbolToken, true) => word_tokens.push(token),
                (_, _) => (),
            }
        }
        word_tokens.shrink_to_fit();
        word_tokens
    }

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

    pub fn add_change_in_token_log(
        &mut self,
        token_idx: usize,
        new_token_str: String,
        change_seq: usize,
        kind_token: Option<TokenType>,
    ) {
        let kind = kind_token.unwrap_or(TokenType::WordToken);
        let change_log = self.tokens.get_mut(token_idx);
        if let Some(log) = change_log {
            log.add_change(kind, new_token_str, change_seq);
            self.changed_count += 1;
        }
    }

    pub fn get_original_tokens(&self) -> Vec<&Token> {
        self.tokens.iter().map(|ch| ch.get_original()).collect()
    }

    pub fn get_augmented_tokens(&self) -> Vec<&Token> {
        self.tokens.iter().map(|ch| ch.get_latest()).collect()
    }

    pub fn get_augmented_string(&self) -> String {
        Doc::concatenate_tokens(self.get_augmented_tokens())
    }

    pub fn get_changed_count(&self) -> usize {
        self.changed_count
    }

    pub fn set_change_count(&mut self, value: usize) -> () {
        self.changed_count = value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Doc {
        fn from_logs(logs: Vec<ChangeLog>) -> Self {
            Doc {
                input: String::new(),
                tokens: logs,
                changed_count: 0,
            }
        }
    }

    #[test]
    fn test_tokenize() {
        let input_str = String::from("My example sentence?!. ");
        let expected_logs = vec![
            ChangeLog::new(TokenType::WordToken, String::from("My"), None),
            ChangeLog::new(TokenType::SpaceToken, String::from(" "), None),
            ChangeLog::new(TokenType::WordToken, String::from("example"), None),
            ChangeLog::new(TokenType::SpaceToken, String::from(" "), None),
            ChangeLog::new(TokenType::WordToken, String::from("sentence"), None),
            ChangeLog::new(TokenType::SpecSymbolToken, String::from("?"), None),
            ChangeLog::new(TokenType::SpecSymbolToken, String::from("!"), None),
            ChangeLog::new(TokenType::SpecSymbolToken, String::from("."), None),
            ChangeLog::new(TokenType::SpaceToken, String::from(" "), None),
        ];
        assert_eq!(Doc::tokenize(&input_str), expected_logs);

        let mut doc = Doc::new(input_str);
        assert_eq!(doc.tokens, expected_logs);
        let word_token_len = doc.get_word_tokens(false).len();
        assert_eq!(word_token_len, 3);
        let word_spec_token_len = doc.get_word_tokens(true).len();
        assert_eq!(word_spec_token_len, 6);
    }

    #[test]
    fn test_tokenize_cyrillic() {
        let input_str = String::from("Пр0веряем раб0ту с кирилицей! .");
        let expected_logs = vec![
            ChangeLog::new(TokenType::WordToken, String::from("Пр0веряем"), None),
            ChangeLog::new(TokenType::SpaceToken, String::from(" "), None),
            ChangeLog::new(TokenType::WordToken, String::from("раб0ту"), None),
            ChangeLog::new(TokenType::SpaceToken, String::from(" "), None),
            ChangeLog::new(TokenType::WordToken, String::from("с"), None),
            ChangeLog::new(TokenType::SpaceToken, String::from(" "), None),
            ChangeLog::new(TokenType::WordToken, String::from("кирилицей"), None),
            ChangeLog::new(TokenType::SpecSymbolToken, String::from("!"), None),
            ChangeLog::new(TokenType::SpaceToken, String::from(" "), None),
            ChangeLog::new(TokenType::SpecSymbolToken, String::from("."), None),
        ];
        assert_eq!(Doc::tokenize(&input_str), expected_logs);

        let mut doc = Doc::new(input_str);
        assert_eq!(doc.tokens, expected_logs);
        let word_token_len = doc.get_word_tokens(false).len();
        assert_eq!(word_token_len, 4);
        let word_spec_token_len = doc.get_word_tokens(true).len();
        assert_eq!(word_spec_token_len, 6);
    }

    #[test]
    fn test_tokenize_one_more() {
        let input_str = String::from("Create tokens");
        let expected_logs = vec![
            ChangeLog::new(TokenType::WordToken, String::from("Create"), None),
            ChangeLog::new(TokenType::SpaceToken, String::from(" "), None),
            ChangeLog::new(TokenType::WordToken, String::from("tokens"), None),
        ];
        assert_eq!(Doc::tokenize(&input_str), expected_logs);

        let mut doc = Doc::new(input_str);
        assert_eq!(doc.tokens, expected_logs);
        let word_token_len = doc.get_word_tokens(false).len();
        assert_eq!(word_token_len, 2);
        let word_spec_token_len = doc.get_word_tokens(true).len();
        assert_eq!(word_spec_token_len, 2);
    }

    #[test]
    fn test_tokenize_one_more_cyrillic() {
        let input_str = String::from("Делаем токены");
        let expected_logs = vec![
            ChangeLog::new(TokenType::WordToken, String::from("Делаем"), None),
            ChangeLog::new(TokenType::SpaceToken, String::from(" "), None),
            ChangeLog::new(TokenType::WordToken, String::from("токены"), None),
        ];
        assert_eq!(Doc::tokenize(&input_str), expected_logs);

        let mut doc = Doc::new(input_str);
        assert_eq!(doc.tokens, expected_logs);
        let word_token_len = doc.get_word_tokens(false).len();
        assert_eq!(word_token_len, 2);
        let word_spec_token_len = doc.get_word_tokens(true).len();
        assert_eq!(word_spec_token_len, 2);
    }

    #[test]
    fn test_tokenize_one_word() {
        let input_str = String::from("example");
        let expected_logs = vec![ChangeLog::new(
            TokenType::WordToken,
            String::from("example"),
            None,
        )];
        assert_eq!(Doc::tokenize(&input_str), expected_logs);

        let mut doc = Doc::new(input_str);
        assert_eq!(doc.tokens, expected_logs);
        let word_token_len = doc.get_word_tokens(false).len();
        assert_eq!(word_token_len, 1);
        let word_spec_token_len = doc.get_word_tokens(true).len();
        assert_eq!(word_spec_token_len, 1);
    }

    #[test]
    fn test_tokenize_only_spec() {
        let input_str = String::from("!@#");
        let expected_logs = vec![
            ChangeLog::new(TokenType::SpecSymbolToken, String::from("!"), None),
            ChangeLog::new(TokenType::SpecSymbolToken, String::from("@"), None),
            ChangeLog::new(TokenType::SpecSymbolToken, String::from("#"), None),
        ];
        assert_eq!(Doc::tokenize(&input_str), expected_logs);

        let mut doc = Doc::new(input_str);
        assert_eq!(doc.tokens, expected_logs);
        let word_token_len = doc.get_word_tokens(false).len();
        assert_eq!(word_token_len, 0);
        let word_spec_token_len = doc.get_word_tokens(true).len();
        assert_eq!(word_spec_token_len, 3);
    }

    #[test]
    fn test_tokenize_spaces() {
        let input_str = String::from("   ");
        let expected_logs = vec![
            ChangeLog::new(TokenType::SpaceToken, String::from(" "), None),
            ChangeLog::new(TokenType::SpaceToken, String::from(" "), None),
            ChangeLog::new(TokenType::SpaceToken, String::from(" "), None),
        ];
        assert_eq!(Doc::tokenize(&input_str), expected_logs);

        let mut doc = Doc::new(input_str);
        assert_eq!(doc.tokens, expected_logs);
        let word_token_len = doc.get_word_tokens(false).len();
        assert_eq!(word_token_len, 0);
        let word_spec_token_len = doc.get_word_tokens(true).len();
        assert_eq!(word_spec_token_len, 0);
    }

    #[test]
    fn test_tokenize_empty() {
        let input_str = String::from("");
        let expected_logs: Vec<ChangeLog> = vec![];
        assert_eq!(Doc::tokenize(&input_str), expected_logs);

        let mut doc = Doc::new(input_str);
        assert_eq!(doc.tokens, expected_logs);
        let word_token_len = doc.get_word_tokens(false).len();
        assert_eq!(word_token_len, 0);
        let word_spec_token_len = doc.get_word_tokens(true).len();
        assert_eq!(word_spec_token_len, 0);
    }

    #[test]
    fn test_concatenate_tokens() {
        let input_logs = vec![
            ChangeLog::new(TokenType::WordToken, String::from("My"), None),
            ChangeLog::new(TokenType::SpaceToken, String::from(" "), None),
            ChangeLog::new(TokenType::WordToken, String::from("example"), None),
            ChangeLog::new(TokenType::SpaceToken, String::from(" "), None),
            ChangeLog::new(TokenType::WordToken, String::from("sentence"), None),
            ChangeLog::new(TokenType::SpecSymbolToken, String::from("?"), None),
            ChangeLog::new(TokenType::SpecSymbolToken, String::from("!"), None),
            ChangeLog::new(TokenType::SpecSymbolToken, String::from("."), None),
            ChangeLog::new(TokenType::SpaceToken, String::from(" "), None),
        ];
        let expected_str = String::from("My example sentence?!. ");
        let result = Doc::from_logs(input_logs).get_augmented_string();
        assert_eq!(result, expected_str);
    }

    #[test]
    fn test_concatenate_one_token() {
        let input_logs = vec![ChangeLog::new(
            TokenType::WordToken,
            String::from("example"),
            None,
        )];
        let expected_str = String::from("example");
        assert_eq!(
            Doc::from_logs(input_logs).get_augmented_string(),
            expected_str
        );
    }

    #[test]
    fn test_concatenate_one_space_token() {
        let input_logs = vec![ChangeLog::new(
            TokenType::SpaceToken,
            String::from(" "),
            None,
        )];
        let expected_str = String::from(" ");
        assert_eq!(
            Doc::from_logs(input_logs).get_augmented_string(),
            expected_str
        );
    }

    #[test]
    fn test_concatenate_zero_token() {
        let input_logs = vec![ChangeLog::new(
            TokenType::SpaceToken,
            String::from(""),
            None,
        )];
        let expected_str = String::from("");
        assert_eq!(
            Doc::from_logs(input_logs).get_augmented_string(),
            expected_str
        );
    }

    #[test]
    fn test_concatenate_empty_vec() {
        let input_logs = vec![];
        let expected_str = String::from("");
        assert_eq!(
            Doc::from_logs(input_logs).get_augmented_string(),
            expected_str
        );
    }

    #[test]
    fn test_concatenate_tokens_cyrillic() {
        let input_logs = vec![
            ChangeLog::new(TokenType::WordToken, String::from("Этот"), None),
            ChangeLog::new(TokenType::SpaceToken, String::from(" "), None),
            ChangeLog::new(TokenType::WordToken, String::from("пр1мер"), None),
            ChangeLog::new(TokenType::SpaceToken, String::from(" "), None),
            ChangeLog::new(TokenType::WordToken, String::from("раб0тает"), None),
            ChangeLog::new(TokenType::SpecSymbolToken, String::from("?"), None),
            ChangeLog::new(TokenType::SpecSymbolToken, String::from("!"), None),
        ];
        let expected_str = String::from("Этот пр1мер раб0тает?!");
        let result = Doc::from_logs(input_logs).get_augmented_string();
        assert_eq!(result, expected_str);
    }

    #[test]
    fn test_dont_add_change_in_token_log() {
        let doc = Doc::new(String::from("Test example!"));
        assert_eq!(doc.changed_count, 0);
        assert_eq!(doc.get_augmented_string(), String::from("Test example!"))
    }

    #[test]
    fn test_add_change_in_token_log() {
        let mut doc = Doc::new(String::from("Test example!"));
        doc.add_change_in_token_log(2, String::from("cat"), 1, None);
        assert_eq!(doc.changed_count, 1);
        assert_eq!(doc.get_augmented_string(), String::from("Test cat!"))
    }

    #[test]
    fn test_add_change_outside_array_in_token_log() {
        let mut doc = Doc::new(String::from("Test example!"));
        doc.add_change_in_token_log(7, String::from("cat"), 1, None);
        assert_eq!(doc.changed_count, 0);
        assert_eq!(doc.get_augmented_string(), String::from("Test example!"))
    }
}
