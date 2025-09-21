# Change Log

All notable changes to the "razen-language-support" extension will be documented in this file.

## [1.0.0] - 2024-09-21

### Added
- Initial release of Razen Language Support extension
- Complete syntax highlighting for Razen programming language
- 30+ code snippets for common Razen patterns
- IntelliSense and auto-completion support
- Language configuration for bracket matching and comments
- Document symbol provider for navigation
- Basic code formatting support
- Hover information for keywords and built-ins
- Custom Razen Dark theme
- Compile and run commands integration
- Support for `.rzn` and `.razen` file extensions

### Features
- **Syntax Highlighting**: Full support for all Razen language constructs
- **Code Snippets**: Quick scaffolding for functions, structs, enums, control flow
- **IntelliSense**: Smart completion for keywords, types, and built-in functions
- **Language Features**: Bracket matching, auto-closing, comment toggling, folding
- **Commands**: Integrated compile and run functionality
- **Theme**: Custom dark theme optimized for Razen code

### Supported Language Elements
- Keywords: `fun`, `var`, `const`, `struct`, `enum`, `if`, `while`, `for`, `match`, etc.
- Types: `int`, `str`, `bool`, `char`, `array`, `map`, `any`, `float`
- Built-ins: `print`, `println`, `input`, `read`, `write`, `open`, `close`
- Operators: Arithmetic, assignment, comparison, logical, bitwise
- String interpolation with `f"Hello {name}"`
- Comments: Single-line `//`, multi-line `/* */`, documentation `///`

### Technical Details
- Built with TypeScript and VS Code Extension API
- TextMate grammar for syntax highlighting
- JSON-based language configuration
- Comprehensive snippet library
- Document symbol parsing for navigation
- Basic formatting with indentation rules

## [Unreleased]

### Planned Features
- Language Server Protocol (LSP) integration
- Advanced error diagnostics
- Go-to-definition and find references
- Refactoring support
- Debugging integration
- More advanced code completion
- Semantic highlighting
- Code lens features
