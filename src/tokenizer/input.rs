use crate::parser::Token;

use super::TokenizerGrammar;

pub struct TokenizerInput {
    text: String,
    grammar: TokenizerGrammar,
}

impl TokenizerInput {
    pub fn new(text: String, grammar: TokenizerGrammar) -> Self {
        Self { text, grammar }
    }

    pub(crate) fn try_parse(&self, offset: usize) -> Option<(Token, usize)> {
        let (_, text) = self.text.split_at(offset);

        if text.is_empty() {
            Some((Token::EndOfFile, 0))
        } else {
            self.grammar.try_parse(text)
        }

    }
}
