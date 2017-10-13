/// Parser
///
/// A hand-coded recursive descent parser, build a derivation of the input
/// program and construct a parse tree if successful.
///
/// Full grammar of yatc:
/// ```
/// program ::= expression .
/// expression ::= term expression' .
/// expression' ::= + term expression'
///               | - term expression'
///               | e .
/// term ::= factor term' .
/// term' ::= * factor term'
///         | / factor term'
///         | e .
/// factor := "(" expression ")"
///         | Ident
///         | Integer
///         | Float
///         | Str .
/// ```

use std::collections::VecDeque;
use super::{Token, TokenType};

#[derive(Debug)]
pub struct Parser<'a> {
    token_stream: VecDeque<Token<'a>>,
    ptr: usize,
    token: Option<Token<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(token_stream: VecDeque<Token<'a>>) -> Self {
        Parser { token_stream, ptr: 0, token: None }
    }

    fn next_token(&mut self) {
        self.token = self.token_stream.pop_front();
    }

    // program ::= expression .
    pub fn parse(&mut self) -> Result<(), String> {
        self.next_token();
        if self.parse_expression()? {
            if self.token == None {
                return Ok(());
            }
        }
        Err(format!("Parser Error: At main program"))
    }

    // expression ::= term expression' .
    fn parse_expression(&mut self) -> Result<bool, String> {
        if self.parse_term()? {
            self.parse_expression_prime()
        } else {
            Err(format!("Parser Error: term not found in expr at {:?}", self.token))
        }
    }

    // expression' ::= + term expression'
    //               | - term expression'
    //               | e .
    fn parse_expression_prime(&mut self) -> Result<bool, String> {
        if let Some(Token { value, .. }) = self.token {
            match value {
                "+" | "-" => {
                    self.next_token();
                    if self.parse_term()? {
                        return self.parse_expression_prime();
                    } else {
                        return Err(format!("Parser Error: Expecting term"));
                    }
                }
                ")" => return Ok(true),
                _ => {}
            }
        } else {
            return Ok(true);
        }

        Err(format!("Parser Error: not an expression at {:?}", self.token))
    }

    // term ::= factor term' .
    fn parse_term(&mut self) -> Result<bool, String> {
        if self.parse_factor()? {
            self.parse_term_prime()
        } else {
            Err(format!("Parser Error: something went wrong"))
        }
    }

    // term' ::= * factor term'
    //         | / factor term'
    //         | e .
    fn parse_term_prime(&mut self) -> Result<bool, String> {
        if let Some(Token { value, .. }) = self.token {
            match value {
                "*" | "/" => {
                    self.next_token();
                    if self.parse_factor()? {
                        return self.parse_term_prime();
                    } else {
                        return Err(format!("Parser Error: Expecting a factor"));
                    }
                }
                "+" | "-" | ")" => return Ok(true),
                _ => {}
            }
        } else {
            return Ok(true);
        }

        Err(format!("Parser Error: not an expression at {:?}", self.token))
    }

    // factor ::= "(" expression ")"
    //          | Ident
    //          | Integer
    //          | Float
    //          | Str .
    fn parse_factor(&mut self) -> Result<bool, String> {
        match self.token {
            Some(Token { ty: TokenType::Symbol, value: "(", .. }) => {
                self.next_token();
                if !self.parse_expression()? {
                    return Err(format!("Parser Error: Something went wrong"));
                }
                match self.token {
                    Some(Token { ty: TokenType::Symbol, value: ")", .. }) => {},
                    _ => return Err(format!("Parser Error: Unclosed bracket in factor"))
                }
                self.next_token();
                Ok(true)
            }
            Some(Token { ty: TokenType::Ident, .. })
            | Some(Token { ty: TokenType::Integer, .. })
            | Some(Token { ty: TokenType::Float, .. })
            | Some(Token { ty: TokenType::Str, .. }) => {
                self.next_token();
                Ok(true)
            }
            _ => Err(format!("Parser Error: Expecting factor"))
        }
    }
}
