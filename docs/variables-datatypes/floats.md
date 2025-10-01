# Floats

Floats are numbers with decimal points, used for representing fractional values. This guide covers float operations, precision, and common use cases.

## Declaration

Floats can be declared with or without type annotations.

### Basic Declaration

```razen
var price = 19.99
var pi = 3.14159
var temperature = -273.15
var zero = 0.0
```

### With Type Annotation

```razen
var amount: float = 99.99
var rate: float = 0.05
var balance: float = -50.25
```

## Float Literals

### Decimal Notation

```razen
var positive = 42.5
var negative = -10.75
var small = 0.001
var large = 1000000.0
```

### Scientific Notation -

```razen
var scientific = 1.5e10      // 15000000000.0
var small = 2.5e-3           // 0.0025
var negative = -3.2e5        // -320000.0
```

## Arithmetic Operations

### Basic Operations

```razen
var a = 10.5
var b = 2.5

var sum = a + b          // 13.0
var difference = a - b   // 8.0
var product = a * b      // 26.25
var quotient = a / b     // 4.2
```

### Mixed Integer and Float

When mixing integers and floats, the result is a float:

```razen
var result1 = 10 + 2.5    // 12.5 (float)
var result2 = 5.0 * 3     // 15.0 (float)
var result3 = 10 / 2.0    // 5.0 (float)
```

### Precision

Floats have limited precision:

```razen
var a = 0.1
var b = 0.2
var sum = a + b          // May be 0.30000000000000004
```

## Compound Assignment

```razen
var x = 10.5

x = x + 5.5    // 16.0
x = x - 3.0    // 13.0
x = x * 2.0    // 26.0
x = x / 4.0    // 6.5
```

## Comparison Operations

```razen
var a = 10.5
var b = 20.5

var equal = a == b           // false
var notEqual = a != b        // true
var lessThan = a < b         // true
var greaterThan = a > b      // false
var lessOrEqual = a <= b     // true
var greaterOrEqual = a >= b  // false
```

### Comparing Floats -

Due to precision issues, avoid direct equality comparisons:

```razen
// Avoid
var a = 0.1 + 0.2
var b = 0.3
var equal = a == b  // May be false due to precision

// Better: Use epsilon comparison
fun floatEquals(a: float, b: float) -> bool {
    var epsilon = 0.0001
    var diff = a - b
    if diff < 0.0 {
        diff = -diff
    }
    return diff < epsilon
}
```

## Common Use Cases

### Prices and Money

```razen
var price = 19.99
var quantity = 3.0
var total = price * quantity
println(f"Total: ${total}")
```

### Percentages

```razen
var amount = 100.0
var taxRate = 0.08
var tax = amount * taxRate
var totalWithTax = amount + tax
println(f"Total with tax: ${totalWithTax}")
```

### Measurements

```razen
var distance = 5.5  // kilometers
var time = 0.5      // hours
var speed = distance / time
println(f"Speed: {speed} km/h")
```

### Scientific Calculations

```razen
var radius = 5.0
var pi = 3.14159
var area = pi * radius * radius
println(f"Circle area: {area}")
```

## Mathematical Operations

### Absolute Value

```razen
fun abs(n: float) -> float {
    if n < 0.0 {
        return -n
    }
    return n
}

var result = abs(-42.5)  // 42.5
```

### Maximum and Minimum

```razen
fun max(a: float, b: float) -> float {
    if a > b {
        return a
    }
    return b
}

fun min(a: float, b: float) -> float {
    if a < b {
        return a
    }
    return b
}

var maximum = max(10.5, 20.3)  // 20.3
var minimum = min(10.5, 20.3)  // 10.5
```

### Rounding

```razen
fun floor(n: float) -> int {
    return toint(n)
}

fun ceil(n: float) -> int {
    var floored = toint(n)
    if n > tofloat(floored) {
        return floored + 1
    }
    return floored
}

fun round(n: float) -> int {
    return toint(n + 0.5)
}

var f1 = floor(4.7)   // 4
var c1 = ceil(4.2)    // 5
var r1 = round(4.5)   // 5
```

### Power and Square Root

```razen
fun power(base: float, exponent: int) -> float {
    var result = 1.0
    for i in 1..=exponent {
        result = result * base
    }
    return result
}

fun sqrt(n: float) -> float {
    // Newton's method for square root
    var guess = n / 2.0
    var epsilon = 0.00001
    
    while true {
        var nextGuess = (guess + n / guess) / 2.0
        var diff = guess - nextGuess
        if diff < 0.0 {
            diff = -diff
        }
        if diff < epsilon {
            break
        }
        guess = nextGuess
    }
    
    return guess
}

var squared = power(2.5, 2)  // 6.25
var root = sqrt(16.0)        // 4.0
```

## Type Conversion

### Float to Integer

```razen
var decimal = 42.7
var integer = toint(decimal)
println(integer)  // 42 (truncated)
```

### Float to String

```razen
var number = 3.14159
var text = tostr(number)
println(f"Pi is approximately {text}")
```

### String to Float

```razen
var text = "3.14"
var number = tofloat(text)
println(number)  // 3.14
```

### Integer to Float

```razen
var integer = 10
var decimal = tofloat(integer)
println(decimal)  // 10.0
```

## Common Patterns

### Average Calculation

```razen
fun average(numbers: [float]) -> float {
    var sum = 0.0
    var count = 0
    
    for num in numbers {
        sum = sum + num
        count = count + 1
    }
    
    if count == 0 {
        return 0.0
    }
    
    return sum / tofloat(count)
}

var avg = average([10.5, 20.3, 15.7])
```

### Distance Calculation

```razen
fun distance(x1: float, y1: float, x2: float, y2: float) -> float {
    var dx = x2 - x1
    var dy = y2 - y1
    var sumSquares = dx * dx + dy * dy
    return sqrt(sumSquares)
}

var dist = distance(0.0, 0.0, 3.0, 4.0)  // 5.0
```

### Percentage Calculation

```razen
fun percentage(value: float, total: float) -> float {
    if total == 0.0 {
        return 0.0
    }
    return (value / total) * 100.0
}

var percent = percentage(25.0, 100.0)  // 25.0
```

### Compound Interest

```razen
fun compoundInterest(principal: float, rate: float, years: int) -> float {
    var amount = principal
    for i in 1..=years {
        amount = amount * (1.0 + rate)
    }
    return amount
}

var finalAmount = compoundInterest(1000.0, 0.05, 10)
```

## Limitations

### Precision Issues

Floats cannot represent all decimal numbers exactly:

```razen
var a = 0.1 + 0.2
println(a)  // May print 0.30000000000000004
```

### Rounding Errors

Accumulation of rounding errors in repeated operations:

```razen
var sum = 0.0
for i in 1..1000 {
    sum = sum + 0.1
}
// sum may not be exactly 100.0
```

### Division by Zero

Avoid division by zero:

```razen
var a = 10.5
var b = 0.0

// var result = a / b  // Error: division by zero

// Safe division
if b != 0.0 {
    var result = a / b
} else {
    println("Cannot divide by zero")
}
```

## Best Practices

### Use Meaningful Names

```razen
// Good
var price = 19.99
var taxRate = 0.08
var totalAmount = 125.50

// Avoid
var x = 19.99
var r = 0.08
var t = 125.50
```

### Initialize Before Use

```razen
// Good
var sum = 0.0
var average = 0.0

// Avoid
var sum
sum = sum + 5.5  // Error: uninitialized
```

### Use Type Annotations for Clarity

```razen
// Good: Clear intent
var price: float = 19.99
var rate: float = 0.05

// OK: Type is obvious
var amount = 100.0
var total = 0.0
```

### Handle Precision Carefully

```razen
// For money, consider using integers (cents)
var priceInCents: int = 1999  // $19.99
var totalInCents = priceInCents * 3
var totalDollars = tofloat(totalInCents) / 100.0
```

### Check for Special Cases

```razen
fun safeDivide(a: float, b: float) -> float {
    if b == 0.0 {
        println("Error: Division by zero")
        return 0.0
    }
    return a / b
}
```

## Examples

See [examples/variables-datatypes/](../../examples/variables-datatypes/) for complete programs demonstrating float operations.

## Next Steps

- Learn about [Integers](integers.md) for whole number operations
- Explore [Type Conversion](type-conversion.md) for converting between types
- Study [Strings](strings.md) for formatting float values
