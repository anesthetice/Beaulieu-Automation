mod cli;
mod compiler;
mod dirs;
mod keymap;
mod mousemap;

use anyhow::Context;
use std::ffi::CStr;
use tracing::{info, level_filters::LevelFilter};

use directories::ProjectDirs;
use tracing_subscriber::{
    Layer, Registry,
    fmt::{self, time::Uptime},
    layer::SubscriberExt,
};

use windows::Win32::UI::{
    HiDpi::{DPI_AWARENESS_CONTEXT_SYSTEM_AWARE, SetThreadDpiAwarenessContext},
    Input::KeyboardAndMouse::GetKeyboardLayoutNameA,
    WindowsAndMessaging::{GetSystemMetrics, SYSTEM_METRICS_INDEX},
};

#[cfg(not(debug_assertions))]
use tracing_subscriber::EnvFilter;

fn main() -> anyhow::Result<()> {
    let local_time = jiff::Zoned::try_from(std::time::SystemTime::now()).unwrap_or_else(|err| {
        tracing::warn!(
            "Failed to get local system time, defaulting to UTC: {}",
            err
        );
        jiff::Zoned::new(jiff::Timestamp::now(), jiff::tz::TimeZone::UTC)
    });
    let local_time_string_pretty = format!(
        "{:0>4}-{:0>2}-{:0>2} {:0>2}:{:0>2}:{:0>2}",
        local_time.year(),
        local_time.month(),
        local_time.day(),
        local_time.hour(),
        local_time.minute(),
        local_time.second(),
    );
    let local_time_string = format!(
        "{:0>4}_{:0>2}_{:0>2}-{:0>2}_{:0>2}_{:0>2}",
        local_time.year(),
        local_time.month(),
        local_time.day(),
        local_time.hour(),
        local_time.minute(),
        local_time.second(),
    );

    let log_filepath = {
        let project_dirs = ProjectDirs::from("", "", "BeaulieuAutomation")
            .context("Failed to get project directories")?;
        let log_directory = project_dirs.data_local_dir().join("logs");
        dirs::DIRS.set(project_dirs).unwrap();
        if !log_directory.exists() {
            std::fs::create_dir_all(&log_directory).context(format!(
                "Failed to create directories for logs '{}'",
                &log_directory.display()
            ))?;
        }
        log_directory.join(local_time_string + ".log")
    };

    let (non_blocking, _guard) = tracing_appender::non_blocking(
        std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create_new(true)
            .open(&log_filepath)
            .context(format!(
                "Failed to open log file with path '{}'",
                &log_filepath.display()
            ))?,
    );

    let format_logfile = fmt::format()
        .compact()
        .with_ansi(false)
        .with_thread_ids(true)
        .with_thread_names(true);

    let format_terminal = fmt::format()
        .pretty()
        .with_ansi(true)
        .with_thread_ids(true)
        .with_thread_names(true);

    let filter_logfile = LevelFilter::TRACE;

    #[cfg(debug_assertions)]
    let filter_terminal = LevelFilter::TRACE;
    #[cfg(not(debug_assertions))]
    let filter_terminal =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn"));

    let subscriber = Registry::default()
        .with(
            fmt::layer()
                .event_format(format_terminal)
                .with_timer(Uptime::default())
                .with_writer(std::io::stderr)
                .with_filter(filter_terminal),
        )
        .with(
            fmt::Layer::new()
                .event_format(format_logfile)
                .with_timer(Uptime::default())
                .with_writer(non_blocking)
                .with_filter(filter_logfile),
        );

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set global subscriber");
    info!("{}", local_time_string_pretty);

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

    // get the width and height of the primary montior, SetThreadDpiAwarenessContext fixes issues with scale
    unsafe { SetThreadDpiAwarenessContext(DPI_AWARENESS_CONTEXT_SYSTEM_AWARE) };
    let width = unsafe { GetSystemMetrics(SYSTEM_METRICS_INDEX(0)) };
    let height = unsafe { GetSystemMetrics(SYSTEM_METRICS_INDEX(1)) };
    tracing::info!("Primary monitor detected - {width}x{height}");

    if let Err(e) = cli::cli((width, height)) {
        tracing::error!("{e}");
        Err(e)?
    } else {
        Ok(())
    }
}
