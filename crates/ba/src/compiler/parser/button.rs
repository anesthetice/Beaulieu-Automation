use inputbot::{KeybdKey, MouseButton};

#[derive(Debug)]
pub(super) enum Button {
    K(KeybdKey),
    M(MouseButton),
}

pub(super) enum ButtonAction {
    Press,
    Release,
    Tap,
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