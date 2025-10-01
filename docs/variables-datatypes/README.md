# Variables and Data Types

This section covers variables, data types, and the type system in Razen.

## Topics

### Core Concepts
- [Variables](variables.md) - Variable declaration, naming, scope, and reassignment
- [Type System](type-system.md) - Type annotations, inference, and type safety

### Primitive Types
- [Integers](integers.md) - Working with whole numbers
- [Floats](floats.md) - Working with decimal numbers
- [Strings](strings.md) - Working with text data
- [Booleans](booleans.md) - Working with true/false values
- [Characters](characters.md) - Working with single characters
- [Null](null.md) - Understanding null values

### Advanced Topics
- [Type Conversion](type-conversion.md) - Converting between types
- [Complex Types](complex-types.md) - Arrays, maps, and custom types

## Quick Reference

### Variable Declaration

```razen
var name = "Hanuman"           // Type inferred as str
var age: int = 25            // Explicit type annotation
var price = 19.99            // Type inferred as float
var isActive = true          // Type inferred as bool
```

### Type Annotations

```razen
var count: int = 0           // Strict: only int values allowed
var value = 0                // Flexible: type can change
```

### Common Operations

```razen
// Arithmetic
var sum = 10 + 5
var product = 10 * 5

// String concatenation
var fullName = firstName + " " + lastName

// String interpolation
var message = f"Hello, {name}!"

// Comparisons
var isEqual = x == y
var isGreater = x > y
```

## Learning Path

1. Start with [Variables](variables.md) to understand declaration and scope
2. Learn about [Type System](type-system.md) for type safety concepts
3. Study individual primitive types as needed:
   - [Integers](integers.md) for whole number operations
   - [Floats](floats.md) for decimal calculations
   - [Strings](strings.md) for text manipulation
   - [Booleans](booleans.md) for logical operations
4. Explore [Type Conversion](type-conversion.md) when working with multiple types
5. Move to [Complex Types](complex-types.md) for advanced data structures

## Examples

See the [examples folder](../../examples/variables-datatypes/) for complete, runnable programs demonstrating these concepts.
