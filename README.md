# RCalc
A terminal-based calculator with TUI written in Rust.

## Libraries Used
```rust
ratatui
crossterm
```

## Features
### Arithmetic Operations
- **Addition (+)** - Add two numbers
- **Subtraction (-)** - Subtract two numbers
- **Multiplication (*)** - Multiply two numbers
- **Division (/)** - Divide with zero protection
- **Power (**)** - Raise to power
- **Root (//)** - Calculate square root

### Interface
- **TUI Display** - Terminal User Interface with Ratatui
- **Real-time Input** - Interactive input field
- **Calculation History** - Last 20 calculations shown
- **Color Coding** - Red borders, yellow results, green input
- **Keyboard Controls** - Q to quit, Enter to calculate, C to clear

### Safety
- **Input Validation** - Checks for correct format
- **Error Handling** - Clear error messages
- **Division by Zero Protection** - Prevents crashes

## Quick Start
### Prerequisites
**Ubuntu/Debian:**
```bash
sudo apt install rustc cargo
```

**Fedora:**
```bash
sudo dnf install rust cargo
```

**Arch Linux:**
```bash
sudo pacman -S rust
```

### Build from Source
```bash
git clone https://github.com/Max-Mend/rcalc.git
cd rcalc
cargo build --release
cargo run
```

## Usage
Type calculations directly in the input field.

**Examples:**
```
3+5
Result: 8

10-2
Result: 8

4*7
Result: 28

20/4
Result: 5

2**3
Result: 8

4//
Result: 2
```

### Commands
- `help` - Show available commands
- `info` - Show calculator info
- `exit` or `Q` - Exit calculator
- `c` - Clear history

### Keyboard Controls
- **Q** - Quit application
- **Enter** - Calculate expression
- **Backspace** - Delete character
- **C** - Clear history (shows confirmation for 0.5s)

### Input Format
- No spaces required
- Supports floating-point numbers
- Operators: `+`, `-`, `*`, `/`, `**`, `//`

## Technical Details
### Built With
- **Rust** - Systems programming language
- **Ratatui** - Terminal UI framework
- **Crossterm** - Cross-platform terminal manipulation
- **f64** - Double-precision floating-point

### Architecture
- Modular structure with separated concerns
- `main.rs` - Entry point
- `ui.rs` - User interface and event handling
- `func.rs` - Calculation logic
- Pattern matching for operations
- Error handling with Result types

## Customization
Want to modify the calculator?

**Change colors:**
```rust
.border_style(Style::default().fg(Color::Red))  // Change Red to any color
```

**Change history size:**
```rust
.take(20)  // Change 20 to desired number
```

**Change temp message duration:**
```rust
Duration::from_millis(500)  // Change 500 to desired milliseconds
```

## Known Limitations
- Basic arithmetic only
- No parentheses support
- No multi-step calculations
- No scientific functions
- Single operation per calculation

## Contributing
Feel free to:
- Report bugs via Issues
- Suggest features
- Submit pull requests
- Improve documentation

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Requirements
- **Rust** 1.70+
- **Cargo** package manager

## Author
**Max-Mend**

---
*Simple, fast, and reliable calculations with a modern TUI.*
