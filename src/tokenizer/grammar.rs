use crate::parser::Token;

pub struct TokenizerGrammar {
    strings_to_tokens: Box<[(Token, usize, String)]>,
}

impl TokenizerGrammar {
    pub fn new(strings_to_tokens: Vec<(String, Token)>) -> Self {
        let strings_to_tokens = strings_to_tokens
            .into_iter()
            .map(|(str, token)| (token, str.chars().count(), str))
            .collect::<Vec<_>>()
            .into_boxed_slice();

        Self { strings_to_tokens }
    }

    pub(crate) fn try_parse(&self, text: &str) -> Option<(Token, usize)> {
        self.try_parse_from_map(text).or_else(|| {
            self.try_parse_identifier(text)
                .map(|offset| (Token::Identifier, offset))
                .or_else(|| {
                    self.try_parse_number_literal(text)
                        .map(|offset| (Token::NumberLiteral, offset))
                })
        })
    }

    fn try_parse_from_map(&self, text: &str) -> Option<(Token, usize)> {
        self.strings_to_tokens
            .iter()
            .find_map(|(token, offset, str)| text.starts_with(str).then_some((*token, *offset)))
    }

    fn try_parse_identifier(&self, text: &str) -> Option<usize> {
        text.starts_with(|c: char| c.is_alphabetic()).then_some(
            text.chars()
                .take_while(|c| c.is_alphanumeric() || *c == '_')
                .count(),
        )
    }

    fn try_parse_number_literal(&self, text: &str) -> Option<usize> {
        text.starts_with(|c: char| c.is_digit(10)).then_some(
            text.chars()
                .take_while(|c| c.is_digit(10) || *c == '_')
                .count(),
        )
    }
}
