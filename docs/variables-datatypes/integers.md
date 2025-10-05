# Integers

Integers are whole numbers without decimal points. This guide covers integer operations, ranges, and common use cases.

## Declaration

Integers can be declared with or without type annotations.

### Basic Declaration

```razen
var age = 25
var count = 0
var temperature = -10
var population = 1000000
```

### With Type Annotation

```razen
var score: int = 100
var level: int = 5
var balance: int = -50
```

## Integer Literals

### Decimal Notation

```razen
var decimal = 42
var negative = -100
var zero = 0
```

### Large Numbers

```razen
var million = 1000000
var billion = 1000000000
```

## Arithmetic Operations

### Basic Operations

```razen
var a = 10
var b = 3

var sum = a + b                    // 13
var difference = a - b             // 7
var product = a * b                // 30
var quotient = (a / b).toint()     // 3 (integer division)
var remainder = a % b              // 1
```

### Integer Division

Integer division truncates the decimal part:

```razen
var result1 = (10 / 3).toint()     // 3 (not 3.333...)
var result2 = (7 / 2).toint()      // 3 (not 3.5)
var result3 = (-10 / 3).toint()    // -3
```

### Modulo Operation

The modulo operator returns the remainder:

```razen
var mod1 = 10 % 3        // 1
var mod2 = 15 % 4        // 3
var mod3 = 20 % 5        // 0
```

## Compound Assignment

Combine operations with assignment:

```razen
var x = 10

x = x + 5    // 15
x = x - 3    // 12
x = x * 2    // 24
x = x / 4    // 6
x = x % 4    // 2
```

## Increment and Decrement

```razen
var count = 0

count = count + 1    // Increment by 1
count = count - 1    // Decrement by 1
```

## Comparison Operations

All comparison operations return boolean values:

```razen
var a = 10
var b = 20

var equal = a == b           // false
var notEqual = a != b        // true
var lessThan = a < b         // true
var greaterThan = a > b      // false
var lessOrEqual = a <= b     // true
var greaterOrEqual = a >= b  // false
```

## Common Use Cases

### Counters

```razen
var count = 0
for i in 1..=10 {
    count = count + 1
}
println(f"Total: {count}")
```

### Accumulators

```razen
var sum = 0
for i in 1..=100 {
    sum = sum + i
}
println(f"Sum of 1 to 100: {sum}")
```

### Index Variables 

```razen
var index = 0
var items = ["apple", "banana", "cherry"]

while index < 3 {
    println(items[index])
    index = index + 1
}
```

### Flags and Status Codes

```razen
var statusCode = 200
var errorCode = -1
var exitCode = 0
```

## Mathematical Operations

### Absolute Value

```razen
fun abs(n: int) -> int {
    if n < 0 {
        return -n
    }
    return n
}

var result = abs(-42)  // 42
```

### Maximum and Minimum

```razen
fun max(a: int, b: int) -> int {
    if a > b {
        return a
    }
    return b
}

fun min(a: int, b: int) -> int {
    if a < b {
        return a
    }
    return b
}

var maximum = max(10, 20)  // 20
var minimum = min(10, 20)  // 10
```

### Even and Odd

```razen
fun isEven(n: int) -> bool {
    return n % 2 == 0
}

fun isOdd(n: int) -> bool {
    return n % 2 != 0
}

var check1 = isEven(4)   // true
var check2 = isOdd(7)    // true
```

## Range Operations

### Exclusive Range

```razen
for i in 1..5 {
    println(i)  // Prints: 1, 2, 3, 4
}
```

### Inclusive Range

```razen
for i in 1..=5 {
    println(i)  // Prints: 1, 2, 3, 4, 5
}
```

### Range with Variables

```razen
var start = 1
var end = 10

for i in start..=end {
    println(i)
}
```

## Type Conversion

### Integer to String

```razen
var number = 42
var text = tostr(number)
println(f"The number is {text}")
```

### Integer to Float

```razen
var integer = 10
var decimal = tofloat(integer)
println(decimal)  // 10.0
```

### String to Integer

```razen
var text = "123"
var number = toint(text)
println(number)  // 123
```

## Common Patterns

### Sum of Range

```razen
fun sumRange(start: int, end: int) -> int {
    var sum = 0
    for i in start..=end {
        sum = sum + i
    }
    return sum
}

var total = sumRange(1, 10)  // 55
```

### Factorial

```razen
fun factorial(n: int) -> int {
    if n <= 1 {
        return 1
    }
    var result = 1
    for i in 2..=n {
        result = result * i
    }
    return result
}

var fact5 = factorial(5)  // 120
```

### Power

```razen
fun power(base: int, exponent: int) -> int {
    var result = 1
    for i in 1..=exponent {
        result = result * base
    }
    return result
}

var result = power(2, 8)  // 256
```

### Greatest Common Divisor

```razen
fun gcd(a: int, b: int) -> int {
    while b != 0 {
        var temp = b
        b = a % b
        a = temp
    }
    return a
}

var result = gcd(48, 18)  // 6
```

## Limitations

### Integer Overflow

Be aware of integer size limitations. Very large calculations may overflow:

```razen
// Large numbers may cause overflow
var large = 1000000000
var product = large * large  // May overflow
```

### Division by Zero

Avoid division by zero:

```razen
var a = 10
var b = 0

// var result = a / b  // Error: division by zero

// Safe division
if b != 0 {
    var result = a / b
} else {
    println("Cannot divide by zero")
}
```

## Best Practices

### Use Meaningful Names

```razen
// Good
var userAge = 25
var itemCount = 10
var maxRetries = 3

// Avoid
var x = 25
var n = 10
var m = 3
```

### Initialize Before Use

```razen
// Good
var count = 0
var sum = 0

// Avoid
var count
count = count + 1  // Error: uninitialized
```

### Use Type Annotations for Clarity

```razen
// Good: Clear intent
var userId: int = 12345
var maxConnections: int = 100

// OK: Type is obvious
var count = 0
var index = 1
```

### Check for Edge Cases

```razen
fun divide(a: int, b: int) -> int {
    if b == 0 {
        println("Error: Division by zero")
        return 0
    }
    return a / b
}
```

## Examples

See [examples/variables-datatypes/](../../examples/variables-datatypes/) for complete programs demonstrating integer operations.

## Next Steps

- Learn about [Floats](floats.md) for decimal numbers
- Explore [Type Conversion](type-conversion.md) for converting between types
- Study [Booleans](booleans.md) for comparison results
