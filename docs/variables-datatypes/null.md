# Null Values

Null represents the absence of a value in Razen. This guide covers null usage, checking, and best practices.

## Declaration

Null is a special value representing "no value" or "nothing".

### Basic Declaration

```razen
var value = null
var result = null
var data = null
```

### With Type Annotation

```razen
var value: null = null
var optional = null
```

## Understanding Null

Null is used to represent:
- Uninitialized or missing data
- Optional values that may not exist
- Function returns when no value is available
- Default state before assignment

## Checking for Null

### Equality Check

```razen
var value = null

if value == null {
    println("Value is null")
}

if value != null {
    println("Value is not null")
}
```

### Null in Conditions

```razen
var data = null

if data == null {
    println("No data available")
} else {
    println(f"Data: {data}")
}
```

## Common Use Cases

### Optional Return Values

```razen
fun findUser(id: int) -> str {
    if id == 1 {
        return "Hanuman"
    } elif id == 2 {
        return "Ram"
    } else {
        return null  // User not found
    }
}

var user = findUser(5)
if user == null {
    println("User not found")
} else {
    println(f"Found user: {user}")
}
```

### Default Values

```razen
fun getConfig(key: str) {
    // Return null if config not found
    return null
}

var config = getConfig("theme")
if config == null {
    config = "default"  // Use default value
}
println(f"Theme: {config}")
```

### Initialization State

```razen
var connection = null

fun connect() {
    connection = "Connected"
}

if connection == null {
    println("Not connected")
    connect()
} else {
    println("Already connected")
}
```

### Search Results

```razen
fun search(items: [str], target: str) {
    for item in items {
        if item == target {
            return item
        }
    }
    return null  // Not found
}

var result = search(["apple", "banana"], "cherry")
if result == null {
    println("Item not found")
} else {
    println(f"Found: {result}")
}
```

## Null Safety Patterns

### Guard Clauses

```razen
fun processData(data) {
    if data == null {
        println("Error: No data provided")
        return
    }
    
    // Process data
    println(f"Processing: {data}")
}

processData(null)      // Output: Error: No data provided
processData("data")    // Output: Processing: data
```

### Default Value Pattern

```razen
fun getValue(optional) {
    if optional == null {
        return "default"
    }
    return optional
}

var value1 = getValue(null)     // "default"
var value2 = getValue("custom") // "custom"
```

### Null Coalescing

```razen
fun getOrDefault(value, defaultValue) {
    if value == null {
        return defaultValue
    }
    return value
}

var name = getOrDefault(null, "Guest")
println(f"Hello, {name}")  // Output: Hello, Guest
```

### Validation

```razen
fun isValid(value) -> bool {
    return value != null
}

var data = null
if !isValid(data) {
    println("Invalid data")
}
```

## Null in Data Structures

### Arrays with Null

```razen
var items = [1, 2, null, 4, 5]

for item in items {
    if item == null {
        println("Null item found")
    } else {
        println(f"Item: {item}")
    }
}
```

### Optional Fields

```razen
struct User {
    name: str,
    email: str,
    phone: null  // Optional field
}

var user = User {
    name: "Hanuman",
    email: "Hanuman@example.com",
    phone: null
}

if user.phone == null {
    println("No phone number provided")
}
```

## Functions Returning Null

### Indicating Failure

```razen
fun divide(a: int, b: int) {
    if b == 0 {
        return null  // Cannot divide by zero
    }
    return a / b
}

var result = divide(10, 0)
if result == null {
    println("Division failed")
} else {
    println(f"Result: {result}")
}
```

### Optional Data Retrieval

```razen
fun getFirstElement(array: [int]) {
    if len(array) == 0 {
        return null
    }
    return array[0]
}

var empty = []
var first = getFirstElement(empty)

if first == null {
    println("Array is empty")
}
```

## Best Practices

### Check Before Use

```razen
// Good: Check for null before using
var value = getValue()
if value != null {
    println(f"Value: {value}")
}

// Avoid: Using without checking
var value = getValue()
println(f"Value: {value}")  // May print "null"
```

### Provide Defaults

```razen
// Good: Provide default value
fun getConfig(key: str) -> str {
    var value = lookupConfig(key)
    if value == null {
        return "default"
    }
    return value
}

// Avoid: Returning null without handling
fun getConfig(key: str) {
    return lookupConfig(key)  // May return null
}
```

### Document Null Returns

```razen
// Good: Document when null is returned
// Returns user name or null if not found
fun findUser(id: int) {
    // Implementation
    return null
}

// Avoid: Unclear null behavior
fun findUser(id: int) {
    return null
}
```

### Use Meaningful Null Checks

```razen
// Good: Clear intent
var user = findUser(id)
if user == null {
    println("User not found")
    return
}

// Avoid: Unclear logic
var user = findUser(id)
if !user {
    return
}
```

## Common Patterns

### Lazy Initialization

```razen
var cache = null

fun getCache() {
    if cache == null {
        cache = initializeCache()
    }
    return cache
}
```

### Optional Parameters

```razen
fun greet(name, greeting) {
    var actualGreeting = greeting
    if actualGreeting == null {
        actualGreeting = "Hello"
    }
    println(f"{actualGreeting}, {name}!")
}

greet("Hanuman", null)      // Output: Hello, Hanuman!
greet("Ram", "Hi")        // Output: Hi, Ram!
```

### Chaining with Null Checks

```razen
fun processUser(userId: int) {
    var user = findUser(userId)
    if user == null {
        return null
    }
    
    var profile = getProfile(user)
    if profile == null {
        return null
    }
    
    return formatProfile(profile)
}
```

### Null Object Pattern

```razen
fun getUser(id: int) {
    if id <= 0 {
        // Return default "null" user instead of null
        return User { name: "Guest", id: 0 }
    }
    return lookupUser(id)
}
```

## Type Conversion

### Null to String

```razen
var value = null
var text = tostr(value)
println(text)  // Output: null
```

### Null to Boolean

```razen
var value = null
var flag = tobool(value)
println(flag)  // Output: false
```

### Checking Type

```razen
var value = null
var valueType = typeof(value)
println(valueType)  // Output: null
```

## Common Mistakes

### Not Checking for Null

```razen
// Wrong: Using value without checking
var user = findUser(5)
println(user.name)  // Error if user is null

// Correct: Check before use
var user = findUser(5)
if user != null {
    println(user.name)
}
```

### Comparing with Wrong Type

```razen
// Wrong: Comparing null with other types
var value = null
if value == 0 {  // null is not 0
    println("Zero")
}

// Correct: Check for null explicitly
if value == null {
    println("Null value")
}
```

### Forgetting to Handle Null

```razen
// Wrong: Not handling null case
fun processData(data) {
    // Assumes data is never null
    return data.length
}

// Correct: Handle null case
fun processData(data) {
    if data == null {
        return 0
    }
    return data.length
}
```

## Null vs Other Values

### Null vs Empty String

```razen
var nullValue = null
var emptyString = ""

println(nullValue == emptyString)  // false
println(nullValue == null)         // true
println(emptyString == "")         // true
```

### Null vs Zero

```razen
var nullValue = null
var zero = 0

println(nullValue == zero)  // false
println(nullValue == null)  // true
println(zero == 0)          // true
```

### Null vs False

```razen
var nullValue = null
var falseValue = false

println(nullValue == falseValue)  // false
println(nullValue == null)        // true
println(falseValue == false)      // true
```

## When to Use Null

Use null when:
- A value may not exist
- A function may not return a value
- Representing optional data
- Indicating absence of data

Avoid null when:
- A default value makes more sense
- Empty collections can be used
- A sentinel value is more appropriate

## Examples

See [examples/variables-datatypes/](../../examples/variables-datatypes/) for complete programs demonstrating null handling.

## Next Steps

- Learn about [Type System](type-system.md) for handling optional types
- Explore [Functions](../functions.md) for optional return values
- Study [Error Handling](../error-handling.md) for alternatives to null
