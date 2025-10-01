# Variables

Variables are named containers that store data values in your program. This guide covers variable declaration, naming conventions, scope, and reassignment.

## Declaration

Variables in Razen are declared using the `var` keyword.

### Basic Syntax

```razen
var variableName = value
```

### Examples

```razen
var name = "Hanuman"
var age = 25
var price = 19.99
var isActive = true
```

## Naming Rules

Variable names must follow these rules:

### Valid Names

- Must start with a letter (a-z, A-Z) or underscore (_)
- Can contain letters, numbers, and underscores
- Case-sensitive

```razen
var userName = "Ram"
var user_name = "Ram"
var _privateValue = 42
var value1 = 100
var myVariable2 = "test"
```

### Invalid Names

```razen
var 1value = 100      // Error: Cannot start with number
var user-name = "Ram" // Error: Cannot contain hyphens
var fun = "test"      // Error: Cannot use reserved keyword
var my variable = 5   // Error: Cannot contain spaces
```

### Naming Conventions

Follow these conventions for readable code:

```razen
// Use camelCase for variable names
var firstName = "John"
var totalAmount = 100

// Use descriptive names
var userAge = 25              // Good
var x = 25                    // Avoid single letters

// Use underscores for private/internal variables
var _internalCounter = 0

// Boolean variables should be questions
var isActive = true
var hasPermission = false
var canEdit = true
```

## Variable Scope

Variables are scoped to the block where they are declared.

### Function Scope

```razen
fun main() {
    var x = 10
    println(x)  // OK: x is accessible
}

// println(x)  // Error: x is not accessible outside function
```

### Block Scope

```razen
fun main() {
    var x = 10
    
    if true {
        var y = 20
        println(x)  // OK: x is accessible from outer scope
        println(y)  // OK: y is accessible in this block
    }
    
    println(x)  // OK: x is still accessible
    // println(y)  // Error: y is not accessible outside if block
}
```

### Nested Scopes

```razen
fun main() {
    var outer = "outer"
    
    if true {
        var middle = "middle"
        
        while true {
            var inner = "inner"
            println(outer)   // OK: accessible
            println(middle)  // OK: accessible
            println(inner)   // OK: accessible
            break
        }
        
        println(outer)   // OK: accessible
        println(middle)  // OK: accessible
        // println(inner)   // Error: not accessible
    }
    
    println(outer)  // OK: accessible
    // println(middle) // Error: not accessible
    // println(inner)  // Error: not accessible
}
```

## Reassignment

Variables can be reassigned to new values after declaration.

### Basic Reassignment

```razen
var count = 0
println(count)  // Output: 0

count = 10
println(count)  // Output: 10

count = count + 5
println(count)  // Output: 15
```

### Type Changes (Without Type Annotation)

Variables without type annotations can change type:

```razen
var value = 10        // Initially int
println(value)        // Output: 10

value = "hello"       // Now str
println(value)        // Output: hello

value = true          // Now bool
println(value)        // Output: true
```

### Type Restrictions (With Type Annotation)

Variables with type annotations cannot change type:

```razen
var count: int = 0
count = 10           // OK: same type
count = count + 5    // OK: same type

// count = "hello"   // Error: cannot assign str to int
// count = true      // Error: cannot assign bool to int
```

## Shadowing

You can declare a new variable with the same name in a nested scope:

```razen
fun main() {
    var x = 10
    println(x)  // Output: 10
    
    if true {
        var x = 20  // New variable, shadows outer x
        println(x)  // Output: 20
    }
    
    println(x)  // Output: 10 (outer x unchanged)
}
```

## Unused Variables

Razen warns about unused variables. Prefix with underscore to suppress:

```razen
fun main() {
    var unused = 10        // Warning: unused variable
    var _intentional = 20  // No warning: underscore prefix
}
```

## Best Practices

### Use Descriptive Names

```razen
// Good
var userAge = 25
var totalPrice = 99.99
var isLoggedIn = true

// Avoid
var a = 25
var x = 99.99
var flag = true
```

### Initialize Variables

```razen
// Good
var count = 0
var name = ""
var items = []

// Avoid
var count  // Avoid for conflicts
var name   // Avoid for conflicts
```

### Minimize Scope

Declare variables in the smallest scope needed:

```razen
// Good
fun processData() {
    if needsProcessing {
        var tempResult = calculate()
        println(tempResult)
    }
}

// Avoid
fun processData() {
    var tempResult  // Declared too early
    if needsProcessing {
        tempResult = calculate()
        println(tempResult)
    }
}
```

### Use Type Annotations for Important Variables

```razen
// Good: Type annotation for clarity and safety
var userId: int = 12345
var accountBalance: float = 1000.50
var userName: str = "Hanuman"

// OK: Type inference for obvious cases
var message = "Hello"
var count = 0
```

## Common Patterns

### Counter Variables

```razen
var count = 0
while count < 10 {
    println(count)
    count = count + 1
}
```

### Accumulator Variables

```razen
var sum = 0
for i in 1..=10 {
    sum = sum + i
}
println(f"Sum: {sum}")
```

### Flag Variables

```razen
var found = false
for item in items {
    if item == target {
        found = true
        break
    }
}
```

### Temporary Variables

```razen
var temp = a
a = b
b = temp
```

## Examples

See [examples/variables-datatypes/](../../examples/variables-datatypes/) for complete programs demonstrating variable usage.

## Next Steps

- Learn about [Type System](type-system.md) for type safety
- Explore [Integers](integers.md) for numeric operations
- Study [Strings](strings.md) for text manipulation
