use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

use crate::value::Value;
use crate::functions::apilib;

/// LibraryFunction represents a callable function in a library
pub type LibraryFunction = fn(Vec<Value>) -> Result<Value, String>;

/// Library represents a collection of functions
#[derive(Clone)]
pub struct Library {
    name: String,
    functions: HashMap<String, LibraryFunction>,
}

impl Library {
    /// Create a new library with the given name
    pub fn new(name: &str) -> Self {
        Library {
            name: name.to_string(),
            functions: HashMap::new(),
        }
    }

    /// Register a function in the library
    pub fn register_function(&mut self, name: &str, function: LibraryFunction) {
        self.functions.insert(name.to_string(), function);
    }

    /// Call a function in the library
    pub fn call_function(&self, function_name: &str, args: Vec<Value>) -> Result<Value, String> {
        match self.functions.get(function_name) {
            Some(function) => function(args),
            None => Err(format!("Function '{}' not found in library '{}'", function_name, self.name)),
        }
    }

    /// Get the name of the library
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Check if a function exists in the library
    pub fn has_function(&self, function_name: &str) -> bool {
        self.functions.contains_key(function_name)
    }

    /// Get all function names in the library
    pub fn function_names(&self) -> Vec<String> {
        self.functions.keys().cloned().collect()
    }
}

impl fmt::Debug for Library {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Library {{ name: {}, functions: {:?} }}", self.name, self.function_names())
    }
}

/// LibraryManager manages all libraries in the system
#[derive(Debug, Clone)]
pub struct LibraryManager {
    libraries: HashMap<String, Library>,
}

impl LibraryManager {
    /// Create a new library manager
    pub fn new() -> Self {
        LibraryManager {
            libraries: HashMap::new(),
        }
    }

    /// Register a library
    pub fn register_library(&mut self, library: Library) {
        let name = library.name().to_string();
        self.libraries.insert(name.to_lowercase(), library);
    }

    /// Get a library by name (case-insensitive)
    pub fn get_library(&self, name: &str) -> Option<&Library> {
        self.libraries.get(&name.to_lowercase())
    }

    /// Call a library function
    pub fn call_library(&self, library_name: &str, function_name: &str, args: Vec<Value>) -> Result<Value, String> {
        // Handle case-insensitive library names
        let library_name = library_name.to_lowercase();
        
        // Support both PascalCase and lowercase for library names
        match self.libraries.get(&library_name) {
            Some(library) => library.call_function(function_name, args),
            None => Err(format!("Library '{}' not found", library_name)),
        }
    }

    /// Initialize all standard libraries
    pub fn initialize_standard_libraries(&mut self) {
        self.register_standard_libraries();
    }

    /// Register all standard libraries
    fn register_standard_libraries(&mut self) {
        // Array library
        let mut arr_lib = Library::new("arrlib");
        arr_lib.register_function("push", crate::functions::arrlib::push);
        arr_lib.register_function("pop", crate::functions::arrlib::pop);
        arr_lib.register_function("join", crate::functions::arrlib::join);
        arr_lib.register_function("length", crate::functions::arrlib::length);
        // These functions are not implemented yet, so we'll comment them out for now
        // arr_lib.register_function("map", crate::functions::arrlib::map);
        // arr_lib.register_function("filter", crate::functions::arrlib::filter);
        arr_lib.register_function("unique", crate::functions::arrlib::unique);
        self.register_library(arr_lib);

        // String library
        let mut str_lib = Library::new("strlib");
        str_lib.register_function("upper", crate::functions::strlib::upper);
        str_lib.register_function("lower", crate::functions::strlib::lower);
        str_lib.register_function("substring", crate::functions::strlib::substring);
        str_lib.register_function("replace", crate::functions::strlib::replace);
        str_lib.register_function("length", crate::functions::strlib::length);
        str_lib.register_function("split", crate::functions::strlib::split);
        str_lib.register_function("trim", crate::functions::strlib::trim);
        str_lib.register_function("starts_with", crate::functions::strlib::starts_with);
        str_lib.register_function("ends_with", crate::functions::strlib::ends_with);
        str_lib.register_function("contains", crate::functions::strlib::contains);
        str_lib.register_function("repeat", crate::functions::strlib::repeat);
        self.register_library(str_lib);

        // Math library
        let mut math_lib = Library::new("mathlib");
        math_lib.register_function("add", crate::functions::mathlib::add);
        math_lib.register_function("subtract", crate::functions::mathlib::subtract);
        math_lib.register_function("multiply", crate::functions::mathlib::multiply);
        math_lib.register_function("divide", crate::functions::mathlib::divide);
        math_lib.register_function("power", crate::functions::mathlib::power);
        math_lib.register_function("sqrt", crate::functions::mathlib::sqrt);
        math_lib.register_function("abs", crate::functions::mathlib::abs);
        math_lib.register_function("round", crate::functions::mathlib::round);
        math_lib.register_function("floor", crate::functions::mathlib::floor);
        math_lib.register_function("ceil", crate::functions::mathlib::ceil);
        math_lib.register_function("sin", crate::functions::mathlib::sin);
        math_lib.register_function("cos", crate::functions::mathlib::cos);
        math_lib.register_function("tan", crate::functions::mathlib::tan);
        math_lib.register_function("log", crate::functions::mathlib::log);
        math_lib.register_function("exp", crate::functions::mathlib::exp);
        math_lib.register_function("random", crate::functions::mathlib::random);
        math_lib.register_function("max", crate::functions::mathlib::max);
        math_lib.register_function("min", crate::functions::mathlib::min);
        math_lib.register_function("modulo", crate::functions::mathlib::modulo);
        self.register_library(math_lib);

        // Time library
        let mut time_lib = Library::new("timelib");
        time_lib.register_function("now", crate::functions::timelib::now);
        time_lib.register_function("format", crate::functions::timelib::format);
        time_lib.register_function("parse", crate::functions::timelib::parse);
        time_lib.register_function("add", crate::functions::timelib::add);
        time_lib.register_function("year", crate::functions::timelib::year);
        time_lib.register_function("month", crate::functions::timelib::month);
        time_lib.register_function("day", crate::functions::timelib::day);
        self.register_library(time_lib);

        // Random library
        let mut random_lib = Library::new("random");
        random_lib.register_function("int", crate::functions::randomlib::int);
        random_lib.register_function("float", crate::functions::randomlib::float);
        random_lib.register_function("choice", crate::functions::randomlib::choice);
        random_lib.register_function("shuffle", crate::functions::randomlib::shuffle);
        self.register_library(random_lib);

        // File library (legacy)
        let mut file_lib = Library::new("file");
        file_lib.register_function("read", crate::functions::filelib::read);
        file_lib.register_function("write", crate::functions::filelib::write);
        file_lib.register_function("append", crate::functions::filelib::append);
        file_lib.register_function("exists", crate::functions::filelib::exists);
        file_lib.register_function("delete", crate::functions::filelib::delete);
        self.register_library(file_lib);

        // Filesystem library (extended)
        let mut fs_lib = Library::new("filesystem");
        fs_lib.register_function("exists", crate::functions::filesystemlib::exists);
        fs_lib.register_function("is_file", crate::functions::filesystemlib::is_file);
        fs_lib.register_function("is_dir", crate::functions::filesystemlib::is_dir);
        fs_lib.register_function("create_dir", crate::functions::filesystemlib::create_dir);
        fs_lib.register_function("remove", crate::functions::filesystemlib::remove);
        fs_lib.register_function("read_file", crate::functions::filesystemlib::read_file);
        fs_lib.register_function("write_file", crate::functions::filesystemlib::write_file);
        fs_lib.register_function("list_dir", crate::functions::filesystemlib::list_dir);
        fs_lib.register_function("metadata", crate::functions::filesystemlib::metadata);
        fs_lib.register_function("absolute_path", crate::functions::filesystemlib::absolute_path);
        fs_lib.register_function("copy", crate::functions::filesystemlib::copy_file);
        fs_lib.register_function("move", crate::functions::filesystemlib::move_file);
        fs_lib.register_function("extension", crate::functions::filesystemlib::extension);
        fs_lib.register_function("file_stem", crate::functions::filesystemlib::file_stem);
        fs_lib.register_function("parent_dir", crate::functions::filesystemlib::parent_dir);
        fs_lib.register_function("join_path", crate::functions::filesystemlib::join_path);
        fs_lib.register_function("change_dir", crate::functions::filesystemlib::change_dir);
        fs_lib.register_function("current_dir", crate::functions::filesystemlib::current_dir);
        fs_lib.register_function("temp_file", crate::functions::filesystemlib::temp_file);
        fs_lib.register_function("temp_dir", crate::functions::filesystemlib::temp_dir);
        self.register_library(fs_lib);
        
        // API library
        let mut api_lib = Library::new("apilib");
        api_lib.register_function("get", crate::functions::apilib::get);
        api_lib.register_function("post", crate::functions::apilib::post);
        api_lib.register_function("putmethod", crate::functions::apilib::putmethod);
        api_lib.register_function("delete", crate::functions::apilib::delete);
        api_lib.register_function("patch", crate::functions::apilib::patch);
        api_lib.register_function("call", crate::functions::apilib::call);
        api_lib.register_function("parse_json", crate::functions::apilib::parse_json);
        api_lib.register_function("to_json", crate::functions::apilib::to_json);
        api_lib.register_function("create_api", crate::functions::apilib::create_api);
        api_lib.register_function("execute_api", crate::functions::apilib::execute_api);
        api_lib.register_function("url_encode", crate::functions::apilib::url_encode);
        api_lib.register_function("url_decode", crate::functions::apilib::url_decode);
        api_lib.register_function("form_data", crate::functions::apilib::form_data);
        api_lib.register_function("is_success", crate::functions::apilib::is_success);
        api_lib.register_function("is_client_error", crate::functions::apilib::is_client_error);
        api_lib.register_function("is_server_error", crate::functions::apilib::is_server_error);
        self.register_library(api_lib);

        // JSON library
        let mut json_lib = Library::new("json");
        json_lib.register_function("parse", crate::functions::jsonlib::parse);
        json_lib.register_function("stringify", crate::functions::jsonlib::stringify);
        self.register_library(json_lib);

        // Bolt library
        let mut bolt_lib = Library::new("bolt");
        bolt_lib.register_function("run", crate::functions::boltlib::run);
        bolt_lib.register_function("parallel", crate::functions::boltlib::parallel);
        bolt_lib.register_function("threads", crate::functions::boltlib::threads);
        self.register_library(bolt_lib);

        // Seed library
        let mut seed_lib = Library::new("seed");
        seed_lib.register_function("generate", crate::functions::seedlib::generate);
        seed_lib.register_function("map_seed", crate::functions::seedlib::map_seed);
        seed_lib.register_function("noise_map", crate::functions::seedlib::noise_map);
        seed_lib.register_function("name", crate::functions::seedlib::name);
        self.register_library(seed_lib);

        // Memory library for memory management operations
        let mut memory_lib = Library::new("memorylib");
        memory_lib.register_function("addressof", crate::functions::memorylib::addressof);
        memory_lib.register_function("deref", crate::functions::memorylib::deref);
        memory_lib.register_function("add_offset", crate::functions::memorylib::add_offset);
        memory_lib.register_function("alloc", crate::functions::memorylib::alloc);
        memory_lib.register_function("free", crate::functions::memorylib::free);
        memory_lib.register_function("write_byte", crate::functions::memorylib::write_byte);
        memory_lib.register_function("read_byte", crate::functions::memorylib::read_byte);
        memory_lib.register_function("create_buffer", crate::functions::memorylib::create_buffer);
        memory_lib.register_function("free_buffer", crate::functions::memorylib::free_buffer);
        memory_lib.register_function("buffer_write_string", crate::functions::memorylib::buffer_write_string);
        memory_lib.register_function("buffer_read_string", crate::functions::memorylib::buffer_read_string);
        memory_lib.register_function("buffer_copy", crate::functions::memorylib::buffer_copy);
        memory_lib.register_function("stats", crate::functions::memorylib::stats);
        self.register_library(memory_lib);

        // Binary library for binary file operations
        let mut binary_lib = Library::new("binarylib");
        binary_lib.register_function("create", crate::functions::binarylib::create);
        binary_lib.register_function("open", crate::functions::binarylib::open);
        binary_lib.register_function("close", crate::functions::binarylib::close);
        binary_lib.register_function("write_bytes", crate::functions::binarylib::write_bytes);
        binary_lib.register_function("read_bytes", crate::functions::binarylib::read_bytes);
        binary_lib.register_function("seek", crate::functions::binarylib::seek);
        binary_lib.register_function("tell", crate::functions::binarylib::tell);
        binary_lib.register_function("bytes_to_string", crate::functions::binarylib::bytes_to_string);
        binary_lib.register_function("string_to_bytes", crate::functions::binarylib::string_to_bytes);
        binary_lib.register_function("stats", crate::functions::binarylib::stats);
        self.register_library(binary_lib);

        // Bitwise library for bit manipulation
        let mut bitwise_lib = Library::new("bitwiselib");
        bitwise_lib.register_function("and", crate::functions::bitwiselib::and);
        bitwise_lib.register_function("or", crate::functions::bitwiselib::or);
        bitwise_lib.register_function("xor", crate::functions::bitwiselib::xor);
        bitwise_lib.register_function("not", crate::functions::bitwiselib::not);
        bitwise_lib.register_function("left_shift", crate::functions::bitwiselib::left_shift);
        bitwise_lib.register_function("right_shift", crate::functions::bitwiselib::right_shift);
        bitwise_lib.register_function("unsigned_right_shift", crate::functions::bitwiselib::unsigned_right_shift);
        bitwise_lib.register_function("get_bit", crate::functions::bitwiselib::get_bit);
        bitwise_lib.register_function("set_bit", crate::functions::bitwiselib::set_bit);
        bitwise_lib.register_function("count_bits", crate::functions::bitwiselib::count_bits);
        bitwise_lib.register_function("to_binary", crate::functions::bitwiselib::to_binary);
        bitwise_lib.register_function("to_hex", crate::functions::bitwiselib::to_hex);
        bitwise_lib.register_function("from_binary", crate::functions::bitwiselib::from_binary);
        bitwise_lib.register_function("from_hex", crate::functions::bitwiselib::from_hex);
        self.register_library(bitwise_lib);

        // Syscall library for system operations
        let mut syscall_lib = Library::new("systemlib");
        syscall_lib.register_function("getpid", crate::functions::syscalllib::getpid);
        syscall_lib.register_function("getcwd", crate::functions::syscalllib::getcwd);
        syscall_lib.register_function("execute", crate::functions::syscalllib::execute);
        syscall_lib.register_function("getenv", crate::functions::syscalllib::getenv);
        syscall_lib.register_function("setenv", crate::functions::syscalllib::setenv);
        syscall_lib.register_function("environ", crate::functions::syscalllib::environ);
        syscall_lib.register_function("args", crate::functions::syscalllib::args);
        syscall_lib.register_function("path_exists", crate::functions::syscalllib::path_exists);
        syscall_lib.register_function("realpath", crate::functions::syscalllib::realpath);
        syscall_lib.register_function("exit", crate::functions::syscalllib::exit);
        syscall_lib.register_function("sleep", crate::functions::syscalllib::sleep);
        syscall_lib.register_function("hostname", crate::functions::syscalllib::hostname);
        syscall_lib.register_function("username", crate::functions::syscalllib::username);
        syscall_lib.register_function("current_time", crate::functions::systemlib::current_time);
        syscall_lib.register_function("system_name", crate::functions::systemlib::system_name);
        self.register_library(syscall_lib);

        // Process library for process management
        let mut process_lib = Library::new("processlib");
        process_lib.register_function("create", crate::functions::processlib::create);
        process_lib.register_function("wait", crate::functions::processlib::wait);
        process_lib.register_function("is_running", crate::functions::processlib::is_running);
        process_lib.register_function("kill", crate::functions::processlib::kill);
        process_lib.register_function("signal", crate::functions::processlib::signal);
        process_lib.register_function("info", crate::functions::processlib::info);
        process_lib.register_function("read_stdout", crate::functions::processlib::read_stdout);
        process_lib.register_function("read_stderr", crate::functions::processlib::read_stderr);
        process_lib.register_function("write_stdin", crate::functions::processlib::write_stdin);
        self.register_library(process_lib);

        // Thread library for threading operations
        let mut thread_lib = Library::new("threadlib");
        thread_lib.register_function("create", crate::functions::threadlib::create);
        thread_lib.register_function("join", crate::functions::threadlib::join);
        thread_lib.register_function("is_running", crate::functions::threadlib::is_running);
        thread_lib.register_function("sleep", crate::functions::threadlib::sleep);
        thread_lib.register_function("mutex_create", crate::functions::threadlib::mutex_create);
        thread_lib.register_function("mutex_lock", crate::functions::threadlib::mutex_lock);
        thread_lib.register_function("mutex_unlock", crate::functions::threadlib::mutex_unlock);
        thread_lib.register_function("mutex_destroy", crate::functions::threadlib::mutex_destroy);
        thread_lib.register_function("current", crate::functions::threadlib::current);
        thread_lib.register_function("cpu_count", crate::functions::threadlib::cpu_count);
        thread_lib.register_function("thread_id", crate::functions::threadlib::thread_id);
        thread_lib.register_function("thread_count", crate::functions::threadlib::thread_count);
        self.register_library(thread_lib);

        // Compiler library for compiler operations
        let mut compiler_lib = Library::new("compilerlib");
        compiler_lib.register_function("create_node", crate::functions::compilerlib::create_node);
        compiler_lib.register_function("add_child", crate::functions::compilerlib::add_child);
        compiler_lib.register_function("node_to_string", crate::functions::compilerlib::node_to_string);
        compiler_lib.register_function("create_symbol_table", crate::functions::compilerlib::create_symbol_table);
        compiler_lib.register_function("add_symbol", crate::functions::compilerlib::add_symbol);
        compiler_lib.register_function("lookup_symbol", crate::functions::compilerlib::lookup_symbol);
        compiler_lib.register_function("generate_ir", crate::functions::compilerlib::generate_ir);
        compiler_lib.register_function("optimize_ir", crate::functions::compilerlib::optimize_ir);
        compiler_lib.register_function("generate_assembly", crate::functions::compilerlib::generate_assembly);
        compiler_lib.register_function("parse", crate::functions::compilerlib::parse);
        compiler_lib.register_function("tokenize", crate::functions::compilerlib::tokenize);
        compiler_lib.register_function("compile", crate::functions::compilerlib::compile);
        self.register_library(compiler_lib);

        // Register Lexer library functions
        let mut lexer_lib = Library::new("lexerlib");
        lexer_lib.register_function("create_lexer", crate::functions::lexerlib::create_lexer);
        lexer_lib.register_function("tokenize", crate::functions::lexerlib::tokenize);
        lexer_lib.register_function("define_token", crate::functions::lexerlib::define_token);
        self.register_library(lexer_lib);

        // Register Parser library functions
        let mut parser_lib = Library::new("parserlib");
        parser_lib.register_function("create_parser", crate::functions::parserlib::create_parser);
        parser_lib.register_function("parse", crate::functions::parserlib::parse);
        parser_lib.register_function("define_rule", crate::functions::parserlib::define_rule);
        parser_lib.register_function("create_grammar", crate::functions::parserlib::create_grammar);
        self.register_library(parser_lib);

        // Register AST library functions
        let mut ast_lib = Library::new("astlib");
        ast_lib.register_function("create_node", crate::functions::astlib::create_node);
        ast_lib.register_function("define_node_type", crate::functions::astlib::define_node_type);
        ast_lib.register_function("traverse", crate::functions::astlib::traverse);
        ast_lib.register_function("create_visitor", crate::functions::astlib::create_visitor);
        self.register_library(ast_lib);

        // Register Symbol library functions
        let mut symbol_lib = Library::new("symbollib");
        symbol_lib.register_function("create_symbol_table", crate::functions::symbollib::create_symbol_table);
        symbol_lib.register_function("define_symbol", crate::functions::symbollib::define_symbol);
        symbol_lib.register_function("add_symbol", crate::functions::symbollib::add_symbol);
        symbol_lib.register_function("lookup_symbol", crate::functions::symbollib::lookup_symbol);
        self.register_library(symbol_lib);

        // Register Type library functions
        let mut type_lib = Library::new("typelib");
        type_lib.register_function("define_type", crate::functions::typelib::define_type);
        type_lib.register_function("check_type", crate::functions::typelib::check_type);
        type_lib.register_function("create_type_system", crate::functions::typelib::create_type_system);
        type_lib.register_function("infer_type", crate::functions::typelib::infer_type);
        self.register_library(type_lib);

        // Register IR library functions
        let mut ir_lib = Library::new("irlib");
        ir_lib.register_function("create_instruction", crate::functions::irlib::create_instruction);
        ir_lib.register_function("generate", crate::functions::irlib::generate);
        ir_lib.register_function("optimize", crate::functions::irlib::optimize);
        ir_lib.register_function("to_string", crate::functions::irlib::to_string);
        self.register_library(ir_lib);

        // Register CodeGen library functions
        let mut codegen_lib = Library::new("codegenlib");
        codegen_lib.register_function("create_generator", crate::functions::codegenlib::create_generator);
        codegen_lib.register_function("generate", crate::functions::codegenlib::generate);
        codegen_lib.register_function("define_target", crate::functions::codegenlib::define_target);
        codegen_lib.register_function("emit_code", crate::functions::codegenlib::emit_code);
        self.register_library(codegen_lib);

        // Register Optimize library functions
        let mut optimize_lib = Library::new("optimizelib");
        optimize_lib.register_function("create_pass", crate::functions::optimizelib::create_pass);
        optimize_lib.register_function("apply", crate::functions::optimizelib::apply);
        optimize_lib.register_function("analyze", crate::functions::optimizelib::analyze);
        optimize_lib.register_function("create_pipeline", crate::functions::optimizelib::create_pipeline);
        self.register_library(optimize_lib);

        // Register Color library functions
        let mut color_lib = Library::new("color");
        color_lib.register_function("hex_to_rgb", crate::functions::colorlib::hex_to_rgb);
        color_lib.register_function("rgb_to_hex", crate::functions::colorlib::rgb_to_hex);
        color_lib.register_function("lighten", crate::functions::colorlib::lighten);
        color_lib.register_function("darken", crate::functions::colorlib::darken);
        color_lib.register_function("get_ansi_color", crate::functions::colorlib::get_ansi_color);
        self.register_library(color_lib);

        // Register crypto library functions
        let mut crypto_lib = Library::new("crypto");
        crypto_lib.register_function("hash", crate::functions::cryptolib::hash);
        crypto_lib.register_function("encrypt", crate::functions::cryptolib::encrypt);
        crypto_lib.register_function("decrypt", crate::functions::cryptolib::decrypt);
        self.register_library(crypto_lib);

        // Register regex library functions
        let mut regex_lib = Library::new("regex");
        regex_lib.register_function("match", crate::functions::regexlib::match_pattern);
        regex_lib.register_function("search", crate::functions::regexlib::search);
        regex_lib.register_function("replace", crate::functions::regexlib::replace);
        self.register_library(regex_lib);

        // Register UUID library functions
        let mut uuid_lib = Library::new("uuid");
        uuid_lib.register_function("generate", crate::functions::uuidlib::generate);
        uuid_lib.register_function("parse", crate::functions::uuidlib::parse);
        uuid_lib.register_function("is_valid", crate::functions::uuidlib::is_valid);
        self.register_library(uuid_lib);

        // Register OS library functions
        let mut os_lib = Library::new("os");
        os_lib.register_function("env", crate::functions::oslib::env_var);
        os_lib.register_function("cwd", crate::functions::oslib::cwd);
        os_lib.register_function("platform", crate::functions::oslib::platform);
        self.register_library(os_lib);

        // Register Validation library functions
        let mut validation_lib = Library::new("validation");
        validation_lib.register_function("email", crate::functions::validationlib::email);
        validation_lib.register_function("phone", crate::functions::validationlib::phone);
        validation_lib.register_function("required", crate::functions::validationlib::required);
        validation_lib.register_function("min_length", crate::functions::validationlib::min_length);
        self.register_library(validation_lib);

        // Register System library functions
        let mut system_lib = Library::new("system");
        system_lib.register_function("exec", crate::functions::systemlib::exec);
        system_lib.register_function("uptime", crate::functions::systemlib::uptime);
        system_lib.register_function("info", crate::functions::systemlib::info);
        system_lib.register_function("current_time", crate::functions::systemlib::current_time);
        system_lib.register_function("system_name", crate::functions::systemlib::system_name);
        self.register_library(system_lib); 

        // Register Box library functions
        let mut box_lib = Library::new("boxlib");
        box_lib.register_function("put", crate::functions::boxutillib::put);
        box_lib.register_function("get", crate::functions::boxutillib::get);
        box_lib.register_function("is_box", crate::functions::boxutillib::is_box);
        self.register_library(box_lib);

        // Register Log library functions
        let mut log_lib = Library::new("loglib");
        log_lib.register_function("infolog", crate::functions::loglib::info);
        log_lib.register_function("warnlog", crate::functions::loglib::warn);
        log_lib.register_function("errorlog", crate::functions::loglib::error);
        log_lib.register_function("debuglog", crate::functions::loglib::debug);
        self.register_library(log_lib);

        // Register HT (Head/Tails) library functions
        let mut ht_lib = Library::new("htlib");
        ht_lib.register_function("coin", crate::functions::htlib::coin);
        ht_lib.register_function("bool_tos", crate::functions::htlib::bool_tos);
        self.register_library(ht_lib);

        // Register Audio library functions
        let mut audio_lib = Library::new("audio");
        audio_lib.register_function("play", crate::functions::audiolib::play);
        audio_lib.register_function("pause", crate::functions::audiolib::pause);
        audio_lib.register_function("stop", crate::functions::audiolib::stop);
        audio_lib.register_function("record", crate::functions::audiolib::record);
        self.register_library(audio_lib);

        // Register Image library functions
        let mut image_lib = Library::new("image");
        image_lib.register_function("load", crate::functions::imagelib::load);
        image_lib.register_function("save", crate::functions::imagelib::save);
        image_lib.register_function("resize", crate::functions::imagelib::resize);
        image_lib.register_function("crop", crate::functions::imagelib::crop);
        self.register_library(image_lib);

        // Register Date library functions
        let mut date_lib = Library::new("date");
        // Use our own date functions for all operations
        date_lib.register_function("now", crate::functions::datelib::now);
        date_lib.register_function("year", crate::functions::datelib::year);
        date_lib.register_function("month", crate::functions::datelib::month);
        date_lib.register_function("day", crate::functions::datelib::day);
        date_lib.register_function("format", crate::functions::datelib::format);
        date_lib.register_function("parse", crate::functions::datelib::parse);
        date_lib.register_function("add_days", crate::functions::datelib::add_days);
        date_lib.register_function("add_months", crate::functions::datelib::add_months);
        date_lib.register_function("add_years", crate::functions::datelib::add_years);
        date_lib.register_function("weekday", crate::functions::datelib::weekday);
        date_lib.register_function("weekday_name", crate::functions::datelib::weekday_name);
        date_lib.register_function("days_in_month", crate::functions::datelib::days_in_month);
        date_lib.register_function("is_leap_year", crate::functions::datelib::is_leap_year);
        date_lib.register_function("diff_days", crate::functions::datelib::diff_days);
        self.register_library(date_lib);

        // Register Net library functions
        let mut net_lib = Library::new("netlib");
        net_lib.register_function("ping", crate::functions::netlib::ping);
        net_lib.register_function("get", crate::functions::netlib::get);
        net_lib.register_function("post", crate::functions::netlib::post);
        self.register_library(net_lib);
    }
}

// Global library manager instance
lazy_static::lazy_static! {
    static ref LIBRARY_MANAGER: std::sync::Mutex<LibraryManager> = std::sync::Mutex::new(LibraryManager::new());
}

/// Initialize the library system
pub fn initialize() {
    let mut manager = LIBRARY_MANAGER.lock().unwrap();
    manager.initialize_standard_libraries();
}

/// Call a library function
pub fn call_library(library_name: &str, function_name: &str, args: Vec<Value>) -> Result<Value, String> {
    let manager = LIBRARY_MANAGER.lock().unwrap();
    manager.call_library(library_name, function_name, args)
}

/// Register a custom library
pub fn register_library(library: Library) {
    let mut manager = LIBRARY_MANAGER.lock().unwrap();
    manager.register_library(library);
}

/// Get a list of all registered libraries
pub fn get_library_names() -> Vec<String> {
    let manager = LIBRARY_MANAGER.lock().unwrap();
    manager.libraries.keys().cloned().collect()
}

/// Get a list of all functions in a library
pub fn get_library_functions(library_name: &str) -> Result<Vec<String>, String> {
    let manager = LIBRARY_MANAGER.lock().unwrap();
    match manager.get_library(library_name) {
        Some(library) => Ok(library.function_names()),
        None => Err(format!("Library '{}' not found", library_name)),
    }
}