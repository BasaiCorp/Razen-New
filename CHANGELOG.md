# Razen Language - Changelog

All notable changes to the Razen programming language will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased - Preview Release]

### Added

#### String Escape Sequences Support
- **Full escape sequence processing** in the lexer scanner for both regular strings and f-strings
- Supported escape sequences:
  - `\n` - Newline
  - `\t` - Tab
  - `\r` - Carriage return
  - `\\` - Backslash
  - `\"` - Double quote
  - `\0` - Null character
- **Enhanced string handling** allows for proper formatting in output, tables, and multi-line text
- **F-string escape sequences** now work correctly with interpolated expressions
- Added comprehensive test suite (`tests/escape_sequences_test.rzn`) demonstrating all escape sequence functionality

**Example:**
```razen
var message = "Line 1\nLine 2\nLine 3"
var path = "C:\\Users\\Documents"
var quote = "She said \"Hello!\""
var table = "Name\tAge\tCity"
```

#### Character Literal Support
- **Full character literal parsing** with single quote syntax
- Supported features:
  - Basic characters: `'A'`, `'5'`, `'@'`, `' '`
  - Escape sequences: `'\n'`, `'\t'`, `'\r'`, `'\\'`, `'\''`
  - Character comparison operators: `==`, `!=`, `<`, `>`, `<=`, `>=`
  - ASCII ordering for comparisons
- **Type system integration** with `char` type
- **Seamless string interoperability** for output and conversion
- Added comprehensive test suite (`tests/character_test.rzn`)

**Example:**
```razen
var initial: char = 'J'
var grade: char = 'A'
var newline = '\n'
var tab = '\t'

// Character comparison
var result = 'A' < 'B'  // true (ASCII order)
```

#### Scientific Notation Support
- **Full scientific notation parsing** for floating-point numbers
- Supported formats:
  - Standard notation: `1.5e10`, `2.5e-3`
  - Uppercase E: `2.5E8`, `3.14E-2`
  - Positive/negative exponents: `6.022e23`, `6.626e-34`
- **Seamless integration** with all arithmetic operations
- **Precision handling** for very large and very small numbers
- Added comprehensive test suite (`tests/scientific_notation_test.rzn`)

**Example:**
```razen
var avogadro = 6.022e23      // 6.022 × 10²³
var planck = 6.626e-34       // 6.626 × 10⁻³⁴
var billion = 1e9            // 1,000,000,000
var tiny = 2.5e-3            // 0.0025
```

#### Enhanced Diagnostic System
- **Improved warning system** with better error reporting and suggestions
- **Fixed warning display** to show accurate line numbers and context
- **Enhanced error messages** with clearer descriptions and actionable fixes
- **Better diagnostic rendering** for compilation errors and warnings
- **Contextual error information** showing exact location of issues in source code

**Improvements:**
- Warning messages now display with proper formatting and color coding
- Line numbers accurately point to the exact location of issues
- Suggestions provided for common mistakes and fixes
- Improved error context showing surrounding code for better debugging

### Fixed

- **F-string parsing bug** where the first character after 'f' was being incorrectly consumed
- **String literal processing** now correctly handles escape sequences without data loss
- **Lexer token generation** for f-strings now preserves all characters correctly

### Changed

- **Documentation updated** to reflect working escape sequence features (removed '-' markers)
- **String documentation** (`docs/variables-datatypes/strings.md`) now accurately represents all supported features

---

## Development Notes

### Preview Release vs Stable Release
- **Dev/Beta releases**: Incremental updates and bug fixes
- **Preview releases**: Major feature additions and significant improvements
- **Stable releases**: Production-ready with extensive testing and documentation

### Upcoming Features
- String indexing and slicing
- Advanced string methods (split, replace, trim, etc.)
- Regular expression support
- Unicode and UTF-8 handling improvements

---

## Version History

### [0.1.9] - Current Development Version
- Active development of core language features
- RAJIT (Razen Adaptive JIT) compiler optimizations
- Module system implementation
- Standard library expansion

---

## Contributing

For bug reports and feature requests, please visit:
- GitHub: [BasaiCorp/Razen-New](https://github.com/BasaiCorp/Razen-New)
- Issues: [GitHub Issues](https://github.com/BasaiCorp/Razen-New/issues)

---

## License

Razen Language is licensed under the Apache 2.0 License. See LICENSE file for details.
