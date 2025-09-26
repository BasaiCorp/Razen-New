# Razen Language Feature Documentation

## Overview

Razen is a modern, efficient programming language with clean syntax and powerful features. This document provides a comprehensive overview of the language's current capabilities and implementation status.

**Current Version**: 0.1-beta.5  
**Language Completion**: ~97%

## Language Features

### Core Language

The Razen language provides a complete foundation for modern programming:

**Variables and Constants**
- Variable declarations with type inference: `var x = 5`
- Explicit type annotations: `var y: int = 10`
- Constant declarations: `const name = "value"`

**Functions**
- Function declarations: `fun name(param: type) -> type { }`
- Function calls with arguments: `function(args)`
- Return statements: `return value`

**Data Types**
- Integers: `42`, `100`
- Floating-point numbers: `3.14`, `2.5`
- Strings: `"hello"`, `'world'`
- Booleans: `true`, `false`
- Null values: `null`
- Arrays: `[1, 2, 3]`, `["a", "b", "c"]`
- Maps/Dictionaries: `{"key": "value", "count": 42}`

### Operators

Razen supports a comprehensive set of operators for various operations:

**Arithmetic Operators**
- Basic arithmetic: `+`, `-`, `*`, `/`, `%`
- Unary operators: `-` (negation), `+` (positive)

**Assignment Operators**
- Simple assignment: `=`
- Compound assignment: `+=`, `-=`, `*=`, `/=`, `%=`
- Bitwise assignment: `&=`, `|=`, `^=`, `<<=`, `>>=`

**Comparison Operators**
- Equality: `==`, `!=`
- Relational: `<`, `>`, `<=`, `>=`

**Logical Operators**
- Boolean logic: `&&`, `||`, `!`

**Bitwise Operators**
- Bitwise operations: `&`, `|`, `^`, `~`
- Bit shifting: `<<`, `>>`

**Increment/Decrement Operators**
- Pre-increment/decrement: `++var`, `--var`
- Post-increment/decrement: `var++`, `var--`

### Control Flow

**Conditional Statements**
- If statements: `if condition { }`
- If-else chains: `if condition { } else { }`
- Multi-branch conditions: `if condition { } elif condition { } else { }`

**Loops**
- While loops: `while condition { }`
- For loops with iterables: `for item in iterable { }`
- Range iteration: `for i in 1..5 { }` (exclusive), `for i in 1..=5 { }` (inclusive)
- Array iteration: `for item in [1, 2, 3] { }`
- Loop control: `break`, `continue`

**Pattern Matching**
- Match statements: `match value { pattern => result }`
- Literal patterns and wildcards supported

### Data Structures

**Structs**
- Struct declarations: `struct Person { name: str, age: int }`
- Struct instantiation: `Person { name: "Alice", age: 30 }`
- Member access: `person.name`, `person.age`

**Enums**
- Enum declarations: `enum Color { Red, Green, Blue }`
- Enum variants: `Color.Red`, `Color.Green`
- Tuple variants: `RGB(int, int, int)`

### String Features

**String Interpolation**
- F-string syntax: `f"Hello, {name}!"`
- Expression interpolation: `f"Result: {x + y}"`

**String Operations**
- String concatenation: `"hello" + " world"`
- String literals with escape sequences

### Input/Output

**Console I/O**
- Print functions: `print()`, `println()`
- Input function: `input()` with optional prompts
- Type conversion methods: `.toint()`, `.tofloat()`, `.tostr()`, `.tobool()`

**File I/O** (Basic Support)
- File operations: `read()`, `write()`, `open()`, `close()`

### Error Handling

**Exception Handling**
- Try-catch blocks: `try { } catch error { }`
- Throw statements: `throw "error message"`
- Basic exception propagation

## Implementation Status

### Completed Features

The following features are fully implemented and tested:

- **Core Language**: Complete variable system, function declarations, and basic syntax
- **All Operators**: Full operator support including modern assignment and bitwise operators
- **Control Flow**: Complete conditional and loop constructs with proper scoping
- **Data Structures**: Struct and enum support with member access
- **String Processing**: F-string interpolation and string operations
- **I/O Operations**: Console input/output and basic file operations
- **Error Handling**: Try-catch exception handling

### Known Limitations

**Minor Issues**
- Complex expressions in f-strings may have type inference edge cases
- Recursive function type inference needs refinement

**Future Enhancements**
- Module system for code organization
- Advanced pattern matching capabilities
- Generic types and functions
- Closure and lambda expressions

## Language Stability

Razen has reached a high level of stability and completeness:

- **Core Language**: Production ready
- **Standard Operations**: Fully functional
- **Error Handling**: Robust and reliable
- **Performance**: Optimized compilation pipeline

The language is suitable for:
- Educational programming
- Scripting and automation
- Small to medium applications
- Prototype development

## Getting Started

To begin using Razen:

1. **Installation**: Download and install the Razen compiler
2. **Hello World**: Create your first program with `fun main() { println("Hello, Razen!") }`
3. **Documentation**: Refer to language tutorials and examples
4. **Community**: Join the Razen programming community for support

## Version History

**0.1-beta.5** (Current)
- Complete operator support implementation
- Enhanced data structure capabilities
- Improved error handling and diagnostics
- Professional CLI tooling

For detailed version history and changelog, see the project repository.
