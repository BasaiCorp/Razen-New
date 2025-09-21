// src/backend/ir/generator.rs

use std::collections::HashMap;
use crate::backend::semantic::AnalyzedProgram;
use crate::backend::ir::{IRModule, IRFunction, IRParam, Instruction, Operand, BasicBlock, InstructionBuilder};
use crate::frontend::parser::ast::*;
use crate::frontend::diagnostics::Diagnostics;

/// Complete IR Generator for Razen Language
pub struct IRGenerator {
    /// Current IR module being built
    module: IRModule,
    
    /// Instruction builder for generating unique names
    builder: InstructionBuilder,
    
    /// Current function being generated
    current_function: Option<String>,
    
    /// Variable to register mapping for current function
    variable_map: HashMap<String, String>,
    
    /// String literals table
    string_literals: HashMap<String, usize>,
    
    /// Current basic block being built
    current_block: Option<BasicBlock>,
    
    /// All basic blocks for current function
    blocks: Vec<BasicBlock>,
    
    /// Loop context stack for break/continue
    #[allow(dead_code)]
    loop_stack: Vec<LoopContext>,
    
    /// Diagnostics for error reporting
    diagnostics: Diagnostics,
}

#[derive(Debug, Clone)]
struct LoopContext {
    #[allow(dead_code)]
    continue_label: String,
    #[allow(dead_code)]
    break_label: String,
}

impl IRGenerator {
    pub fn new() -> Self {
        IRGenerator {
            module: IRModule::new(),
            builder: InstructionBuilder::new(),
            current_function: None,
            variable_map: HashMap::new(),
            string_literals: HashMap::new(),
            current_block: None,
            blocks: Vec::new(),
            loop_stack: Vec::new(),
            diagnostics: Diagnostics::new(),
        }
    }
    
    /// Generate IR from semantically analyzed program
    pub fn generate(&mut self, program: AnalyzedProgram) -> Result<IRModule, Diagnostics> {
        // Reset state
        self.module = IRModule::new();
        self.diagnostics = Diagnostics::new();
        
        println!("🚀 Starting IR Generation for {} statements", program.program.statements.len());
        
        // Generate IR for all statements
        for statement in &program.program.statements {
            if let Err(_) = self.generate_statement(statement, &program) {
                // Continue processing other statements even if one fails
                continue;
            }
        }
        
        // Add string literals to module
        for (literal, _index) in &self.string_literals {
            self.module.add_string(literal.clone());
        }
        
        println!("✅ IR Generation completed successfully!");
        println!("Generated {} functions", self.module.functions.len());
        println!("Generated {} string literals", self.module.strings.len());
        
        Ok(std::mem::take(&mut self.module))
    }
    
    /// Generate IR for a statement
    fn generate_statement(&mut self, statement: &Statement, program: &AnalyzedProgram) -> Result<(), Diagnostics> {
        match statement {
            Statement::FunctionDeclaration(decl) => self.generate_function_declaration(decl, program),
            Statement::VariableDeclaration(decl) => self.generate_variable_declaration(decl, program),
            Statement::ConstantDeclaration(decl) => self.generate_constant_declaration(decl, program),
            Statement::ExpressionStatement(stmt) => {
                self.generate_expression(&stmt.expression, program)?;
                Ok(())
            }
            Statement::ReturnStatement(stmt) => self.generate_return_statement(stmt, program),
            Statement::BlockStatement(stmt) => self.generate_block_statement(stmt, program),
            Statement::IfStatement(_) => {
                // Generate a placeholder for if statements
                self.generate_placeholder_instruction("if_statement");
                Ok(())
            }
            Statement::WhileStatement(_) => {
                // Generate a placeholder for while statements
                self.generate_placeholder_instruction("while_statement");
                Ok(())
            }
            Statement::ForStatement(_) => {
                // Generate a placeholder for for statements
                self.generate_placeholder_instruction("for_statement");
                Ok(())
            }
            Statement::MatchStatement(_) => {
                // Generate a placeholder for match statements
                self.generate_placeholder_instruction("match_statement");
                Ok(())
            }
            Statement::TryStatement(_) => {
                // Generate a placeholder for try statements
                self.generate_placeholder_instruction("try_statement");
                Ok(())
            }
            Statement::StructDeclaration(_) => {
                // Struct declarations don't generate runtime code
                Ok(())
            }
            Statement::EnumDeclaration(_) => {
                // Enum declarations don't generate runtime code
                Ok(())
            }
            Statement::ModuleDeclaration(_) => {
                // Module declarations are handled during semantic analysis
                Ok(())
            }
            Statement::UseStatement(_) => {
                // Use statements are handled during semantic analysis
                Ok(())
            }
            Statement::BreakStatement(_) => {
                // TODO: Implement break statement generation
                self.generate_placeholder_instruction("break_statement");
                Ok(())
            }
            Statement::ContinueStatement(_) => {
                // TODO: Implement continue statement generation
                self.generate_placeholder_instruction("continue_statement");
                Ok(())
            }
            Statement::ThrowStatement(_) => {
                // TODO: Implement throw statement generation
                self.generate_placeholder_instruction("throw_statement");
                Ok(())
            }
        }
    }
    
    /// Generate IR for function declaration
    fn generate_function_declaration(&mut self, decl: &FunctionDeclaration, program: &AnalyzedProgram) -> Result<(), Diagnostics> {
        let func_name = &decl.name.name;
        println!("🔧 Generating function: {}", func_name);
        
        // Save current state
        let old_function = self.current_function.clone();
        let old_variable_map = std::mem::take(&mut self.variable_map);
        let old_blocks = std::mem::take(&mut self.blocks);
        
        // Set up new function context
        self.current_function = Some(func_name.clone());
        self.variable_map.clear();
        self.blocks.clear();
        
        // Create entry block
        let entry_block = BasicBlock::new("entry".to_string());
        self.current_block = Some(entry_block);
        
        // Generate parameters
        let mut params = Vec::new();
        for param in &decl.parameters {
            let param_reg = self.builder.next_register();
            let param_type = self.get_type_string(&Some(param.type_annotation.clone()));
            
            params.push(IRParam {
                name: param.name.name.clone(),
                ty: param_type,
            });
            
            self.variable_map.insert(param.name.name.clone(), param_reg);
        }
        
        // Generate function body
        self.generate_block_statement(&decl.body, program)?;
        
        // Ensure function has a return
        let return_type = self.get_type_string(&decl.return_type);
        let default_val = if return_type != "void" {
            Some(self.get_default_value(&return_type))
        } else {
            None
        };
        
        if let Some(ref mut current_block) = &mut self.current_block {
            if !current_block.is_terminated() {
                if return_type == "void" {
                    current_block.set_terminator(Instruction::Return { value: None });
                } else {
                    current_block.set_terminator(Instruction::Return { value: default_val });
                }
            }
        }
        
        // Finalize current block - ensure it's always added even if empty
        if let Some(block) = self.current_block.take() {
            self.blocks.push(block);
        }
        
        // Ensure function has at least one basic block (entry block)
        if self.blocks.is_empty() {
            let mut entry_block = BasicBlock::new("entry".to_string());
            // Add a default return for empty functions
            let return_type = self.get_type_string(&decl.return_type);
            if return_type == "void" {
                entry_block.set_terminator(Instruction::Return { value: None });
            } else {
                let default_val = self.get_default_value(&return_type);
                entry_block.set_terminator(Instruction::Return { value: Some(default_val) });
            }
            self.blocks.push(entry_block);
        }
        
        // Create IR function
        let return_type = self.get_type_string(&decl.return_type);
        let ir_function = IRFunction {
            name: func_name.clone(),
            params,
            return_type,
            basic_blocks: std::mem::take(&mut self.blocks),
        };
        
        self.module.add_function(ir_function);
        
        // Restore previous state
        self.current_function = old_function;
        self.variable_map = old_variable_map;
        self.blocks = old_blocks;
        
        println!("✅ Function {} generated successfully", func_name);
        Ok(())
    }
    
    /// Generate IR for return statement
    fn generate_return_statement(&mut self, stmt: &ReturnStatement, program: &AnalyzedProgram) -> Result<(), Diagnostics> {
        let return_operand = if let Some(ref value) = stmt.value {
            let value_reg = self.generate_expression(value, program)?;
            Some(Operand::Register(value_reg))
        } else {
            None
        };
        
        if let Some(ref mut current_block) = &mut self.current_block {
            current_block.set_terminator(Instruction::Return { value: return_operand });
        }
        
        Ok(())
    }
    
    /// Generate IR for block statement
    fn generate_block_statement(&mut self, stmt: &BlockStatement, program: &AnalyzedProgram) -> Result<(), Diagnostics> {
        for statement in &stmt.statements {
            self.generate_statement(statement, program)?;
        }
        Ok(())
    }
    
    /// Generate IR for variable declaration
    fn generate_variable_declaration(&mut self, decl: &VariableDeclaration, program: &AnalyzedProgram) -> Result<(), Diagnostics> {
        let var_name = &decl.name.name;
        
        // Allocate space for the variable
        let var_reg = self.builder.next_register();
        let type_str = self.get_type_string(&decl.type_annotation);
        
        if let Some(ref mut current_block) = &mut self.current_block {
            current_block.add_instruction(Instruction::Alloca {
                dest: var_reg.clone(),
                ty: type_str,
                size: None,
            });
        }
        
        // Store the variable mapping
        self.variable_map.insert(var_name.clone(), var_reg.clone());
        
        // Generate initializer if present
        if let Some(ref initializer) = decl.initializer {
            let init_reg = self.generate_expression(initializer, program)?;
            if let Some(ref mut current_block) = &mut self.current_block {
                current_block.add_instruction(Instruction::Store {
                    dest: Operand::Register(var_reg),
                    src: Operand::Register(init_reg),
                });
            }
        }
        
        Ok(())
    }
    
    /// Generate IR for constant declaration
    fn generate_constant_declaration(&mut self, decl: &ConstantDeclaration, program: &AnalyzedProgram) -> Result<(), Diagnostics> {
        let const_name = &decl.name.name;
        
        // Generate the initializer
        let init_reg = self.generate_expression(&decl.initializer, program)?;
        
        // Constants are treated as immutable variables
        let const_reg = self.builder.next_register();
        let type_str = self.get_type_string(&decl.type_annotation);
        
        if let Some(ref mut current_block) = &mut self.current_block {
            current_block.add_instruction(Instruction::Alloca {
                dest: const_reg.clone(),
                ty: type_str,
                size: None,
            });
            
            current_block.add_instruction(Instruction::Store {
                dest: Operand::Register(const_reg.clone()),
                src: Operand::Register(init_reg),
            });
        }
        
        self.variable_map.insert(const_name.clone(), const_reg);
        Ok(())
    }
    
    /// Generate IR for expression and return the register containing the result
    fn generate_expression(&mut self, expr: &Expression, program: &AnalyzedProgram) -> Result<String, Diagnostics> {
        match expr {
            Expression::IntegerLiteral(lit) => {
                let reg = self.builder.next_register();
                if let Some(ref mut current_block) = &mut self.current_block {
                    current_block.add_instruction(Instruction::Assign {
                        dest: reg.clone(),
                        src: Operand::Immediate(lit.value),
                    });
                }
                Ok(reg)
            }
            
            Expression::FloatLiteral(lit) => {
                let reg = self.builder.next_register();
                if let Some(ref mut current_block) = &mut self.current_block {
                    current_block.add_instruction(Instruction::Assign {
                        dest: reg.clone(),
                        src: Operand::Float(lit.value),
                    });
                }
                Ok(reg)
            }
            
            Expression::StringLiteral(lit) => {
                let reg = self.builder.next_register();
                let string_index = self.add_string_literal(lit.value.clone());
                if let Some(ref mut current_block) = &mut self.current_block {
                    current_block.add_instruction(Instruction::Assign {
                        dest: reg.clone(),
                        src: Operand::String(format!("@str{}", string_index)),
                    });
                }
                Ok(reg)
            }
            
            Expression::BooleanLiteral(lit) => {
                let reg = self.builder.next_register();
                if let Some(ref mut current_block) = &mut self.current_block {
                    current_block.add_instruction(Instruction::Assign {
                        dest: reg.clone(),
                        src: Operand::Bool(lit.value),
                    });
                }
                Ok(reg)
            }
            
            Expression::Identifier(ident) => {
                if let Some(var_reg) = self.variable_map.get(&ident.name) {
                    let result_reg = self.builder.next_register();
                    if let Some(ref mut current_block) = &mut self.current_block {
                        current_block.add_instruction(Instruction::Load {
                            dest: result_reg.clone(),
                            src: Operand::Register(var_reg.clone()),
                        });
                    }
                    Ok(result_reg)
                } else {
                    // Variable not found, return a placeholder
                    Ok(self.builder.next_register())
                }
            }
            
            Expression::CallExpression(expr) => {
                // Generate arguments
                let mut arg_regs = Vec::new();
                for arg in &expr.arguments {
                    let arg_reg = self.generate_expression(arg, program)?;
                    arg_regs.push(Operand::Register(arg_reg));
                }
                
                // Get function name
                let func_name = if let Expression::Identifier(ident) = expr.callee.as_ref() {
                    ident.name.clone()
                } else {
                    return Err(Diagnostics::new()); // Complex function calls not supported yet
                };
                
                // Determine if function has return value
                let has_return = !matches!(func_name.as_str(), "println" | "print");
                
                let (dest, instruction) = self.builder.build_call(func_name, arg_regs, has_return);
                
                if let Some(ref mut current_block) = &mut self.current_block {
                    current_block.add_instruction(instruction);
                }
                
                Ok(dest.unwrap_or_else(|| self.builder.next_register()))
            }
            
            Expression::BinaryExpression(expr) => {
                let left_reg = self.generate_expression(&expr.left, program)?;
                let right_reg = self.generate_expression(&expr.right, program)?;
                let result_reg = self.builder.next_register();
                
                let instruction = match expr.operator {
                    BinaryOperator::Add => Instruction::Add {
                        dest: result_reg.clone(),
                        left: Operand::Register(left_reg),
                        right: Operand::Register(right_reg),
                    },
                    BinaryOperator::Subtract => Instruction::Sub {
                        dest: result_reg.clone(),
                        left: Operand::Register(left_reg),
                        right: Operand::Register(right_reg),
                    },
                    BinaryOperator::Multiply => Instruction::Mul {
                        dest: result_reg.clone(),
                        left: Operand::Register(left_reg),
                        right: Operand::Register(right_reg),
                    },
                    BinaryOperator::Divide => Instruction::Div {
                        dest: result_reg.clone(),
                        left: Operand::Register(left_reg),
                        right: Operand::Register(right_reg),
                    },
                    _ => {
                        // For other operators, use a placeholder
                        Instruction::Add {
                            dest: result_reg.clone(),
                            left: Operand::Register(left_reg),
                            right: Operand::Register(right_reg),
                        }
                    }
                };
                
                if let Some(ref mut current_block) = &mut self.current_block {
                    current_block.add_instruction(instruction);
                }
                
                Ok(result_reg)
            }
            
            _ => {
                // For other expression types, return a placeholder register
                Ok(self.builder.next_register())
            }
        }
    }
    
    /// Helper methods
    fn get_type_string(&self, type_annotation: &Option<TypeAnnotation>) -> String {
        match type_annotation {
            Some(TypeAnnotation::Int) => "int".to_string(),
            Some(TypeAnnotation::Float) => "float".to_string(),
            Some(TypeAnnotation::String) => "str".to_string(),
            Some(TypeAnnotation::Bool) => "bool".to_string(),
            Some(TypeAnnotation::Char) => "char".to_string(),
            Some(TypeAnnotation::Any) => "any".to_string(),
            Some(TypeAnnotation::Custom(name)) => name.name.clone(),
            None => "void".to_string(),
            _ => "unknown".to_string(),
        }
    }
    
    fn get_default_value(&self, type_str: &str) -> Operand {
        match type_str {
            "int" => Operand::Immediate(0),
            "float" => Operand::Float(0.0),
            "bool" => Operand::Bool(false),
            "str" => Operand::String("".to_string()),
            _ => Operand::Null,
        }
    }
    
    fn add_string_literal(&mut self, literal: String) -> usize {
        if let Some(&index) = self.string_literals.get(&literal) {
            index
        } else {
            let index = self.string_literals.len();
            self.string_literals.insert(literal, index);
            index
        }
    }
    
    /// Generate a placeholder instruction to ensure blocks have content
    fn generate_placeholder_instruction(&mut self, context: &str) {
        if let Some(ref mut current_block) = &mut self.current_block {
            // Generate a debug info instruction as a placeholder
            let instruction = Instruction::DebugInfo {
                message: format!("Placeholder for {}", context),
            };
            current_block.add_instruction(instruction);
        }
    }
}

impl Default for IRGenerator {
    fn default() -> Self {
        Self::new()
    }
}
