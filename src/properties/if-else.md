# Razen If-Else Control Flow System

## Basic If-Else Syntax

### Simple If Statement
```razen
if condition {
    // code block
}
```

### If-Else Statement
```razen
if condition {
    // true block
} else {
    // false block
}
```

### If-Elif-Else Chain
```razen
if condition1 {
    // first condition true
} elif condition2 {
    // second condition true
} elif condition3 {
    // third condition true
} else {
    // all conditions false
}
```

## Comparison Operators

### Equality Operators
```razen
if x == y { println("Equal") }
if x != y { println("Not equal") }
```

### Relational Operators
```razen
if x < y { println("Less than") }
if x <= y { println("Less than or equal") }
if x > y { println("Greater than") }
if x >= y { println("Greater than or equal") }
```

## Logical Operators

### AND, OR, NOT
```razen
// AND
if x > 0 && y > 0 {
    println("Both positive")
}

// OR
if x == 0 || y == 0 {
    println("At least one is zero")
}

// NOT
if !is_empty {
    println("Not empty")
}
```

## String Comparisons

```razen
var name = "Razen"

// String equality
if name == "Razen" {
    println("Language name matches")
}

// String contains
if name.contains("zen") {
    println("Contains 'zen'")
}

// String length
if name.length > 0 {
    println("Not empty string")
}
```

## Numeric Comparisons

```razen
var age = 25
var score = 85.5

// Integer comparisons
if age >= 18 {
    println("Adult")
}

// Float comparisons
if score > 90.0 {
    println("Excellent")
} elif score >= 80.0 {
    println("Good")
} else {
    println("Needs improvement")
}

// Range checking
if age >= 13 && age <= 19 {
    println("Teenager")
}
```

## Complex Conditions

```razen
// Multiple conditions with parentheses
if (age >= 18 && age <= 65) && (has_license || has_permit) && !is_banned {
    println("Can drive")
}

// Null checking
if user != null && user.name != null {
    println(f"Hello, {user.name}")
}

// Type checking
if typeof(value) == "string" && value.length > 0 {
    println("Valid string")
}
```

## Nested If-Else

```razen
fun check_weather(weather: str, temperature: float) {
    if weather == "sunny" {
        if temperature > 25.0 {
            println("Perfect beach weather")
        } elif temperature > 15.0 {
            println("Nice for a walk")
        } else {
            println("Sunny but cold")
        }
    } elif weather == "rainy" {
        if temperature > 20.0 {
            println("Warm rain")
        } else {
            println("Cold and wet")
        }
    } else {
        println("Check forecast")
    }
}
```

## Practical Examples

### Grade System
```razen
fun get_grade(score: float) -> str {
    if score >= 90.0 {
        return "A"
    } elif score >= 80.0 {
        return "B"
    } elif score >= 70.0 {
        return "C"
    } elif score >= 60.0 {
        return "D"
    } else {
        return "F"
    }
}
```

### User Authentication
```razen
fun authenticate_user(username: str, password: str) -> bool {
    if username.length == 0 {
        println("Username required")
        return false
    }
    
    if password.length < 8 {
        println("Password too short")
        return false
    }
    
    if verify_credentials(username, password) {
        println("Login successful")
        return true
    } else {
        println("Invalid credentials")
        return false
    }
}
```

## Ternary Operator

```razen
// Simple ternary
var status = age >= 18 ? "adult" : "minor"

// Nested ternary
var grade = score >= 90 ? "A" : score >= 80 ? "B" : "C"

// With function calls
var message = is_logged_in() ? "Welcome back!" : "Please login"
```

## Best Practices

1. **Use clear conditions**
```razen
// Good
var is_adult = age >= 18
if is_adult {
    println("Can vote")
}

// Avoid deep nesting - use early returns
fun process_user(user: User?) {
    if user == null { return }
    if !user.is_active { return }
    
    // Main logic here
    println("Processing user")
}
```

2. **Handle all cases**
```razen
// Always include else for completeness
if condition1 {
    // handle case 1
} elif condition2 {
    // handle case 2
} else {
    // handle unexpected cases
    println("Unknown condition")
}
```