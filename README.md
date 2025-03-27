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

## Detailed Usage and Examples

> [!NOTE]  
> For the following assume the executable was renamed to `ba.exe` for the sake of brevity.

Creating a new BA-application named `test`:
```
ba.exe new test
```

This results in the creation of a new folder called `test` in the current working directory. It contains the following files:
1. `main.ba` -> the BA script file
2. `keymap.json` -> maps strings (used to represent a key in the script) to valid keys
3. `mousemap.json` -> maps strings (used to represent a mouse button in the script) to valid mouse buttons
4. `README.md` -> the application's README file

Let's go through each one-by-one in reverse:

### `README.md`
Quite self-explanatory, contains information relevent to the use of the BA application
```
## Title

### description
* describe what your application is used for here

### layout
* describe your desktop's initial layout for the application to function properly
* please also include a screenshot of the entire initial desktop in the application folder
```

### `keymap.json` and `mousemap.json`
Contains (value, key/button) pairs, the left values are what you can use in the BA script to represent a button or a key
``` 
  [
    "escape", <-- Its name in the script
    "EscapeKey" <-- The actual key
  ],
  [
    "esc", <-- A key/button can have multiple names
    "EscapeKey"
  ],
```

### `main.ba`
The most important file, it should currently by default look something like this:
```
// Resolution of the primary monitor for which this script was created
// DO NOT MODIFY
define RESOLUTION = 1920, 1080

// The standard delay between actions given in milliseconds
define DELAY_BETWEEN_ACTIONS = 50

// The button used to stop the application if required
define GLOBAL_HALT_KEY = Esc
```

Here are all the available expressions:
```
Move [int], [int]      // Moves the mouse cursor to the specified (x, y) coordinates  
Tap [key/button]       // Taps a key or button once  
Press [key/button]     // Holds down a key or button  
Release [key/button]   // Releases a held key or button  
Sleep [float]          // Pauses execution for the given time (in seconds)  
Type [string]          // Types a string (e.g., "Hello, World!" will be written at once)  
Await                  // Suspends execution indefinitely (until the global halt key is pressed) for hotkey scripts  
Await [key]            // Waits for a key press before resuming execution  
Bind [key] {  
  [expression]         // Binds an expression to a key press  
}  
Print [string]         // Prints a message to the console  
PrintClipboard         // Prints the contents of the clipboard  
Scroll [int]           // Scrolls by the specified amount  
```


Here's a simple script that copies a line to the clipboard, prints it, then pastes it somewhere else:
```
define RESOLUTION = 1920, 1080
define DELAY_BETWEEN_ACTIONS = 200
define GLOBAL_HALT_KEY = Esc

Sleep 3.0

Move 1400, 270
Press lmb
Move 270, 270
Release lmb

Press ctrl; Tap c; Release ctrl
PrintClipboard

Move 1400, 500
Tap lmb
Tap enter
Press ctrl; Tap v; Release ctrl
```

And here is another that binds the numrow1 and numrow2 keys to scroll up and down by a large amount:
```
define RESOLUTION = 1920, 1080
define DELAY_BETWEEN_ACTIONS = 50
define GLOBAL_HALT_KEY = Esc

Bind nr1 {
    ScrollUp 400
}
Bind nr2 {
    ScrollDown 400
}
Await
```
