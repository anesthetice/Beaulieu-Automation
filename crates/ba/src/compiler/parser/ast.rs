use super::button::Button;

pub(super) enum Expression {
    Resolution((i32, i32)),
    DelayBetweenActions(u32),
    GlobalHaltButton(Button),
    Move((i32, i32)),
    Tap(Button),
    Press(Button),
    Release(Button),
    Sleep(f64),
    Type(String),
}
