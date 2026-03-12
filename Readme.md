# Kern Programming Language Documentation

## Overview
Kern is a lightweight programming language with Python-like syntax and Rust-level performance. It transpiles `.kern` files into Rust code and compiles them to Windows `.exe` executables.

## Features
- **fn main**: Defines the main function.
- **print**: Outputs text to the console.
- **pause**: Pauses execution until the user presses Enter.
- **}**: Closes a block.

### Limitations
- Variables are currently **not supported**.
- Loops (`for`, `while`) are **not supported**.
- Conditionals (`if`, `else`) are **not supported**.
- Functions beyond `fn main` are **not supported**.
- Any unsupported lines are ignored during transpilation.

## Installation
1. **Prerequisites:** Make sure the following are installed:
   - [Rust](https://www.rust-lang.org/tools/install)
   - Visual Studio Build Tools 2022 (for `rustc` compilation on Windows)
2. Place the Kern folder in `C:\` so your folder structure looks like this:
```
C:\Kern\
    compiler\kernc.exe
```
3. Add the `compiler` folder to your PATH:
   - Press **Windows + S** → type **Environment Variables** → click **Edit the system environment variables**.
   - Click **Environment Variables…**
   - Under **User variables**, select **Path** → **Edit…** → **New** → type:
     ```
     C:\Kern\compiler
     ```
   - Click **OK** on all windows.
4. Open a **new Command Prompt or PowerShell** to apply the changes.

## Commands
| Command | Description |
|---------|-------------|
| `kernc build <file.kern>` | Transpile and compile to `.exe`. |
| `kernc run <file.kern>`   | Transpile, compile, and execute. |
| `kernc clean <file.kern>` | Remove generated `.rs` and `.exe` files. |

## Usage Examples
### Build a Kern Program
```cmd
kernc build main.kern
```

### Run a Kern Program
```cmd
kernc run main.kern
```

### Clean Generated Files
```cmd
kernc clean main.kern
```

## Example Kern Program
```kern
fn main
    print "Hello, Kern!"
    pause
}
```

### Transpiled Rust Code
```rust
fn main() {
    println!("Hello, Kern!");
    use std::io::{self, Write};
    print!("Press Enter to continue...");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
}
```

## Roadmap / Future Features
- Variable declarations and assignments.
- Loops: `for` and `while`.
- Conditionals: `if` and `else`.
- Support for multiple functions beyond `fn main`.
- Error handling and debugging features.
- Cross-platform compilation (Linux/Mac support).

## Notes
- Rust compiler (`rustc`) and Visual Studio Build Tools 2022 are required.
- Works on Windows only (generates `.exe`).
- Make sure to **place Kern in `C:\`** and **add the `compiler` folder to your PATH** so `kernc` can be run from any terminal.

