use std::collections::HashMap;
use inputbot::KeybdKey::{self, *};

#[derive(serde::Serialize, serde::Deserialize)]
struct KeyMap{inner: HashMap<String, KeybdKey>}



static DEFAULT_KEYMAP: [(&'static str, KeybdKey); 33] = [
    ("backspace", BackspaceKey),

    ("tab", TabKey),

    ("enter", EnterKey),

    ("escape", EscapeKey),
    ("esc", EscapeKey),

    ("space", SpaceKey),
    ("espace", SpaceKey),

    ("pageup", PageUpKey),
    ("pgup", PageUpKey),

    ("pagedown", PageDownKey),
    ("pgdown", PageDownKey),

    ("end", EndKey),

    ("home", HomeKey),

    ("left", LeftKey),
    ("leftarrow", LeftKey),
    ("gauche", LeftKey),
    ("flèchegauche", LeftKey),
    ("flechegauche", LeftKey),

    ("up", UpKey),
    ("uparrow", UpKey),
    ("haut", UpKey),
    ("flèchehaut", UpKey),
    ("flechehaut", UpKey),

    ("right", RightKey),
    ("rightarrow", RightKey),
    ("droite", RightKey),
    ("flèchedroite", RightKey),
    ("flechedroite", RightKey),

    ("down", DownKey),
    ("downarrow", DownKey),
    ("bas", DownKey),
    ("flèchebas", DownKey),
    ("flechebas", DownKey),
];

impl std::ops::Deref for KeyMap {
    type Target = HashMap<String, KeybdKey>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}