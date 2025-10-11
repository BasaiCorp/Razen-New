// src/backend/execution/raze/mir.rs
//! MIR - Mid-level Intermediate Representation
//! 
//! A register-based, strongly-typed IR optimized for native code generation.
//! Designed to be:
//! - Easy to optimize (SSA-like form)
//! - Easy to translate to machine code
//! - Type-aware for better code generation
//! - Platform-independent

use std::collections::HashMap;
use std::fmt;
use crate::backend::execution::value::Value;
use crate::backend::execution::ir::IR;

/// Virtual register identifier (0-255 for easy encoding)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Reg(pub u8);

impl fmt::Display for Reg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "r{}", self.0)
    }
}

/// Label for control flow
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Label(pub u32);

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "L{}", self.0)
    }
}

/// Function identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FuncId(pub u32);

impl fmt::Display for FuncId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "F{}", self.0)
    }
}

/// MIR value types (for type-aware code generation)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MIRType {
    Int64,
    Float64,
    Bool,
    Ptr,      // Pointer to heap object (String, Array, Map, Struct)
    Void,
}

impl fmt::Display for MIRType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MIRType::Int64 => write!(f, "i64"),
            MIRType::Float64 => write!(f, "f64"),
            MIRType::Bool => write!(f, "bool"),
            MIRType::Ptr => write!(f, "ptr"),
            MIRType::Void => write!(f, "void"),
        }
    }
}

/// MIR immediate values
#[derive(Debug, Clone)]
pub enum MIRImmediate {
    Int(i64),
    Float(f64),
    Bool(bool),
    Null,
}

impl fmt::Display for MIRImmediate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MIRImmediate::Int(i) => write!(f, "{}", i),
            MIRImmediate::Float(fl) => write!(f, "{}", fl),
            MIRImmediate::Bool(b) => write!(f, "{}", b),
            MIRImmediate::Null => write!(f, "null"),
        }
    }
}

/// MIR instructions - register-based, strongly-typed
#[derive(Debug, Clone)]
pub enum MIR {
    // Register operations
    LoadImm { dest: Reg, value: MIRImmediate, ty: MIRType },
    Move { dest: Reg, src: Reg, ty: MIRType },
    
    // Arithmetic operations (type-specialized for better codegen)
    AddInt { dest: Reg, left: Reg, right: Reg },
    AddFloat { dest: Reg, left: Reg, right: Reg },
    SubInt { dest: Reg, left: Reg, right: Reg },
    SubFloat { dest: Reg, left: Reg, right: Reg },
    MulInt { dest: Reg, left: Reg, right: Reg },
    MulFloat { dest: Reg, left: Reg, right: Reg },
    DivInt { dest: Reg, left: Reg, right: Reg },
    DivFloat { dest: Reg, left: Reg, right: Reg },
    ModInt { dest: Reg, left: Reg, right: Reg },
    NegInt { dest: Reg, src: Reg },
    NegFloat { dest: Reg, src: Reg },
    
    // Comparison operations
    CmpEqInt { dest: Reg, left: Reg, right: Reg },
    CmpEqFloat { dest: Reg, left: Reg, right: Reg },
    CmpNeInt { dest: Reg, left: Reg, right: Reg },
    CmpNeFloat { dest: Reg, left: Reg, right: Reg },
    CmpLtInt { dest: Reg, left: Reg, right: Reg },
    CmpLtFloat { dest: Reg, left: Reg, right: Reg },
    CmpLeInt { dest: Reg, left: Reg, right: Reg },
    CmpLeFloat { dest: Reg, left: Reg, right: Reg },
    CmpGtInt { dest: Reg, left: Reg, right: Reg },
    CmpGtFloat { dest: Reg, left: Reg, right: Reg },
    CmpGeInt { dest: Reg, left: Reg, right: Reg },
    CmpGeFloat { dest: Reg, left: Reg, right: Reg },
    
    // Logical operations
    And { dest: Reg, left: Reg, right: Reg },
    Or { dest: Reg, left: Reg, right: Reg },
    Not { dest: Reg, src: Reg },
    
    // Bitwise operations
    BitAnd { dest: Reg, left: Reg, right: Reg },
    BitOr { dest: Reg, left: Reg, right: Reg },
    BitXor { dest: Reg, left: Reg, right: Reg },
    BitNot { dest: Reg, src: Reg },
    Shl { dest: Reg, left: Reg, right: Reg },
    Shr { dest: Reg, left: Reg, right: Reg },
    
    // Memory operations
    Load { dest: Reg, addr: Reg, offset: i32, ty: MIRType },
    Store { src: Reg, addr: Reg, offset: i32, ty: MIRType },
    LoadVar { dest: Reg, name: String, ty: MIRType },
    StoreVar { src: Reg, name: String, ty: MIRType },
    
    // Control flow
    Label(Label),
    Jump { target: Label },
    JumpIfZero { cond: Reg, target: Label },
    JumpIfNotZero { cond: Reg, target: Label },
    
    // Function calls
    Call { dest: Option<Reg>, func: String, args: Vec<Reg> },
    CallIndirect { dest: Option<Reg>, func_ptr: Reg, args: Vec<Reg> },
    Return { value: Option<Reg> },
    
    // Type conversions
    IntToFloat { dest: Reg, src: Reg },
    FloatToInt { dest: Reg, src: Reg },
    
    // Heap operations (for complex types)
    AllocArray { dest: Reg, size: Reg },
    AllocMap { dest: Reg },
    AllocStruct { dest: Reg, type_name: String },
    ArrayGet { dest: Reg, array: Reg, index: Reg },
    ArraySet { array: Reg, index: Reg, value: Reg },
    MapGet { dest: Reg, map: Reg, key: Reg },
    MapSet { map: Reg, key: Reg, value: Reg },
    FieldGet { dest: Reg, object: Reg, field: String },
    FieldSet { object: Reg, field: String, value: Reg },
    
    // Built-in operations
    Print { value: Reg },
    Input { dest: Reg },
    
    // Debug/profiling
    DebugPrint { msg: String },
    ProfilePoint { id: u32 },
}

impl fmt::Display for MIR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MIR::LoadImm { dest, value, ty } => write!(f, "{} = load.{} {}", dest, ty, value),
            MIR::Move { dest, src, ty } => write!(f, "{} = move.{} {}", dest, ty, src),
            
            MIR::AddInt { dest, left, right } => write!(f, "{} = add.i64 {}, {}", dest, left, right),
            MIR::AddFloat { dest, left, right } => write!(f, "{} = add.f64 {}, {}", dest, left, right),
            MIR::SubInt { dest, left, right } => write!(f, "{} = sub.i64 {}, {}", dest, left, right),
            MIR::SubFloat { dest, left, right } => write!(f, "{} = sub.f64 {}, {}", dest, left, right),
            MIR::MulInt { dest, left, right } => write!(f, "{} = mul.i64 {}, {}", dest, left, right),
            MIR::MulFloat { dest, left, right } => write!(f, "{} = mul.f64 {}, {}", dest, left, right),
            MIR::DivInt { dest, left, right } => write!(f, "{} = div.i64 {}, {}", dest, left, right),
            MIR::DivFloat { dest, left, right } => write!(f, "{} = div.f64 {}, {}", dest, left, right),
            MIR::ModInt { dest, left, right } => write!(f, "{} = mod.i64 {}, {}", dest, left, right),
            MIR::NegInt { dest, src } => write!(f, "{} = neg.i64 {}", dest, src),
            MIR::NegFloat { dest, src } => write!(f, "{} = neg.f64 {}", dest, src),
            
            MIR::CmpEqInt { dest, left, right } => write!(f, "{} = cmp.eq.i64 {}, {}", dest, left, right),
            MIR::CmpEqFloat { dest, left, right } => write!(f, "{} = cmp.eq.f64 {}, {}", dest, left, right),
            MIR::CmpLtInt { dest, left, right } => write!(f, "{} = cmp.lt.i64 {}, {}", dest, left, right),
            MIR::CmpLtFloat { dest, left, right } => write!(f, "{} = cmp.lt.f64 {}, {}", dest, left, right),
            
            MIR::And { dest, left, right } => write!(f, "{} = and {}, {}", dest, left, right),
            MIR::Or { dest, left, right } => write!(f, "{} = or {}, {}", dest, left, right),
            MIR::Not { dest, src } => write!(f, "{} = not {}", dest, src),
            
            MIR::LoadVar { dest, name, ty } => write!(f, "{} = load_var.{} {}", dest, ty, name),
            MIR::StoreVar { src, name, ty } => write!(f, "store_var.{} {}, {}", ty, name, src),
            
            MIR::Label(label) => write!(f, "{}:", label),
            MIR::Jump { target } => write!(f, "jump {}", target),
            MIR::JumpIfZero { cond, target } => write!(f, "jz {}, {}", cond, target),
            MIR::JumpIfNotZero { cond, target } => write!(f, "jnz {}, {}", cond, target),
            
            MIR::Call { dest, func, args } => {
                if let Some(d) = dest {
                    write!(f, "{} = call {}(", d, func)?;
                } else {
                    write!(f, "call {}(", func)?;
                }
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", arg)?;
                }
                write!(f, ")")
            }
            
            MIR::Return { value } => {
                if let Some(v) = value {
                    write!(f, "ret {}", v)
                } else {
                    write!(f, "ret")
                }
            }
            
            MIR::Print { value } => write!(f, "print {}", value),
            
            _ => write!(f, "{:?}", self),
        }
    }
}

/// MIR function representation
#[derive(Debug, Clone)]
pub struct MIRFunction {
    pub name: String,
    pub params: Vec<(String, MIRType)>,
    pub return_type: MIRType,
    pub instructions: Vec<MIR>,
    pub local_count: usize,
    pub max_reg: u8,
}

impl MIRFunction {
    pub fn new(name: String, params: Vec<(String, MIRType)>, return_type: MIRType) -> Self {
        Self {
            name,
            params,
            return_type,
            instructions: Vec::new(),
            local_count: 0,
            max_reg: 0,
        }
    }
}

/// MIR module (collection of functions)
#[derive(Debug, Clone)]
pub struct MIRModule {
    pub functions: HashMap<String, MIRFunction>,
    pub globals: HashMap<String, MIRType>,
}

impl MIRModule {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
            globals: HashMap::new(),
        }
    }
    
    pub fn add_function(&mut self, func: MIRFunction) {
        self.functions.insert(func.name.clone(), func);
    }
    
    pub fn add_global(&mut self, name: String, ty: MIRType) {
        self.globals.insert(name, ty);
    }
}

impl Default for MIRModule {
    fn default() -> Self {
        Self::new()
    }
}

/// MIR builder - converts stack-based IR to register-based MIR
pub struct MIRBuilder {
    current_function: Option<MIRFunction>,
    module: MIRModule,
    next_reg: u8,
    next_label: u32,
    label_map: HashMap<String, Label>,
    stack: Vec<Reg>,  // Simulated stack for IR translation
    variables: HashMap<String, (Reg, MIRType)>,
}

impl MIRBuilder {
    pub fn new() -> Self {
        Self {
            current_function: None,
            module: MIRModule::new(),
            next_reg: 0,
            next_label: 0,
            label_map: HashMap::new(),
            stack: Vec::new(),
            variables: HashMap::new(),
        }
    }
    
    /// Allocate a new virtual register
    fn alloc_reg(&mut self) -> Reg {
        let reg = Reg(self.next_reg);
        self.next_reg += 1;
        reg
    }
    
    /// Allocate a new label
    fn alloc_label(&mut self) -> Label {
        let label = Label(self.next_label);
        self.next_label += 1;
        label
    }
    
    /// Get or create label from string name
    fn get_label(&mut self, name: &str) -> Label {
        if let Some(&label) = self.label_map.get(name) {
            label
        } else {
            let label = self.alloc_label();
            self.label_map.insert(name.to_string(), label);
            label
        }
    }
    
    /// Emit MIR instruction to current function
    fn emit(&mut self, instr: MIR) {
        if let Some(ref mut func) = self.current_function {
            func.instructions.push(instr);
            func.max_reg = func.max_reg.max(self.next_reg);
        }
    }
    
    /// Push register onto simulated stack
    fn push(&mut self, reg: Reg) {
        self.stack.push(reg);
    }
    
    /// Pop register from simulated stack
    fn pop(&mut self) -> Option<Reg> {
        self.stack.pop()
    }
    
    /// Peek at top of simulated stack
    fn peek(&self) -> Option<Reg> {
        self.stack.last().copied()
    }
    
    /// Convert stack-based IR to register-based MIR
    pub fn translate_ir(&mut self, ir: &[IR]) -> Result<MIRModule, String> {
        // Start with main function
        self.start_function("main".to_string(), vec![], MIRType::Void);
        
        for instr in ir {
            self.translate_instruction(instr)?;
        }
        
        // Finish main function
        self.finish_function();
        
        Ok(self.module.clone())
    }
    
    fn translate_instruction(&mut self, instr: &IR) -> Result<(), String> {
        match instr {
            // Stack operations -> Register operations
            IR::PushInteger(i) => {
                let reg = self.alloc_reg();
                self.emit(MIR::LoadImm {
                    dest: reg,
                    value: MIRImmediate::Int(*i),
                    ty: MIRType::Int64,
                });
                self.push(reg);
            }
            
            IR::PushNumber(n) => {
                let reg = self.alloc_reg();
                self.emit(MIR::LoadImm {
                    dest: reg,
                    value: MIRImmediate::Float(*n),
                    ty: MIRType::Float64,
                });
                self.push(reg);
            }
            
            IR::PushBoolean(b) => {
                let reg = self.alloc_reg();
                self.emit(MIR::LoadImm {
                    dest: reg,
                    value: MIRImmediate::Bool(*b),
                    ty: MIRType::Bool,
                });
                self.push(reg);
            }
            
            IR::PushNull => {
                let reg = self.alloc_reg();
                self.emit(MIR::LoadImm {
                    dest: reg,
                    value: MIRImmediate::Null,
                    ty: MIRType::Ptr,
                });
                self.push(reg);
            }
            
            IR::Pop => {
                self.pop();
            }
            
            IR::Dup => {
                if let Some(top) = self.peek() {
                    let new_reg = self.alloc_reg();
                    self.emit(MIR::Move {
                        dest: new_reg,
                        src: top,
                        ty: MIRType::Int64, // Default type
                    });
                    self.push(new_reg);
                }
            }
            
            // Arithmetic operations
            IR::Add => {
                let right = self.pop().ok_or("Stack underflow")?;
                let left = self.pop().ok_or("Stack underflow")?;
                let dest = self.alloc_reg();
                // Default to integer addition (type inference would improve this)
                self.emit(MIR::AddInt { dest, left, right });
                self.push(dest);
            }
            
            IR::Subtract => {
                let right = self.pop().ok_or("Stack underflow")?;
                let left = self.pop().ok_or("Stack underflow")?;
                let dest = self.alloc_reg();
                self.emit(MIR::SubInt { dest, left, right });
                self.push(dest);
            }
            
            IR::Multiply => {
                let right = self.pop().ok_or("Stack underflow")?;
                let left = self.pop().ok_or("Stack underflow")?;
                let dest = self.alloc_reg();
                self.emit(MIR::MulInt { dest, left, right });
                self.push(dest);
            }
            
            IR::Divide => {
                let right = self.pop().ok_or("Stack underflow")?;
                let left = self.pop().ok_or("Stack underflow")?;
                let dest = self.alloc_reg();
                self.emit(MIR::DivInt { dest, left, right });
                self.push(dest);
            }
            
            IR::Modulo => {
                let right = self.pop().ok_or("Stack underflow")?;
                let left = self.pop().ok_or("Stack underflow")?;
                let dest = self.alloc_reg();
                self.emit(MIR::ModInt { dest, left, right });
                self.push(dest);
            }
            
            IR::Negate => {
                let src = self.pop().ok_or("Stack underflow")?;
                let dest = self.alloc_reg();
                self.emit(MIR::NegInt { dest, src });
                self.push(dest);
            }
            
            // Comparison operations
            IR::Equal => {
                let right = self.pop().ok_or("Stack underflow")?;
                let left = self.pop().ok_or("Stack underflow")?;
                let dest = self.alloc_reg();
                self.emit(MIR::CmpEqInt { dest, left, right });
                self.push(dest);
            }
            
            IR::LessThan => {
                let right = self.pop().ok_or("Stack underflow")?;
                let left = self.pop().ok_or("Stack underflow")?;
                let dest = self.alloc_reg();
                self.emit(MIR::CmpLtInt { dest, left, right });
                self.push(dest);
            }
            
            // Logical operations
            IR::And => {
                let right = self.pop().ok_or("Stack underflow")?;
                let left = self.pop().ok_or("Stack underflow")?;
                let dest = self.alloc_reg();
                self.emit(MIR::And { dest, left, right });
                self.push(dest);
            }
            
            IR::Or => {
                let right = self.pop().ok_or("Stack underflow")?;
                let left = self.pop().ok_or("Stack underflow")?;
                let dest = self.alloc_reg();
                self.emit(MIR::Or { dest, left, right });
                self.push(dest);
            }
            
            IR::Not => {
                let src = self.pop().ok_or("Stack underflow")?;
                let dest = self.alloc_reg();
                self.emit(MIR::Not { dest, src });
                self.push(dest);
            }
            
            // Variable operations
            IR::StoreVar(name) => {
                let src = self.pop().ok_or("Stack underflow")?;
                self.emit(MIR::StoreVar {
                    src,
                    name: name.clone(),
                    ty: MIRType::Int64, // Default type
                });
                self.variables.insert(name.clone(), (src, MIRType::Int64));
            }
            
            IR::LoadVar(name) => {
                let dest = self.alloc_reg();
                self.emit(MIR::LoadVar {
                    dest,
                    name: name.clone(),
                    ty: MIRType::Int64, // Default type
                });
                self.push(dest);
            }
            
            // Control flow
            IR::Label(name) => {
                let label = self.get_label(name);
                self.emit(MIR::Label(label));
            }
            
            IR::Jump(addr) => {
                let label = Label(*addr as u32);
                self.emit(MIR::Jump { target: label });
            }
            
            IR::JumpIfFalse(addr) => {
                let cond = self.pop().ok_or("Stack underflow")?;
                let label = Label(*addr as u32);
                self.emit(MIR::JumpIfZero { cond, target: label });
            }
            
            IR::JumpIfTrue(addr) => {
                let cond = self.pop().ok_or("Stack underflow")?;
                let label = Label(*addr as u32);
                self.emit(MIR::JumpIfNotZero { cond, target: label });
            }
            
            IR::Call(name, argc) => {
                let mut args = Vec::new();
                for _ in 0..*argc {
                    if let Some(arg) = self.pop() {
                        args.insert(0, arg);
                    }
                }
                let dest = self.alloc_reg();
                self.emit(MIR::Call {
                    dest: Some(dest),
                    func: name.clone(),
                    args,
                });
                self.push(dest);
            }
            
            IR::Return => {
                let value = self.pop();
                self.emit(MIR::Return { value });
            }
            
            IR::Print => {
                let value = self.pop().ok_or("Stack underflow")?;
                self.emit(MIR::Print { value });
            }
            
            // Other instructions - simplified for now
            _ => {
                // For unimplemented instructions, emit a debug print
                self.emit(MIR::DebugPrint {
                    msg: format!("Unimplemented IR: {:?}", instr),
                });
            }
        }
        
        Ok(())
    }
    
    fn start_function(&mut self, name: String, params: Vec<(String, MIRType)>, return_type: MIRType) {
        self.current_function = Some(MIRFunction::new(name, params, return_type));
        self.next_reg = 0;
        self.stack.clear();
        self.variables.clear();
    }
    
    fn finish_function(&mut self) {
        if let Some(func) = self.current_function.take() {
            self.module.add_function(func);
        }
    }
    
    pub fn build(self) -> MIRModule {
        self.module
    }
}

impl Default for MIRBuilder {
    fn default() -> Self {
        Self::new()
    }
}
