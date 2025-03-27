<img src="assets/icon.svg" width=70%/>

---

A lightweight, fast, and feature-rich interpreter for a custom scripting language designed to automate repetitive manual user input tasks on Windows. Written in Rust, it prioritizes failsafe guarantees, detailed logging, and reliable execution.

## Features
* beautiful logging using the tracing crate
* automatic scaling of scripts when executed
* hotkey creation
* robust failsafes: global halt key, await expression
* usable as a simple portable executable

## Installation

This application is meant to be used as a stand-alone portable executable, you can either download a pre-compiled binary from the releases or compile it yourself by running the following:

```
git clone https://github.com/anesthetice/Beaulieu-Automation.git
cd Beaulieu-Automation
cargo build --release
```

## Usage

> [!IMPORTANT]  
> BA is currently configured to be used with a `Swiss French` keyboard layout, if you wish to change this, please take a look at main.rs

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
