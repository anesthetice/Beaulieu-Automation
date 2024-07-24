use anyhow::Context;
use inputbot::MouseButton::{self, *};
use std::{collections::HashMap, io::Read, path::Path, sync::OnceLock};

static MOUSEMAP: OnceLock<MouseMap> = OnceLock::new();

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct MouseMap {
    inner: HashMap<String, MouseButton>,
}

impl MouseMap {
    fn init(mousemap_filepath: &Path) -> anyhow::Result<()> {
        let mut data = Vec::new();
        let mut file = std::fs::OpenOptions::new()
            .create(false)
            .read(true)
            .open(&mousemap_filepath)
            .context(format!("Failed to open file with path '{}'", mousemap_filepath.display()))?;

        file.read_to_end(&mut data)?;
        let data: Vec<(String, MouseButton)> = serde_json::from_slice(&data)?;
        MOUSEMAP.set(MouseMap::from(data)).unwrap();
        Ok(())
    }

    pub fn get() -> &'static Self {
        MOUSEMAP.get().unwrap()
    }
}

impl std::ops::Deref for MouseMap {
    type Target = HashMap<String, MouseButton>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Default for MouseMap {
    fn default() -> Self {
        Self::from(&DEFAULT_MOUSEMAP[..])
    }
}

impl From<&[(&str, MouseButton)]> for MouseMap {
    fn from(value: &[(&str, MouseButton)]) -> Self {
        Self {
            inner: HashMap::from_iter(value.into_iter().map(|(s, k)| (s.to_string(), *k))),
        }
    }
}

impl From<Vec<(String, MouseButton)>> for MouseMap {
    fn from(value: Vec<(String, MouseButton)>) -> Self {
        Self { inner: HashMap::from_iter(value.into_iter()) }
    }
}

static DEFAULT_MOUSEMAP: [(&'static str, MouseButton); 5] = [
    ("lmb", LeftButton),
    ("rmb", RightButton),
    ("mmb", MiddleButton),
    ("x1", X1Button),
    ("x2", X2Button)
];