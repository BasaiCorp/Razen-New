use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenType {
    // Variable declaration keywords
    Num,    // Number variable (replaces Let)
    Str,    // String variable (replaces Take)
    Bool,   // Boolean variable (replaces Hold)
    Var,    // Any variable (replaces Put)
    
    // String operations are handled with built-in operators
    
    // List & Array Variables
    List,   // Dynamic arrays
    Arr,    // Fixed-size arrays
    Append, // Add elements to lists
    Remove, // Remove elements from lists
    
    // Dictionary/Map Variables
    Map,    // Key-value storage
    Key,    // Dictionary keys
    Value,  // Dictionary values
    
    // Date & Time operations are handled with libraries
    
    // User-Defined Variables
    Store,  // Persistent storage
    Box,    // Temporary storage
    Ref,    // Reference variables
    
    // Logical Variables
    Is,     // Equality comparison
    When,   // Pattern matching
    
    // Mathematical operators
    Plus,       // +
    Minus,      // -
    Asterisk,   // *
    Slash,      // /
    Percent,    // %
    Power,      // **
    FloorDiv,   // //
    
    // Assignment operators
    Assign,         // =
    PlusAssign,     // +=
    MinusAssign,    // -=
    AsteriskAssign, // *=
    SlashAssign,    // /=
    PercentAssign,  // %=
    
    // Comparison operators
    Equal,      // ==
    NotEqual,   // !=
    Greater,    // >
    GreaterEqual, // >=
    Less,       // <
    LessEqual,  // <=
    
    // Logical operators
    And,    // &&
    Or,     // ||
    Not,    // !
    
    // Control flow keywords
    If,
    Else,
    Elif,
    While,
    For,
    In,
    Break,
    Continue,
    Return,
    
    // Function declaration
    Fun,
    
    // Exception handling
    Try,
    Catch,
    Throw,
    Finally,
    
    // I/O keywords
    Show,
    Read,
    Exit,
    Load,    // Loading animation
    
    // Boolean literals
    True,
    False,
    Null,
    
    // Document Type Declaration
    DocumentType,  // type declaration (script, cli)
    
    // Module system
    Use,           // use module imports
    Export,        // export module items
    As,            // namespace alias
    From,          // module source
    
    // Debug and developer tools
    Debug,         // debug mode
    Assert,        // assertions
    Trace,         // execution tracing
    
    // 12 - OOP Keywords
    Class,         // class declaration
    
    // 13 - API Keywords
    Api,           // API declaration
    Call,          // API calls
    Get,           // API responses
    Post,          // API requests
    Await,         // For async operations
    
    // 14 - Connection Keywords
    Connect,       // connecting to external services
    To,            // for export destination
    
    // 15 - Import/Export Keywords
    Import,        // importing modules
    
    // 16 - Library Keywords
    Lib,           // libraries
    Random,        // random number generation
    HTLib,         // head and tails library
    Coin,          // coin toss
    MathLib,       // math operations
    Ping,          // website ping
    Bolt,          // performance optimization
    Seed,          // seed generation
    NetLib,        // network operations
    File,          // file operations
    Json,          // JSON operations
    Date,          // date and time operations
    StrLib,        // string utilities
    ArrLib,        // array utilities
    Os,            // operating system info
    Regex,         // regular expressions
    Crypto,        // cryptography
    Color,         // color manipulation
    System,        // system information
    Ui,            // user interface
    MemoryLib,     // memory management
    BinaryLib,     // binary file operations
    BitwiseLib,    // bitwise operations
    SystemLib,     // system call operations
    ProcessLib,    // process management
    ThreadLib,     // thread management
    CompilerLib,   // compiler operations
    Storage,       // persistent storage
    Audio,         // audio operations
    Image,         // image processing
    Validation,    // data validation
    LogLib,        // logging utilities
    Uuid,          // UUID generation
    BoxLib,        // box operations library
    IOLib,         // input/output operations library
    NumLib,        // numbers utilities library
    RefLib,        // reference operations library
    TimeLib,       // time library
    TypeCheckLib,  // type checking library
    TypeConvertLib, // type conversion library
    
    // Delimiters
    LeftParen,      // (
    RightParen,     // )
    LeftBrace,      // {
    RightBrace,     // }
    LeftBracket,    // [
    RightBracket,   // ]
    Comma,          // ,
    Dot,            // .
    Semicolon,      // ;
    Colon,          // :
    ColonColon,     // ::
    
    // Literals
    Identifier,     // Variable names, function names, etc.
    StringLiteral,  // "hello"
    NumberLiteral,  // 123, 3.14
    
    // Comments
    Comment,        // # Comment
    
    // Special
    EOF,            // End of file
    Illegal,        // Invalid token

    // 17 - Compiler Construction Keywords
    Token,          // Token representation
    Lexer,          // Lexical analyzer
    Parser,         // Syntax analyzer
    AST,            // Abstract syntax tree
    Node,           // AST node
    Visitor,        // AST visitor pattern
    Symbol,         // Symbol table entry
    Scope,          // Scope management
    Type,           // Type checking
    IR,             // Intermediate representation
    CodeGen,        // Code generation
    Optimize,       // Optimization
    Target,         // Target code
    Grammar,        // Grammar definition
    Rule,           // Grammar rule
    Attribute,      // Semantic attribute

    // 18 - Compiler Construction Libraries
    LexerLib,       // Lexical analysis library
    ParserLib,      // Syntax analysis library
    ASTLib,         // AST manipulation library
    SymbolLib,      // Symbol table library
    TypeLib,        // Type checking library
    IRLib,          // IR operations library
    CodeGenLib,     // Code generation library
    OptimizeLib,    // Optimization library
    
    // 19 - Performance and Type Safety Keywords
    Const,          // For declaring constant values
    Enum,           // For defining enumerated types
    Inline,         // For suggesting function inlining
    Final,          // For declaring classes that cannot be extended
    Volatile,       // For declaring variables that might change externally
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // Variable declaration keywords
            TokenType::Num => write!(f, "NUM"),
            TokenType::Str => write!(f, "STR"),
            TokenType::Bool => write!(f, "BOOL"),
            TokenType::Var => write!(f, "VAR"),
            
            // String Variables
            // String-related tokens removed
            
            // List & Array Variables
            TokenType::List => write!(f, "LIST"),
            TokenType::Arr => write!(f, "ARR"),
            TokenType::Append => write!(f, "APPEND"),
            TokenType::Remove => write!(f, "REMOVE"),
            
            // Dictionary/Map Variables
            TokenType::Map => write!(f, "MAP"),
            TokenType::Key => write!(f, "KEY"),
            TokenType::Value => write!(f, "VALUE"),
            
            // Date & Time operations are handled with libraries
            
            // User-Defined Variables
            TokenType::Store => write!(f, "STORE"),
            TokenType::Box => write!(f, "BOX"),
            TokenType::Ref => write!(f, "REF"),
            
            // Logical Variables
            TokenType::Is => write!(f, "IS"),
            TokenType::When => write!(f, "WHEN"),
            
            // Mathematical operators
            TokenType::Plus => write!(f, "+"),
            TokenType::Minus => write!(f, "-"),
            TokenType::Asterisk => write!(f, "*"),
            TokenType::Slash => write!(f, "/"),
            TokenType::Percent => write!(f, "%"),
            TokenType::Power => write!(f, "**"),
            TokenType::FloorDiv => write!(f, "//"),
            
            // Assignment operators
            TokenType::Assign => write!(f, "="),
            TokenType::PlusAssign => write!(f, "+="),
            TokenType::MinusAssign => write!(f, "-="),
            TokenType::AsteriskAssign => write!(f, "*="),
            TokenType::SlashAssign => write!(f, "/="),
            TokenType::PercentAssign => write!(f, "%="),
            
            // Comparison operators
            TokenType::Equal => write!(f, "=="),
            TokenType::NotEqual => write!(f, "!="),
            TokenType::Greater => write!(f, ">"),
            TokenType::GreaterEqual => write!(f, ">="),
            TokenType::Less => write!(f, "<"),
            TokenType::LessEqual => write!(f, "<="),
            
            // Logical operators
            TokenType::And => write!(f, "&&"),
            TokenType::Or => write!(f, "||"),
            TokenType::Not => write!(f, "!"),
            
            // Control flow keywords
            TokenType::If => write!(f, "IF"),
            TokenType::Else => write!(f, "ELSE"),
            TokenType::Elif => write!(f, "ELIF"),
            TokenType::While => write!(f, "WHILE"),
            TokenType::For => write!(f, "FOR"),
            TokenType::In => write!(f, "IN"),
            TokenType::Break => write!(f, "BREAK"),
            TokenType::Continue => write!(f, "CONTINUE"),
            TokenType::Return => write!(f, "RETURN"),
            
            // Function declaration
            TokenType::Fun => write!(f, "FUN"),
            
            // Exception handling
            TokenType::Try => write!(f, "TRY"),
            TokenType::Catch => write!(f, "CATCH"),
            TokenType::Throw => write!(f, "THROW"),
            TokenType::Finally => write!(f, "FINALLY"),
            
            // I/O keywords
            TokenType::Show => write!(f, "SHOW"),
            TokenType::Read => write!(f, "READ"),
            TokenType::Exit => write!(f, "EXIT"),
            TokenType::Load => write!(f, "LOAD"),
            
            // Boolean literals
            TokenType::True => write!(f, "TRUE"),
            TokenType::False => write!(f, "FALSE"),
            TokenType::Null => write!(f, "NULL"),
            
            // Document Type Declaration
            TokenType::DocumentType => write!(f, "TYPE"),
            
            // Module system
            TokenType::Use => write!(f, "USE"),
            TokenType::Export => write!(f, "EXPORT"),
            TokenType::As => write!(f, "AS"),
            TokenType::From => write!(f, "FROM"),
            
            // Debug and developer tools
            TokenType::Debug => write!(f, "DEBUG"),
            TokenType::Assert => write!(f, "ASSERT"),
            TokenType::Trace => write!(f, "TRACE"),
            
            // OOP Keywords
            TokenType::Class => write!(f, "CLASS"),
            
            // API Keywords
            TokenType::Api => write!(f, "API"),
            TokenType::Call => write!(f, "CALL"),
            TokenType::Get => write!(f, "GET"),
            TokenType::Post => write!(f, "POST"),
            TokenType::Await => write!(f, "AWAIT"),
            
            // Connection Keywords
            TokenType::Connect => write!(f, "CONNECT"),
            TokenType::To => write!(f, "TO"),
            
            // Import/Export Keywords
            TokenType::Import => write!(f, "IMPORT"),
            
            // Library Keywords
            TokenType::Lib => write!(f, "LIB"),
            TokenType::Random => write!(f, "RANDOM"),
            TokenType::HTLib => write!(f, "HTLIB"),
            TokenType::Coin => write!(f, "COIN"),
            TokenType::MathLib => write!(f, "MATH"),
            TokenType::Ping => write!(f, "PING"),
            TokenType::Bolt => write!(f, "BOLT"),
            TokenType::Seed => write!(f, "SEED"),
            TokenType::NetLib => write!(f, "NETLIB"),
            TokenType::File => write!(f, "FILE"),
            TokenType::Json => write!(f, "JSON"),
            TokenType::Date => write!(f, "DATE"),
            TokenType::StrLib => write!(f, "STRING"),
            TokenType::ArrLib => write!(f, "ARRAY"),
            TokenType::Os => write!(f, "OS"),
            TokenType::Regex => write!(f, "REGEX"),
            TokenType::Crypto => write!(f, "CRYPTO"),
            TokenType::Color => write!(f, "COLOR"),
            TokenType::System => write!(f, "SYSTEM"),
            TokenType::Ui => write!(f, "UI"),
            TokenType::Storage => write!(f, "STORAGE"),
            TokenType::Audio => write!(f, "AUDIO"),
            TokenType::Image => write!(f, "IMAGE"),
            TokenType::Validation => write!(f, "VALIDATION"),
            TokenType::MemoryLib => write!(f, "MEMORYLIB"),
            TokenType::BinaryLib => write!(f, "BINARYLIB"),
            TokenType::BitwiseLib => write!(f, "BITWISELIB"),
            TokenType::SystemLib => write!(f, "SYSTEMLIB"),
            TokenType::ProcessLib => write!(f, "PROCESSLIB"),
            TokenType::ThreadLib => write!(f, "THREADLIB"),
            TokenType::CompilerLib => write!(f, "COMPILERLIB"),
            TokenType::LogLib => write!(f, "LOGLIB"),
            TokenType::Uuid => write!(f, "UUID"),
            TokenType::BoxLib => write!(f, "BOX"),
            TokenType::IOLib => write!(f, "IO"),
            TokenType::NumLib => write!(f, "NUM"),
            TokenType::RefLib => write!(f, "REF"),
            TokenType::TimeLib => write!(f, "TIME"),
            TokenType::TypeCheckLib => write!(f, "TYPECHECK"),
            TokenType::TypeConvertLib => write!(f, "TYPECONVERT"),
            
            // Delimiters
            TokenType::LeftParen => write!(f, "("),
            TokenType::RightParen => write!(f, ")"),
            TokenType::LeftBrace => write!(f, "{{"),
            TokenType::RightBrace => write!(f, "}}"),
            TokenType::LeftBracket => write!(f, "["),
            TokenType::RightBracket => write!(f, "]"),
            TokenType::Comma => write!(f, ","),
            TokenType::Dot => write!(f, "."),
            TokenType::Semicolon => write!(f, ";"),
            TokenType::Colon => write!(f, ":"),
            TokenType::ColonColon => write!(f, "::"),
            
            // Literals
            TokenType::Identifier => write!(f, "IDENTIFIER"),
            TokenType::StringLiteral => write!(f, "STRING"),
            TokenType::NumberLiteral => write!(f, "NUMBER"),
            
            // Comments
            TokenType::Comment => write!(f, "COMMENT"),
            
            // Special
            TokenType::EOF => write!(f, "EOF"),
            TokenType::Illegal => write!(f, "ILLEGAL"),
            
            // 17 - Compiler Construction Keywords
            TokenType::Token => write!(f, "TOKEN"),
            TokenType::Lexer => write!(f, "LEXER"),
            TokenType::Parser => write!(f, "PARSER"),
            TokenType::AST => write!(f, "AST"),
            TokenType::Node => write!(f, "NODE"),
            TokenType::Visitor => write!(f, "VISITOR"),
            TokenType::Symbol => write!(f, "SYMBOL"),
            TokenType::Scope => write!(f, "SCOPE"),
            TokenType::Type => write!(f, "TYPE"),
            TokenType::IR => write!(f, "IR"),
            TokenType::CodeGen => write!(f, "CODEGEN"),
            TokenType::Optimize => write!(f, "OPTIMIZE"),
            TokenType::Target => write!(f, "TARGET"),
            TokenType::Grammar => write!(f, "GRAMMAR"),
            TokenType::Rule => write!(f, "RULE"),
            TokenType::Attribute => write!(f, "ATTRIBUTE"),
            
            // 18 - Compiler Construction Libraries
            TokenType::LexerLib => write!(f, "LEXERLIB"),
            TokenType::ParserLib => write!(f, "PARSERLIB"),
            TokenType::ASTLib => write!(f, "ASTLIB"),
            TokenType::SymbolLib => write!(f, "SYMBOLLIB"),
            TokenType::TypeLib => write!(f, "TYPELIB"),
            TokenType::IRLib => write!(f, "IRLIB"),
            TokenType::CodeGenLib => write!(f, "CODEGENLIB"),
            TokenType::OptimizeLib => write!(f, "OPTIMIZELIB"),
            
            // 19 - Performance and Type Safety Keywords
            TokenType::Const => write!(f, "CONST"),
            TokenType::Enum => write!(f, "ENUM"),
            TokenType::Inline => write!(f, "INLINE"),
            TokenType::Final => write!(f, "FINAL"),
            TokenType::Volatile => write!(f, "VOLATILE"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(token_type: TokenType, literal: impl Into<String>, line: usize, column: usize) -> Self {
        Token {
            token_type,
            literal: literal.into(),
            line,
            column,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:'{}' at {}:{}", self.token_type, self.literal, self.line, self.column)
    }
}

// Helper function to lookup keywords
pub fn lookup_identifier(identifier: &str) -> TokenType {
    match identifier {
        // Variable declaration keywords
        "num" => TokenType::Num,
        "str" => TokenType::Str,
        "bool" => TokenType::Bool,
        "var" => TokenType::Var,
        
        // String Variables
        // String-related tokens removed
        
        // List & Array Variables
        "list" => TokenType::List,
        "arr" => TokenType::Arr,
        "append" => TokenType::Append,
        "remove" => TokenType::Remove,
        
        // Dictionary/Map Variables
        "map" => TokenType::Map,
        "key" => TokenType::Key,
        "value" => TokenType::Value,
        
        // Date & Time operations are handled with libraries
        
        // User-Defined Variables
        "store" => TokenType::Store,
        "box" => TokenType::Box,
        "ref" => TokenType::Ref,
        
        // Logical Variables
        "is" => TokenType::Is,
        "when" => TokenType::When,
        
        // Control flow keywords
        "if" => TokenType::If,
        "else" => TokenType::Else,
        "elif" => TokenType::Elif,
        "while" => TokenType::While,
        "for" => TokenType::For,
        "in" => TokenType::In,
        "break" => TokenType::Break,
        "continue" => TokenType::Continue,
        "return" => TokenType::Return,
        
        // Function declaration
        "fun" => TokenType::Fun,
        
        // Exception handling
        "try" => TokenType::Try,
        "catch" => TokenType::Catch,
        "throw" => TokenType::Throw,
        "finally" => TokenType::Finally,
        
        // I/O keywords
        "show" => TokenType::Show,
        "read" => TokenType::Read,
        "exit" => TokenType::Exit,
        "load" => TokenType::Load,
        
        // Boolean literals
        "true" => TokenType::True,
        "false" => TokenType::False,
        "null" => TokenType::Null,
        
        // Document Type Declaration
        "type" => TokenType::DocumentType,
        
        // Module system
        "use" => TokenType::Use,
        "export" => TokenType::Export,
        "as" => TokenType::As,
        "from" => TokenType::From,
        
        // Debug and developer tools
        "debug" => TokenType::Debug,
        "assert" => TokenType::Assert,
        "trace" => TokenType::Trace,
        
        // OOP Keywords (Section 12)
        "class" => TokenType::Class,
        
        // API Keywords (Section 13)
        "api" => TokenType::Api,
        "call" => TokenType::Call,
        "get" => TokenType::Get,
        "post" => TokenType::Post,
        "await" => TokenType::Await,
        
        // Connection Keywords (Section 14)
        "connect" => TokenType::Connect,
        "to" => TokenType::To,
        
        // Import/Export Keywords (Section 15)
        "import" => TokenType::Import,
        
        // Library Keywords (Section 16)
        "lib" => TokenType::Lib,
        "random" => TokenType::Random,
        "htlib" => TokenType::HTLib,
        "coin" => TokenType::Coin,
        "mathlib" => TokenType::MathLib,
        "ping" => TokenType::Ping,
        "bolt" => TokenType::Bolt,
        "seed" => TokenType::Seed,
        "netlib" => TokenType::NetLib,
        "file" => TokenType::File,
        "json" => TokenType::Json,
        "date" => TokenType::Date,
        "strlib" => TokenType::StrLib,
        "arrlib" => TokenType::ArrLib,
        "os" => TokenType::Os,
        "regex" => TokenType::Regex,
        "crypto" => TokenType::Crypto,
        "color" => TokenType::Color,
        "system" => TokenType::System,
        "ui" => TokenType::Ui,
        "memorylib" => TokenType::MemoryLib,
        "binarylib" => TokenType::BinaryLib,
        "bitwiselib" => TokenType::BitwiseLib,
        "systemlib" => TokenType::SystemLib,
        "processlib" => TokenType::ProcessLib,
        "threadlib" => TokenType::ThreadLib,
        "compilerlib" => TokenType::CompilerLib,
        // Keep backward compatibility with old names
        "memlib" => TokenType::MemoryLib,
        "binlib" => TokenType::BinaryLib,
        "bitlib" => TokenType::BitwiseLib,
        "syslib" => TokenType::SystemLib,
        "proclib" => TokenType::ProcessLib,
        "thrlib" => TokenType::ThreadLib,
        "complib" => TokenType::CompilerLib,
        "storage" => TokenType::Storage,
        "audio" => TokenType::Audio,
        "image" => TokenType::Image,
        "validation" => TokenType::Validation,
        "loglib" => TokenType::LogLib,
        "uuid" => TokenType::Uuid,
        "boxlib" => TokenType::BoxLib,
        "iolib" => TokenType::IOLib,
        "numlib" => TokenType::NumLib,
        "reflib" => TokenType::RefLib,
        "timelib" => TokenType::TimeLib,
        "typechecklib" => TokenType::TypeCheckLib,
        "typeconvertlib" => TokenType::TypeConvertLib,
        
        // 17 - Compiler Construction Keywords
        "token" => TokenType::Token,
        "lexer" => TokenType::Lexer,
        "parser" => TokenType::Parser,
        "ast" => TokenType::AST,
        "node" => TokenType::Node,
        "visitor" => TokenType::Visitor,
        "symbol" => TokenType::Symbol,
        "scope" => TokenType::Scope,
        "typesys" => TokenType::Type,
        "ir" => TokenType::IR,
        "codegen" => TokenType::CodeGen,
        "optimize" => TokenType::Optimize,
        "target" => TokenType::Target,
        "grammar" => TokenType::Grammar,
        "rule" => TokenType::Rule,
        "attribute" => TokenType::Attribute,
        
        // 18 - Compiler Construction Libraries
        "lexerlib" => TokenType::LexerLib,
        "parserlib" => TokenType::ParserLib,
        "astlib" => TokenType::ASTLib,
        "symbollib" => TokenType::SymbolLib,
        "typelib" => TokenType::TypeLib,
        "irlib" => TokenType::IRLib,
        "codegenlib" => TokenType::CodeGenLib,
        "optimizelib" => TokenType::OptimizeLib,
        
        // 19 - Performance and Type Safety Keywords
        "const" => TokenType::Const,
        "enum" => TokenType::Enum,
        "inline" => TokenType::Inline,
        "final" => TokenType::Final,
        "volatile" => TokenType::Volatile,
        
        // If not a keyword, it's an identifier
        _ => TokenType::Identifier,
    }
}