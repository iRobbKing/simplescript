mod grammar;
mod input;
mod output;

pub use grammar::*;
pub use input::*;
pub use output::*;

use crate::parser::Token;

pub struct Tokenizer<'o> {
    input: TokenizerInput,
    output: &'o mut TokenizerOutput,
    offset: usize,
}

impl<'o> Tokenizer<'o> {
    pub fn new(
        input: TokenizerInput,
        output: &'o mut TokenizerOutput,
    ) -> Self {
        Self {
            input,
            output,
            offset: 0,
        }
    }

    pub fn parse_tokens(&mut self) {
        loop {
            if let Some((token, offset)) = self.input.try_parse(self.offset) {
                self.output.push(token, self.offset);
                self.offset += offset;

                if token == Token::EndOfFile {
                    break;
                }

                continue;
            }

            self.output.push(Token::BadToken, self.offset);
            self.offset += 1;
        }
    }
}
