use super::button::Button;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Resolution((i32, i32)),
    DelayBetweenActions(u64),
    GlobalHaltKey(Button),
    Move((i32, i32)),
    Tap(Button),
    Press(Button),
    Release(Button),
    Sleep(f64),
    Type(String),
    Await,
    Bind(Button, Vec<Expression>),
}

impl Expression {
    pub(super) fn execute(&self) {
        match self {
            // Handled during engine creation
            Self::Resolution(_) => (),
            Self::DelayBetweenActions(_) => (),
            Self::GlobalHaltKey(_) => (),
            Self::Bind(..) => (),

            // Handled directly
            Self::Move(pos) => inputbot::MouseCursor::move_abs(pos.0, pos.1),
            Self::Tap(button) => button.tap(),
            Self::Press(button) => button.press(),
            Self::Release(button) => button.release(),
            Self::Sleep(float) => std::thread::sleep(std::time::Duration::from_secs_f64(*float)),
            Self::Type(string) => inputbot::send_sequence(string),
            
            // Handled seperately by engine
            Self::Await => (),
        }
    }

    pub(super) fn is_definition(&self) -> bool {
        matches!(
            self,
            Self::Resolution(_) | Self::DelayBetweenActions(_) | Self::GlobalHaltKey(_)
        )
    }
}
