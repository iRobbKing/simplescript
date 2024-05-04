use crate::parser::Token;

#[derive(Debug)]
pub struct TokenizerOutput {
    tokens: Vec<Token>,
    offsets: Vec<usize>,
}

impl TokenizerOutput {
    pub fn new() -> Self {
        Self::new_in(Vec::new(), Vec::new())
    }

    pub fn new_in(tokens: Vec<Token>, offsets: Vec<usize>) -> Self {
        Self { tokens, offsets }
    }

    pub(crate) fn push(&mut self, token: Token, offset: usize) {
        self.tokens.push(token);
        self.offsets.push(offset);
    }

    pub(crate) fn iter<'a>(&'a self) -> impl Iterator<Item = (Token, usize)> + 'a {
        self.tokens
            .iter()
            .map(|token| *token)
            .zip(self.offsets.iter().map(|offset| *offset))
    }
}
