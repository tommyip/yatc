/// Parser
///
/// A hand-coded recursive descent parser.
///
/// Grammar of yatc:
/// ```
/// program     ::= statement .
/// statement   ::= expression ";" .
/// expression  ::= term expression' .
/// expression' ::= + term expression'
///               | - term expression'
///               | e .
/// term        ::= factor term' .
/// term'       ::= * factor term'
///               | / factor term'
///               | e .
/// factor      ::= "(" expression ")"
///               | Ident
///               | Integer
///               | Float
///               | Str .
/// ```

use std::collections::VecDeque;
use super::Token;

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

    // program ::= statement .
    pub fn parse(&mut self) -> Result<(), String> {
        self.next_token();
        if self.parse_expression()? {
            if self.token == None {
                return Ok(());
            }
        }
        Err(format!("Parser Error: At main program"))
    }

    fn parse_statement(&self) -> Result<bool, String> {
        Ok(true)
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
        if let Some(Token { value, ..}) = self.token {
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

    fn parse_term(&self) -> Result<bool, String> {
        Ok(true)
    }

    fn parse_term_prime(&self) -> Result<bool, String> {
        Ok(true)
    }

    fn parse_factor(&self) -> Result<bool, String> {
        Ok(true)
    }
}
