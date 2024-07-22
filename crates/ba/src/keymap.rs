use inputbot::KeybdKey::{self, *};
use std::collections::HashMap;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct KeyMap {
    inner: HashMap<String, KeybdKey>,
}

impl std::ops::Deref for KeyMap {
    type Target = HashMap<String, KeybdKey>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Default for KeyMap {
    fn default() -> Self {
        Self::from(&DEFAULT_KEYMAP[..])
    }
}

impl From<&[(&str, KeybdKey)]> for KeyMap {
    fn from(value: &[(&str, KeybdKey)]) -> Self {
        Self {
            inner: HashMap::from_iter(value.into_iter().map(|(s, k)| (s.to_string(), *k))),
        }
    }
}

static DEFAULT_KEYMAP: [(&'static str, KeybdKey); 70] = [
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
    ("flechehaut", UpKey),
    ("right", RightKey),
    ("rightarrow", RightKey),
    ("droite", RightKey),
    ("flechedroite", RightKey),
    ("down", DownKey),
    ("downarrow", DownKey),
    ("bas", DownKey),
    ("flechebas", DownKey),
    ("insert", InsertKey),
    ("ins", InsertKey),
    ("delete", DeleteKey),
    ("del", DeleteKey),
    ("NR0", Numrow0Key),
    ("NR1", Numrow1Key),
    ("NR2", Numrow2Key),
    ("NR3", Numrow3Key),
    ("NR4", Numrow4Key),
    ("NR5", Numrow5Key),
    ("NR6", Numrow6Key),
    ("NR7", Numrow7Key),
    ("NR8", Numrow8Key),
    ("NR9", Numrow9Key),
    ("a", AKey),
    ("b", BKey),
    ("c", CKey),
    ("d", DKey),
    ("e", EKey),
    ("f", FKey),
    ("g", GKey),
    ("h", HKey),
    ("i", IKey),
    ("j", JKey),
    ("k", KKey),
    ("l", LKey),
    ("m", MKey),
    ("n", NKey),
    ("o", OKey),
    ("p", PKey),
    ("q", QKey),
    ("r", RKey),
    ("s", SKey),
    ("t", TKey),
    ("u", VKey),
    ("v", VKey),
    ("w", WKey),
    ("x", XKey),
    ("y", YKey),
    ("z", ZKey),
];
