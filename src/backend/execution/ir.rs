// src/backend/execution/ir.rs
//! Professional stack-based IR system based on the proven old implementation

use std::fmt;

/// Intermediate representation for code generation
/// Exact implementation from the proven old compiler design
#[derive(Debug, Clone)]
pub enum IR {
    // Stack operations
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

    // Memory operations
    StoreVar(String),
    LoadVar(String),
    SetGlobal(String),  // Global variable operations

    // Arithmetic operations
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
    FloorDiv,
    Negate,

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

    // Control flow
    Jump(usize),
    JumpIfFalse(usize),
    JumpIfTrue(usize),
    Call(String, usize),  // function name, arg count
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
            IR::Add => write!(f, "ADD"),
            IR::Subtract => write!(f, "SUB"),
            IR::Multiply => write!(f, "MUL"),
            IR::Divide => write!(f, "DIV"),
            IR::Modulo => write!(f, "MOD"),
            IR::Power => write!(f, "POW"),
            IR::FloorDiv => write!(f, "FLOOR_DIV"),
            IR::Negate => write!(f, "NEG"),
            IR::Equal => write!(f, "EQ"),
            IR::NotEqual => write!(f, "NE"),
            IR::GreaterThan => write!(f, "GT"),
            IR::GreaterEqual => write!(f, "GE"),
            IR::LessThan => write!(f, "LT"),
            IR::LessEqual => write!(f, "LE"),
            IR::And => write!(f, "AND"),
            IR::Or => write!(f, "OR"),
            IR::Not => write!(f, "NOT"),
            IR::Jump(addr) => write!(f, "JUMP {}", addr),
            IR::JumpIfFalse(addr) => write!(f, "JIF {}", addr),
            IR::JumpIfTrue(addr) => write!(f, "JIT {}", addr),
            IR::Call(name, argc) => write!(f, "CALL {} {}", name, argc),
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
