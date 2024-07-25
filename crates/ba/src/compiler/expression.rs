use super::button::Button;

#[derive(Debug, Clone)]
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

impl Expression {
    pub(super) fn execute(&self) {
        match self {
            Self::Resolution(_) => tracing::warn!("RESOLUTION definition executed"),
            Self::DelayBetweenActions(_) => {
                tracing::warn!("DELAY_BETWEEN_ACTIONS definition executed")
            }
            Self::GlobalHaltButton(_) => tracing::warn!("GLOBAL_HALT_BUTTON definition executed"),
            Self::Move(pos) => inputbot::MouseCursor::move_abs(pos.0, pos.1),
            Self::Tap(button) => button.tap(),
            Self::Press(button) => button.press(),
            Self::Release(button) => button.release(),
            Self::Sleep(float) => std::thread::sleep(std::time::Duration::from_secs_f64(*float)),
            Self::Type(string) => inputbot::send_sequence(string),
        }
    }

    pub(super) fn is_definition(&self) -> bool {
        matches!(self, Self::Resolution(_) | Self::DelayBetweenActions(_) | Self::GlobalHaltButton(_))
    }
}
