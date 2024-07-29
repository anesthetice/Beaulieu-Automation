use crate::public::*;
use std::{
    ffi::c_int,
    mem::{size_of, MaybeUninit},
    thread,
};
use windows::Win32::UI::{
    Input::KeyboardAndMouse::{
        GetAsyncKeyState, GetKeyState, MapVirtualKeyW, RegisterHotKey, SendInput,
        HOT_KEY_MODIFIERS, INPUT, INPUT_0, INPUT_KEYBOARD, INPUT_MOUSE, KEYBDINPUT,
        KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP, KEYEVENTF_SCANCODE, KEYEVENTF_UNICODE,
        MAP_VIRTUAL_KEY_TYPE, MOUSEEVENTF_HWHEEL, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP,
        MOUSEEVENTF_MIDDLEDOWN, MOUSEEVENTF_MIDDLEUP, MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP,
        MOUSEEVENTF_WHEEL, MOUSEEVENTF_XDOWN, MOUSEEVENTF_XUP, MOUSEINPUT, MOUSE_EVENT_FLAGS,
        VIRTUAL_KEY, VK_PACKET,
    },
    WindowsAndMessaging::{GetCursorPos, GetMessageW, SetCursorPos, MSG, PM_REMOVE},
};

mod inputs;

pub fn send_sequence(input: &str) {
    let inputs: Vec<INPUT> = input
        .encode_utf16()
        .map(|code| {
            let keybd_input = KEYBDINPUT {
                wVk: VK_PACKET,
                wScan: code,
                dwFlags: KEYEVENTF_UNICODE,
                time: 0,
                dwExtraInfo: 0,
            };
            INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 { ki: keybd_input },
            }
        })
        .collect();

    unsafe {
        SendInput(&inputs, std::mem::size_of::<INPUT>() as std::ffi::c_int);
    }
}

impl KeybdKey {
    /// Returns true if a given `KeybdKey` is currently pressed (in the down position).
    pub fn is_pressed(self) -> bool {
        (unsafe { GetAsyncKeyState(u64::from(self) as i32) } >> 15) != 0
    }

    /// Presses a given `KeybdKey`. Note: this means the key will remain in the down
    /// position. You must manually call release to create a full 'press'.
    pub fn press(self) {
        send_keybd_input(KEYEVENTF_SCANCODE, self);
    }

    /// Releases a given `KeybdKey`. This means the key would be in the up position.
    pub fn release(self) {
        send_keybd_input(KEYEVENTF_SCANCODE | KEYEVENTF_KEYUP, self);
    }

    pub fn tap(self) {
        send_keybd_inputs(vec![
            (KEYEVENTF_SCANCODE, self),
            (KEYEVENTF_SCANCODE | KEYEVENTF_KEYUP, self),
        ])
    }

    /// Returns true if a keyboard key which supports toggling (ScrollLock, NumLock,
    /// CapsLock) is on.
    pub fn is_toggled(self) -> bool {
        unsafe { GetKeyState(u64::from(self) as i32) & 15 != 0 }
    }

    pub fn listen_once<F: FnOnce() -> () + Send + 'static>(
        self,
        hotkey_id: i32,
        callback: F,
    ) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            if let Err(err) = unsafe { RegisterHotKey(None, hotkey_id, HOT_KEY_MODIFIERS(0), u64::from(self) as u32) } {
                tracing::error!("Failed to bind HotKey, '{}'", err);
                callback();
            } else {
                let mut msg: MSG = unsafe { MaybeUninit::zeroed().assume_init() };
                unsafe { GetMessageW(&mut msg, None, 0, 0) };
                callback();
            }
        })
    }
}

impl MouseButton {
    /// Returns true if a given `MouseButton` is currently pressed (in the down position).
    pub fn is_pressed(self) -> bool {
        (unsafe { GetAsyncKeyState(u32::from(self) as i32) } >> 15) != 0
    }

    /// Presses a given `MouseButton`. Note: this means the button will remain in the down
    /// position. You must manually call release to create a full 'click'.
    pub fn press(self) {
        match self {
            MouseButton::LeftButton => send_mouse_input(MOUSEEVENTF_LEFTDOWN, 0, 0, 0),
            MouseButton::RightButton => send_mouse_input(MOUSEEVENTF_RIGHTDOWN, 0, 0, 0),
            MouseButton::MiddleButton => send_mouse_input(MOUSEEVENTF_MIDDLEDOWN, 0, 0, 0),
            MouseButton::X1Button => send_mouse_input(MOUSEEVENTF_XDOWN, 1, 0, 0),
            MouseButton::X2Button => send_mouse_input(MOUSEEVENTF_XDOWN, 2, 0, 0),
            _ => {}
        }
    }

    /// Releases a given `MouseButton`. This means the button would be in the up position.
    pub fn release(self) {
        match self {
            MouseButton::LeftButton => send_mouse_input(MOUSEEVENTF_LEFTUP, 0, 0, 0),
            MouseButton::RightButton => send_mouse_input(MOUSEEVENTF_RIGHTUP, 0, 0, 0),
            MouseButton::MiddleButton => send_mouse_input(MOUSEEVENTF_MIDDLEUP, 0, 0, 0),
            MouseButton::X1Button => send_mouse_input(MOUSEEVENTF_XUP, 1, 0, 0),
            MouseButton::X2Button => send_mouse_input(MOUSEEVENTF_XUP, 2, 0, 0),
            _ => {}
        }
    }

    pub fn tap(self) {
        match self {
            MouseButton::LeftButton => send_mouse_inputs(vec![
                (MOUSEEVENTF_LEFTDOWN, 0, 0, 0),
                (MOUSEEVENTF_LEFTUP, 0, 0, 0),
            ]),
            MouseButton::RightButton => send_mouse_inputs(vec![
                (MOUSEEVENTF_RIGHTDOWN, 0, 0, 0),
                (MOUSEEVENTF_RIGHTDOWN, 0, 0, 0),
            ]),
            MouseButton::MiddleButton => send_mouse_inputs(vec![
                (MOUSEEVENTF_MIDDLEDOWN, 0, 0, 0),
                (MOUSEEVENTF_MIDDLEUP, 0, 0, 0),
            ]),
            MouseButton::X1Button => send_mouse_inputs(vec![
                (MOUSEEVENTF_XDOWN, 1, 0, 0),
                (MOUSEEVENTF_XUP, 1, 0, 0),
            ]),
            MouseButton::X2Button => send_mouse_inputs(vec![
                (MOUSEEVENTF_XDOWN, 2, 0, 0),
                (MOUSEEVENTF_XUP, 2, 0, 0),
            ]),
            _ => {}
        }
    }
}

impl MouseCursor {
    pub fn pos() -> (i32, i32) {
        let mut point = MaybeUninit::uninit();
        unsafe { GetCursorPos(point.as_mut_ptr()).unwrap() };
        let point = unsafe { point.assume_init() };
        (point.x, point.y)
    }

    /// Moves the mouse relative to its current position by a given amount of pixels.
    pub fn move_rel(dx: i32, dy: i32) {
        let (x, y) = Self::pos();
        Self::move_abs(x + dx, y + dy);
    }

    /// Moves the mouse to a given position based on absolute coordinates. The top left
    /// corner of the screen is (0, 0).
    pub fn move_abs(x: i32, y: i32) {
        unsafe {
            SetCursorPos(x, y).unwrap();
        }
    }
}

impl MouseWheel {
    /// Scrolls the mouse wheel vertically by a given amount of "wheel clicks".
    pub fn scroll_ver(dwheel: i32) {
        send_mouse_input(MOUSEEVENTF_WHEEL, dwheel * 120, 0, 0);
    }

    /// Scrolls the mouse wheel horizontally by a given amount of "wheel clicks".
    pub fn scroll_hor(dwheel: i32) {
        send_mouse_input(MOUSEEVENTF_HWHEEL, dwheel * 120, 0, 0);
    }

    /// Scrolls the mouse wheel vertically by a given amount.
    pub fn scroll_ver_unscaled(dwheel: i32) {
        send_mouse_input(MOUSEEVENTF_WHEEL, dwheel, 0, 0);
    }

    /// Scrolls the mouse wheel horizontally by a given amount.
    pub fn scroll_hor_unscaled(dwheel: i32) {
        send_mouse_input(MOUSEEVENTF_HWHEEL, dwheel, 0, 0);
    }
}

fn send_mouse_input(flags: MOUSE_EVENT_FLAGS, data: i32, dx: i32, dy: i32) {
    let mouse = MOUSEINPUT {
        dx,
        dy,
        mouseData: data,
        dwFlags: flags,
        time: 0,
        dwExtraInfo: 0,
    };

    let input = INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0 { mi: mouse },
    };

    unsafe { SendInput(&[input], size_of::<INPUT>() as c_int) };
}

fn send_mouse_inputs(inputs: Vec<(MOUSE_EVENT_FLAGS, i32, i32, i32)>) {
    let inputs: Vec<INPUT> = inputs
        .into_iter()
        .map(|(flags, data, dx, dy)| {
            let mouse_input = MOUSEINPUT {
                dx,
                dy,
                mouseData: data,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            };
            INPUT {
                r#type: INPUT_MOUSE,
                Anonymous: INPUT_0 { mi: mouse_input },
            }
        })
        .collect();

    unsafe { SendInput(&inputs, size_of::<INPUT>() as c_int) };
}

fn send_keybd_input(flags: KEYBD_EVENT_FLAGS, key_code: KeybdKey) {
    let keybd = KEYBDINPUT {
        wVk: VIRTUAL_KEY(0),
        wScan: unsafe {
            MapVirtualKeyW(u64::from(key_code) as u32, MAP_VIRTUAL_KEY_TYPE(0)) as u16
        },
        dwFlags: flags,
        time: 0,
        dwExtraInfo: 0,
    };

    let input = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 { ki: keybd },
    };

    unsafe { SendInput(&[input], size_of::<INPUT>() as c_int) };
}

fn send_keybd_inputs(inputs: Vec<(KEYBD_EVENT_FLAGS, KeybdKey)>) {
    let inputs: Vec<INPUT> = inputs
        .into_iter()
        .map(|(flags, key_code)| {
            let keybd_input = KEYBDINPUT {
                wVk: VIRTUAL_KEY(0),
                wScan: unsafe {
                    MapVirtualKeyW(u64::from(key_code) as u32, MAP_VIRTUAL_KEY_TYPE(0)) as u16
                },
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            };
            INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 { ki: keybd_input },
            }
        })
        .collect();

    unsafe { SendInput(&inputs, size_of::<INPUT>() as c_int) };
}
