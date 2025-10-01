# Booleans

Booleans represent truth values: true or false. This guide covers boolean operations, comparisons, and logical expressions.

## Declaration

Booleans can be declared with or without type annotations.

### Basic Declaration

```razen
var isActive = true
var isComplete = false
var hasPermission = true
var isValid = false
```

### With Type Annotation

```razen
var isLoggedIn: bool = true
var canEdit: bool = false
var isAdmin: bool = true
```

## Boolean Literals

Only two boolean values exist:

```razen
var truthValue = true
var falseValue = false
```

## Logical Operations

### AND Operator (&&)

Returns true only if both operands are true:

```razen
var a = true
var b = true
var result = a && b     // true

var c = true
var d = false
var result2 = c && d    // false
```

Truth table for AND:
```
true  && true  = true
true  && false = false
false && true  = false
false && false = false
```

### OR Operator (||)

Returns true if at least one operand is true:

```razen
var a = true
var b = false
var result = a || b     // true

var c = false
var d = false
var result2 = c || d    // false
```

Truth table for OR:
```
true  || true  = true
true  || false = true
false || true  = true
false || false = false
```

### NOT Operator (!)

Inverts the boolean value:

```razen
var a = true
var result = !a         // false

var b = false
var result2 = !b        // true
```

Truth table for NOT:
```
!true  = false
!false = true
```

## Comparison Operations

All comparison operations return boolean values.

### Equality

```razen
var x = 10
var y = 10
var z = 20

var equal = x == y          // true
var notEqual = x != z       // true
```

### Relational Comparisons

```razen
var a = 10
var b = 20

var lessThan = a < b            // true
var greaterThan = a > b         // false
var lessOrEqual = a <= b        // true
var greaterOrEqual = a >= b     // false
```

### String Comparisons

```razen
var str1 = "apple"
var str2 = "banana"

var equal = str1 == str2        // false
var less = str1 < str2          // true (lexicographic)
```

## Compound Expressions

Combine multiple logical operations:

```razen
var age = 25
var hasLicense = true
var hasInsurance = true

var canDrive = age >= 18 && hasLicense && hasInsurance
println(canDrive)  // true
```

### Operator Precedence

```razen
// NOT (!) has highest precedence
// AND (&&) has medium precedence
// OR (||) has lowest precedence

var result = true || false && false     // true (same as: true || (false && false))
var result2 = (true || false) && false  // false (parentheses change order)
```

### Using Parentheses

```razen
var a = true
var b = false
var c = true

var result1 = a || b && c       // true (b && c evaluated first)
var result2 = (a || b) && c     // true (a || b evaluated first)
```

## Common Use Cases

### Validation

```razen
var username = "Hanuman"
var password = "secret123"

var isValidUsername = len(username) >= 3
var isValidPassword = len(password) >= 8

var isValid = isValidUsername && isValidPassword

if isValid {
    println("Credentials are valid")
} else {
    println("Invalid credentials")
}
```

### Access Control

```razen
var isLoggedIn = true
var isAdmin = false
var hasPermission = true

var canAccess = isLoggedIn && (isAdmin || hasPermission)

if canAccess {
    println("Access granted")
} else {
    println("Access denied")
}
```

### State Management

```razen
var isLoading = false
var hasError = false
var hasData = true

var canDisplay = !isLoading && !hasError && hasData

if canDisplay {
    println("Displaying data")
}
```

### Range Checking

```razen
var age = 25
var isAdult = age >= 18
var isSenior = age >= 65
var isWorkingAge = age >= 18 && age < 65

println(f"Adult: {isAdult}")
println(f"Senior: {isSenior}")
println(f"Working age: {isWorkingAge}")
```

## Control Flow with Booleans

### If Statements

```razen
var isRaining = true

if isRaining {
    println("Take an umbrella")
}
```

### If-Else Statements

```razen
var hasTicket = false

if hasTicket {
    println("Enter the venue")
} else {
    println("Please purchase a ticket")
}
```

### If-Elif-Else Chains

```razen
var score = 85
var isPassing = score >= 60
var isExcellent = score >= 90

if isExcellent {
    println("Excellent!")
} elif isPassing {
    println("Passing")
} else {
    println("Needs improvement")
}
```

### While Loops

```razen
var isRunning = true
var count = 0

while isRunning {
    println(f"Count: {count}")
    count = count + 1
    
    if count >= 5 {
        isRunning = false
    }
}
```

## Boolean Functions

Functions that return boolean values:

### Validation Functions

```razen
fun isEven(n: int) -> bool {
    return n % 2 == 0
}

fun isOdd(n: int) -> bool {
    return n % 2 != 0
}

fun isPositive(n: int) -> bool {
    return n > 0
}

var check1 = isEven(4)      // true
var check2 = isOdd(7)       // true
var check3 = isPositive(10) // true
```

### Range Checking Functions

```razen
fun inRange(value: int, min: int, max: int) -> bool {
    return value >= min && value <= max
}

var isValid = inRange(50, 0, 100)  // true
```

### String Validation Functions

```razen
fun isEmpty(text: str) -> bool {
    return len(text) == 0
}

fun isValidEmail(email: str) -> bool {
    // Simplified validation
    return len(email) > 0 && email != ""
}

var empty = isEmpty("")           // true
var valid = isValidEmail("a@b.c") // true
```

## Common Patterns

### Toggle Pattern

```razen
var isEnabled = false

// Toggle the value
isEnabled = !isEnabled
println(isEnabled)  // true

isEnabled = !isEnabled
println(isEnabled)  // false
```

### Flag Pattern

```razen
var found = false

for item in items {
    if item == target {
        found = true
        break
    }
}

if found {
    println("Item found")
} else {
    println("Item not found")
}
```

### Guard Pattern

```razen
fun processData(data: str) {
    var isEmpty = len(data) == 0
    
    if isEmpty {
        println("Error: Empty data")
        return
    }
    
    // Process data
    println(f"Processing: {data}")
}
```

### Multiple Conditions

```razen
fun canVote(age: int, isCitizen: bool, isRegistered: bool) -> bool {
    var isOldEnough = age >= 18
    return isOldEnough && isCitizen && isRegistered
}

var eligible = canVote(25, true, true)  // true
```

## Short-Circuit Evaluation

Logical operators use short-circuit evaluation:

### AND Short-Circuit

```razen
// If first condition is false, second is not evaluated
var a = false
var b = true

var result = a && b  // b is not evaluated because a is false
```

### OR Short-Circuit

```razen
// If first condition is true, second is not evaluated
var a = true
var b = false

var result = a || b  // b is not evaluated because a is true
```

### Practical Use

```razen
fun safeDivide(a: int, b: int) -> int {
    // b != 0 is checked first, preventing division by zero
    if b != 0 && a / b > 0 {
        return a / b
    }
    return 0
}
```

## Type Conversion

### Boolean to String

```razen
var flag = true
var text = tostr(flag)
println(text)  // Output: true
```

### Integer to Boolean

```razen
// Non-zero values are truthy
var num = 5
var flag = num != 0
println(flag)  // true
```

### String to Boolean

```razen
var text = "true"
var flag = text == "true"
println(flag)  // true
```

## Best Practices

### Use Descriptive Names

```razen
// Good: Names indicate boolean nature
var isActive = true
var hasPermission = false
var canEdit = true
var shouldUpdate = false

// Avoid: Unclear names
var active = true
var permission = false
var edit = true
```

### Avoid Redundant Comparisons

```razen
// Good
if isActive {
    println("Active")
}

// Avoid
if isActive == true {
    println("Active")
}
```

### Use Positive Logic

```razen
// Good: Positive logic is clearer
var isValid = true
if isValid {
    process()
}

// Avoid: Double negatives
var isNotInvalid = true
if !isNotInvalid {
    process()
}
```

### Simplify Complex Conditions

```razen
// Good: Break into meaningful variables
var isOldEnough = age >= 18
var hasLicense = licenseNumber != ""
var canDrive = isOldEnough && hasLicense

if canDrive {
    println("Can drive")
}

// Avoid: Complex inline conditions
if age >= 18 && licenseNumber != "" {
    println("Can drive")
}
```

### Use Functions for Complex Logic

```razen
// Good: Encapsulate logic in function
fun isValidUser(age: int, email: str, hasConsent: bool) -> bool {
    var isOldEnough = age >= 13
    var hasEmail = len(email) > 0
    return isOldEnough && hasEmail && hasConsent
}

if isValidUser(15, "user@example.com", true) {
    println("Valid user")
}
```

## Common Mistakes

### Comparing Booleans

```razen
// Avoid
if isActive == true {
    println("Active")
}

// Better
if isActive {
    println("Active")
}
```

### Assignment vs Comparison

```razen
var isActive = true

// Wrong: Assignment instead of comparison
// if isActive = false {  // Error
//     println("Not active")
// }

// Correct: Comparison
if isActive == false {
    println("Not active")
}

// Better: Use NOT operator
if !isActive {
    println("Not active")
}
```

## Examples

See [examples/variables-datatypes/](../../examples/variables-datatypes/) for complete programs demonstrating boolean operations.

## Next Steps

- Learn about [Type Conversion](type-conversion.md) for converting between types
- Explore [Control Flow](../control-flow.md) for using booleans in decisions
- Study [Functions](../functions.md) for creating boolean functions
