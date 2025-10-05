# Strings

Strings represent text data in Razen. This guide covers string operations, interpolation, and common use cases.

## Declaration

Strings are enclosed in double quotes.

### Basic Declaration

```razen
var name = "Hanuman"
var message = "Hello, World!"
var empty = ""
var multiWord = "This is a sentence"
```

### With Type Annotation

```razen
var username: str = "john_doe"
var email: str = "user@example.com"
var description: str = "A sample description"
```

## String Literals

### Basic Strings

```razen
var greeting = "Hello"
var farewell = "Goodbye"
```

### Strings with Special Characters

```razen
var withQuotes = "She said \"Hello\""
var withNewline = "Line 1\nLine 2"
var withTab = "Column1\tColumn2"
```

### Escape Sequences

```razen
var backslash = "Path: C:\\Users\\Name"
var quote = "He said \"Hi\""
var newline = "First line\nSecond line"
var tab = "Name:\tJohn"
```

## String Concatenation

Combine strings using the `+` operator:

```razen
var firstName = "John"
var lastName = "Doe"
var fullName = firstName + " " + lastName
println(fullName)  // Output: John Doe
```

### Concatenating Multiple Strings

```razen
var greeting = "Hello"
var name = "Hanuman"
var punctuation = "!"
var message = greeting + ", " + name + punctuation
println(message)  // Output: Hello, Hanuman!
```

### Concatenating with Numbers

```razen
var name = "Hanuman"
var age = 7000
var message = name + " is " + tostr(age) + " years old"
println(message)  // Output: Hanuman is 7000 years old
```

## String Interpolation (F-Strings)

F-strings provide a clean way to embed expressions in strings:

### Basic F-String

```razen
var name = "Hanuman"
var message = f"Hello, {name}!"
println(message)  // Output: Hello, Hanuman!
```

### F-Strings with Variables

```razen
var name = "Ram"
var age = 7000
var city = "Ayodhya"

var info = f"{name} is {age} years old and lives in {city}"
println(info)  // Output: Ram is 7000 years old and lives in New York
```

### F-Strings with Expressions

```razen
var x = 10
var y = 20
var result = f"The sum of {x} and {y} is {x + y}"
println(result)  // Output: The sum of 10 and 20 is 30
```

### F-Strings with Object Properties

```razen
struct Person {
    name: str,
    age: int
}

var person = Person { name: "Hanuman", age: 25 }
var message = f"Name: {person.name}, Age: {person.age}"
println(message)  // Output: Name: Hanuman, Age: 25
```

## String Operations

### Length

Get the length of a string:

```razen
var text = "Hello"
var length = len(text)
println(length)  // Output: 5
```

### Empty String Check

```razen
var text = ""
if text == "" {
    println("String is empty")
}

// Or check length
if len(text) == 0 {
    println("String is empty")
}
```

### String Comparison

```razen
var str1 = "hello"
var str2 = "hello"
var str3 = "world"

var equal = str1 == str2        // true
var notEqual = str1 != str3     // true
```

Strings are compared lexicographically:

```razen
var a = "apple"
var b = "banana"

var less = a < b                // true
var greater = a > b             // false
```

## Common Use Cases

### User Input

```razen
print("Enter your name: ")
var name = input()
var greeting = f"Hello, {name}!"
println(greeting)
```

### Building Messages

```razen
var username = "Hanuman"
var action = "logged in"
var timestamp = "2024-01-15"

var logMessage = f"[{timestamp}] User {username} {action}"
println(logMessage)
```

### Formatting Output

```razen
var product = "Laptop"
var price = 999.99
var quantity = 2

var invoice = f"Product: {product}\nPrice: ${price}\nQuantity: {quantity}"
println(invoice)
```

### Creating Labels

```razen
var firstName = "John"
var lastName = "Doe"
var id = 12345

var label = f"{lastName}, {firstName} (ID: {id})"
println(label)  // Output: Doe, John (ID: 12345)
```

## String Patterns

### Building Paths

```razen
var directory = "/home/user"
var filename = "document.txt"
var path = directory + "/" + filename
println(path)  // Output: /home/user/document.txt
```

### Creating URLs

```razen
var protocol = "https"
var domain = "example.com"
var path = "/api/users"
var url = f"{protocol}://{domain}{path}"
println(url)  // Output: https://example.com/api/users
```

### Formatting Tables

```razen
var header = "Name\tAge\tCity"
var row1 = "Hanuman\t25\tNew York"
var row2 = "Ram\t30\tLondon"

println(header)
println(row1)
println(row2)
```

### Building SQL Queries

```razen
var table = "users"
var column = "name"
var value = "Hanuman"

var query = f"SELECT * FROM {table} WHERE {column} = '{value}'"
println(query)
```

## Type Conversion

### Integer to String

```razen
var number = 42
var text = tostr(number)
println(f"The answer is {text}")
```

### Float to String

```razen
var price = 19.99
var text = tostr(price)
println(f"Price: ${text}")
```

### Boolean to String

```razen
var flag = true
var text = tostr(flag)
println(f"Flag is {text}")
```

### String to Integer

```razen
var text = "123"
var number = toint(text)
println(number)  // Output: 123
```

### String to Float

```razen
var text = "3.14"
var number = tofloat(text)
println(number)  // Output: 3.14
```

## Common Patterns

### Repeating Strings

```razen
fun repeat(text: str, count: int) -> str {
    var result = ""
    for i in 1..=count {
        result = result + text
    }
    return result
}

var line = repeat("-", 20)
println(line)  // Output: --------------------
```

### Padding Strings

```razen
fun padLeft(text: str, width: int, chara: str) -> str {
    var padding = width - len(text)
    var result = ""
    for i in 1..=padding {
        result = result + chara
    }
    return result + text
}

var padded = padLeft("42", 5, "0")
println(padded)  // Output: 00042
```

### Joining Strings

```razen
fun join(strings: [str], separator: str) -> str {
    var result = ""
    var first = true
    
    for s in strings {
        if !first {
            result = result + separator
        }
        result = result + s
        first = false
    }
    
    return result
}

var words = ["Hello", "World", "Razen"]
var sentence = join(words, " ")
println(sentence)  // Output: Hello World Razen
```

### Reversing Strings

```razen
fun reverse(text: str) -> str {
    var result = ""
    var length = len(text)
    
    for i in 1..=length {
        var index = length - i
        result = result + text[index]
    }
    
    return result
}

var reversed = reverse("hello")
println(reversed)  // Output: olleh
```

## Colored Output

Razen provides built-in colored printing functions:

### Basic Colors

```razen
printlnc("Success!", "green")
printlnc("Warning!", "yellow")
printlnc("Error!", "red")
printlnc("Info", "blue")
```

### Available Colors

```razen
printlnc("Black text", "black")
printlnc("Red text", "red")
printlnc("Green text", "green")
printlnc("Yellow text", "yellow")
printlnc("Blue text", "blue")
printlnc("Magenta text", "magenta")
printlnc("Cyan text", "cyan")
printlnc("White text", "white")
```

### Bright Colors

```razen
printlnc("Bright red", "bright_red")
printlnc("Bright green", "bright_green")
printlnc("Bright blue", "bright_blue")
```

### Hex Colors

```razen
printlnc("Custom color", "#FF6600")
printlnc("Another color", "#00AAFF")
```

### Combining with F-Strings

```razen
var name = "Hanuman"
var score = 95

printlnc(f"{name} scored {score}%", "green")
```

## Best Practices

### Use Meaningful Names

```razen
// Good
var username = "Hanuman"
var errorMessage = "Invalid input"
var welcomeText = "Welcome to Razen!"

// Avoid
var s = "Hanuman"
var msg = "Invalid input"
var txt = "Welcome to Razen!"
```

### Use F-Strings for Interpolation

```razen
// Good
var name = "Hanuman"
var message = f"Hello, {name}!"

// Avoid
var message = "Hello, " + name + "!"
```

### Keep Strings Readable

```razen
// Good
var query = f"SELECT * FROM users WHERE id = {userId}"

// Avoid
var query = "SELECT * FROM users WHERE id = " + tostr(userId)
```

### Use Constants for Repeated Strings

```razen
// Good
var ERROR_PREFIX = "[ERROR]"
var WARNING_PREFIX = "[WARNING]"

var errorMsg = f"{ERROR_PREFIX} File not found"
var warningMsg = f"{WARNING_PREFIX} Low memory"

// Avoid
var errorMsg = "[ERROR] File not found"
var warningMsg = "[WARNING] Low memory"
```

## Examples

See [examples/variables-datatypes/](../../examples/variables-datatypes/) for complete programs demonstrating string operations.

## Next Steps

- Learn about [Booleans](booleans.md) for logical operations
- Explore [Type Conversion](type-conversion.md) for converting between types
- Study [Complex Types](complex-types.md) for arrays of strings
