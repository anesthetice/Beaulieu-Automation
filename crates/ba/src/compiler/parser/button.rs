use inputbot::{KeybdKey, MouseButton};
use std::{path::Path, collections::HashMap, sync::OnceLock};

static STR_TO_KEYBD_MAP: OnceLock<HashMap<String, KeybdKey>> = OnceLock::new();
static STR_TO_MOUSE_MAP: OnceLock<HashMap<String, KeybdKey>> = OnceLock::new();

enum Button {
    M(MouseButton),
    K(KeybdKey),
}

impl Button {
    fn init(keymap_path: &Path, mousemap_path: &Path) {

    }
}