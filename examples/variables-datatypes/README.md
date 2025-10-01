# Variables and Data Types Examples

This directory contains practical examples demonstrating variables and data types in Razen.

## Running Examples

To run any example:

```bash
razen run <filename>.rzn
```

For detailed output:

```bash
razen dev <filename>.rzn
```

## Example Files

### 01-variables-basic.rzn
Demonstrates basic variable declaration, reassignment, type annotations, and flexible typing.

**Topics Covered:**
- Variable declaration
- Variable reassignment
- Type annotations
- Flexible typing without annotations

**Run:**
```bash
razen run 01-variables-basic.rzn
```

### 02-integers.rzn
Shows integer operations, arithmetic, comparisons, and common patterns.

**Topics Covered:**
- Arithmetic operations (add, subtract, multiply, divide, modulo)
- Comparison operations
- Counter patterns
- Sum calculations
- Even/odd checking

**Run:**
```bash
razen run 02-integers.rzn
```

### 03-floats.rzn
Demonstrates floating-point operations and real-world calculations.

**Topics Covered:**
- Float arithmetic
- Price calculations
- Tax calculations
- Circle area
- Temperature conversion
- Average calculations

**Run:**
```bash
razen run 03-floats.rzn
```

### 04-strings.rzn
Shows string operations, concatenation, and f-string interpolation.

**Topics Covered:**
- String concatenation
- F-string interpolation
- Building messages
- User input
- Colored output
- String formatting

**Run:**
```bash
razen run 04-strings.rzn
```

### 05-booleans.rzn
Demonstrates boolean operations, logical expressions, and control flow.

**Topics Covered:**
- Logical operations (AND, OR, NOT)
- Comparison operations
- Validation patterns
- Access control
- Range checking
- Control flow with booleans

**Run:**
```bash
razen run 05-booleans.rzn
```

### 06-type-conversion.rzn
Shows how to convert between different types.

**Topics Covered:**
- String conversions (to/from int, float, bool)
- Numeric conversions (int to float, float to int)
- User input processing
- F-string automatic conversion
- Mathematical operations with conversion

**Run:**
```bash
razen run 06-type-conversion.rzn
```

### 07-type-system.rzn
Demonstrates Razen's type system features.

**Topics Covered:**
- Type inference
- Flexible typing
- Strict typing with annotations
- Typed function parameters
- Type safety
- Mixed type operations

**Run:**
```bash
razen run 07-type-system.rzn
```

## Learning Path

1. Start with **01-variables-basic.rzn** to understand variable basics
2. Learn numeric types with **02-integers.rzn** and **03-floats.rzn**
3. Master text with **04-strings.rzn**
4. Understand logic with **05-booleans.rzn**
5. Learn conversions with **06-type-conversion.rzn**
6. Explore type safety with **07-type-system.rzn**

## Key Concepts

### Variables
- Declared with `var` keyword
- Can have optional type annotations
- Support reassignment
- Scoped to their block

### Type Annotations
```razen
var name: str = "Hanuman"
var age: int = 25
var price: float = 19.99
var isActive: bool = true
```

### Type Inference
```razen
var name = "Hanuman"     // Inferred as str
var age = 25           // Inferred as int
var price = 19.99      // Inferred as float
```

### F-String Interpolation
```razen
var name = "Hanuman"
var age = 25
var message = f"{name} is {age} years old"
```

### Type Conversion
```razen
var number = 42
var text = tostr(number)      // int to str
var value = toint("123")      // str to int
var decimal = tofloat(10)     // int to float
```

## Common Patterns

### Counter Pattern
```razen
var count = 0
for i in 1..=10 {
    count = count + 1
}
```

### Validation Pattern
```razen
var age = 25
var hasLicense = true
var canDrive = age >= 18 && hasLicense
```

### Input Processing
```razen
var input = input("Enter your age: ")
var age = toint(input)
```

### String Building
```razen
var firstName = "John"
var lastName = "Doe"
var fullName = firstName + " " + lastName
```

## Documentation

For detailed documentation, see:
- [Variables](../../docs/variables-datatypes/variables.md)
- [Integers](../../docs/variables-datatypes/integers.md)
- [Floats](../../docs/variables-datatypes/floats.md)
- [Strings](../../docs/variables-datatypes/strings.md)
- [Booleans](../../docs/variables-datatypes/booleans.md)
- [Type System](../../docs/variables-datatypes/type-system.md)
- [Type Conversion](../../docs/variables-datatypes/type-conversion.md)

## Next Steps

After mastering variables and data types:
1. Learn about [Control Flow](../control-flow/)
2. Explore [Functions](../functions/)
3. Study [Object-Oriented Programming](../oop/)
