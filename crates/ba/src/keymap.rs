use anyhow::Context;
use inputbot::KeybdKey::{self, *};
use std::{collections::HashMap, io::Read, path::Path, sync::OnceLock};

static KEYMAP: OnceLock<KeyMap> = OnceLock::new();

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct KeyMap {
    inner: HashMap<String, KeybdKey>,
}

impl KeyMap {
    pub fn init(keymap_filepath: &Path) -> anyhow::Result<()> {
        let mut data = Vec::new();
        std::fs::OpenOptions::new()
            .create(false)
            .read(true)
            .open(&keymap_filepath)
            .context(format!(
                "Failed to read/open file with path '{}'",
                keymap_filepath.display()
            ))?
            .read_to_end(&mut data)?;

        let data: Vec<(String, KeybdKey)> = serde_json::from_slice(&data)?;
        KEYMAP.set(KeyMap::from(data)).unwrap();
        Ok(())
    }

    pub fn get() -> &'static Self {
        KEYMAP.get().unwrap()
    }
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

impl From<Vec<(String, KeybdKey)>> for KeyMap {
    fn from(value: Vec<(String, KeybdKey)>) -> Self {
        Self {
            inner: HashMap::from_iter(value.into_iter()),
        }
    }
}

pub static DEFAULT_KEYMAP: [(&'static str, KeybdKey); 148] = [
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
    ("nr0", Numrow0Key),
    ("nr1", Numrow1Key),
    ("nr2", Numrow2Key),
    ("nr3", Numrow3Key),
    ("nr4", Numrow4Key),
    ("nr5", Numrow5Key),
    ("nr6", Numrow6Key),
    ("nr7", Numrow7Key),
    ("nr8", Numrow8Key),
    ("nr9", Numrow9Key),
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
    ("lsuper", LSuper),
    ("super", LSuper),
    ("windows", LSuper),
    ("win", LSuper),
    ("np0", Numpad0Key),
    ("np1", Numpad1Key),
    ("np2", Numpad2Key),
    ("np3", Numpad3Key),
    ("np4", Numpad4Key),
    ("np5", Numpad5Key),
    ("np6", Numpad6Key),
    ("np7", Numpad7Key),
    ("np8", Numpad8Key),
    ("np9", Numpad9Key),
    ("f1", F1Key),
    ("f2", F2Key),
    ("f3", F3Key),
    ("f4", F4Key),
    ("f5", F5Key),
    ("f6", F6Key),
    ("f7", F7Key),
    ("f8", F8Key),
    ("f9", F9Key),
    ("f10", F10Key),
    ("f11", F11Key),
    ("f12", F12Key),
    ("f13", F13Key),
    ("f14", F14Key),
    ("f15", F15Key),
    ("f16", F16Key),
    ("f17", F17Key),
    ("f18", F18Key),
    ("f19", F19Key),
    ("f20", F20Key),
    ("f21", F21Key),
    ("f22", F22Key),
    ("f23", F23Key),
    ("f24", F24Key),
    ("numlock", NumLockKey),
    ("scrolllock", ScrollLockKey),
    ("capslock", CapsLockKey),
    ("lshift", LShiftKey),
    ("shift", LShiftKey),
    ("lcontrol", LControlKey),
    ("lctrl", LControlKey),
    ("control", LControlKey),
    ("ctrl", LControlKey),
    ("lalt", LAltKey),
    ("alt", LAltKey),
    ("browserback", BrowserBackKey),
    ("browserforward", BrowserForwardKey),
    ("browserrefresh", BrowserRefreshKey),
    ("volumemute", VolumeMuteKey),
    ("volmute", VolumeMuteKey),
    ("volumedown", VolumeDownKey),
    ("voldown", VolumeDownKey),
    ("volumeup", VolumeUpKey),
    ("volup", VolumeUpKey),
    ("virgule", CommaKey),
    ("comma", CommaKey),
    ("point", PeriodKey),
    ("period", PeriodKey),
    ("tiret", DashKey),
    ("moins", DashKey),
    ("dash", DashKey),
    ("minus", DashKey),
    ("eaccentgrave", EAccGraveKey),
    ("eaccgrave", EAccGraveKey),
    ("paragraphe", SectionKey),
    ("section", SectionKey),
    ("trema", TremaKey),
    ("circonflexe", CircumflexKey),
    ("circumflex", CircumflexKey),
    ("eaccentaigu", EAccAiguKey),
    ("eaccaigu", EAccAiguKey),
    ("dollar", DollarSignKey),
    ("pluspetitque", LessThanKey),
    ("lessthan", LessThanKey),
];
