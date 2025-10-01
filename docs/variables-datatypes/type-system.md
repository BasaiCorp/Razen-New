# Type System

Razen's type system provides both flexibility and safety. This guide covers type annotations, type inference, and type safety concepts.

## Overview

Razen supports two typing modes:

1. **Flexible Typing**: Variables without type annotations can change types
2. **Strict Typing**: Variables with type annotations enforce type safety

## Type Annotations

Type annotations explicitly specify the type of a variable.

### Syntax

```razen
var variableName: type = value
```

### Basic Type Annotations

```razen
var name: str = "Hanuman"
var age: int = 7000
var price: float = 19.99
var isActive: bool = true
```

### Benefits of Type Annotations

1. **Type Safety**: Prevents accidental type changes
2. **Documentation**: Makes code self-documenting
3. **Early Error Detection**: Catches type errors at compile time
4. **Better Tooling**: Enables IDE autocomplete and suggestions

### Example: Type Safety

```razen
var count: int = 10
count = 20           // OK: int value
count = count + 5    // OK: int operation

// count = "hello"   // Error: cannot assign str to int
// count = 3.14      // Error: cannot assign float to int
```

## Type Inference

Razen can automatically infer types from initial values.

### Automatic Inference

```razen
var name = "Hanuman"     // Inferred as str
var age = 7000           // Inferred as int
var price = 19.99      // Inferred as float
var isActive = true    // Inferred as bool
```

### How Inference Works

The compiler determines type from the initial value:

```razen
var x = 10          // int (no decimal point)
var y = 10.0        // float (has decimal point)
var z = "10"        // str (in quotes)
var w = true        // bool (boolean literal)
```

### Inference in Expressions

```razen
var sum = 10 + 20           // int (both operands are int)
var result = 10.5 + 5       // float (one operand is float)
var message = "Hello" + "!" // str (string concatenation)
```

## Flexible vs Strict Typing

### Flexible Typing (No Annotation)

Variables without type annotations can change type:

```razen
var value = 10        // Initially int
println(value)        // Output: 10

value = "hello"       // Now str
println(value)        // Output: hello

value = true          // Now bool
println(value)        // Output: true
```

Use flexible typing when:
- Prototyping or experimenting
- Type changes are intentional
- Working with dynamic data

### Strict Typing (With Annotation)

Variables with type annotations enforce type safety:

```razen
var count: int = 0
count = 10           // OK: same type
count = count + 5    // OK: same type

// count = "hello"   // Error: type mismatch
// count = 3.14      // Error: type mismatch
```

Use strict typing when:
- Type safety is important
- Building production code
- Working in teams
- Type should never change

## Type Compatibility

### Exact Type Matching

With type annotations, types must match exactly:

```razen
var age: int = 25
// age = 25.0        // Error: float is not int

var price: float = 19.99
// price = 19        // Error: int is not float
```

### Type Coercion

Some operations allow implicit type coercion:

```razen
// Integer + Float = Float
var result = 10 + 5.5       // 15.5 (float)

// String + Any = String
var message = "Count: " + tostr(10)
```

## Function Parameter Types

Functions can specify parameter types:

### Typed Parameters

```razen
fun greet(name: str) {
    println(f"Hello, {name}!")
}

greet("Hanuman")      // OK
// greet(42)        // Error: expected str, got int
```

### Return Type Annotations

```razen
fun add(a: int, b: int) -> int {
    return a + b
}

var sum = add(10, 20)  // sum is int
```

### Multiple Parameter Types

```razen
fun formatPrice(amount: float, currency: str) -> str {
    return f"{currency}{amount}"
}

var price = formatPrice(19.99, "$")
println(price)  // Output: $19.99
```

## Type Checking

Razen performs compile-time type checking:

### Compile-Time Errors

```razen
var count: int = 10

// These cause compile-time errors:
// count = "hello"           // Error: type mismatch
// count = 3.14              // Error: type mismatch
// var x: str = 42           // Error: type mismatch
```

### Type Validation

```razen
fun divide(a: int, b: int) -> int {
    if b == 0 {
        println("Error: Division by zero")
        return 0
    }
    return a / b
}

var result = divide(10, 2)  // OK: both parameters are int
// var bad = divide(10, "2") // Error: second parameter must be int
```

## Common Type Patterns

### Optional Type Annotations

Use annotations where clarity is needed:

```razen
// Clear without annotation
var count = 0
var name = "Hanuman"
var price = 19.99

// Better with annotation
var userId: int = 12345
var accountBalance: float = 1000.50
var isAdmin: bool = false
```

### Type Guards

Check types before operations:

```razen
fun processValue(value) {
    // Type checking at runtime
    if typeof(value) == "int" {
        println(f"Integer: {value}")
    } elif typeof(value) == "str" {
        println(f"String: {value}")
    } else {
        println("Unknown type")
    }
}
```

### Type Conversion

Convert between types explicitly:

```razen
var text = "123"
var number = toint(text)        // str -> int

var count = 42
var message = tostr(count)      // int -> str

var integer = 10
var decimal = tofloat(integer)  // int -> float
```

## Best Practices

### Use Type Annotations for Public APIs

```razen
// Good: Clear function signature
fun calculateTotal(price: float, quantity: int) -> float {
    return price * tofloat(quantity)
}

// Avoid: Unclear parameter types
fun calculateTotal(price, quantity) {
    return price * quantity
}
```

### Use Inference for Local Variables

```razen
// Good: Type is obvious from context
fun processOrder() {
    var total = 0.0
    var count = 0
    var items = []
    
    // Process order...
}
```

### Annotate Complex Types

```razen
// Good: Complex type is documented
var userMap: map<int, str> = {}
var coordinates: [float] = [10.5, 20.3, 15.7]

// Avoid: Type is unclear
var data = {}
var values = []
```

### Be Consistent

```razen
// Good: Consistent style
var userId: int = 12345
var userName: str = "Hanuman"
var userAge: int = 25

// Avoid: Inconsistent style
var userId: int = 12345
var userName = "Hanuman"
var userAge = 25
```

## Type Safety Benefits

### Catch Errors Early

```razen
var count: int = 0

// Caught at compile time, not runtime
// count = "hello"  // Error: type mismatch
```

### Prevent Invalid Operations

```razen
fun divide(a: int, b: int) -> int {
    return a / b
}

// Caught at compile time
// divide(10, "2")  // Error: expected int, got str
```

### Document Intent

```razen
// Type annotations document expected types
fun createUser(name: str, age: int, email: str) -> bool {
    // Implementation
    return true
}
```

### Enable Better Tooling

Type annotations enable:
- IDE autocomplete
- Type checking
- Refactoring tools
- Documentation generation

## Advanced Type Concepts

### Generic Types

Arrays and maps can hold specific types:

```razen
var numbers: [int] = [1, 2, 3, 4, 5]
var names: [str] = ["Hanuman", "Ram", "Brahma"]
var prices: [float] = [19.99, 29.99, 39.99]
```

### Nullable Types

Variables can hold null values:

```razen
var value = null
var name: str = "Hanuman"

// Check for null
if value == null {
    println("Value is null")
}
```

### Type Aliases

Create meaningful type names:

```razen
// Define type alias
type UserId = int
type EmailAddress = str

// Use aliases
var id: UserId = 12345
var email: EmailAddress = "user@example.com"
```

## Common Type Errors

### Type Mismatch

```razen
var count: int = 10
// count = "hello"  // Error: cannot assign str to int
```

### Invalid Operation

```razen
var name: str = "Hanuman"
// var result = name + 10  // Error: cannot add int to str
```

### Wrong Parameter Type

```razen
fun greet(name: str) {
    println(f"Hello, {name}!")
}

// greet(42)  // Error: expected str, got int
```

### Wrong Return Type

```razen
fun getCount() -> int {
    // return "10"  // Error: expected int, got str
    return 10      // OK
}
```

## Type System Philosophy

Razen's type system balances:

1. **Flexibility**: Rapid prototyping without type annotations
2. **Safety**: Compile-time checking with type annotations
3. **Clarity**: Self-documenting code with explicit types
4. **Productivity**: Minimal boilerplate, maximum value

Choose the right approach for your use case:
- Use flexible typing for scripts and experiments
- Use strict typing for production code and libraries
- Mix both approaches as needed

## Examples

See [examples/variables-datatypes/](../../examples/variables-datatypes/) for complete programs demonstrating type system features.

## Next Steps

- Learn about [Type Conversion](type-conversion.md) for converting between types
- Explore [Complex Types](complex-types.md) for arrays and maps
- Study [Functions](../functions.md) for typed function parameters
