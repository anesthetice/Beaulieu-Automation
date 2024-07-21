use std::collections::HashMap;

#[derive(serde::Serialize, serde::Deserialize)]
struct KeyMap{inner: HashMap<String, u64>}

impl std::ops::Deref for KeyMap {
    type Target = HashMap<String, u64>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Default for KeyMap {
    fn default() -> Self {
        Self { inner: 
            HashMap::from(
                [


                ]
            )
        }
    }
}