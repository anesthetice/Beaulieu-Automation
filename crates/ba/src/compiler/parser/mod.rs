use std::iter::Peekable;

use ast::{token_to_button, token_to_float, token_to_position, token_to_string};

use super::{expression::Expression, lexer::Lexer, Token, TokenKind};
use crate::TK;

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

    // Get the next token.
    fn next(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    fn consume(&mut self, expected: TokenKind) -> anyhow::Result<Token> {
        let token = self.next().ok_or(anyhow::anyhow!("Expected to consume '{}', but there was no next token", expected))?;
        if token.kind != expected {
            Err(anyhow::anyhow!("Expected to consume '{}', but found '{}'", expected, token.kind))?;
        }
        Ok(token)
    }

    pub fn process(&mut self) -> anyhow::Result<Vec<Expression>> {
        let mut expressions: Vec<Expression> = Vec::new();
        while let Some(expr) = self.parse_expression()? {
            expressions.push(expr);
        }
        Ok(expressions)
    }

    fn parse_expression(&mut self) -> anyhow::Result<Option<Expression>> {
        match self.peek() 
        {
            TK![def] => {
                let _ = self.consume(TK![def])?;
                let name_token = self.consume(TK![Word])?;
                let name = self.text(name_token).to_uppercase();
                let _ = self.consume(TK![=]);
                match name.as_str() {
                    "RESOLUTION" => {
                        let resolution = token_to_position(self.consume(TK![Position])?, &self.input)?;
                        Ok(Some(Expression::Resolution(resolution)))
                    },
                    "DELAY_BETWEEN_ACTIONS" => {
                        let milliseconds = token_to_float(self.consume(TK![Float])?, &self.input)? as u64;
                        Ok(Some(Expression::DelayBetweenActions(milliseconds)))
                    },
                    "GLOBAL_HALT_BUTTON" => {
                        let button = token_to_button(self.consume(TK![Word])?, &self.input)?;
                        Ok(Some(Expression::GlobalHaltButton(button)))
                    },
                    _ => {
                        tracing::error!("Failed to assing the unknown global definition '{}'", &name);
                        Err(anyhow::anyhow!("Unkown definition"))
                    }
                }
            }
            TK![Move] => {
                let _ = self.consume(TK![Move]);
                let position = token_to_position(self.consume(TK![Position])?, &self.input)?;
                Ok(Some(Expression::Move(position)))
            },
            TK![Tap] => {
                let _ = self.consume(TK![Tap]);
                let button = token_to_button(self.consume(TK![Word])?, &self.input)?;
                Ok(Some(Expression::Tap(button)))
            },
            TK![Press] => {
                let _ = self.consume(TK![Press]);
                let button = token_to_button(self.consume(TK![Word])?, &self.input)?;
                Ok(Some(Expression::Press(button)))
            },
            TK![Release] => {
                let _ = self.consume(TK![Release]);
                let button = token_to_button(self.consume(TK![Word])?, &self.input)?;
                Ok(Some(Expression::Release(button)))
            },
            TK![Sleep] => {
                let _ = self.consume(TK![Sleep]);
                let time = token_to_float(self.consume(TK![Float])?, &self.input).unwrap();
                Ok(Some(Expression::Sleep(time)))
            },
            TK![Type] => {
                let _ = self.consume(TK![Type]);
                let string = token_to_string(self.consume(TK![String])?, &self.input).unwrap();
                Ok(Some(Expression::Type(string)))
            }
            TK![EOI] => {
                let _ = self.consume(TK![EOI]);
                self.parse_expression()
            }
            TK![EOF] => {
                Ok(None)
            }
            undefined => {
                tracing::error!("Undefined parsing behavior for TokenKind '{:?}'", undefined);
                Err(anyhow::anyhow!("Parsing failed"))
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