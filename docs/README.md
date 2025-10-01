# Razen Documentation

Professional documentation for the Razen programming language.

## Getting Started

- [Getting Started](getting-started.md) - Installation and first program
- [Installation Guide](installation.md) - Detailed installation instructions

## Core Concepts

### Variables and Data Types

- [Overview](variables-datatypes/README.md) - Variables and data types overview
- [Variables](variables-datatypes/variables.md) - Declaration, scope, and reassignment
- [Type System](variables-datatypes/type-system.md) - Type annotations and inference
- [Type Conversion](variables-datatypes/type-conversion.md) - Converting between types

### Primitive Types

- [Integers](variables-datatypes/integers.md) - Working with whole numbers
- [Floats](variables-datatypes/floats.md) - Working with decimal numbers
- [Strings](variables-datatypes/strings.md) - Working with text data
- [Booleans](variables-datatypes/booleans.md) - Working with true/false values
- [Characters](variables-datatypes/characters.md) - Working with single characters
- [Null](variables-datatypes/null.md) - Understanding null values

## Documentation Structure

### By Topic

Each topic has its own focused documentation file:
- Clear explanations with examples
- Common use cases and patterns
- Best practices and pitfalls
- Links to related topics

### By Learning Path

1. **Start Here**
   - [Getting Started](getting-started.md)
   - [Installation](installation.md)

2. **Learn Variables**
   - [Variables](variables-datatypes/variables.md)
   - [Type System](variables-datatypes/type-system.md)

3. **Master Data Types**
   - [Integers](variables-datatypes/integers.md)
   - [Floats](variables-datatypes/floats.md)
   - [Strings](variables-datatypes/strings.md)
   - [Booleans](variables-datatypes/booleans.md)

4. **Advanced Topics**
   - [Type Conversion](variables-datatypes/type-conversion.md)
   - [Characters](variables-datatypes/characters.md)
   - [Null](variables-datatypes/null.md)

## Examples

All documentation topics include runnable examples in the [examples](../examples/) directory:

- [Variables and Data Types Examples](../examples/variables-datatypes/)

## Quick Reference

### Variable Declaration

```razen
var name = "Hanuman"           // Type inferred
var age: int = 25            // Type annotation
```

### Type Annotations

```razen
var count: int = 0           // Strict typing
var value = 0                // Flexible typing
```

### F-String Interpolation

```razen
var name = "Hanuman"
var message = f"Hello, {name}!"
```

### Type Conversion

```razen
var number = toint("123")    // str to int
var text = tostr(42)         // int to str
var decimal = tofloat(10)    // int to float
```

## Documentation Style

This documentation follows professional standards:
- Clear, concise explanations
- Practical, runnable examples
- Best practices and common patterns
- No unnecessary verbosity
- Focus on user needs

## Contributing

To contribute to documentation:
1. Follow the existing style and structure
2. Include practical examples
3. Keep explanations clear and concise
4. Test all code examples
5. Link to related topics

## Getting Help

- Read the relevant documentation section
- Check the [examples](../examples/) directory
- Review common patterns and best practices
- Report issues on GitHub

## Next Steps

After learning the basics:
1. Explore [Control Flow](control-flow.md) (coming soon)
2. Study [Functions](functions.md) (coming soon)
3. Learn [Object-Oriented Programming](oop.md) (coming soon)
4. Master [Modules](modules.md) (coming soon)
