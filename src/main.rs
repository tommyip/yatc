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
    let src = "";

    let token_stream = lexer::lex(src).unwrap();
    println!("token_stream: {:?}", token_stream);

    let ast = parser::Parser::new(token_stream).parse();
    println!("ast: {:?}", ast);
}
