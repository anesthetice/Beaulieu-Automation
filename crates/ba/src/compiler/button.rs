use inputbot::{KeybdKey, MouseButton};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Button {
    K(KeybdKey),
    M(MouseButton),
}

impl TryFrom<&str> for Button {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.to_lowercase();
        if let Some(key) = crate::keymap::KeyMap::get().get(&value) {
            Ok(Button::K(*key))
        } else if let Some(button) = crate::mousemap::MouseMap::get().get(&value) {
            Ok(Button::M(*button))
        } else {
            Err(anyhow::anyhow!(
                "No key or mouse button found associated with '{}'",
                value
            ))
        }
    }
}

impl Button {
    pub(super) fn tap(&self) {
        match self {
            Self::K(a) => a.tap(),
            Self::M(a) => a.tap(),
        }
    }
    pub(super) fn press(&self) {
        match self {
            Self::K(a) => a.press(),
            Self::M(a) => a.press(),
        }
    }
    pub(super) fn release(&self) {
        match self {
            Self::K(a) => a.release(),
            Self::M(a) => a.release(),
        }
    }
    pub(super) fn listen_once<F: FnOnce() + Send + 'static>(
        self,
        callback: F,
    ) -> anyhow::Result<std::thread::JoinHandle<()>> {
        match self {
            Self::K(key) => Ok(key.listen_once(callback)),
            Self::M(_) => {
                tracing::error!("Mouse buttons cannot be bound, use keys instead");
                Err(anyhow::anyhow!("Cannot bind mouse button"))
            }
        }
    }
    pub(super) fn await_in_place(self) -> anyhow::Result<()> {
        match self {
            Self::K(key) => key.await_in_place(),
            Self::M(_) => {
                tracing::error!("Mouse buttons cannot be bound, use keys instead");
                Err(anyhow::anyhow!("Cannot bind mouse button"))
            }
        }
    }
    pub(super) fn detached_hotkey<F: Fn() + Send + 'static>(
        self,
        callback: F,
    ) -> anyhow::Result<()> {
        match self {
            Self::K(key) => {
                key.detached_hotkey(callback);
                Ok(())
            }
            Self::M(_) => {
                tracing::error!("Mouse buttons cannot be bound, use keys instead");
                Err(anyhow::anyhow!("Cannot bind mouse button"))
            }
        }
    }
}
