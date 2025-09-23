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

---

## 🚀 **IMPLEMENTATION STATUS**

### ✅ **COMPLETED FEATURES (Phase 1 & 2)**
1. **Lexer Support**: ✅ All keywords (`if`, `elif`, `else`) and comparison operators (`==`, `!=`, `<`, `>`, `<=`, `>=`) implemented
2. **Parser Support**: ✅ Complete if-elif-else parsing with proper AST nodes (`IfStatement`, `ElifBranch`)
3. **AST Structure**: ✅ Full AST support for if statements with multiple elif branches and optional else
4. **Semantic Analysis**: ✅ Complete semantic analysis for if statements and all branches
5. **Basic Comparison Operators**: ✅ All comparison operators working (`==`, `!=`, `<`, `>`, `<=`, `>=`)
6. **Backend Compilation**: ✅ Complete elif branch compilation - fully working!
7. **IR Generation**: ✅ Full if-elif-else chain support with proper jump handling
8. **Logical Operators**: ✅ `&&`, `||`, `!` operators for complex conditions - fully implemented!
9. **String Comparisons**: ✅ String equality and inequality comparisons working
10. **Complex Conditions**: ✅ Multiple logical operators in single conditions
11. **Nested If-Else**: ✅ Complex nested conditional structures working
12. **Boolean Logic**: ✅ Full boolean operations and negation support

### ⏳ **PLANNED FEATURES (Phase 3)**
1. **String Methods**: ⏳ `.contains()`, `.length` for advanced string comparisons
2. **Ternary Operator**: ⏳ `condition ? true_value : false_value` syntax
3. **Null Checking**: ⏳ `!= null` and null safety features
4. **Type Checking**: ⏳ `typeof()` function for runtime type checking
5. **Pattern Matching**: ⏳ Advanced pattern matching with `match` statements
6. **Range Operators**: ⏳ `in` operator for range checking (`x in 1..10`)

### 🎯 **CURRENT WORKING FEATURES - 6 TYPES OF IF-ELIF-ELSE**

#### **1. ✅ Basic If-Elif-Else Chains**
```razen
if score >= 90 {
    println("A - Excellent!")
} elif score >= 80 {
    println("B - Good")
} elif score >= 70 {
    println("C - Average")
} elif score >= 60 {
    println("D - Pass")
} else {
    println("F - Fail")
}
```

#### **2. ✅ Logical Operators (&&, ||, !)**
```razen
// Complex AND conditions
if age >= 18 && age <= 65 && hasLicense {
    println("Can drive")
}

// Complex OR conditions  
if isAdmin || hasPermission || isOwner {
    println("Access granted")
}

// Negation logic
if !isLoggedIn {
    println("Please log in")
}
```

#### **3. ✅ String Comparisons**
```razen
var name = "Razen"
if name == "Razen" {
    println("Language name matches")
} elif name != "Unknown" {
    println("Different language")
} else {
    println("Unknown language")
}
```

#### **4. ✅ Multiple Variable Comparisons**
```razen
if x < y && y > z && z > x {
    println("Complex relationship")
} elif x == y || y == z {
    println("Some values equal")
} else {
    println("Other relationship")
}
```

#### **5. ✅ Nested If-Else Structures**
```razen
if weather == "sunny" {
    if temperature > 30 {
        println("Perfect beach weather!")
    } elif temperature > 20 {
        println("Nice day for a walk")
    } else {
        println("Sunny but cold")
    }
} elif weather == "rainy" {
    println("Stay inside")
}
```

#### **6. ✅ Complex Boolean Logic**
```razen
if isLoggedIn && (hasPermission || isAdmin) {
    println("Full access")
} elif isLoggedIn && !hasPermission && !isAdmin {
    println("Limited access")
} elif !isLoggedIn {
    println("Please log in")
}
```

#### **✅ All Comparison Operators Working**
```razen
if x == y { }    // Equal
if x != y { }    // Not equal
if x < y { }     // Less than
if x > y { }     // Greater than
if x <= y { }    // Less than or equal
if x >= y { }    // Greater than or equal
```

**🎉 ACHIEVEMENT**: Razen now supports **6 comprehensive types** of if-elif-else comparisons with full medium-level functionality!