// src/frontend/lexer/token.rs

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Variable & Function Declaration
    Const, Var, Fun, Type,

    // Data Structures
    Struct, Enum, Impl,

    // Control Flow
    If, Else, Elif, While, For, In, Return, Break, Continue, Match, Try, Catch, Throw,

    // Module System
    Mod, Use, Pub, From, As,

    // Types
    Int, Str, Bool, Char, Array, Map, Any, FloatType, // Added FloatType

    // Literals
    Identifier, String(String), FString(String), Integer(i64), Float(f64), True, False, Null, Self_,

    // Arithmetic Operators
    Plus, Minus, Star, Slash, Percent, StarStar,

    // Assignment Operators
    Equal, PlusEqual, MinusEqual, StarEqual, SlashEqual, PercentEqual,
    AmpersandEqual, PipeEqual, CaretEqual, LessLessEqual, GreaterGreaterEqual,

    // Comparison Operators
    EqualEqual, BangEqual, Less, Greater, LessEqual, GreaterEqual,

    // Logical Operators
    AmpersandAmpersand, PipePipe, Bang,

    // Bitwise Operators
    Ampersand, Pipe, Caret, Tilde, LessLess, GreaterGreater,

    // Unary Operators
    PlusPlus, MinusMinus,

    // Special Operators
    Question, QuestionQuestion, DotDot, DotDotEqual, DotDotDot,

    // Punctuators & Delimiters
    LeftBrace, RightBrace, LeftParen, RightParen, LeftBracket, RightBracket,
    Semicolon, Comma, Dot, Colon, ColonColon, Arrow, FatArrow,

    // I/O Functions (will be treated as identifiers and resolved later)
    Print, Println, Input, Read, Write, Open, Close,

    // Special Symbols
    Hash, At,

    // End of File
    Eof,

    // Illegal/Unknown token
    Illegal,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, line: usize) -> Self {
        Token { kind, lexeme, line }
    }
}
