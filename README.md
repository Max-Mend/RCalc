# RCalc
A simple calculator written in Rust.
For this project, I used one basic library:
```
use std::io;
```
The program uses a "loop" to continuously read user input.
Input is handled using the "io::stdin().read_line()" function.

##  Features
### Arithmetic Operations
- **Addition (+)** - Add two numbers
- **Subtraction (-)** - Subtract two numbers
- **Multiplication (*)** - Multiply two numbers
- **Division (/)** - Divide two numbers with zero protection

### Safety
- **Input Validation** - Ensures correct input format
- **Error Handling** - Clear error messages for invalid operations
- **Division by Zero Protection** - Prevents crashes from division errors

### Interface
- **Continuous Loop** - Perform multiple calculations without restarting
- **Clear Format** - Simple "number operator number" syntax
- **Instant Results** - Fast calculation and display

##  Quick Start
### Prerequisites
**Ubuntu/Debian:**
```bash
sudo apt install rustc
```

**Fedora:**
```bash
sudo dnf install rust
```

**Arch Linux:**
```bash
sudo pacman -S rust
```

### Build from Source
```bash
git clone https://github.com/yourusername/rcalc.git
cd rcalc
rustc main.rs -o rcalc
./rcalc
```

##  Usage
### Basic Calculations
Enter calculations in the format: `number operator number`

**Examples:**
```
3 + 5
Result: 8

10 - 2
Result: 8

4 * 7
Result: 28

20 / 4
Result: 5
```

### Input Format
```
<number> <operator> <number>
```
- **number**: Any floating-point number (e.g., 3, 5.5, -2.7)
- **operator**: One of: `+`, `-`, `*`, `/`
- **Format**: Numbers and operator must be separated by spaces

##  Technical Details
### Built With
- **Rust** - Systems programming language
- **std::io** - Standard input/output library
- **f64** - Double-precision floating-point numbers

### Architecture
- Single-file application for simplicity
- Pattern matching for operations
- Robust error handling with Result types

##  Supported Operations
Performs **basic arithmetic**:
- `+` - Addition
- `-` - Subtraction
- `*` - Multiplication
- `/` - Division

##  Customization
### Change Number Type
Edit `main.rs` lines 18 and 25:
```rust
let num1: f64 = match parts[0].parse() {  // Change f64 to f32 or i32
```

### Add New Operations
Edit `main.rs` line 35:
```rust
let res = match operation {
    "+" => num1 + num2,
    "^" => num1.powf(num2),  // Add power operation
    // Add more operations here
```

##  Known Limitations
- Only basic arithmetic operations
- No parentheses support
- No multi-step calculations
- Requires spaces between numbers and operators
- No calculation history
- No scientific functions

##  Contributing
Contributions are welcome! Feel free to:
- Report bugs via Issues
- Suggest new features
- Submit pull requests
- Improve documentation

##  License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

##  Requirements
- **Rust** 1.70+ (or latest stable)
- **rustc** (Rust compiler)

##  Use Cases
Perfect for:
- Quick calculations in terminal
- Learning Rust basics
- Simple arithmetic tasks
- Command-line workflows

##  Author
**Max-Mend**

##  Acknowledgments
- Built with Rust programming language
- Inspired by simple CLI calculators

---
*Simple, fast, and reliable calculations.*
