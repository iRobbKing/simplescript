// TODO: tokenizer input should accept text by reference

use simplescript::{Token, Tokenizer, TokenizerGrammar, TokenizerInput, TokenizerOutput};

fn main() {
    let map = vec![
        ("struct".to_owned(), Token::StructKeyword),
        ("tuple".to_owned(), Token::TupleKeyword),
        ("enum".to_owned(), Token::EnumKeyword),
        ("=".to_owned(), Token::ConstAssignment),
        (":=".to_owned(), Token::MutableAssignment),
        (":".to_owned(), Token::Colon),
        ("fn".to_owned(), Token::FnKeyword),
        ("(".to_owned(), Token::LeftParen),
        (")".to_owned(), Token::RightParen),
        (".{".to_owned(), Token::AnonimusStructOpening),
        ("{".to_owned(), Token::LeftCurly),
        ("}".to_owned(), Token::RightCurly),
        (".".to_owned(), Token::Dot),
        (",".to_owned(), Token::Comma),
        ("return".to_owned(), Token::ReturnKeyword),
        (" ".to_owned(), Token::WhiteSpace),
        ("\n".to_owned(), Token::EndOfStatement),
        ("->".to_owned(), Token::ThinArrowToRight),
        ("-".to_owned(), Token::MinusOperator),
        ("+".to_owned(), Token::PlusOperator),
        ("*".to_owned(), Token::StarOperator),
        ("/".to_owned(), Token::SlashOperator),
    ];

    let code = "added = -two + 42 * 69 - 1337".to_owned();

    let grammar = TokenizerGrammar::new(map);
    let input = TokenizerInput::new(code, grammar);
    let mut output = TokenizerOutput::new();
    let mut tokenizer = Tokenizer::new(input, &mut output);

    tokenizer.parse_tokens();

    println!("{:#?}", &output);
}
