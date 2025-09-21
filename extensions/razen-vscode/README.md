# Razen Language Support for Visual Studio Code

A comprehensive Visual Studio Code extension that provides rich language support for the **Razen programming language**.

## Features

### **Syntax Highlighting**
- Complete syntax highlighting for all Razen language constructs
- Support for keywords, operators, types, strings, numbers, and comments
- Custom color theme optimized for Razen code
- String interpolation highlighting with `f"Hello {name}"`
- Enhanced function and variable recognition

### **Code Snippets**
- 30+ built-in code snippets for common Razen patterns
- Function declarations, control structures, data types
- Quick scaffolding for main functions, structs, enums, and more
- Type `main` + Tab to create a main function instantly

### **IntelliSense & Auto-completion**
- Smart auto-completion for keywords, types, and built-in functions
- Context-aware suggestions with detailed documentation
- Parameter hints for built-in functions
- Hover information for language constructs

### **Language Features**
- **Bracket Matching**: Automatic bracket, parentheses, and brace matching
- **Auto-closing Pairs**: Smart closing of brackets, quotes, and braces
- **Comment Toggling**: Quick comment/uncomment with `Ctrl+/`
- **Code Folding**: Fold functions, structs, and code blocks
- **Document Symbols**: Navigate through functions, structs, and variables
- **Code Formatting**: Basic indentation and formatting support

### **Compilation & Execution**
- **Compile Command**: `Ctrl+Shift+P` → "Razen: Compile"
- **Run Command**: `Ctrl+Shift+P` → "Razen: Run"
- **Create Sample**: `Ctrl+Shift+P` → "Razen: Create Sample File"
- **Documentation**: `Ctrl+Shift+P` → "Razen: Show Documentation"
- Integrated terminal support for Razen compiler
- Context menu integration for .rzn files

## Supported File Extensions

- `.rzn` - Primary Razen source files
- `.razen` - Alternative Razen source files

## Language Constructs Supported

### Keywords
```razen
const var fun struct enum if else elif while for in return break continue
match try catch throw mod use pub from as
```

### Types
```razen
int str bool char array map any float
```

### Built-in Functions
```razen
print println input read write open close
```

### Operators
```razen
+ - * / % ** += -= *= /= %= == != < > <= >= && || ! & | ^ ~ << >> ++ --
```

## Code Snippets Examples

| Trigger | Description | Result |
|---------|-------------|--------|
| `main` | Main function | `fun main() { ... }` |
| `fun` | Function with return type | `fun name(params) -> type { ... }` |
| `if` | If statement | `if condition { ... }` |
| `for` | For loop | `for item in iterable { ... }` |
| `struct` | Struct definition | `struct Name { ... }` |
| `enum` | Enum definition | `enum Name { ... }` |
| `println` | Print with newline | `println("message")` |

## Installation

### From Source (Development)

1. Clone the Razen language repository
2. Navigate to `extensions/razen-vscode/`
3. Install dependencies:
   ```bash
   npm install
   ```
4. Compile the extension:
   ```bash
   npm run compile
   ```
5. Press `F5` to launch a new VS Code window with the extension loaded

### Package and Install

1. Install `vsce` (Visual Studio Code Extension manager):
   ```bash
   npm install -g vsce
   ```
2. Package the extension:
   ```bash
   vsce package
   ```
3. Install the generated `.vsix` file:
   ```bash
   code --install-extension razen-language-support-1.0.0.vsix
   ```

## Usage

1. Open a `.rzn` or `.razen` file
2. Enjoy syntax highlighting and IntelliSense
3. Use code snippets by typing triggers and pressing Tab
4. Compile and run Razen programs using the command palette

### Example Razen Code

```razen
fun main() {
    println("Hello, Razen!")
    
    var name: str = input("Enter your name: ")
    var age: int = 25
    
    println(f"Hello {name}, you are {age} years old!")
    
    if age >= 18 {
        println("You are an adult")
    } else {
        println("You are a minor")
    }
    
    for i in 0..5 {
        println(f"Count: {i}")
    }
}

struct Person {
    name: str,
    age: int
}

enum Color {
    Red,
    Green,
    Blue
}
```

## Requirements

- Visual Studio Code 1.74.0 or higher
- Razen compiler (for compilation and execution features)

## Extension Settings

This extension contributes the following settings:

- File associations for `.rzn` and `.razen` files
- Syntax highlighting configuration
- Code snippet definitions
- Language configuration for brackets and comments

## Commands

- `razen.compile`: Compile the current Razen file
- `razen.run`: Compile and run the current Razen file

## Known Issues

- Advanced language server features (go-to-definition, find references) are not yet implemented
- Error diagnostics require the Razen compiler to be in PATH

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

## Release Notes

### 1.0.0

- Initial release
- Complete syntax highlighting
- 30+ code snippets
- IntelliSense and auto-completion
- Basic language features (bracket matching, comments, folding)
- Compile and run commands
- Custom Razen Dark theme

## License

MIT License - see LICENSE file for details.

---

**Enjoy coding in Razen!** 🚀

For more information about the Razen programming language, visit the [Razen GitHub repository](https://github.com/razen-lang/razen).
