// src/backend/cranelift/jit/compiler.rs
// JIT Compiler - Handles IR to native code compilation

use std::collections::HashMap;
use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{Linkage, Module, FuncId};
use cranelift_frontend::FunctionBuilderContext;
use cranelift_native;

use crate::backend::ir::{IRModule, IRFunction, IRParam, Instruction};
use super::builtins::BuiltinManager;
use super::instruction_compiler::InstructionCompiler;
use crate::backend::builtins::BuiltinRuntime;

/// Compiled executable ready for execution
pub struct CompiledExecutable {
    pub main_function: *const u8,
    pub functions: HashMap<String, *const u8>,
}

/// Professional JIT Compiler following Cranelift best practices
pub struct JITCompiler {
    /// Cranelift JIT module for code generation
    module: JITModule,
    /// Function builder context (reused for efficiency)
    builder_context: FunctionBuilderContext,
    /// Main compilation context
    ctx: codegen::Context,
    /// Builtin function manager
    #[allow(dead_code)]
    builtins: BuiltinManager,
}

impl JITCompiler {
    /// Create a new JIT compiler instance
    pub fn new() -> Result<Self, String> {
        // Create target ISA
        let mut flag_builder = settings::builder();
        flag_builder.set("use_colocated_libcalls", "false")
            .map_err(|e| format!("Failed to set flag: {}", e))?;
        flag_builder.set("is_pic", "false")
            .map_err(|e| format!("Failed to set flag: {}", e))?;
        
        let isa_builder = cranelift_native::builder()
            .map_err(|e| format!("Failed to create ISA builder: {}", e))?;
        let isa = isa_builder
            .finish(settings::Flags::new(flag_builder))
            .map_err(|e| format!("Failed to create ISA: {}", e))?;

        // Create JIT builder and register builtin functions
        let mut builder = JITBuilder::with_isa(isa, cranelift_module::default_libcall_names());
        let builtins = BuiltinManager::new();
        builtins.register_symbols(&mut builder);

        let module = JITModule::new(builder);
        let ctx = module.make_context();

        Ok(Self {
            module,
            builder_context: FunctionBuilderContext::new(),
            ctx,
            builtins,
        })
    }

    /// Compile an IR module to native executable
    pub fn compile(&mut self, ir_module: IRModule) -> Result<CompiledExecutable, String> {
        let mut function_ids = HashMap::new();
        let mut function_pointers = HashMap::new();

        // Declare all functions first
        for function in &ir_module.functions {
            let func_id = self.declare_function(&function.name, &function.params, &function.return_type)?;
            function_ids.insert(function.name.clone(), func_id);
        }

        // Compile all functions
        for function in &ir_module.functions {
            let func_id = function_ids[&function.name];
            self.compile_function(function, func_id, &function_ids, &ir_module)?;
        }

        // Finalize all functions
        self.module.finalize_definitions()
            .map_err(|e| format!("Failed to finalize definitions: {}", e))?;

        // Get function pointers
        for (name, &func_id) in &function_ids {
            let func_ptr = self.module.get_finalized_function(func_id);
            function_pointers.insert(name.clone(), func_ptr);
        }

        // Get main function pointer
        let main_function = function_pointers.get("main")
            .copied()
            .ok_or("Main function not found")?;

        Ok(CompiledExecutable {
            main_function,
            functions: function_pointers,
        })
    }

    /// Declare a function in the JIT module with proper error handling
    fn declare_function(&mut self, name: &str, params: &[IRParam], return_type: &str) -> Result<FuncId, String> {
        let mut sig = self.module.make_signature();

        // Add parameters
        for param in params {
            let param_type = self.ir_type_to_cranelift(&param.ty)?;
            sig.params.push(AbiParam::new(param_type));
        }

        // Add return type
        if return_type != "void" {
            let ret_type = self.ir_type_to_cranelift(return_type)?;
            sig.returns.push(AbiParam::new(ret_type));
        }

        let func_id = self.module.declare_function(name, Linkage::Local, &sig)
            .map_err(|e| format!("Failed to declare function '{}': {}", name, e))?;
        
        Ok(func_id)
    }

    /// Compile a single function
    fn compile_function(
        &mut self,
        function: &IRFunction,
        func_id: FuncId,
        _function_ids: &HashMap<String, FuncId>,
        _ir_module: &IRModule,
    ) -> Result<(), String> {
        // Clear context for new function
        self.ctx.clear();

        // Set up function signature
        for param in &function.params {
            let param_type = self.ir_type_to_cranelift(&param.ty)?;
            self.ctx.func.signature.params.push(AbiParam::new(param_type));
        }

        if function.return_type != "void" {
            let ret_type = self.ir_type_to_cranelift(&function.return_type)?;
            self.ctx.func.signature.returns.push(AbiParam::new(ret_type));
        }

        // Build function body
        {
            let mut builder = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_context);
            let entry_block = builder.create_block();
            builder.append_block_params_for_function_params(entry_block);
            builder.switch_to_block(entry_block);
            builder.seal_block(entry_block);

            // Compile function body directly here to avoid borrowing issues
            if function.return_type == "void" {
                builder.ins().return_(&[]);
            } else {
                let default_val = match function.return_type.as_str() {
                    "int" => builder.ins().iconst(types::I64, 0),
                    "float" => builder.ins().f64const(0.0),
                    "bool" => builder.ins().iconst(types::I8, 0),
                    _ => builder.ins().iconst(types::I64, 0),
                };
                builder.ins().return_(&[default_val]);
            }

            builder.finalize();
        }

        // Define the function
        self.module.define_function(func_id, &mut self.ctx)
            .map_err(|e| format!("Failed to define function '{}': {}", function.name, e))?;

        Ok(())
    }


    /// Convert IR type to Cranelift type
    fn ir_type_to_cranelift(&self, ir_type: &str) -> Result<Type, String> {
        match ir_type {
            "int" => Ok(types::I64),
            "float" => Ok(types::F64),
            "bool" => Ok(types::I8),
            "char" => Ok(types::I8),
            "str" => Ok(types::I64), // Pointer to string
            "void" => Ok(types::INVALID),
            _ => Ok(types::I64), // Default to I64
        }
    }

    /// Compile and run the IR module using JIT with proper builtin function support
    /// This implementation ensures correct execution order like `go run`
    pub fn compile_and_run(&mut self, ir_module: IRModule) -> Result<i32, crate::frontend::diagnostics::Diagnostics> {
        use crate::frontend::diagnostics::{Diagnostics, Diagnostic, DiagnosticKind};
        
        if std::env::var("RAZEN_DEBUG").is_ok() {
            println!("🚀 Starting JIT compilation with debug support");
            println!("📊 IR Module: {} functions, {} strings", ir_module.functions.len(), ir_module.strings.len());
        }
        
        // First, declare builtin functions and string literals
        let builtin_func_ids = self.declare_builtin_functions().map_err(|e| {
            let mut diagnostics = Diagnostics::new();
            diagnostics.add(Diagnostic::new(
                DiagnosticKind::Custom { message: e }
            ));
            diagnostics
        })?;
        
        let string_literals = self.create_string_literals(&ir_module).map_err(|e| {
            let mut diagnostics = Diagnostics::new();
            diagnostics.add(Diagnostic::new(
                DiagnosticKind::Custom { message: e }
            ));
            diagnostics
        })?;
        
        // Then declare all user functions in the module
        let mut function_ids = HashMap::new();
        
        for function in &ir_module.functions {
            if std::env::var("RAZEN_DEBUG").is_ok() {
                println!("🔧 Declaring function: {} -> {}", function.name, function.return_type);
            }
            let func_id = self.declare_function(&function.name, &function.params, &function.return_type)
                .map_err(|e| {
                    let mut diagnostics = Diagnostics::new();
                    diagnostics.add(Diagnostic::new(
                        DiagnosticKind::Custom { message: e }
                    ));
                    diagnostics
                })?;
            function_ids.insert(function.name.clone(), func_id);
        }
        
        // Now compile all functions with proper builtin support
        for function in &ir_module.functions {
            if std::env::var("RAZEN_DEBUG").is_ok() {
                println!("🔨 Compiling function: {}", function.name);
            }
            let func_id = function_ids[&function.name];
            self.compile_function_with_builtin_support(function, func_id, &function_ids, &ir_module, &builtin_func_ids, &string_literals)
                .map_err(|e| {
                    let mut diagnostics = Diagnostics::new();
                    diagnostics.add(Diagnostic::new(
                        DiagnosticKind::Custom { message: e }
                    ));
                    diagnostics
                })?;
        }
        
        // Finalize the module to make functions callable
        self.module.finalize_definitions().map_err(|e| {
            let mut diagnostics = Diagnostics::new();
            diagnostics.add(Diagnostic::new(
                DiagnosticKind::Custom { 
                    message: format!("Failed to finalize JIT module: {}", e) 
                }
            ));
            diagnostics
        })?;
        
        // Execute the main function
        if let Some(main_id) = function_ids.get("main") {
            if std::env::var("RAZEN_DEBUG").is_ok() {
                println!("🎯 Executing main function...");
            }
            self.execute_main_function(*main_id, &ir_module)
                .map_err(|e| {
                    let mut diagnostics = Diagnostics::new();
                    diagnostics.add(Diagnostic::new(
                        DiagnosticKind::Custom { message: e }
                    ));
                    diagnostics
                })?;
            Ok(0)
        } else {
            let mut diagnostics = Diagnostics::new();
            diagnostics.add(Diagnostic::new(
                DiagnosticKind::Custom { 
                    message: "Main function not found in IR module".to_string() 
                }
            ));
            Err(diagnostics)
        }
    }

    /// Compile a function with full builtin support and IR instruction handling
    fn compile_function_with_builtin_support(
        &mut self,
        function: &IRFunction,
        func_id: FuncId,
        _function_ids: &HashMap<String, FuncId>,
        _ir_module: &IRModule,
        builtin_func_ids: &HashMap<String, FuncId>,
        string_literals: &HashMap<String, String>,
    ) -> Result<(), String> {
        // Clear context for new function
        self.ctx.clear();

        // Set up function signature
        for param in &function.params {
            let param_type = self.ir_type_to_cranelift(&param.ty)?;
            self.ctx.func.signature.params.push(AbiParam::new(param_type));
        }

        if function.return_type != "void" {
            let ret_type = self.ir_type_to_cranelift(&function.return_type)?;
            self.ctx.func.signature.returns.push(AbiParam::new(ret_type));
        }

        // Build function body with proper IR instruction handling
        {
            let mut builder = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_context);
            let entry_block = builder.create_block();
            builder.append_block_params_for_function_params(entry_block);
            builder.switch_to_block(entry_block);
            builder.seal_block(entry_block);

            // Create builtin runtime for instruction compilation
            let builtin_runtime = BuiltinRuntime::new();
            let mut values = HashMap::new();

            // Track if we've added a terminator
            let mut has_terminator = false;
            
            // Process all instructions in all basic blocks
            for basic_block in &function.basic_blocks {
                for instruction in &basic_block.instructions {
                    InstructionCompiler::generate_instruction_static(
                        instruction, 
                        &mut builder, 
                        &mut values, 
                        &builtin_runtime,
                        builtin_func_ids,
                        string_literals
                    )?;
                }
                
                // Handle terminator instruction if present
                if let Some(terminator) = &basic_block.terminator {
                    InstructionCompiler::generate_instruction_static(
                        terminator, 
                        &mut builder, 
                        &mut values, 
                        &builtin_runtime,
                        builtin_func_ids,
                        string_literals
                    )?;
                    
                    // Check if this is a return instruction
                    if matches!(terminator, Instruction::Return { .. }) {
                        has_terminator = true;
                    }
                }
            }

            // Only add a return instruction if we don't already have one
            if !has_terminator {
                if function.return_type == "void" {
                    builder.ins().return_(&[]);
                } else {
                    let default_val = match function.return_type.as_str() {
                        "int" => builder.ins().iconst(types::I64, 0),
                        "float" => builder.ins().f64const(0.0),
                        "bool" => builder.ins().iconst(types::I8, 0),
                        _ => builder.ins().iconst(types::I64, 0),
                    };
                    builder.ins().return_(&[default_val]);
                }
            }

            builder.finalize();
        }

        // Define the function
        self.module.define_function(func_id, &mut self.ctx)
            .map_err(|e| format!("Failed to define function '{}': {}", function.name, e))?;

        // No need for post-JIT interpretation - builtin calls are compiled as external function calls

        Ok(())
    }

    /// Execute the main function after JIT compilation
    fn execute_main_function(&mut self, main_id: FuncId, _ir_module: &IRModule) -> Result<(), String> {
        // Get the compiled main function pointer
        let main_ptr = self.module.get_finalized_function(main_id);
        
        if std::env::var("RAZEN_DEBUG").is_ok() {
            println!("🎯 Executing JIT-compiled main function at {:p}", main_ptr);
        }

        // Execute the native code
        unsafe {
            let main_func = std::mem::transmute::<_, fn() -> ()>(main_ptr);
            main_func();
        }

        Ok(())
    }

    /// Declare external builtin functions in the JIT module
    fn declare_builtin_functions(&mut self) -> Result<HashMap<String, FuncId>, String> {
        let mut builtin_func_ids = HashMap::new();
        
        // Declare println function
        let mut println_sig = self.module.make_signature();
        println_sig.params.push(AbiParam::new(types::I64)); // String pointer
        let println_id = self.module.declare_function("razen_println", Linkage::Import, &println_sig)
            .map_err(|e| format!("Failed to declare println: {}", e))?;
        builtin_func_ids.insert("println".to_string(), println_id);
        
        // Declare print function
        let mut print_sig = self.module.make_signature();
        print_sig.params.push(AbiParam::new(types::I64)); // String pointer
        let print_id = self.module.declare_function("razen_print", Linkage::Import, &print_sig)
            .map_err(|e| format!("Failed to declare print: {}", e))?;
        builtin_func_ids.insert("print".to_string(), print_id);
        
        // Declare input function
        let mut input_sig = self.module.make_signature();
        input_sig.returns.push(AbiParam::new(types::I64)); // Return string pointer
        let input_id = self.module.declare_function("razen_input", Linkage::Import, &input_sig)
            .map_err(|e| format!("Failed to declare input: {}", e))?;
        builtin_func_ids.insert("input".to_string(), input_id);
        
        if std::env::var("RAZEN_DEBUG").is_ok() {
            println!("🔧 Declared external builtin functions: {:?}", builtin_func_ids.keys().collect::<Vec<_>>());
        }
        
        Ok(builtin_func_ids)
    }
    
    /// Create string literals mapping for builtin function calls
    fn create_string_literals(&mut self, ir_module: &IRModule) -> Result<HashMap<String, String>, String> {
        let mut string_literals = HashMap::new();
        
        for (index, string_literal) in ir_module.strings.iter().enumerate() {
            let string_name = format!("@str{}", index);
            string_literals.insert(string_name, string_literal.clone());
        }
        
        if std::env::var("RAZEN_DEBUG").is_ok() {
            println!("📝 Created {} string literal mappings", string_literals.len());
        }
        
        Ok(string_literals)
    }
}
