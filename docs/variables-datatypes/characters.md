# Characters

Characters represent single characters in Razen. This guide covers character literals, operations, and common use cases.

## Declaration

Characters are enclosed in single quotes.

### Basic Declaration

```razen
var letter = 'A'
var digit = '5'
var symbol = '@'
var space = ' '
```

### With Type Annotation

```razen
var initial: char = 'J'
var grade: char = 'A'
var symbol: char = '#'
```

## Character Literals

### Letters

```razen
var uppercase = 'A'
var lowercase = 'a'
```

### Digits

```razen
var zero = '0'
var five = '5'
var nine = '9'
```

### Symbols

```razen
var at = '@'
var hash = '#'
var dollar = '$'
var percent = '%'
var ampersand = '&'
```

### Special Characters

```razen
var space = ' '
var newline = '\n'
var tab = '\t'
var backslash = '\\'
var singleQuote = '\''
```

## Character vs String

Characters are single characters, strings are sequences:

```razen
var char = 'A'        // Single character
var string = "A"      // String with one character

var multiChar = "AB"  // String with multiple characters
// var invalid = 'AB' // Error: char can only hold one character
```

## Character Operations

### Comparison

```razen
var a = 'A'
var b = 'B'

var equal = a == b        // false
var notEqual = a != b     // true
var less = a < b          // true (ASCII order)
var greater = a > b       // false
```

### ASCII Ordering

Characters are compared by their ASCII values:

```razen
var result1 = 'A' < 'B'   // true
var result2 = 'a' > 'A'   // true (lowercase > uppercase)
var result3 = '0' < '9'   // true
var result4 = 'Z' < 'a'   // true
```

## Common Use Cases

### Character Classification

```razen
fun isUppercase(c: char) -> bool {
    return c >= 'A' && c <= 'Z'
}

fun isLowercase(c: char) -> bool {
    return c >= 'a' && c <= 'z'
}

fun isDigit(c: char) -> bool {
    return c >= '0' && c <= '9'
}

fun isLetter(c: char) -> bool {
    return isUppercase(c) || isLowercase(c)
}

var check1 = isUppercase('A')  // true
var check2 = isLowercase('z')  // true
var check3 = isDigit('5')      // true
var check4 = isLetter('M')     // true
```

### Character Conversion

```razen
fun toUppercase(c: char) -> char {
    if c >= 'a' && c <= 'z' {
        // Convert lowercase to uppercase
        var offset = 'a' - 'A'
        return c - offset
    }
    return c
}

fun toLowercase(c: char) -> char {
    if c >= 'A' && c <= 'Z' {
        // Convert uppercase to lowercase
        var offset = 'a' - 'A'
        return c + offset
    }
    return c
}

var upper = toUppercase('a')  // 'A'
var lower = toLowercase('Z')  // 'z'
```

### Grades and Ratings

```razen
fun getGrade(score: int) -> char {
    if score >= 90 {
        return 'A'
    } elif score >= 80 {
        return 'B'
    } elif score >= 70 {
        return 'C'
    } elif score >= 60 {
        return 'D'
    } else {
        return 'F'
    }
}

var grade = getGrade(85)
println(f"Grade: {grade}")  // Output: Grade: B
```

### Menu Options

```razen
fun processMenuChoice(choice: char) {
    if choice == 'A' || choice == 'a' {
        println("Option A selected")
    } elif choice == 'B' || choice == 'b' {
        println("Option B selected")
    } elif choice == 'C' || choice == 'c' {
        println("Option C selected")
    } else {
        println("Invalid option")
    }
}

processMenuChoice('B')  // Output: Option B selected
```

## Type Conversion

### Character to String

```razen
var char = 'A'
var string = tostr(char)
println(string)  // Output: A
```

### Character to Integer (ASCII)

```razen
var char = 'A'
var ascii = toint(char)
println(ascii)  // Output: 65
```

### Integer to Character

```razen
var ascii = 65
var char = tochar(ascii)
println(char)  // Output: A
```

### String to Character

```razen
var string = "A"
var char = string[0]  // Get first character
println(char)  // Output: A
```

## Common Patterns

### Character Validation

```razen
fun isValidGrade(grade: char) -> bool {
    return grade == 'A' || grade == 'B' || 
           grade == 'C' || grade == 'D' || 
           grade == 'F'
}

var valid = isValidGrade('A')    // true
var invalid = isValidGrade('X')  // false
```

### Character Counting

```razen
fun countChar(text: str, target: char) -> int {
    var count = 0
    var length = len(text)
    
    for i in 0..length {
        if text[i] == target {
            count = count + 1
        }
    }
    
    return count
}

var count = countChar("hello world", 'l')
println(count)  // Output: 3
```

### Character Replacement

```razen
fun replaceChar(text: str, oldChar: char, newChar: char) -> str {
    var result = ""
    var length = len(text)
    
    for i in 0..length {
        if text[i] == oldChar {
            result = result + tostr(newChar)
        } else {
            result = result + tostr(text[i])
        }
    }
    
    return result
}

var replaced = replaceChar("hello", 'l', 'L')
println(replaced)  // Output: heLLo
```

### First and Last Character

```razen
fun firstChar(text: str) -> char {
    if len(text) > 0 {
        return text[0]
    }
    return ' '
}

fun lastChar(text: str) -> char {
    var length = len(text)
    if length > 0 {
        return text[length - 1]
    }
    return ' '
}

var first = firstChar("hello")  // 'h'
var last = lastChar("hello")    // 'o'
```

## ASCII Values

Common ASCII character ranges:

### Digits

```razen
'0' = 48
'1' = 49
...
'9' = 57
```

### Uppercase Letters

```razen
'A' = 65
'B' = 66
...
'Z' = 90
```

### Lowercase Letters

```razen
'a' = 97
'b' = 98
...
'z' = 122
```

### Special Characters

```razen
' ' = 32  (space)
'!' = 33
'@' = 64
'[' = 91
'{' = 123
```

## Character Arrays

Work with arrays of characters:

```razen
var vowels = ['a', 'e', 'i', 'o', 'u']

fun isVowel(c: char) -> bool {
    for vowel in vowels {
        if c == vowel {
            return true
        }
    }
    return false
}

var check = isVowel('e')  // true
```

## Best Practices

### Use Meaningful Names

```razen
// Good
var initial = 'J'
var grade = 'A'
var separator = ','

// Avoid
var c = 'J'
var x = 'A'
var s = ','
```

### Use Characters for Single Values

```razen
// Good: Use char for single character
var delimiter = ','
var separator = ';'

// Avoid: Use string for single character
var delimiter = ","
var separator = ";"
```

### Document Character Meanings

```razen
// Good: Clear purpose
var YES_OPTION = 'Y'
var NO_OPTION = 'N'
var QUIT_OPTION = 'Q'

if choice == YES_OPTION {
    println("Confirmed")
}
```

### Use Constants for Special Characters

```razen
// Good: Named constants
var NEWLINE = '\n'
var TAB = '\t'
var SPACE = ' '

var formatted = "Name:" + TAB + name + NEWLINE

// Avoid: Magic characters
var formatted = "Name:\t" + name + "\n"
```

## Common Mistakes

### Single vs Double Quotes

```razen
// Correct: Single quotes for char
var char = 'A'

// Wrong: Double quotes create string
var notChar = "A"  // This is a string, not char
```

### Multiple Characters

```razen
// Correct: Single character
var char = 'A'

// Wrong: Multiple characters
// var invalid = 'AB'  // Error: char can only hold one character
```

### Empty Character

```razen
// Wrong: Cannot have empty char
// var empty = ''  // Error: char must have exactly one character

// Use space if needed
var space = ' '
```

## Examples

See [examples/variables-datatypes/](../../examples/variables-datatypes/) for complete programs demonstrating character operations.

## Next Steps

- Learn about [Strings](strings.md) for working with text
- Explore [Type Conversion](type-conversion.md) for converting between types
- Study [Arrays](complex-types.md) for character arrays
