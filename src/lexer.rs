/// Lexical analysis phase
///
/// Tokenize a stream of characters into a vector of tokens. For example, the
/// statement `let a = 5 ;` would become:
/// ```
/// [
///     Token { ty: TokenType::Keyword, value: "let", span: Span { start: 0, length: 3 } },
///     Token { ty: TokenType::Ident, value: "a", span: Span { start: 4, length: 1 } },
///     Token { ty: TokenType::Operator, value: "=", span: Span { start: 6, length: 1 } },
///     Token { ty: TokenType::Integer, value: "5", span: Span { start: 8, length: 1 } },
///     Token { ty: TokenType::Symbol, value: ";", span: Span { start: 10, length: 1 } },
/// ]
/// ```

use super::{TokenType, Token, Span};

#[derive(Clone, Debug, PartialEq)]
struct Context<'ctx> {
    src: &'ctx str,
    vec: Vec<char>,
    ptr: usize,
}

impl<'ctx> Context<'ctx> {
    fn new(src: &'ctx str) -> Self {
        let vec = src.chars().collect();
        Context { src, vec, ptr: 0 }
    }

    fn skip_to_<P>(&mut self, f: P)
        where P: Fn(char) -> bool
    {
        while !self.finished() {
            if f(self.vec[self.ptr]) {
                break;
            }
            self.ptr += 1;
        }
    }

    fn finished(&self) -> bool {
        self.ptr >= self.vec.len()
    }
}

pub fn lex<'a>(src: &'a str) -> Result<Vec<Token<'a>>, String> {
    let mut ctx = Context::new(src);
    let mut tokens = vec![];

    while !ctx.finished() {
        let start: usize = ctx.ptr;

        match ctx.vec[ctx.ptr] {
            ' ' | '\n' => {}
            '"' => {
                ctx.ptr += 1;
                ctx.skip_to_(quote);
                if ctx.ptr == ctx.vec.len() {
                    return Err(format!("Unterminated string at position {}", start));
                }
                ctx.ptr += 1;

                tokens.push(Token {
                    ty: TokenType::Str,
                    value: &ctx.src[start..ctx.ptr],
                    span: Span { start, length: ctx.ptr - start },
                });
            }
            '/' => {
                if ctx.ptr == ctx.vec.len() - 1 {
                    return Err(format!("Dangling '/' at position {}", ctx.ptr));
                } else if ctx.vec[ctx.ptr + 1] == '/' {
                    // This is a line comment
                    ctx.skip_to_(newline);
                } else {
                    tokens.push(Token {
                        ty: TokenType::Operator,
                        value: "/",
                        span: Span { start, length: 1 },
                    });
                }
            }
            _ => {
                ctx.skip_to_(blank);
                let value = &ctx.src[start..ctx.ptr];
                let ty;

                ty = if is_symbol(value) {
                    TokenType::Symbol
                } else if is_operator(value) {
                    TokenType::Operator
                } else if is_integer(value) {
                    TokenType::Integer
                } else if is_float(value) {
                    TokenType::Float
                } else if is_keyword(value) {
                    TokenType::Keyword
                } else {
                    TokenType::Ident
                };

                tokens.push(Token {
                    ty, value,
                    span: Span { start, length: ctx.ptr - start },
                });
            }
        }

        ctx.ptr += 1;
    }

    Ok(tokens)
}

fn quote(c: char) -> bool { c == '"' }
fn space(c: char) -> bool { c == ' ' }
fn newline(c: char) -> bool { c == '\n' }
fn blank(c: char) -> bool { space(c) || newline(c) }

fn is_symbol(token: &str) -> bool {
    match token {
        "{" | "}" | "(" | ")" | ";" | "," => true,
        _ => false,
    }
}

fn is_keyword(token: &str) -> bool {
    match token {
        "let" | "if" | "else" | "for" | "fn" | "return" => true,
        _ => false,
    }
}

fn is_operator(token: &str) -> bool {
    match token {
        "+" | "-" | "*" | "/" | "=" | ">" | "<" => true,
        _ => false,
    }
}

fn is_integer(token: &str) -> bool {
    token.parse::<i32>().is_ok()
}

fn is_float(token: &str) -> bool {
    token.parse::<f32>().is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_statement() {
        let src = "let there = \"be lights\" * 13.37 ;";

        let tokens = lex(src);
        assert_eq!(tokens, Ok(vec![
            Token { ty: TokenType::Keyword, value: "let", span: Span { start: 0, length: 3 } },
            Token { ty: TokenType::Ident, value: "there", span: Span { start: 4, length: 5 } },
            Token { ty: TokenType::Operator, value: "=", span: Span { start: 10, length: 1 } },
            Token { ty: TokenType::Str, value: "\"be lights\"", span: Span { start: 12, length: 11 } },
            Token { ty: TokenType::Operator, value: "*", span: Span { start: 24, length: 1 } },
            Token { ty: TokenType::Float, value: "13.37", span: Span { start: 26, length: 5 } },
            Token { ty: TokenType::Symbol, value: ";", span: Span { start: 32, length: 1 } }
       ]));
    }
}
