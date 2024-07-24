use inputbot::{KeybdKey, MouseButton};

#[derive(Debug, Clone, Copy)]
pub(super) enum Button {
    K(KeybdKey),
    M(MouseButton),
}

impl TryFrom<&str> for Button {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.to_lowercase();
        if let Some(key) = crate::keymap::KeyMap::get().get(&value) {
            Ok(Button::K(key.clone()))
        } else if let Some(button) = crate::mousemap::MouseMap::get().get(&value) {
            Ok(Button::M(button.clone()))
        } else {
            Err(anyhow::anyhow!("No key or mouse button found associated with '{}'", value))
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
    pub(super) fn bind<F: Fn() + Send + Sync + 'static>(self, callback: F) {
        match self {
            Self::K(a) => a.bind(callback),
            Self::M(a) => a.bind(callback),
        }
    }
    pub(super) fn unbind(self) {
        match self {
            Self::K(a) => a.unbind(),
            Self::M(a) => a.unbind(),
        }
    }
}