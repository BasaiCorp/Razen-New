// Import modules from the functions directory
mod array;
mod string;
mod math;
mod time;
mod random;
mod file;
mod json;
mod net;
mod bolt;
mod seed;
mod color;
mod crypto;
mod regex;
mod uuid;
mod os;
mod validation;
mod system;
mod boxutil;
mod log;
mod ht;
mod audio;
mod image;
mod date;
mod filesystem;
mod api;

// New modules for self-compilation
mod memory;
mod binary;
mod bitwise;
mod syscall;
mod process;
mod thread;
mod compiler;

// Compiler construction modules
mod lexer;
mod parser;
mod ast;
mod symbol;
mod typesys;
mod ir;
mod codegen;
mod optimize;

// Re-export common utilities and types
use crate::value::Value;

// Library function modules for external use
pub mod arrlib {
    pub use super::array::*;
}

pub mod strlib {
    pub use super::string::*;
}

pub mod mathlib {
    pub use super::math::*;
}

pub mod timelib {
    pub use super::time::*;
}

pub mod randomlib {
    pub use super::random::*;
}

pub mod filelib {
    pub use super::file::*;
}

pub mod jsonlib {
    pub use super::json::*;
}

pub mod netlib {
    pub use super::net::*;
}

pub mod boltlib {
    pub use super::bolt::*;
}

pub mod seedlib {
    pub use super::seed::*;
}

pub mod colorlib {
    pub use super::color::*;
}

pub mod cryptolib {
    pub use super::crypto::*;
}

pub mod regexlib {
    pub use super::regex::*;
}

pub mod apilib {
    pub use super::api::*;
}

pub mod uuidlib {
    pub use super::uuid::*;
}

pub mod oslib {
    pub use super::os::*;
}

pub mod validationlib {
    pub use super::validation::*;
}

pub mod systemlib {
    pub use super::system::*;
}

pub mod boxutillib {
    pub use super::boxutil::*;
}

pub mod loglib {
    pub use super::log::*;
}

pub mod htlib {
    pub use super::ht::*;
}

pub mod audiolib {
    pub use super::audio::*;
}

pub mod imagelib {
    pub use super::image::*;
}

pub mod datelib {
    pub use super::date::*;
}

pub mod filesystemlib {
    pub use super::filesystem::*;
}

// New library modules for self-compilation
pub mod memorylib {
    pub use super::memory::*;
}

pub mod binarylib {
    pub use super::binary::*;
}

pub mod bitwiselib {
    pub use super::bitwise::*;
}

pub mod syscalllib {
    pub use super::syscall::*;
}

pub mod processlib {
    pub use super::process::*;
}

pub mod threadlib {
    pub use super::thread::*;
}

pub mod compilerlib {
    pub use super::compiler::*;
}

// Compiler construction libraries
pub mod lexerlib {
    pub use super::lexer::*;
}

pub mod parserlib {
    pub use super::parser::*;
}

pub mod astlib {
    pub use super::ast::*;
}

pub mod symbollib {
    pub use super::symbol::*;
}

pub mod typelib {
    pub use super::typesys::*;
}

pub mod irlib {
    pub use super::ir::*;
}

pub mod codegenlib {
    pub use super::codegen::*;
}

pub mod optimizelib {
    pub use super::optimize::*;
}