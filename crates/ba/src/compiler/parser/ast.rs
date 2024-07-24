use crate::compiler::{Span, Token};
use super::button::Button;

#[derive(Debug)]
pub enum Expression {
    Resolution((i32, i32)),
    DelayBetweenActions(u64),
    GlobalHaltButton(Button),
    Move((i32, i32)),
    Tap(Button),
    Press(Button),
    Release(Button),
    Sleep(f64),
    Type(String),
}

// TokenKind check done by consume
pub(super) fn token_to_button(token: Token, input: &str) -> anyhow::Result<Button> {
    let input = &input[token.span];
    Button::try_from(input)
}

pub(super) fn token_to_position(token: Token, input: &str) -> anyhow::Result<(i32, i32)> {
    let input = &input[token.span];
    // unwrapping because regex rules
    let (width, height) = input.split_once(',').unwrap();
    Ok((width.trim().parse()?, height.trim().parse()?))
}

pub(super) fn token_to_string(token: Token, input: &str) -> anyhow::Result<String> {
    let span_without_quotes = Span {
        start: token.span.start + 1,
        end: token.span.end - 1
    };
    Ok(input[span_without_quotes].to_string())
}

pub(super) fn token_to_float(token: Token, input: &str) -> anyhow::Result<f64> {
    let input = &input[token.span];
    Ok(input.parse()?)
}




