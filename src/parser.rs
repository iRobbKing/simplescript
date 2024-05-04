mod token;

use std::collections::HashMap;

pub use token::*;

use crate::{Ast, BindingAstNode, BindingKind, IdentifierAstNode, TokenizerOutput};

#[derive(Clone, Copy)]
enum State {
    Start,

    StructFieldsBinding {
        stage: BindingStage,
        names_from: usize,
        count: usize,
        type_name: Option<usize>,
        value: Option<usize>,
    },
}

#[derive(Clone, Copy)]
enum BindingStage {
    Binding,
    Type,
    Value,
}

pub struct Parser<'a, 't, 's> {
    text: &'a str,
    input: &'t TokenizerOutput,
    output: &'s mut Ast,
}

impl<'a, 't, 's> Parser<'a, 't, 's> {
    pub fn new(text: &'a str, input: &'t TokenizerOutput, output: &'s mut Ast) -> Self {
        Self {
            text,
            input,
            output,
        }
    }

    pub fn parse_ast(&mut self) {
        let mut identifiers: HashMap<usize, &'a str> = HashMap::new();
        let mut offset = 0;
        let mut state = State::Start;

        for (token, current_offset) in self.input.iter() {
            let token_value = self.text.split_at(offset).1.split_at(current_offset).0;
            offset += current_offset;

            match &mut state {
                State::Start => match token {
                    Token::AnonimusStructOpening => {
                        state = State::StructFieldsBinding {
                            stage: BindingStage::Binding,
                            names_from: identifiers.len(),
                            count: 0,
                            type_name: None,
                            value: None,
                        };
                    }

                    Token::Identifier => {
                        let identifier_id = identifiers.len();
                        identifiers.insert(identifier_id, token_value);
                        let node = IdentifierAstNode { identifier_id };
                        self.output.push_identifier(node)
                    }

                    _ => panic!("unexpected token"),
                },

                State::StructFieldsBinding {
                    stage, names_from, count, ..
                } => match token {
                    Token::Identifier => {
                        let identifier_id = identifiers.len();
                        identifiers.insert(identifier_id, token_value);
                        *count += 1;
                    }

                    Token::RightCurly => {
                        *stage = BindingStage::Type;
                    }
                },
            }
        }
    }
}
