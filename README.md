<img src="assets/icon.svg" width=70%/>

---

A lightweight, fast, and feature-rich interpreter for a custom scripting language designed to automate repetitive manual user input tasks on Windows. Written in Rust, it prioritizes failsafe guarantees, detailed logging, and reliable execution.

## Features

* Beautiful logging using the tracing crate
* Automatic scaling of scripts to active monitor
* Hotkey creation
* Robust failsafes: global halt key, `await` expression
* Simple portable executable
* Real key press and mouse movement emulation (bypassing restrictions in most applications, except for the Type expression, which uses virtual key packets)

## Installation

Beaulieu Automation is designed as a standalone portable executable. You can either:
* download a [pre-compiled binary](https://github.com/anesthetice/Beaulieu-Automation/releases)
* build it from source:
  ``` bash
  git clone https://github.com/anesthetice/Beaulieu-Automation.git
  cd Beaulieu-Automation
  cargo build --release
  ```

## Usage

> [!IMPORTANT]
> BA is currently configured for the `Swiss French` keyboard layout. If you need to use a different layout, modify the following sections accordingly:
> 1. Update [main.rs](crates/ba/src/main.rs) to match your desired keyboard layout. Refer to this [documentation](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getkeyboardlayoutnamea) for more details.
>   ``` rust
>      // check keyboard layout
>      let mut pwszklid = [0_u8; 9];
>      unsafe {
>          GetKeyboardLayoutNameA(&mut pwszklid).context("Failed to get keyboard layout")?;
>      }
>      let keyboard_layout = CStr::from_bytes_with_nul(&pwszklid)?.to_str()?;
>      if keyboard_layout != "0000100C" {
>          tracing::error!(
>              "Expected '0000100C' for keyboard layout, got '{}'",
>              keyboard_layout
>          );
>          Err(anyhow::anyhow!(
>              "Invalid keyboard layout, switch to Swiss-French"
>          ))?
>      }
>   ```
> 2. Modify the relevant keycodes in [input.rs](crates/input-bot/src/windows/inputs.rs) to support your preferred keyboard layout.
>   ``` rust
>   impl From<KeybdKey> for u64 {
>      // https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes?redirectedfrom=MSDN
>      // modified for swiss-french keyboard
>      fn from(key: KeybdKey) -> u64 {
>          match key {
>              BackspaceKey => 0x08,
>              TabKey => 0x09,
>              EnterKey => 0x0D,
>              EscapeKey => 0x1B,
>              ...
>              // <
>              OEM102 => 0xE2,
>              OtherKey(code) => code,
>          }
>      }
>   }  
>   ```

### Commands

Beaulieu Automation provides two simple commands:
```
Usage: BeaulieuAutomation.exe [COMMAND]

Commands:
  new   Create a new BA application
  run   Runs the specified BA application
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

**Creating a new application (`new`)**:
```
Usage: BeaulieuAutomation.exe new <path>

Arguments:
  <path>
          path/name of the new application
          e.g. 'new test' will create a new application named test in the terminal's current working directory
```

**Running an application (`run`)**:
```
Usage: BeaulieuAutomation.exe run <path> [repetitions]

Arguments:
  <path>
          path of the application folder

  [repetitions]
          number of times to repeat the script
```

## Example and BA-script guide

> [!NOTE]  
> For the following assume the executable was renamed to ba.exe for ease of use

Creating a new BA-application named `test`:
```
ba.exe new test
```

This creates a new folder in the current working directory, the folder ...
