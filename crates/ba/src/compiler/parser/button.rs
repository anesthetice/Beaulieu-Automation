use inputbot::{KeybdKey, MouseButton};
pub(super) enum Button {
    K(KeybdKey),
    M(MouseButton),
}

pub(super) enum ButtonAction {
    Press,
    Release,
    Tap,
}

impl Button {
    fn from_str(value: &str) -> Option<Self> {
        let value = value.to_lowercase();
        if let Some(key) = crate::keymap::KeyMap::get().get(&value) {
            Some(Button::K(key.clone()))
        } else if let Some(button) = crate::mousemap::MouseMap::get().get(&value) {
            Some(Button::M(button.clone()))
        } else {
            None
        }
    }
}