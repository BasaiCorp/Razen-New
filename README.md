# The Razen Programming Language

Razen is a modern, efficient programming language designed for building reliable and performant software with clean, readable syntax.

**Current Version: v0.1-beta.621**

This is the main source code repository for Razen. It contains the compiler, standard library, and documentation.

## Why Razen?

- **Performance**: Fast compilation and efficient execution, suitable for system programming, web services, and general-purpose applications.

- **Simplicity**: Clean, intuitive syntax that reduces cognitive overhead and makes code easier to read and maintain.

- **Reliability**: Strong type system and compile-time checks help catch errors early and ensure program correctness.

- **Productivity**: Professional toolchain with comprehensive CLI, clear error messages, and seamless development workflow.

## Quick Start

### Installation

#### One-Line Installation (Recommended)
```bash
curl -sSf https://raw.githubusercontent.com/BasaiCorp/Razen-New/main/install.sh | bash
```

#### Manual Installation
```bash
# Download the installer
curl -O https://raw.githubusercontent.com/BasaiCorp/Razen-New/main/install.sh

# Make it executable and run
chmod +x install.sh
./install.sh
```

#### Windows Users
Windows users should use **Git Bash** to run the installation commands above. Git Bash provides the necessary Unix-like environment for the installation script.

### Your First Program

Create a file named `hello.rzn`:
```razen
fun main() {
    println("Hello, Razen!")
    println("Welcome to modern programming!")
}
```

Run your program:
```bash
razen run hello.rzn
```

### More Examples

#### Object-Oriented Programming
```razen
struct Person {
    name: str,
    age: int
}

impl Person {
    fun new(name: str, age: int) -> Person {
        return Person { name: name, age: age }
    }
    
    fun greet(self) {
        printlnc(f"Hello, I'm {self.name}!", "green")
    }
}

fun main() {
    var person = Person.new("Alice", 25)
    person.greet()
}
```

#### Colored Output & F-Strings
```razen
fun main() {
    var name = "Razen"
    var version = "0.1-beta.621"
    
    // Colored output
    printlnc("Welcome to Razen!", "cyan")
    printc("Language: ", "yellow")
    printlnc(name, "bright_green")
    
    // F-string interpolation
    println(f"Version: {version}")
    printlnc(f"Hello from {name} v{version}!", "#FF6600")
}
```

#### Loops & Iteration
```razen
fun main() {
    // Range iteration
    for i in 1..=5 {
        printlnc(f"Count: {i}", "blue")
    }
    
    // Array iteration
    for name in ["Alice", "Bob", "Charlie"] {
        printlnc(f"Hello, {name}!", "green")
    }
    
    // While loops with break/continue
    var i = 1
    while i <= 10 {
        if i == 5 {
            i = i + 1
            continue
        }
        if i == 8 {
            break
        }
        println(f"Number: {i}")
        i = i + 1
    }
}
```

#### Module System
```razen
// math.rzn
pub fun add(a: int, b: int) -> int {
    return a + b
}

pub const PI = 3.14159

// main.rzn
use "./math.rzn"

fun main() {
    var result = math.add(5, 3)
    println(f"5 + 3 = {result}")
    println(f"PI = {math.PI}")
}
```
## Usage

After installation, you can use Razen with the following commands:

```bash
# Compile and run immediately (like go run)
razen run program.rzn

# Development mode with detailed compiler output
razen dev program.rzn

# Create a new Razen file
razen new hello --main

# Create a new Razen project
razen create my-project --template basic

# Initialize razen.toml in existing directory
razen init --name my-project

# Compile to native executable
razen compile program.rzn -o myprogram

# Run test files
razen test program.rzn

# Show help
razen --help

# Show version information
razen --version
```

## Installation Details

The installer performs the following actions:
1. Downloads the latest Razen compiler from GitHub
2. Installs it to `~/.razen/`
3. Adds `~/.razen/bin` to your PATH
4. Creates a global `razen` command

### Installation Directory Structure
```
~/.razen/
├── bin/
│   ├── razen-lang    # Main compiler binary
│   └── razen         # Symlink for easy access
├── version           # Version tracking for updates
└── scripts/          # Additional utility scripts
```

### Updating Razen

The installer automatically checks for updates when run again:
```bash
./install.sh
```

This will check your current version against the latest release and prompt you to update if a newer version is available.

### Uninstalling

To remove Razen from your system:
```bash
# Remove installation directory
rm -rf ~/.razen

# Remove from PATH (edit your shell profile)
# Remove the line: export PATH="$HOME/.razen/bin:$PATH"
```

## Language Features

### Core Language
- **Modern Syntax**: Clean, readable code structure with intuitive keywords and professional design
- **Static Typing**: Compile-time type checking with intelligent type inference and flexible typing modes
- **Memory Safety**: Automatic memory management with predictable performance characteristics

### Data Types & Structures
- **Primitive Types**: Integers, floats, strings, booleans, characters, and null values
- **Complex Types**: Arrays, maps, and custom data structures with full type safety
- **Structs & Enums**: User-defined types with field access and pattern matching support
- **Type Conversion**: Built-in conversion functions (`toint()`, `tostr()`, `tofloat()`, `tobool()`)

### Object-Oriented Programming
- **Impl Blocks**: Rust-like implementation blocks for methods and associated functions
- **Method Calls**: Dot notation for method invocation with proper `self` parameter handling
- **Static Methods**: Associated functions without `self` for constructor patterns
- **Member Access**: Direct field access and method chaining support

### Control Flow & Loops
- **Conditional Statements**: `if`, `elif`, `else` with proper scoping and type checking
- **Loop Constructs**: `while` loops and `for` loops with comprehensive iteration support
- **Range Iteration**: Both exclusive (`1..10`) and inclusive (`1..=10`) range syntax
- **Array Iteration**: Direct iteration over array literals and collections
- **Break & Continue**: Full support for loop control with proper nested loop handling

### String Processing
- **F-String Interpolation**: Python-style string formatting with `f"Hello, {name}!"` syntax
- **Expression Support**: Full expression evaluation within f-string braces including dot notation
- **String Operations**: Concatenation, length calculation, and manipulation functions
- **Color Output**: Built-in colored printing with `printc()` and `printlnc()` functions supporting 16+ colors and hex codes

### Module System
- **Module Imports**: `use` statements for importing external modules and libraries
- **Namespace Management**: Clean module organization with proper scoping and visibility
- **File-Based Modules**: Each `.rzn` file can be imported as a module
- **Visibility Control**: `pub` keyword for public declarations and controlled access

### Advanced Features
- **Pattern Matching**: `match` statements with comprehensive pattern support
- **Exception Handling**: `try`/`catch` blocks for robust error management
- **Operator Overloading**: Complete operator support including increment/decrement and compound assignment
- **Method Chaining**: Fluent interfaces with dot notation method calls
- **Type Inference**: Smart type detection while maintaining compile-time safety

### Development Experience
- **Professional Error Messages**: Clear, helpful diagnostics with suggestions and context
- **Debug Mode**: Comprehensive development mode with detailed compiler output
- **Fast Compilation**: Quick build times optimized for rapid development cycles
- **Cross-Platform**: Native support for Linux, macOS, and Windows

## Getting Help

For questions, bug reports, and discussions:
- Check the [documentation](docs/)
- Browse [existing issues](https://github.com/BasaiCorp/Razen-New/issues)
- Join our community discussions

## Contributing

We welcome contributions to Razen! To get started:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes and add tests
4. Commit your changes (`git commit -m 'Add amazing feature'`)
5. Push to the branch (`git push origin feature/amazing-feature`)
6. Open a Pull Request

Please read our [Contributing Guidelines](CONTRIBUTING.md) for detailed information about the development process, coding standards, and how to submit patches.

## License

Razen is distributed under the Apache License 2.0. See [LICENSE](LICENSE) for details.

## Trademark

The Razen name and logo are trademarks of BasaiCorp. Please see our trademark policy for usage guidelines.
