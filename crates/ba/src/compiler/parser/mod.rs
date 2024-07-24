use std::iter::Peekable;

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
    pub fn text(&self, token: Token) -> &'input str {
        token.text(&self.input)
    }

    // Look-ahead one token and see what kind of token it is.
    pub fn peek(&mut self) -> TokenKind {
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
    pub fn consume(&mut self, expected: TokenKind) {
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
            /*
            TK![def] => {
                self.consume(TK![def]);
                let name = self.next().expect("missing identifier for defintion");
                let name = self.text(name).to_uppercase();
                self.consume(TK![word]);
                match name {

                }
            }
            */
            _ => unreachable!()
        }
    }
}

pub struct TokenIter<'input> {
    lexer: Lexer<'input>,
}

impl<'input> TokenIter<'input> {
    pub fn new(input: &'input str) -> Self {
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