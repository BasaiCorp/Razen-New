# Syntax Highlighting Not Working - Fix Guide

## ✅ The Issue:
Syntax highlighting is not showing up in Zed for Razen files.

## 🔍 Possible Causes:

### 1. **Extension Not Reloaded**
After installing/updating the extension, Zed needs to reload it.

**Fix:**
1. Open Zed
2. Press `Ctrl+Shift+P` (or `Cmd+Shift+P` on Mac)
3. Type: `zed: reload extensions`
4. Press Enter
5. Open a `.rzn` file again

### 2. **File Not Recognized as Razen**
Check the bottom-right corner of Zed - it should say "Razen", not "Plain Text".

**Fix:**
1. Click on the language indicator (bottom-right)
2. Select "Razen" from the list
3. Or save file with `.rzn` extension

### 3. **Extension Not Enabled**
The extension might be installed but not enabled.

**Fix:**
1. Press `Ctrl+Shift+P`
2. Type: `extensions`
3. Check if "Razen" is listed and enabled
4. If disabled, click to enable it

### 4. **Grammar Not Compiled**
The tree-sitter grammar might not be properly compiled.

**Fix:**
```bash
cd /home/prathmeshbro/Desktop/razen\ project/razen-lang-new/extensions/razen-zed
rm -rf grammars/
```

Then in Zed:
1. Press `Ctrl+Shift+P`
2. Type: `zed: reload extensions`

### 5. **Highlights Query Syntax Error**
There might be an error in the highlights.scm file.

**Check Zed Logs:**
1. Press `Ctrl+Shift+P`
2. Type: `zed: open log`
3. Look for errors related to "razen" or "highlights"

## 🎯 **Step-by-Step Fix:**

### Step 1: Reload Extension
```
Ctrl+Shift+P → "zed: reload extensions"
```

### Step 2: Check File Association
- Bottom-right should show "Razen"
- If not, click and select "Razen"

### Step 3: Restart Zed
Close and reopen Zed completely

### Step 4: Test with Simple File
Create a new file `test.rzn`:
```razen
// This is a comment
fun main() {
    var x = 42
    println("Hello")
}
```

### Step 5: Check What's Working
Even if colors aren't perfect, check if:
- ✅ Comments are different color
- ✅ Keywords (`fun`, `var`) are highlighted
- ✅ Strings are highlighted
- ✅ Numbers are highlighted

## 📋 **Expected Highlighting:**

With our highlights.scm, you should see:
- **Keywords**: `fun`, `var`, `const`, `struct`, `impl`, etc.
- **Control Flow**: `if`, `else`, `while`, `for`, `return`, etc.
- **Types**: `int`, `float`, `str`, `bool`
- **Strings**: `"text"` and f-strings
- **Numbers**: `42`, `3.14`
- **Comments**: `// comment` and `/* block */`
- **Functions**: Function names in declarations and calls
- **Operators**: `+`, `-`, `*`, `/`, `==`, etc.

## 🔧 **If Still Not Working:**

### Check Zed Version
Make sure you're using a recent version of Zed that supports dev extensions.

### Verify Extension Structure
```bash
cd /home/prathmeshbro/Desktop/razen\ project/razen-lang-new/extensions/razen-zed
ls -la languages/razen/
```

Should show:
- config.toml
- highlights.scm
- brackets.scm
- outline.scm
- indents.scm
- injections.scm

### Test Grammar Separately
```bash
cd tree-sitter-razen
bunx tree-sitter parse ../simple.rzn
```

Should show a proper syntax tree without errors.

## 💡 **Quick Test:**

1. Open Zed
2. Create new file: `test.rzn`
3. Type: `fun main() {}`
4. The word `fun` should be highlighted as a keyword
5. If not, try `Ctrl+Shift+P` → `zed: reload extensions`

---

**If none of this works, share the Zed logs and we'll debug further!**
