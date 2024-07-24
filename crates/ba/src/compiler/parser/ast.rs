use crate::compiler::{Token, TokenKind};

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

pub(super) fn token_to_button(token: Token, input: &str) -> anyhow::Result<Button> {
    if token.kind != TokenKind::Word {
        tracing::error!("Expected 'Word' got '{:?}'", token.kind);
        return Err(anyhow::anyhow!("Invalid TokenKind"));
    }
    let input = &input[token.span];
    Button::try_from(input)
}

pub(super) fn token_to_position(token: Token, input: &str) -> anyhow::Result<(i32, i32)> {
    if token.kind != TokenKind::Position {
        tracing::error!("Expected 'Position' got '{:?}'", token.kind);
        return Err(anyhow::anyhow!("Invalid TokenKind"));
    }
    let input = &input[token.span];
    // unwrapping because regex rules
    let (width, height) = input.split_once(',').unwrap();
    Ok((width.trim().parse()?, height.trim().parse()?))
}

pub(super) fn token_to_string(token: Token, input: &str) -> anyhow::Result<String> {
    if token.kind != TokenKind::String {
        tracing::error!("Expected 'String' got '{:?}'", token.kind);
        return Err(anyhow::anyhow!("Invalid TokenKind"));
    }
    Ok(input[token.span].to_string())
}

pub(super) fn token_to_float(token: Token, input: &str) -> anyhow::Result<f64> {
    if token.kind != TokenKind::Float {
        tracing::error!("Expected 'Float' got '{:?}'", token.kind);
        return Err(anyhow::anyhow!("Invalid TokenKind"));
    }
    let input = &input[token.span];
    Ok(input.parse()?)
}




