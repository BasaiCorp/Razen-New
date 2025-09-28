# Razen Module System Implementation Plan

## 🎯 **FINAL DESIGN DECISION**

**File-based module system with simple dot notation calls (NO `mod` keyword needed)**

### **Core Principle:**
- **Last part of path = module name**
- `"./calculator"` → `calculator.Add()`
- `"./math/calculator"` → `calculator.Add()` (same call!)
- `"./utils/string"` → `string.ToUpper()`

---

## 📋 **IMPLEMENTATION PHASES**

### **Phase 1: Frontend Enhancement** ✅ (Already Done)
- [x] Lexer: `Mod`, `Use`, `Pub`, `As` tokens
- [x] Parser: Module and use statement parsing
- [x] AST: ModuleDeclaration, UseStatement nodes

### **Phase 2: Backend Module Resolution** 🔄 (In Progress)
- [ ] Module registry and resolver
- [ ] File path to module name mapping
- [ ] Symbol visibility checking (`pub` vs private)
- [ ] Import resolution and dependency tracking

### **Phase 3: Semantic Analysis Integration** ⏳ (Pending)
- [ ] Module-aware symbol table
- [ ] Cross-module symbol resolution
- [ ] Import validation and error reporting
- [ ] Circular dependency detection

### **Phase 4: CLI and Build System** ⏳ (Pending)
- [ ] Multi-file compilation support
- [ ] Module path resolution in CLI
- [ ] Build system integration
- [ ] Module caching and incremental compilation

---

## 🏗️ **DETAILED IMPLEMENTATION PLAN**

### **Phase 2: Backend Module Resolution**

#### **2.1 Module Registry (`src/backend/module_registry.rs`)**
```rust
pub struct ModuleRegistry {
    modules: HashMap<String, LoadedModule>,
    search_paths: Vec<PathBuf>,
    current_module: Option<String>,
}

pub struct LoadedModule {
    name: String,
    path: PathBuf,
    public_symbols: HashMap<String, SymbolInfo>,
    dependencies: Vec<String>,
    ast: Program,
}
```

#### **2.2 Module Resolver (`src/backend/module_resolver.rs`)**
```rust
pub struct ModuleResolver {
    registry: ModuleRegistry,
}

impl ModuleResolver {
    pub fn resolve_import(&mut self, import_path: &str) -> Result<String, ModuleError>
    pub fn load_module(&mut self, path: &Path) -> Result<LoadedModule, ModuleError>
    pub fn get_module_name(&self, path: &str) -> String  // "./math/calculator" -> "calculator"
}
```

#### **2.3 Symbol Visibility (`src/backend/visibility.rs`)**
```rust
pub enum Visibility {
    Public,    // pub fun, pub struct
    Private,   // fun, struct (default)
}

pub fn check_symbol_access(symbol: &SymbolInfo, from_module: &str, to_module: &str) -> bool
```

### **Phase 3: Semantic Analysis Integration**

#### **3.1 Enhanced Symbol Table**
```rust
pub struct ModuleAwareSymbolTable {
    local_symbols: HashMap<String, Symbol>,
    imported_modules: HashMap<String, String>, // alias -> module_name
    module_registry: Arc<ModuleRegistry>,
}
```

#### **3.2 Import Resolution**
```rust
impl SemanticAnalyzer {
    fn analyze_use_statement(&mut self, use_stmt: &UseStatement) -> Result<(), SemanticError>
    fn resolve_module_call(&mut self, call: &CallExpression) -> Result<Type, SemanticError>
}
```

### **Phase 4: CLI and Build System**

#### **4.1 Multi-file Compilation**
```rust
// Enhanced CLI commands
razen run main.rzn          // Automatically resolves imports
razen build project/        // Builds entire project
razen check main.rzn        // Type checks with imports
```

#### **4.2 Module Path Resolution**
```rust
pub struct BuildContext {
    root_path: PathBuf,
    source_files: Vec<PathBuf>,
    module_graph: DependencyGraph,
}
```

---

## 📝 **SYNTAX SPECIFICATION**

### **Import Syntax**
```razen
// Standard library
use fmt
use math
use strings

// Local files (simple project)
use "./calculator"      // calculator.rzn -> calculator.Add()
use "./utils"          // utils.rzn -> utils.ToUpper()

// Local files (complex project)
use "./math/calculator" // math/calculator.rzn -> calculator.Add()
use "./utils/string"   // utils/string.rzn -> string.ToUpper()

// Aliases
use "./calculator" as calc  // calc.Add()
use "./utils/string" as str_util  // str_util.ToUpper()
```

### **Export Syntax**
```razen
// Public (exported)
pub fun Add(a: int, b: int) -> int { return a + b }
pub struct User { name: str, age: int }
pub const PI: float = 3.14159

// Private (not exported)
fun helper() { }  // Only accessible within this file
struct Internal { }  // Only accessible within this file
```

### **Usage Syntax**
```razen
// main.rzn
use "./calculator"
use "./utils"

fun main() {
    var result = calculator.Add(5, 3)     // Direct call
    var text = utils.ToUpper("hello")     // Direct call
}
```

---

## 🔍 **MODULE RESOLUTION ALGORITHM**

### **Step 1: Parse Import Path**
```
"./math/calculator" -> {
    relative_path: "./math/calculator",
    file_path: "math/calculator.rzn",
    module_name: "calculator"
}
```

### **Step 2: Resolve File**
```
Search paths:
1. Current directory + "math/calculator.rzn"
2. src/ + "math/calculator.rzn"
3. lib/ + "math/calculator.rzn"
```

### **Step 3: Load and Parse**
```
1. Read file content
2. Parse to AST
3. Extract public symbols
4. Register in module registry
```

### **Step 4: Symbol Resolution**
```
calculator.Add() -> {
    module: "calculator",
    symbol: "Add",
    check_visibility: public,
    resolve_to: Function(Add)
}
```

---

## 🧪 **TEST CASES**

### **Test 1: Simple Project**
```
project/
├── main.rzn
├── calculator.rzn
└── utils.rzn
```

### **Test 2: Complex Project**
```
project/
├── main.rzn
├── math/
│   ├── calculator.rzn
│   └── geometry.rzn
└── utils/
    ├── string.rzn
    └── file.rzn
```

### **Test 3: Error Cases**
- Module not found
- Symbol not exported
- Circular dependencies
- Naming conflicts

---

## 🚀 **IMPLEMENTATION ORDER**

1. **Module Registry** - Core data structures
2. **Module Resolver** - File loading and path resolution
3. **Visibility Checker** - Public/private symbol access
4. **Semantic Integration** - Import statement analysis
5. **CLI Enhancement** - Multi-file compilation
6. **Testing** - Comprehensive test suite
7. **Documentation** - Usage examples and guides

---

## ✅ **SUCCESS CRITERIA**

- [x] Simple calls: `calculator.Add(5, 3)`
- [ ] Works with any folder structure
- [ ] No `mod` keyword needed
- [ ] Proper error messages
- [ ] Fast compilation
- [ ] Easy to understand and use

---

## 🎯 **FINAL GOAL**

**Make Razen module system the EASIEST and most POWERFUL among modern languages while maintaining simplicity and clarity.**
