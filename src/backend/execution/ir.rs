// src/backend/execution/ir.rs
//! Professional stack-based IR system based on the proven old implementation

use std::fmt;

/// Intermediate representation for code generation
/// Exact implementation from the proven old compiler design
#[derive(Debug, Clone)]
pub enum IR {
    // Stack operations
    PushInteger(i64),
    PushNumber(f64),
    PushString(String),
    PushBoolean(bool),
    PushNull,
    Pop,
    Dup,
    Swap,

    // Exception handling
    SetupTryCatch,
    ClearTryCatch,
    ThrowException,

    // Memory operations (Stack-based)
    StoreVar(String),
    LoadVar(String),
    SetGlobal(String),  // Global variable operations
    
    // Register-based operations for RAIE optimization
    LoadReg(u8, String),    // Load variable into register: LoadReg(reg_id, var_name)
    StoreReg(u8, String),   // Store register to variable: StoreReg(reg_id, var_name)
    MoveReg(u8, u8),        // Move between registers: MoveReg(dest, src)
    LoadImmediate(u8, i64), // Load immediate value: LoadImmediate(reg_id, value)

    // Arithmetic operations (Stack-based)
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
    FloorDiv,
    Negate,
    
    // Register-based arithmetic operations for RAIE
    AddReg(u8, u8, u8),      // AddReg(dest, src1, src2)
    SubtractReg(u8, u8, u8), // SubtractReg(dest, src1, src2)
    MultiplyReg(u8, u8, u8), // MultiplyReg(dest, src1, src2)
    DivideReg(u8, u8, u8),   // DivideReg(dest, src1, src2)
    ModuloReg(u8, u8, u8),   // ModuloReg(dest, src1, src2)
    NegateReg(u8, u8),       // NegateReg(dest, src)

    // Comparison operations
    Equal,
    NotEqual,
    GreaterThan,
    GreaterEqual,
    LessThan,
    LessEqual,

    // Logical operations
    And,
    Or,
    Not,

    // Bitwise operations
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseNot,
    LeftShift,
    RightShift,

    // Control flow
    Jump(usize),
    JumpIfFalse(usize),
    JumpIfTrue(usize),
    Call(String, usize),  // function name, arg count
    MethodCall(String, usize),  // method name, arg count (including self)
    Return,

    // I/O operations
    Print,
    ReadInput,
    Exit,

    // Array operations
    CreateArray(usize),
    GetIndex,
    SetIndex,

    // Map operations
    CreateMap(usize),
    GetKey,
    SetKey,

    // Function definition
    DefineFunction(String, usize),  // function name, address

    // Labels for jumps
    Label(String),

    // Additional operations
    Sleep,
    LibraryCall(String, String, usize),  // library name, function name, arg count
}

impl fmt::Display for IR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IR::PushInteger(i) => write!(f, "PUSH_INT {}", i),
            IR::PushNumber(n) => write!(f, "PUSH_NUM {}", n),
            IR::PushString(s) => write!(f, "PUSH_STR \"{}\"", s),
            IR::PushBoolean(b) => write!(f, "PUSH_BOOL {}", b),
            IR::PushNull => write!(f, "PUSH_NULL"),
            IR::Pop => write!(f, "POP"),
            IR::Dup => write!(f, "DUP"),
            IR::Swap => write!(f, "SWAP"),
            IR::StoreVar(name) => write!(f, "STORE {}", name),
            IR::LoadVar(name) => write!(f, "LOAD {}", name),
            IR::SetGlobal(name) => write!(f, "SET_GLOBAL {}", name),
            
            // Register-based operations
            IR::LoadReg(reg, name) => write!(f, "LOAD_REG R{} {}", reg, name),
            IR::StoreReg(reg, name) => write!(f, "STORE_REG R{} {}", reg, name),
            IR::MoveReg(dest, src) => write!(f, "MOVE_REG R{} R{}", dest, src),
            IR::LoadImmediate(reg, val) => write!(f, "LOAD_IMM R{} {}", reg, val),
            IR::Add => write!(f, "ADD"),
            IR::Subtract => write!(f, "SUB"),
            IR::Multiply => write!(f, "MUL"),
            IR::Divide => write!(f, "DIV"),
            IR::Modulo => write!(f, "MOD"),
            IR::Power => write!(f, "POW"),
            IR::FloorDiv => write!(f, "FLOOR_DIV"),
            IR::Negate => write!(f, "NEG"),
            
            // Register-based arithmetic
            IR::AddReg(dest, src1, src2) => write!(f, "ADD_REG R{} R{} R{}", dest, src1, src2),
            IR::SubtractReg(dest, src1, src2) => write!(f, "SUB_REG R{} R{} R{}", dest, src1, src2),
            IR::MultiplyReg(dest, src1, src2) => write!(f, "MUL_REG R{} R{} R{}", dest, src1, src2),
            IR::DivideReg(dest, src1, src2) => write!(f, "DIV_REG R{} R{} R{}", dest, src1, src2),
            IR::ModuloReg(dest, src1, src2) => write!(f, "MOD_REG R{} R{} R{}", dest, src1, src2),
            IR::NegateReg(dest, src) => write!(f, "NEG_REG R{} R{}", dest, src),
            IR::Equal => write!(f, "EQ"),
            IR::NotEqual => write!(f, "NE"),
            IR::GreaterThan => write!(f, "GT"),
            IR::GreaterEqual => write!(f, "GE"),
            IR::LessThan => write!(f, "LT"),
            IR::LessEqual => write!(f, "LE"),
            IR::And => write!(f, "AND"),
            IR::Or => write!(f, "OR"),
            IR::Not => write!(f, "NOT"),
            IR::BitwiseAnd => write!(f, "BIT_AND"),
            IR::BitwiseOr => write!(f, "BIT_OR"),
            IR::BitwiseXor => write!(f, "BIT_XOR"),
            IR::BitwiseNot => write!(f, "BIT_NOT"),
            IR::LeftShift => write!(f, "LSHIFT"),
            IR::RightShift => write!(f, "RSHIFT"),
            IR::Jump(addr) => write!(f, "JUMP {}", addr),
            IR::JumpIfFalse(addr) => write!(f, "JIF {}", addr),
            IR::JumpIfTrue(addr) => write!(f, "JIT {}", addr),
            IR::Call(name, argc) => write!(f, "CALL {} {}", name, argc),
            IR::MethodCall(name, argc) => write!(f, "METHOD_CALL {} {}", name, argc),
            IR::Return => write!(f, "RET"),
            IR::Print => write!(f, "PRINT"),
            IR::ReadInput => write!(f, "READ"),
            IR::Exit => write!(f, "EXIT"),
            IR::CreateArray(size) => write!(f, "ARRAY {}", size),
            IR::GetIndex => write!(f, "GET_IDX"),
            IR::SetIndex => write!(f, "SET_IDX"),
            IR::CreateMap(size) => write!(f, "MAP {}", size),
            IR::GetKey => write!(f, "GET_KEY"),
            IR::SetKey => write!(f, "SET_KEY"),
            IR::DefineFunction(name, addr) => write!(f, "DEF_FN {} {}", name, addr),
            IR::Label(name) => write!(f, "{}:", name),
            IR::Sleep => write!(f, "SLEEP"),
            IR::LibraryCall(lib, func, argc) => write!(f, "LIB_CALL {}.{} {}", lib, func, argc),
            IR::SetupTryCatch => write!(f, "TRY_SETUP"),
            IR::ClearTryCatch => write!(f, "TRY_CLEAR"),
            IR::ThrowException => write!(f, "THROW"),
        }
    }
}
