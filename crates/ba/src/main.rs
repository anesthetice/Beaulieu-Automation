mod cli;
mod compiler;
mod keymap;

use anyhow::Context;
use std::{ffi::CStr, io::Write};
use tracing_subscriber::FmtSubscriber;
use windows::Win32::UI::{
    Input::KeyboardAndMouse::GetKeyboardLayoutNameA,
    WindowsAndMessaging::{GetSystemMetrics, SYSTEM_METRICS_INDEX},
};

fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    tracing::info!("starting...");

    // check keyboard layout
    let mut pwszklid = [0_u8; 9];
    unsafe {
        GetKeyboardLayoutNameA(&mut pwszklid).context("Failed to get keyboard layout")?;
    }
    let keyboard_layout = CStr::from_bytes_with_nul(&pwszklid)?.to_str()?;
    if keyboard_layout != "0000100C" {
        tracing::error!(
            "Expected '0000100C' for keyboard layout, got '{}'",
            keyboard_layout
        );
        Err(anyhow::anyhow!(
            "Invalid keyboard layout, switch to Swiss-French"
        ))?
    }

    // get primary monitor width and height
    let width = unsafe { GetSystemMetrics(SYSTEM_METRICS_INDEX(0)) };
    let height = unsafe { GetSystemMetrics(SYSTEM_METRICS_INDEX(1)) };
    tracing::info!("primary monitor: {width}x{height}");

    compiler::lexer::test();

    Ok(())
}
