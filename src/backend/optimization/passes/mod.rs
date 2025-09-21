// src/backend/optimization/passes/mod.rs

pub mod dead_code_elimination;
pub mod constant_folding;
pub mod unused_variable_elimination;

pub use dead_code_elimination::DeadCodeElimination;
pub use constant_folding::ConstantFolding;
pub use unused_variable_elimination::UnusedVariableElimination;
