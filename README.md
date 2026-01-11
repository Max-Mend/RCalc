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
- **Division (/)** - Divide with zero protection

### Safety
- **Input Validation** - Checks for correct format
- **Error Handling** - Clear messages for invalid input
- **Division by Zero Protection** - Prevents crashes

### Interface
- **Continuous Loop** - Multiple calculations without restarting
- **Simple Format** - Just "number operator number"
- **Instant Results** - Quick output

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
git clone https://github.com/Max-Mend/rcalc.git
cd rcalc
rustc main.rs -o rcalc
./rcalc
```

##  Usage
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
- **number**: Any floating-point number (3, 5.5, -2.7)
- **operator**: `+`, `-`, `*`, `/`
- Numbers and operator must be separated by spaces

##  Technical Details
### Built With
- **Rust** - Systems programming language
- **std::io** - Standard input/output
- **f64** - Double-precision floating-point

### Architecture
- Single-file application
- Pattern matching for operations
- Error handling with Result types

##  Customization
Want to modify the calculator? Here's how:

**Change number precision:**
```rust
let num1: f64 = match parts[0].parse() {  // Try f32 or i32
```

**Add new operations:**
```rust
let res = match operation {
    "+" => num1 + num2,
    "^" => num1.powf(num2),  // Power operation
    "%" => num1 % num2,       // Modulo
```

##  Known Limitations
- Basic arithmetic only
- No parentheses
- No multi-step calculations
- Spaces required between input
- No calculation history
- No scientific functions

##  Contributing
Feel free to:
- Report bugs via Issues
- Suggest features
- Submit pull requests
- Improve docs

##  License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

##  Requirements
- **Rust** 1.70+
- **rustc** compiler

##  Author
**Max-Mend**

---
*Simple, fast, and reliable calculations.*
