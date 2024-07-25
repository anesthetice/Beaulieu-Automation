use crate::common::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use serde::{Deserialize, Serialize};

pub enum BlockInput {
    Block,
    DontBlock,
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, EnumIter, Serialize, Deserialize)]
pub enum KeybdKey {
    BackspaceKey,
    TabKey,
    EnterKey,
    EscapeKey,
    SpaceKey,
    PageUpKey,
    PageDownKey,
    EndKey,
    HomeKey,
    LeftKey,
    UpKey,
    RightKey,
    DownKey,
    InsertKey,
    DeleteKey,
    Numrow0Key,
    Numrow1Key,
    Numrow2Key,
    Numrow3Key,
    Numrow4Key,
    Numrow5Key,
    Numrow6Key,
    Numrow7Key,
    Numrow8Key,
    Numrow9Key,
    AKey,
    BKey,
    CKey,
    DKey,
    EKey,
    FKey,
    GKey,
    HKey,
    IKey,
    JKey,
    KKey,
    LKey,
    MKey,
    NKey,
    OKey,
    PKey,
    QKey,
    RKey,
    SKey,
    TKey,
    UKey,
    VKey,
    WKey,
    XKey,
    YKey,
    ZKey,
    LSuper,
    RSuper,
    Numpad0Key,
    Numpad1Key,
    Numpad2Key,
    Numpad3Key,
    Numpad4Key,
    Numpad5Key,
    Numpad6Key,
    Numpad7Key,
    Numpad8Key,
    Numpad9Key,
    F1Key,
    F2Key,
    F3Key,
    F4Key,
    F5Key,
    F6Key,
    F7Key,
    F8Key,
    F9Key,
    F10Key,
    F11Key,
    F12Key,
    F13Key,
    F14Key,
    F15Key,
    F16Key,
    F17Key,
    F18Key,
    F19Key,
    F20Key,
    F21Key,
    F22Key,
    F23Key,
    F24Key,
    NumLockKey,
    ScrollLockKey,
    CapsLockKey,
    LShiftKey,
    RShiftKey,
    LControlKey,
    RControlKey,
    LAltKey,
    RAltKey,

    BrowserBackKey,
    BrowserForwardKey,
    BrowserRefreshKey,

    VolumeMuteKey,
    VolumeDownKey,
    VolumeUpKey,

    MediaNextTrackKey,
    MediaPrevTrackKey,
    MediaStopKey,
    MediaPlayPauseKey,

    // http://kbdlayout.info/KBDSF/virtualkeys
    // OEM COMMA
    CommaKey,
    // OEM PERIOD
    PeriodKey,
    // OEM MINUS
    DashKey,
    // OEM 1
    EAccGraveKey,
    // OEM 2
    SectionKey,
    // OEM 3
    TremaKey,
    // OEM 4
    ApostropheKey,
    // OEM 5
    AAccGraveKey,
    // OEM 6
    CircumflexKey,
    // OEM 7
    EAccAiguKey,
    // OEM 8
    DollarSignKey,
    // OEM 102
    LessThanKey,

    #[strum(disabled)]
    OtherKey(u64),
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, EnumIter, Serialize, Deserialize)]
pub enum MouseButton {
    LeftButton,
    MiddleButton,
    RightButton,
    X1Button,
    X2Button,
    MousewheelUp,
    MousewheelDown,
    #[strum(disabled)]
    OtherButton(u32),
}

pub struct MouseCursor;

pub struct MouseWheel;

impl KeybdKey {
    pub fn bind<F: Fn() + Send + Sync + 'static>(self, callback: F) {
        KEYBD_BINDS
            .lock()
            .unwrap()
            .insert(self, Bind::Normal(Arc::new(callback)));
    }

    pub fn bind_release<F: Fn() + Send + Sync + 'static>(self, callback: F) {
        KEYBD_RELEASE_BINDS
            .lock()
            .unwrap()
            .insert(self, Bind::Release(Arc::new(callback)));
    }

    pub fn block_bind<F: Fn() + Send + Sync + 'static>(self, callback: F) {
        KEYBD_BINDS
            .lock()
            .unwrap()
            .insert(self, Bind::Block(Arc::new(callback)));
    }

    pub fn blockable_bind<F: Fn() -> BlockInput + Send + Sync + 'static>(self, callback: F) {
        KEYBD_BINDS
            .lock()
            .unwrap()
            .insert(self, Bind::Blockable(Arc::new(callback)));
    }

    pub fn bind_all<F: Fn(KeybdKey) + Send + Sync + Clone + 'static>(callback: F) {
        for key in KeybdKey::iter() {
            let callback = callback.clone();
            let fire = move || {
                callback(key);
            };

            KEYBD_BINDS
                .lock()
                .unwrap()
                .insert(key, Bind::Normal(Arc::new(fire)));
        }
    }

    pub fn bind_all_release<F: Fn(KeybdKey) + Send + Sync + Clone + 'static>(callback: F) {
        for key in KeybdKey::iter() {
            let callback = callback.clone();
            let fire = move || {
                callback(key);
            };

            KEYBD_RELEASE_BINDS
                .lock()
                .unwrap()
                .insert(key, Bind::Release(Arc::new(fire)));
        }
    }

    pub fn is_bound(self) -> bool {
        KEYBD_BINDS.lock().unwrap().contains_key(&self)
    }

    pub fn unbind(self) {
        KEYBD_BINDS.lock().unwrap().remove(&self);
    }
}

impl MouseButton {
    pub fn bind<F: Fn() + Send + Sync + 'static>(self, callback: F) {
        MOUSE_BINDS
            .lock()
            .unwrap()
            .insert(self, Bind::Normal(Arc::new(callback)));
    }

    pub fn bind_release<F: Fn() + Send + Sync + 'static>(self, callback: F) {
        MOUSE_RELEASE_BINDS
            .lock()
            .unwrap()
            .insert(self, Bind::Release(Arc::new(callback)));
    }

    pub fn block_bind<F: Fn() + Send + Sync + 'static>(self, callback: F) {
        MOUSE_BINDS
            .lock()
            .unwrap()
            .insert(self, Bind::Block(Arc::new(callback)));
    }

    pub fn blockable_bind<F: Fn() -> BlockInput + Send + Sync + 'static>(self, callback: F) {
        MOUSE_BINDS
            .lock()
            .unwrap()
            .insert(self, Bind::Blockable(Arc::new(callback)));
    }

    pub fn bind_all<F: Fn(MouseButton) + Send + Sync + Clone + 'static>(callback: F) {
        for btn in MouseButton::iter() {
            let callback = callback.clone();
            let fire = move || {
                callback(btn);
            };

            MOUSE_BINDS
                .lock()
                .unwrap()
                .insert(btn, Bind::Normal(Arc::new(fire)));
        }
    }

    pub fn bind_all_release<F: Fn(MouseButton) + Send + Sync + Clone + 'static>(callback: F) {
        for btn in MouseButton::iter() {
            let callback = callback.clone();
            let fire = move || {
                callback(btn);
            };

            MOUSE_RELEASE_BINDS
                .lock()
                .unwrap()
                .insert(btn, Bind::Release(Arc::new(fire)));
        }
    }

    pub fn is_bound(self) -> bool {
        MOUSE_BINDS.lock().unwrap().contains_key(&self)
    }

    pub fn unbind(self) {
        MOUSE_BINDS.lock().unwrap().remove(&self);
    }
}

/// Stops `handle_input_events()` (threadsafe)
pub fn stop_handling_input_events() {
    HANDLE_EVENTS.store(false, Ordering::Relaxed);
}
