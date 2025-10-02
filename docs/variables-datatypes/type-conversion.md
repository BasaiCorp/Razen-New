# Type Conversion

Type conversion allows you to transform values from one type to another. This guide covers built-in conversion functions and best practices.

## Overview

Razen provides built-in functions for converting between types:

- `toint()` - Convert to integer
- `tofloat()` - Convert to float
- `tostr()` - Convert to string
- `tobool()` - Convert to boolean

## String Conversions

### Integer to String

```razen
var number = 42
var text = tostr(number)
println(text)           // Output: 42
println(typeof(text))   // Output: str
```

### Float to String

```razen
var price = 19.99
var text = tostr(price)
println(text)           // Output: 19.99
```

### Boolean to String

```razen
var flag = true
var text = tostr(flag)
println(text)           // Output: true
```

### String to Integer

```razen
var text = "123"
var number = toint(text)
println(number)         // Output: 123
println(typeof(number)) // Output: int
```

### String to Float

```razen
var text = "3.14"
var number = tofloat(text)
println(number)         // Output: 3.14
```

### String to Boolean

```razen
var text = "true"
var flag = tobool(text)
println(flag)           // Output: true
```

## Numeric Conversions

### Integer to Float

```razen
var integer = 10
var decimal = tofloat(integer)
println(decimal)        // Output: 10.0
```

### Float to Integer

Converts by truncating the decimal part:

```razen
var decimal = 42.7
var integer = toint(decimal)
println(integer)        // Output: 42 (truncated, not rounded)

var negative = -10.9
var negInt = toint(negative)
println(negInt)         // Output: -10
```

### Integer to Boolean

```razen
var zero = 0
var nonZero = 5

var flag1 = tobool(zero)      // false
var flag2 = tobool(nonZero)   // true
```

## Common Use Cases

### User Input Processing

```razen
println("Enter your age: ")
var userInput = input()
var age = toint(userInput)

if age >= 18 {
    println("You are an adult")
} else {
    println("You are a minor")
}
```

### Formatting Output

```razen
var count = 42
var price = 19.99
var isAvailable = true

var message = "Items: " + tostr(count) + 
              ", Price: $" + tostr(price) + 
              ", Available: " + tostr(isAvailable)
println(message)
```

### Mathematical Operations

```razen
var intValue = 10
var floatValue = 3.5

// Convert int to float for precise division
var result = tofloat(intValue) / floatValue
println(result)  // Output: 2.857...
```

### String Interpolation

F-strings automatically convert values:

```razen
var count = 42
var price = 19.99
var isActive = true

// Automatic conversion in f-strings
var message = f"Count: {count}, Price: {price}, Active: {isActive}"
println(message)
```

## Conversion Patterns

### Safe String to Number Conversion

```razen
fun safeToInt(text: str, defaultValue: int) -> int {
    if text == "" {
        return defaultValue
    }
    return text
}

var input = ""
var number = safeToInt(input, 0)
println(number)  // Output: 0
```

### Rounding Float to Integer

```razen
fun round(value: float) -> int {
    return toint(value + 0.5)
}

var rounded1 = round(4.3)  // 4
var rounded2 = round(4.7)  // 5
```

### Parsing with Validation

```razen
fun parseAge(ageInput: str) -> int {
    var age = toint(ageInput)
    
    if age < 0 {
        println("Age cannot be negative")
        return 0
    }
    
    if age > 150 {
        println("Invalid age")
        return 0
    }
    
    return age
}

var validAge = parseAge("25")    // 25
var invalidAge = parseAge("200") // 0
```

### Boolean Conversion

```razen
fun toBoolFromString(text: str) -> bool {
    if text == "true" || text == "1" || text == "yes" {
        return true
    }
    return false
}

var flag1 = toBoolFromString("true")  // true
var flag2 = toBoolFromString("yes")   // true
var flag3 = toBoolFromString("no")    // false
```

## Type Conversion in Expressions

### Mixed Type Arithmetic

```razen
var intValue = 10
var floatValue = 3.5

// Automatic conversion to float
var result1 = intValue + floatValue      // 13.5 (float)

// Explicit conversion
var result2 = tofloat(intValue) / floatValue  // 2.857...
```

### String Concatenation

```razen
var name = "Hanuman"
var age = 25
var score = 95.5

// Convert numbers to strings for concatenation
var message = name + " is " + tostr(age) + 
              " years old with score " + tostr(score)
println(message)

// Or use f-strings (automatic conversion)
var message2 = f"{name} is {age} years old with score {score}"
println(message2)
```

## Conversion Functions Reference

### toint()
Converts values to integer:

```razen
toint("123")      // 123
toint("42.7")     // 42 (truncates)
toint(3.14)       // 3
toint(true)       // 1
toint(false)      // 0
```

### tofloat()

Converts values to float:

```razen
tofloat("3.14")   // 3.14
tofloat("42")     // 42.0
tofloat(10)       // 10.0
tofloat(true)     // 1.0
tofloat(false)    // 0.0
```

### tostr()

Converts values to string:

```razen
tostr(42)         // "42"
tostr(3.14)       // "3.14"
tostr(true)       // "true"
tostr(false)      // "false"
```

### tobool()

Converts values to boolean:

```razen
tobool("true")    // true
tobool("false")   // false
tobool(1)         // true
tobool(0)         // false
tobool("")        // false
tobool("text")    // true
```

## Error Handling

### Invalid Conversions

Handle invalid conversions gracefully:

```razen
fun safeParseInt(text: str) -> int {
    // Check if string is empty
    if text == "" {
        println("Error: Empty string")
        return 0
    }
    
    // Check if string contains only digits
    // (simplified check)
    var result = toint(text)
    return result
}
```

### Validation Before Conversion

```razen
fun parsePositiveInt(text: str) -> int {
    var value = toint(text)
    
    if value < 0 {
        println("Error: Value must be positive")
        return 0
    }
    
    return value
}

var valid = parsePositiveInt("42")    // 42
var invalid = parsePositiveInt("-10") // 0
```

## Best Practices

### Use F-Strings for Formatting

```razen
// Good: F-strings handle conversion automatically
var count = 42
var message = f"Count: {count}"

// Avoid: Manual conversion
var message2 = "Count: " + tostr(count)
```

### Validate Input Before Conversion

```razen
// Good: Validate before converting
fun getAge(input: str) -> int {
    if input == "" {
        return 0
    }
    
    var age = toint(input)
    
    if age < 0 || age > 150 {
        return 0
    }
    
    return age
}

// Avoid: Convert without validation
fun getAge(input: str) -> int {
    return toint(input)
}
```

### Use Appropriate Types

```razen
// Good: Use correct type from the start
var price: float = 19.99
var quantity: int = 3
var total = price * tofloat(quantity)

// Avoid: Unnecessary conversions
var price = 19.99
var quantity = 3.0  // Should be int
var total = price * quantity
```

### Document Conversion Logic

```razen
// Good: Clear conversion intent
fun calculatePercentage(value: int, total: int) -> float {
    // Convert to float for precise division
    return (tofloat(value) / tofloat(total)) * 100.0
}

// Avoid: Unclear conversion
fun calculatePercentage(value: int, total: int) -> float {
    return (tofloat(value) / tofloat(total)) * 100.0
}
```

## Common Patterns

### Currency Conversion

```razen
fun formatCurrency(amount: float) -> str {
    return f"${tostr(amount)}"
}

var price = 19.99
var formatted = formatCurrency(price)
println(formatted)  // Output: $19.99
```

### Percentage Formatting

```razen
fun formatPercentage(value: float) -> str {
    var rounded = toint(value * 100.0)
    return f"{rounded}%"
}

var rate = 0.075
var formatted = formatPercentage(rate)
println(formatted)  // Output: 7%
```

### Temperature Conversion

```razen
fun celsiusToFahrenheit(celsius: float) -> float {
    return (celsius * 9.0 / 5.0) + 32.0
}

fun fahrenheitToCelsius(fahrenheit: float) -> float {
    return (fahrenheit - 32.0) * 5.0 / 9.0
}

var c = 25.0
var f = celsiusToFahrenheit(c)
println(f"{c}°C = {f}°F")
```

### Time Conversion

```razen
fun secondsToMinutes(seconds: int) -> float {
    return tofloat(seconds) / 60.0
}

fun minutesToHours(minutes: int) -> float {
    return tofloat(minutes) / 60.0
}

var seconds = 3600
var minutes = secondsToMinutes(seconds)
var hours = minutesToHours(toint(minutes))
println(f"{seconds} seconds = {hours} hours")
```

## Type Checking

Check types before conversion:

```razen
fun convertToInt(value) -> int {
    var valueType = typeof(value)
    
    if valueType == "int" {
        return value
    } elif valueType == "str" {
        return toint(value)
    } elif valueType == "float" {
        return toint(value)
    } elif valueType == "bool" {
        return toint(value)
    } else {
        return 0
    }
}
```

## Performance Considerations

### Minimize Conversions

```razen
// Good: Convert once
var input = "123"
var number = toint(input)
var doubled = number * 2
var tripled = number * 3

// Avoid: Multiple conversions
var input = "123"
var doubled = toint(input) * 2
var tripled = toint(input) * 3
```

### Use Appropriate Types

```razen
// Good: Use correct type from start
var count: int = 0
for i in 1..=100 {
    count = count + 1
}

// Avoid: Unnecessary conversions
var count = "0"
for i in 1..=100 {
    count = tostr(toint(count) + 1)
}
```

## Examples

See [examples/variables-datatypes/](../../examples/variables-datatypes/) for complete programs demonstrating type conversion.

## Next Steps

- Learn about [Type System](type-system.md) for type safety concepts
- Explore [Strings](strings.md) for string formatting
- Study [Functions](../functions.md) for typed parameters
