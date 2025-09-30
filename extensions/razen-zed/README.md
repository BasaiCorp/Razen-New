# Razen Language Support for Zed

Comprehensive language support for the Razen programming language in Zed editor.

## Features

- **Syntax Highlighting**: Full syntax highlighting for all Razen language constructs
- **Bracket Matching**: Automatic bracket, brace, and parenthesis matching
- **Code Outline**: Navigate your code structure with the outline view
- **Auto-Indentation**: Smart indentation based on code structure
- **Code Folding**: Fold functions, structs, and blocks
- **Comment Support**: Line comments (`//`) and block comments (`/* */`)

## Supported Language Features

### Core Syntax
- Variable declarations (`var`, `const`)
- Function declarations (`fun`)
- Control flow (`if`, `else`, `elif`, `while`, `for`, `match`)
- Data structures (`struct`, `enum`, `impl`)
- Module system (`use`, `pub`, `mod`)

### Advanced Features
- F-string interpolation (`f"Hello, {name}"`)
- Range expressions (`1..10`, `1..=10`)
- Method calls and member access
- Pattern matching
- Self references in impl blocks
- Array and map literals

### Built-in Types
- `int`, `float`, `str`, `bool`, `char`
- `any` (dynamic type)
- Arrays: `[type]`
- Maps: `{key: value}`

## Installation

### Method 1: Install as Dev Extension (Recommended for Development)

1. Clone or download this extension
2. Open Zed
3. Press `Cmd+Shift+P` (Mac) or `Ctrl+Shift+P` (Linux/Windows)
4. Type "Install Dev Extension"
5. Select the `razen-zed` directory

### Method 2: Publish to Zed Extensions Registry

To publish this extension to the official Zed extensions registry:

1. Fork the [zed-industries/extensions](https://github.com/zed-industries/extensions) repository
2. Add this extension as a submodule:
   ```bash
   git submodule add https://github.com/razen-lang/razen-zed.git extensions/razen
   ```
3. Add entry to `extensions.toml`:
   ```toml
   [razen]
   submodule = "extensions/razen"
   version = "0.1.0"
   ```
4. Run `pnpm sort-extensions`
5. Submit a pull request

## Building the Tree-sitter Grammar

The extension uses a custom Tree-sitter grammar for Razen. To build it:

```bash
cd tree-sitter-razen
npm install
npx tree-sitter generate
npx tree-sitter test
```

## Example Razen Code

```razen
// Variable declarations
var name = "Razen"
const VERSION = "1.0.0"

// Function declaration
fun greet(name: str) {
    println(f"Hello, {name}!")
}

// Struct and impl
struct Person {
    name: str,
    age: int
}

impl Person {
    fun new(name: str, age: int) -> Person {
        return Person { name: name, age: age }
    }
    
    fun greet(self) {
        println(f"Hi, I'm {self.name}")
    }
}

// Main function
fun main() {
    var person = Person::new("Alice", 25)
    person.greet()
    
    // Control flow
    for i in 1..=10 {
        if i % 2 == 0 {
            println(f"{i} is even")
        }
    }
}
```

## File Extensions

The extension recognizes the following file extensions:
- `.rzn`
- `.razen`

## Configuration

The extension uses sensible defaults:
- Tab size: 4 spaces
- Soft tabs (spaces, not tabs)
- Line comments: `//`
- Block comments: `/* */`

You can override these in your Zed settings.

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

### Development Setup

1. Clone the repository
2. Make your changes
3. Test with Zed's dev extension feature
4. Submit a pull request

## Roadmap

- [ ] LSP server integration (in progress)
- [ ] Debugger support
- [ ] Code completion
- [ ] Hover documentation
- [ ] Go to definition
- [ ] Find references
- [ ] Refactoring support

## License

MIT License - see LICENSE file for details

## Links

- [Razen Language Repository](https://github.com/razen-lang/razen-lang-new)
- [Zed Editor](https://zed.dev)
- [Tree-sitter](https://tree-sitter.github.io)

## Support

For issues or questions:
- Open an issue on GitHub
- Join our community discussions
- Check the documentation

---

**Made with ❤️ for the Razen community**
