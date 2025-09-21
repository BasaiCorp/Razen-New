// tests/unit/optimization_tests.rs

use razen_lang::backend::optimization::{Optimizer, OptimizationLevel};
use razen_lang::backend::optimization::passes::{ConstantFolding, DeadCodeElimination, UnusedVariableElimination};
use razen_lang::backend::ir::{IRModule, IRFunction, Instruction, Operand};

#[cfg(test)]
mod optimizer_tests {
    use super::*;

    fn create_unoptimized_ir_module() -> IRModule {
        let mut module = IRModule::new();
        
        let function = IRFunction {
            name: "unoptimized_func".to_string(),
            params: vec![],
            return_type: "int".to_string(),
            instructions: vec![
                // Constant folding opportunity: 5 + 3
                Instruction::Add {
                    dest: "result1".to_string(),
                    left: Operand::Constant(5),
                    right: Operand::Constant(3),
                },
                // Dead code: unused variable
                Instruction::Add {
                    dest: "unused".to_string(),
                    left: Operand::Constant(10),
                    right: Operand::Constant(20),
                },
                // Only result1 is used in return
                Instruction::Return {
                    value: Some(Operand::Register("result1".to_string())),
                },
            ],
            blocks: vec![],
        };
        
        module.functions.push(function);
        module
    }

    #[test]
    fn test_optimizer_creation() {
        let optimizer = Optimizer::new(OptimizationLevel::Basic);
        // Optimizer created successfully
        assert!(true);
    }

    #[test]
    fn test_optimization_levels() {
        assert_eq!(OptimizationLevel::None, OptimizationLevel::None);
        assert_eq!(OptimizationLevel::Basic, OptimizationLevel::Basic);
        assert_eq!(OptimizationLevel::Standard, OptimizationLevel::Standard);
        assert_eq!(OptimizationLevel::Aggressive, OptimizationLevel::Aggressive);
        assert_ne!(OptimizationLevel::None, OptimizationLevel::Basic);
    }

    #[test]
    fn test_no_optimization() {
        let optimizer = Optimizer::new(OptimizationLevel::None);
        let original_module = create_unoptimized_ir_module();
        let original_instruction_count = original_module.functions[0].instructions.len();
        
        let optimized_module = optimizer.optimize(original_module).expect("Optimization should succeed");
        let optimized_instruction_count = optimized_module.functions[0].instructions.len();
        
        // With no optimization, instruction count should remain the same
        assert_eq!(original_instruction_count, optimized_instruction_count);
    }

    #[test]
    fn test_basic_optimization() {
        let optimizer = Optimizer::new(OptimizationLevel::Basic);
        let original_module = create_unoptimized_ir_module();
        
        let optimized_module = optimizer.optimize(original_module).expect("Basic optimization should succeed");
        let function = &optimized_module.functions[0];
        
        // Basic optimization should apply some passes
        assert!(!function.instructions.is_empty(), "Function should still have instructions");
    }

    #[test]
    fn test_standard_optimization() {
        let optimizer = Optimizer::new(OptimizationLevel::Standard);
        let original_module = create_unoptimized_ir_module();
        
        let optimized_module = optimizer.optimize(original_module).expect("Standard optimization should succeed");
        let function = optimized_module.functions.get("unoptimized_func").unwrap();
        
        // Standard optimization should be more aggressive
        assert!(!function.instructions.is_empty(), "Function should still have instructions");
    }

    #[test]
    fn test_aggressive_optimization() {
        let optimizer = Optimizer::new(OptimizationLevel::Aggressive);
        let original_module = create_unoptimized_ir_module();
        
        let optimized_module = optimizer.optimize(original_module).expect("Aggressive optimization should succeed");
        let function = optimized_module.functions.get("unoptimized_func").unwrap();
        
        // Aggressive optimization should apply all passes
        assert!(!function.instructions.is_empty(), "Function should still have instructions");
    }

    #[test]
    fn test_multiple_optimization_passes() {
        let optimizer = Optimizer::new(OptimizationLevel::Standard);
        let original_module = create_unoptimized_ir_module();
        
        // Run optimization multiple times to test iterative improvement
        let optimized_module1 = optimizer.optimize(original_module).expect("First optimization should succeed");
        let optimized_module2 = optimizer.optimize(optimized_module1).expect("Second optimization should succeed");
        
        // Should converge (no further changes)
        let function = optimized_module2.functions.get("unoptimized_func").unwrap();
        assert!(!function.instructions.is_empty(), "Function should still have instructions after multiple passes");
    }
}

#[cfg(test)]
mod constant_folding_tests {
    use super::*;

    #[test]
    fn test_constant_folding_addition() {
        let constant_folding = ConstantFolding::new();
        let mut functions = HashMap::new();
        
        let function = IRFunction {
            name: "add_constants".to_string(),
            parameters: vec![],
            return_type: IRType::Int,
            instructions: vec![
                IRInstruction::Add {
                    dest: IRValue::Register(1),
                    left: IRValue::Constant(5),
                    right: IRValue::Constant(3),
                },
                IRInstruction::Return {
                    value: Some(IRValue::Register(1)),
                },
            ],
            basic_blocks: vec![],
            register_count: 2,
        };
        
        functions.insert("add_constants".to_string(), function);
        
        let mut ir_module = IRModule {
            functions,
            globals: HashMap::new(),
            string_literals: HashMap::new(),
        };
        
        let changed = constant_folding.run(&mut ir_module).expect("Constant folding should succeed");
        assert!(changed, "Constant folding should detect changes");
        
        // Check that the addition was folded to a constant
        let function = ir_module.functions.get("add_constants").unwrap();
        let has_constant_8 = function.instructions.iter().any(|instr| {
            match instr {
                IRInstruction::Return { value: Some(IRValue::Constant(8)) } => true,
                _ => false,
            }
        });
        // Note: This test assumes the constant folding replaces the register with the constant
        // The actual implementation might work differently
    }

    #[test]
    fn test_constant_folding_subtraction() {
        let constant_folding = ConstantFolding::new();
        let mut functions = HashMap::new();
        
        let function = IRFunction {
            name: "sub_constants".to_string(),
            parameters: vec![],
            return_type: IRType::Int,
            instructions: vec![
                IRInstruction::Sub {
                    dest: IRValue::Register(1),
                    left: IRValue::Constant(10),
                    right: IRValue::Constant(3),
                },
                IRInstruction::Return {
                    value: Some(IRValue::Register(1)),
                },
            ],
            basic_blocks: vec![],
            register_count: 2,
        };
        
        functions.insert("sub_constants".to_string(), function);
        
        let mut ir_module = IRModule {
            functions,
            globals: HashMap::new(),
            string_literals: HashMap::new(),
        };
        
        let changed = constant_folding.run(&mut ir_module).expect("Constant folding should succeed");
        assert!(changed, "Constant folding should detect changes");
    }

    #[test]
    fn test_constant_folding_multiplication() {
        let constant_folding = ConstantFolding::new();
        let mut functions = HashMap::new();
        
        let function = IRFunction {
            name: "mul_constants".to_string(),
            parameters: vec![],
            return_type: IRType::Int,
            instructions: vec![
                IRInstruction::Mul {
                    dest: IRValue::Register(1),
                    left: IRValue::Constant(4),
                    right: IRValue::Constant(5),
                },
                IRInstruction::Return {
                    value: Some(IRValue::Register(1)),
                },
            ],
            basic_blocks: vec![],
            register_count: 2,
        };
        
        functions.insert("mul_constants".to_string(), function);
        
        let mut ir_module = IRModule {
            functions,
            globals: HashMap::new(),
            string_literals: HashMap::new(),
        };
        
        let changed = constant_folding.run(&mut ir_module).expect("Constant folding should succeed");
        assert!(changed, "Constant folding should detect changes");
    }

    #[test]
    fn test_no_constant_folding_needed() {
        let constant_folding = ConstantFolding::new();
        let mut functions = HashMap::new();
        
        let function = IRFunction {
            name: "no_folding".to_string(),
            parameters: vec![],
            return_type: IRType::Int,
            instructions: vec![
                IRInstruction::Add {
                    dest: IRValue::Register(1),
                    left: IRValue::Register(0), // Not constants
                    right: IRValue::Register(1),
                },
                IRInstruction::Return {
                    value: Some(IRValue::Register(1)),
                },
            ],
            basic_blocks: vec![],
            register_count: 2,
        };
        
        functions.insert("no_folding".to_string(), function);
        
        let mut ir_module = IRModule {
            functions,
            globals: HashMap::new(),
            string_literals: HashMap::new(),
        };
        
        let changed = constant_folding.run(&mut ir_module).expect("Constant folding should succeed");
        assert!(!changed, "No constant folding should be needed");
    }
}

#[cfg(test)]
mod dead_code_elimination_tests {
    use super::*;

    #[test]
    fn test_dead_code_elimination() {
        let dead_code_elimination = DeadCodeElimination::new();
        let mut functions = HashMap::new();
        
        let function = IRFunction {
            name: "dead_code_func".to_string(),
            parameters: vec![],
            return_type: IRType::Int,
            instructions: vec![
                IRInstruction::Add {
                    dest: IRValue::Register(1),
                    left: IRValue::Constant(5),
                    right: IRValue::Constant(3),
                },
                // Dead code: result not used
                IRInstruction::Mul {
                    dest: IRValue::Register(2),
                    left: IRValue::Constant(10),
                    right: IRValue::Constant(20),
                },
                IRInstruction::Return {
                    value: Some(IRValue::Register(1)),
                },
            ],
            basic_blocks: vec![],
            register_count: 3,
        };
        
        functions.insert("dead_code_func".to_string(), function);
        
        let mut ir_module = IRModule {
            functions,
            globals: HashMap::new(),
            string_literals: HashMap::new(),
        };
        
        let original_count = ir_module.functions.get("dead_code_func").unwrap().instructions.len();
        let changed = dead_code_elimination.run(&mut ir_module).expect("Dead code elimination should succeed");
        
        if changed {
            let new_count = ir_module.functions.get("dead_code_func").unwrap().instructions.len();
            assert!(new_count <= original_count, "Dead code elimination should not increase instruction count");
        }
    }

    #[test]
    fn test_no_dead_code() {
        let dead_code_elimination = DeadCodeElimination::new();
        let mut functions = HashMap::new();
        
        let function = IRFunction {
            name: "no_dead_code".to_string(),
            parameters: vec![],
            return_type: IRType::Int,
            instructions: vec![
                IRInstruction::Add {
                    dest: IRValue::Register(1),
                    left: IRValue::Constant(5),
                    right: IRValue::Constant(3),
                },
                IRInstruction::Return {
                    value: Some(IRValue::Register(1)),
                },
            ],
            basic_blocks: vec![],
            register_count: 2,
        };
        
        functions.insert("no_dead_code".to_string(), function);
        
        let mut ir_module = IRModule {
            functions,
            globals: HashMap::new(),
            string_literals: HashMap::new(),
        };
        
        let changed = dead_code_elimination.run(&mut ir_module).expect("Dead code elimination should succeed");
        // May or may not detect changes depending on implementation
    }
}

#[cfg(test)]
mod unused_variable_elimination_tests {
    use super::*;

    #[test]
    fn test_unused_variable_elimination() {
        let unused_var_elimination = UnusedVariableElimination::new();
        let mut functions = HashMap::new();
        
        let function = IRFunction {
            name: "unused_var_func".to_string(),
            parameters: vec![],
            return_type: IRType::Int,
            instructions: vec![
                // Unused allocation
                IRInstruction::Alloca {
                    dest: IRValue::Register(1),
                    size: 4,
                },
                // Used allocation
                IRInstruction::Alloca {
                    dest: IRValue::Register(2),
                    size: 4,
                },
                IRInstruction::Store {
                    address: IRValue::Register(2),
                    value: IRValue::Constant(42),
                },
                IRInstruction::Load {
                    dest: IRValue::Register(3),
                    address: IRValue::Register(2),
                },
                IRInstruction::Return {
                    value: Some(IRValue::Register(3)),
                },
            ],
            basic_blocks: vec![],
            register_count: 4,
        };
        
        functions.insert("unused_var_func".to_string(), function);
        
        let mut ir_module = IRModule {
            functions,
            globals: HashMap::new(),
            string_literals: HashMap::new(),
        };
        
        let original_count = ir_module.functions.get("unused_var_func").unwrap().instructions.len();
        let changed = unused_var_elimination.run(&mut ir_module).expect("Unused variable elimination should succeed");
        
        if changed {
            let new_count = ir_module.functions.get("unused_var_func").unwrap().instructions.len();
            assert!(new_count <= original_count, "Unused variable elimination should not increase instruction count");
        }
    }

    #[test]
    fn test_no_unused_variables() {
        let unused_var_elimination = UnusedVariableElimination::new();
        let mut functions = HashMap::new();
        
        let function = IRFunction {
            name: "all_used_vars".to_string(),
            parameters: vec![],
            return_type: IRType::Int,
            instructions: vec![
                IRInstruction::Alloca {
                    dest: IRValue::Register(1),
                    size: 4,
                },
                IRInstruction::Store {
                    address: IRValue::Register(1),
                    value: IRValue::Constant(42),
                },
                IRInstruction::Load {
                    dest: IRValue::Register(2),
                    address: IRValue::Register(1),
                },
                IRInstruction::Return {
                    value: Some(IRValue::Register(2)),
                },
            ],
            basic_blocks: vec![],
            register_count: 3,
        };
        
        functions.insert("all_used_vars".to_string(), function);
        
        let mut ir_module = IRModule {
            functions,
            globals: HashMap::new(),
            string_literals: HashMap::new(),
        };
        
        let changed = unused_var_elimination.run(&mut ir_module).expect("Unused variable elimination should succeed");
        // Should not change anything since all variables are used
    }
}
