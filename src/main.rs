/// Yet Another Tiny Compiler (yatc)

mod lexer;
mod parser;

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    Symbol,
    Keyword,
    Operator,
    Ident,
    Integer,
    Float,
    Str,
}

/// The position of a token in a stream of characters.
#[derive(Clone, Debug, PartialEq)]
pub struct Span {
    start: usize,
    length: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token<'a> {
    ty: TokenType,
    value: &'a str,
    span: Span,
}

fn main() {
    let src = "( 1 + 2 * ( 42 / 3 ) ) * 7";

    let token_stream = lexer::lex(src).unwrap();
    println!("token_stream: {:?}", token_stream);

    let parse_tree = parser::Parser::new(token_stream).parse();
    println!("ast: {:?}", parse_tree);
}
