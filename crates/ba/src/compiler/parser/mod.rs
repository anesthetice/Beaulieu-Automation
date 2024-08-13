use std::iter::Peekable;

use super::{expression::Expression, lexer::Lexer, Span, Token, TokenKind};
use crate::TK;
use ast::{token_to_button, token_to_float, token_to_position, token_to_string};

mod ast;

pub struct Parser<'input, I>
where
    I: Iterator<Item = Token>,
{
    input: &'input str,
    tokens: Peekable<I>,
}

impl<'input> Parser<'input, TokenIter<'input>> {
    pub fn new(input: &'input str) -> Parser<'input, TokenIter<'input>> {
        Parser {
            input,
            tokens: TokenIter::new(input).peekable(),
        }
    }
}

impl<'input, I> Parser<'input, I>
where
    I: Iterator<Item = Token>,
{
    // Get the source text of a token.
    fn text(&self, token: Token) -> &'input str {
        token.text(self.input)
    }

    // Look-ahead one token and see what kind of token it is.
    fn peek(&mut self) -> TokenKind {
        self.tokens
            .peek()
            .map(|token| token.kind)
            .unwrap_or(TK![EOF])
    }

    // Get the next token.
    fn next(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    fn consume(&mut self, expected: TokenKind) -> anyhow::Result<Token> {
        let token = self.next().ok_or(anyhow::anyhow!(
            "Expected to consume '{}', but there was no next token",
            expected
        ))?;
        if token.kind != expected {
            Err(anyhow::anyhow!(
                "Expected to consume '{}', but found '{}'",
                expected,
                token.kind
            ))?;
        }
        Ok(token)
    }

    pub fn process(&mut self) -> anyhow::Result<Vec<Expression>> {
        let mut expressions: Vec<Expression> = Vec::new();
        while let Some(expr) = self.parse_expression()? {
            tracing::trace!("Parsed expression '{:?}'", &expr);
            expressions.push(expr);
        }
        Ok(expressions)
    }

    fn parse_expression(&mut self) -> anyhow::Result<Option<Expression>> {
        match self.peek() {
            TK![def] => {
                self.consume(TK![def])?;
                let name_token = self.consume(TK![Word])?;
                let name = self.text(name_token).to_uppercase();
                self.consume(TK![=])?;
                match name.as_str() {
                    "RESOLUTION" => {
                        let resolution =
                            token_to_position(self.consume(TK![Position])?, self.input)?;
                        self.consume(TK![EOI])?;
                        Ok(Some(Expression::Resolution(resolution)))
                    }
                    "DELAY_BETWEEN_ACTIONS" => {
                        let milliseconds =
                            token_to_float(self.consume(TK![Float])?, self.input)? as u64;
                        self.consume(TK![EOI])?;
                        Ok(Some(Expression::DelayBetweenActions(milliseconds)))
                    }
                    "GLOBAL_HALT_KEY" => {
                        let button = token_to_button(self.consume(TK![Word])?, self.input)?;
                        self.consume(TK![EOI])?;
                        Ok(Some(Expression::GlobalHaltKey(button)))
                    }
                    _ => {
                        tracing::error!("Failed to assign the unknown definition '{}'", &name);
                        Err(anyhow::anyhow!("Unkown definition"))
                    }
                }
            }
            TK![Move] => {
                self.consume(TK![Move])?;
                let position = token_to_position(self.consume(TK![Position])?, self.input)?;
                self.consume(TK![EOI])?;
                Ok(Some(Expression::Move(position)))
            }
            TK![Tap] => {
                self.consume(TK![Tap])?;
                let button = token_to_button(self.consume(TK![Word])?, self.input)?;
                self.consume(TK![EOI])?;
                Ok(Some(Expression::Tap(button)))
            }
            TK![Press] => {
                self.consume(TK![Press])?;
                let button = token_to_button(self.consume(TK![Word])?, self.input)?;
                self.consume(TK![EOI])?;
                Ok(Some(Expression::Press(button)))
            }
            TK![Release] => {
                self.consume(TK![Release])?;
                let button = token_to_button(self.consume(TK![Word])?, self.input)?;
                self.consume(TK![EOI])?;
                Ok(Some(Expression::Release(button)))
            }
            TK![Sleep] => {
                self.consume(TK![Sleep])?;
                let time = token_to_float(self.consume(TK![Float])?, self.input).unwrap();
                self.consume(TK![EOI])?;
                Ok(Some(Expression::Sleep(time)))
            }
            TK![Type] => {
                self.consume(TK![Type])?;
                let string = token_to_string(self.consume(TK![String])?, self.input).unwrap();
                self.consume(TK![EOI])?;
                Ok(Some(Expression::Type(string)))
            }
            TK![Await] => {
                self.consume(TK![Await])?;
                let token = self.next().ok_or(anyhow::anyhow!(
                    "Expected either an EOI or Button token after Await, found None"
                ))?;
                match token.kind {
                    TK![EOI] => Ok(Some(Expression::Await)),
                    TK![Word] => Ok(Some(Expression::AwaitKey(token_to_button(
                        token, self.input,
                    )?))),
                    other => Err(anyhow::anyhow!(
                        "Expected either an EOI or Button token after Await, found '{}'",
                        other
                    )),
                }
            }
            TK![Bind] => {
                self.consume(TK![Bind])?;
                let button = token_to_button(self.consume(TK![Word])?, self.input)?;
                self.consume(TK![LBrace])?;

                let mut valid_tokens: Vec<Token> = Vec::new();
                loop {
                    let token = self.next().ok_or_else(|| {
                        tracing::error!("Missing '}}' character, '{{' was never closed");
                        anyhow::anyhow!("Parsing failed")
                    })?;

                    match token.kind {
                        _break @ TK![RBrace] => break,
                        invalid @ TK![LBrace]
                        | invalid @ TK![Await]
                        | invalid @ TK![def]
                        | invalid @ TK![Bind] => {
                            tracing::error!("Invalid token '{}' inside bind", invalid);
                            Err(anyhow::anyhow!("Parsing failed"))?;
                        }
                        _ => valid_tokens.push(token),
                    }
                }

                let span = tracing::span!(tracing::Level::TRACE, "Bind Parsing");
                let _guard = span.enter();
                let inner_expressions: Vec<Expression> = Parser {
                    input: self.input,
                    tokens: valid_tokens.into_iter().peekable(),
                }
                .process()?;
                drop(_guard);

                Ok(Some(Expression::Bind(button, inner_expressions)))
            }
            TK![EOI] => {
                self.consume(TK![EOI])?;
                self.parse_expression()
            }
            TK![EOF] => Ok(None),
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
        Self {
            lexer: Lexer::new(input),
        }
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
    use super::Parser;
    use crate::{
        compiler::{button::Button, expression::Expression},
        keymap, mousemap,
    };

    #[test]
    fn long() {
        keymap::KeyMap::test_init();
        mousemap::MouseMap::test_init();
        let input: &str = "define RESOLUTION = 1920, 1080\nBind NR1 {\n  Move 1070, 234\n  Tap LMB\n}\n Press LMB; Sleep 0.1; Release LMB\nTap Space\nType \"Hello World\"";
        let mut parser = Parser::new(input);
        let expressions = parser.process().unwrap();
        assert_eq!(
            vec![
                Expression::Resolution((1920, 1080)),
                Expression::Bind(
                    Button::K(inputbot::KeybdKey::Numrow1Key),
                    vec![
                        Expression::Move((1070, 234)),
                        Expression::Tap(Button::M(inputbot::MouseButton::LeftButton))
                    ]
                ),
                Expression::Press(Button::M(inputbot::MouseButton::LeftButton)),
                Expression::Sleep(0.1),
                Expression::Release(Button::M(inputbot::MouseButton::LeftButton)),
                Expression::Tap(Button::K(inputbot::KeybdKey::SpaceKey)),
                Expression::Type("Hello World".to_string())
            ],
            expressions
        )
    }
}
