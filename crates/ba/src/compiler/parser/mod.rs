use std::iter::Peekable;

use ast::{token_to_button, token_to_float, token_to_position, token_to_string};

use super::{lexer::Lexer, Token, TokenKind};
use crate::TK;

mod button;
mod ast;

pub struct Parser<'input, I>
where
    I: Iterator<Item = Token>,
{
    input:  &'input str,
    tokens: Peekable<I>,
}

impl<'input> Parser<'input, TokenIter<'input>> {
    pub fn new(input: &'input str)
    -> Parser<'input, TokenIter<'input>> 
    {
        Parser {
            input,
            tokens: TokenIter::new(input).peekable(),
        }
    }
}

impl<'input, I> Parser<'input, I>
where I: Iterator<Item = Token>,
{
    // Get the source text of a token.
    fn text(&self, token: Token) -> &'input str {
        token.text(&self.input)
    }

    // Look-ahead one token and see what kind of token it is.
    fn peek(&mut self) -> TokenKind {
        self.tokens.peek().map(|token| token.kind).unwrap_or(TK![EOF])
    }

    // Check if the next token is some `kind` of token.
    fn at(&mut self, kind: TokenKind) -> bool {
        self.peek() == kind
    }

    // Get the next token.
    fn next(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    // Move forward one token in the input and check 
    // that we pass the kind of token we expect.
    fn consume(&mut self, expected: TokenKind) {
        let token = self.next().expect(&format!(
            "Expected to consume `{}`, but there was no next token",
            expected
        ));
        assert_eq!(
            token.kind, expected,
            "Expected to consume `{}`, but found `{}`",
            expected, token.kind
        );
    }

    pub fn parse_expression(&mut self) -> ast::Expression {
        match self.peek() {
            TK![def] => {
                self.consume(TK![def]);
                let name_token = self.next().expect("missing identifier for defintion");
                let name = self.text(name_token).to_uppercase();
                self.consume(TK![=]);
                match name.as_str() {
                    "RESOLUTION" => {
                        let resolution = token_to_position(self.next().unwrap(), &self.input).unwrap();
                        ast::Expression::Resolution(resolution)
                    },
                    "DELAY_BETWEEN_ACTIONS" => {
                        let milliseconds = token_to_float(self.next().unwrap(), &self.input).unwrap() as u64;
                        ast::Expression::DelayBetweenActions(milliseconds)
                    },
                    "GLOBAL_HALT_KEY" => {
                        let button = token_to_button(self.next().unwrap(), &self.input).unwrap();
                        ast::Expression::GlobalHaltButton(button)
                    },
                    _ => {
                        panic!("unknown")
                    }
                }
            }
            TK![Tap] => {
                self.consume(TK![Tap]);
                let button = token_to_button(self.next().unwrap(), &self.input).unwrap();
                ast::Expression::Tap(button)
            },
            TK![Press] => {
                self.consume(TK![Press]);
                let button = token_to_button(self.next().unwrap(), &self.input).unwrap();
                ast::Expression::Press(button)
            },
            TK![Release] => {
                self.consume(TK![Release]);
                let button = token_to_button(self.next().unwrap(), &self.input).unwrap();
                ast::Expression::Release(button)
            },
            TK![Sleep] => {
                self.consume(TK![Sleep]);
                let time = token_to_float(self.next().unwrap(), &self.input).unwrap();
                ast::Expression::Sleep(time)
            },
            TK![Type] => {
                self.consume(TK![Type]);
                let string = token_to_string(self.next().unwrap(), &self.input).unwrap();
                ast::Expression::Type(string)
            }
            TK![EOI] => {
                self.consume(TK![EOI]);
                self.parse_expression()
            }
            undefined => {
                panic!("undefined behavior for TokenKind '{:?}'", undefined)
            }
        }
    }
}

pub struct TokenIter<'input> {
    lexer: Lexer<'input>,
}

impl<'input> TokenIter<'input> {
    fn new(input: &'input str) -> Self {
        Self { lexer: Lexer::new(input) }
    }
}

impl<'input> Iterator for TokenIter<'input> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next_token = self.lexer.next()?;
            if !matches!(next_token.kind, TK![ws] | TK![Comment]) {
                return Some(next_token);
            }
        }
    }
}

#[cfg(test)]
mod tests {

}