# Quick Start - Install Razen Extension in Zed

## 📋 **Installation Steps:**

### **1. Open Zed Editor**
```bash
zed
```

### **2. Install the Extension**

**Option A: Using Command Palette**
1. Press `Ctrl+Shift+P` (Linux/Windows) or `Cmd+Shift+P` (Mac)
2. Type: `zed: install dev extension`
3. Press Enter
4. Navigate to: `/home/prathmeshbro/Desktop/razen project/razen-lang-new/extensions/razen-zed`
5. Select the folder and click "Open"

**Option B: Using Menu**
1. Go to: `Zed` → `Extensions` → `Install Dev Extension`
2. Navigate to the `razen-zed` directory
3. Click "Open"

### **3. Verify Installation**
1. Open Zed
2. Go to `Zed` → `Extensions`
3. You should see "Razen" listed as a dev extension

### **4. Test It!**

**Open a Razen file:**
```bash
zed /home/prathmeshbro/Desktop/razen\ project/razen-lang-new/extensions/razen-zed/test.rzn
```

**Or create a new file:**
1. In Zed, create new file: `Ctrl+N` / `Cmd+N`
2. Save as `hello.rzn`
3. Type some Razen code:
```razen
fun main() {
    var name = "World"
    println(f"Hello, {name}!")
}
```

### **5. What You Should See:**

✅ **Syntax Highlighting:**
- Keywords in color (fun, var, if, etc.)
- Strings highlighted
- Comments in different color
- Operators visible

✅ **Code Outline:**
- Functions listed in outline panel
- Structs and enums visible
- Easy navigation

✅ **Bracket Matching:**
- Matching brackets highlighted
- Auto-closing brackets

✅ **Auto-Indentation:**
- Proper indentation on Enter
- Smart block formatting

## 🔧 **Troubleshooting:**

### **Extension Not Loading?**
1. Check Zed logs: `Ctrl+Shift+P` → "Open Log"
2. Look for errors related to "razen"
3. Restart Zed

### **No Syntax Highlighting?**
1. Make sure file extension is `.rzn` or `.razen`
2. Check bottom-right of Zed - should say "Razen"
3. If it says "Plain Text", click it and select "Razen"

### **Grammar Not Working?**
1. Rebuild the grammar:
```bash
cd /home/prathmeshbro/Desktop/razen\ project/razen-lang-new/extensions/razen-zed
./build.sh
```
2. Reinstall the extension in Zed

## 📝 **Test Files:**

### **Test 1: Simple Program**
```razen
fun greet(name: str) {
    println(f"Hello, {name}!")
}

fun main() {
    greet("Razen")
}
```

### **Test 2: Struct and Impl**
```razen
struct Person {
    name: str,
    age: int
}

impl Person {
    fun new(name: str, age: int) -> Person {
        return Person { name: name, age: age }
    }
    
    fun greet(self) {
        println(f"Hi, I'm {self.name}")
    }
}

fun main() {
    var person = Person.new("Hanuman", 25)
    person.greet()
}
```

### **Test 3: Control Flow**
```razen
fun main() {
    // For loop
    for i in 1..=5 {
        println(f"Count: {i}")
    }
    
    // If statement
    var x = 10
    if x > 5 {
        println("Greater than 5")
    } else {
        println("Less than or equal to 5")
    }
}
```

## ✅ **Success Checklist:**

- [ ] Extension installed in Zed
- [ ] `.rzn` files show syntax highlighting
- [ ] Code outline shows functions/structs
- [ ] Brackets match and auto-close
- [ ] Indentation works properly
- [ ] File association works (.rzn → Razen)

## 🎉 **Next Steps:**

Once everything works:
1. ✅ Mark extension as complete
2. 🔧 Fix the match statement parsing (minor issue)
3. 📦 Optionally publish to Zed extensions registry
4. 🚀 Start using Razen in Zed!

---

**Ready to test? Open Zed and follow the steps above!** 🎯
