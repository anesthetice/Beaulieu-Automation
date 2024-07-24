use super::expression::Expression;

mod watcher;
use watcher::Watcher;

pub struct Engine {
    inner: Vec<Expression>,
    watcher: Watcher,
    delay_between_actions: std::time::Duration,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            inner: vec![],
            watcher: Watcher::new(super::button::Button::K(inputbot::KeybdKey::EscapeKey)),
            delay_between_actions: std::time::Duration::new(1, 0),
        }
    }

    pub fn start(self) {
        loop {
            if self.watcher.check() {
                self.watcher.clean();
                break;
            }
        }
    }
}

/*
impl TryFrom<Vec<Expression>> for Engine {
    type Error = anyhow::Error;
    fn try_from(value: Vec<Expression>) -> Result<Self, Self::Error> {
    }
}
*/