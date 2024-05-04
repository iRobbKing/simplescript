use std::process::id;

use crate::Token;

pub struct Ast {
    nodes: Vec<AstNode>,
    number_literals: Vec<NumberLiteralAstNode>,
    bindings: Vec<BindingAstNode>,
    identifiers: Vec<IdentifierAstNode>,
    binary_expressions: Vec<BinaryExpressionAstNode>,
}

impl Ast {
    pub fn new() -> Self {
        Self::new_in(Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new())
    }

    pub fn new_in(
        nodes: Vec<AstNode>,
        number_literals: Vec<NumberLiteralAstNode>,
        bindings: Vec<BindingAstNode>,
        identifiers: Vec<IdentifierAstNode>,
        binary_expressions: Vec<BinaryExpressionAstNode>,
    ) -> Self {
        Self {
            nodes,
            number_literals,
            bindings,
            identifiers,
            binary_expressions,
        }
    }

    pub(crate) fn push_number_literal(&mut self, number_literal: NumberLiteralAstNode) {
        let extra = self.number_literals.len();
        let node = AstNode {
            kind: AstNodeKind::NumberLiteral,
            extra,
        };
        self.nodes.push(node);
        self.number_literals.push(number_literal);
    }

    pub(crate) fn push_binding(&mut self, binding: BindingAstNode) {
        let extra = self.bindings.len();
        let node = AstNode {
            kind: AstNodeKind::Binding,
            extra,
        };
        self.nodes.push(node);
        self.bindings.push(binding);
    }

    pub(crate) fn push_identifier(&mut self, identifier: IdentifierAstNode) {
        let extra = self.identifiers.len();
        let node = AstNode {
            kind: AstNodeKind::Identifier,
            extra,
        };
        self.nodes.push(node);
        self.identifiers.push(identifier);
    }

    pub(crate) fn push_binary_expression(&mut self, binary_expression: BinaryExpressionAstNode) {
        let extra = self.binary_expressions.len();
        let node = AstNode {
            kind: AstNodeKind::BinaryExpression,
            extra,
        };
        self.nodes.push(node);
        self.binary_expressions.push(binary_expression);
    }
}

pub struct AstNode {
    kind: AstNodeKind,
    extra: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum AstNodeKind {
    NumberLiteral,
    Binding,
    Identifier,
    BinaryExpression,
}

#[derive(Debug, Clone, Copy)]
pub struct NumberLiteralAstNode(pub f64);

#[derive(Debug, Clone, Copy)]
pub struct BindingAstNode {
    pub kind: BindingKind,
    pub new_identifier_id: usize,
    pub type_identifier_id: Option<usize>,
    pub value_ast_node: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum BindingKind {
    WholeValue,
    StructField { field_name_id: usize },
}

#[derive(Debug, Clone, Copy)]
pub struct IdentifierAstNode {
    pub identifier_id: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct BinaryExpressionAstNode {
    pub kind: BinaryExpressionKind,
    pub left_node: usize,
    pub right_node: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryExpressionKind {
    Sum,
    Sub,
    Mul,
    Div,
}
