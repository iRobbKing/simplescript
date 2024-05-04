#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    StructKeyword,
    TupleKeyword,
    EnumKeyword,
    Identifier,
    Colon,
    ConstAssignment,
    MutableAssignment,
    FnKeyword,
    LeftParen,
    RightParen,
    LeftCurly,
    RightCurly,
    Comma,
    ThinArrowToRight,
    ReturnKeyword,
    PlusOperator,
    MinusOperator,
    StarOperator,
    SlashOperator,
    EndOfStatement,
    WhiteSpace,
    EndOfFile,
    BadToken,
    NumberLiteral,
    AnonimusStructOpening,
    Dot,
}
