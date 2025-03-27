<img src="assets/icon.svg" width=70%/>

---

A lightweight, fast, and feature-rich interpreter for a custom scripting language designed to automate repetitive manual user input tasks on Windows. Written in Rust, it prioritizes failsafe guarantees, detailed logging, and reliable execution.

## Features

* beautiful logging using the tracing crate
* automatic scaling of scripts when executed
* hotkey creation
* robust failsafes: global halt key, await expression
* usable as a simple portable executable
* emulates real key presses and mouse movements, bypassing annoying applications (exception for the `Type` expression as it uses a virtual key packet)

## Installation

This application is meant to be used as a stand-alone portable executable, you can either download a [pre-compiled binary](https://github.com/anesthetice/Beaulieu-Automation/releases) or compile it yourself by running the following:

``` bash
git clone https://github.com/anesthetice/Beaulieu-Automation.git
cd Beaulieu-Automation
cargo build --release
```

## Usage

> [!IMPORTANT]  
> BA is currently configured to be used with a `Swiss French` keyboard layout, if you wish to change this, please adapt the following in [main.rs](crates/ba/src/main.rs) to your desired keyboard layout, see [here](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getkeyboardlayoutnamea) for more information.
> ``` rust
>    // check keyboard layout
>    let mut pwszklid = [0_u8; 9];
>    unsafe {
>        GetKeyboardLayoutNameA(&mut pwszklid).context("Failed to get keyboard layout")?;
>    }
>    let keyboard_layout = CStr::from_bytes_with_nul(&pwszklid)?.to_str()?;
>    if keyboard_layout != "0000100C" {
>        tracing::error!(
>            "Expected '0000100C' for keyboard layout, got '{}'",
>            keyboard_layout
>        );
>        Err(anyhow::anyhow!(
>            "Invalid keyboard layout, switch to Swiss-French"
>        ))?
>    }
> ```
> And also adapt the following in [input.rs](crates/input-bot/src/windows/inputs.rs) to your desired keyboard layout
> ``` rust
> impl From<KeybdKey> for u64 {
>    // https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes?redirectedfrom=MSDN
>    // modified for swiss-french keyboard
>    fn from(key: KeybdKey) -> u64 {
>        match key {
>            BackspaceKey => 0x08,
>            TabKey => 0x09,
>            EnterKey => 0x0D,
>            EscapeKey => 0x1B,
>            ...
>            // <
>            OEM102 => 0xE2,
>            OtherKey(code) => code,
>        }
>    }
> }  
> ```

The application only has 2 commands:
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

Here's how to use `new`:
```
Usage: BeaulieuAutomation.exe new <path>

Arguments:
  <path>
          path/name of the new application
          e.g. 'new test' will create a new application named test in the terminal's current working directory
```

Here's how to use `run`:
```
Usage: BeaulieuAutomation.exe run <path> [repetitions]

Arguments:
  <path>
          path of the application folder

  [repetitions]
          number of times to repeat the script
```

## BA-script

Quite straightforward and intuitive, here's an example that uses every single available expression.

```
TODO
```
