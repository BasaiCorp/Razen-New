// src/frontend/parser/ast.rs

/// The main AST node representing a complete Razen program
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

/// Represents all possible statements in Razen
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    // Module system
    ModuleDeclaration(ModuleDeclaration),
    UseStatement(UseStatement),
    
    // Variable declarations
    VariableDeclaration(VariableDeclaration),
    ConstantDeclaration(ConstantDeclaration),
    TypeAliasDeclaration(TypeAliasDeclaration),
    
    // Function declaration
    FunctionDeclaration(FunctionDeclaration),
    
    // Data structures
    StructDeclaration(StructDeclaration),
    EnumDeclaration(EnumDeclaration),
    ImplBlock(ImplBlock),
    
    // Control flow
    IfStatement(IfStatement),
    WhileStatement(WhileStatement),
    ForStatement(ForStatement),
    MatchStatement(MatchStatement),
    TryStatement(TryStatement),
    
    // Jump statements
    ReturnStatement(ReturnStatement),
    BreakStatement(BreakStatement),
    ContinueStatement(ContinueStatement),
    ThrowStatement(ThrowStatement),
    
    // Expression statement
    ExpressionStatement(ExpressionStatement),
    
    // Block statement
    BlockStatement(BlockStatement),
}

/// Represents all possible expressions in Razen
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    // Literals
    IntegerLiteral(IntegerLiteral),
    FloatLiteral(FloatLiteral),
    StringLiteral(StringLiteral),
    BooleanLiteral(BooleanLiteral),
    NullLiteral(NullLiteral),
    
    // Identifiers
    Identifier(Identifier),
    
    // Binary operations
    BinaryExpression(BinaryExpression),
    
    // Unary operations
    UnaryExpression(UnaryExpression),
    
    // Assignment
    AssignmentExpression(AssignmentExpression),
    
    // Function call
    CallExpression(CallExpression),
    
    // Member access
    MemberExpression(MemberExpression),
    
    // Method call
    MethodCallExpression(MethodCallExpression),
    
    // Self reference
    SelfExpression(SelfExpression),
    
    // Array access
    IndexExpression(IndexExpression),
    
    // Array literal
    ArrayLiteral(ArrayLiteral),
    
    // Map literal
    MapLiteral(MapLiteral),
    
    // Struct instantiation
    StructInstantiation(StructInstantiation),
    
    // Qualified struct instantiation (e.g., module.Type { ... })
    QualifiedStructInstantiation(QualifiedStructInstantiation),
    
    // String interpolation
    InterpolatedString(InterpolatedString),
    
    // Range expression
    RangeExpression(RangeExpression),
    
    // Module call expression (e.g., utils.Function())
    ModuleCallExpression(ModuleCallExpression),
    
    // Grouping (parentheses)
    GroupingExpression(GroupingExpression),
}

// Module System
#[derive(Debug, Clone, PartialEq)]
pub struct ModuleDeclaration {
    pub name: Identifier,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UseStatement {
    pub path: String,  // The module path like "./utils" or "./math/calculator"
    pub alias: Option<Identifier>,  // Optional alias like "as util"
}

// Module reference for dot notation calls like utils.Function()
#[derive(Debug, Clone, PartialEq)]
pub struct ModuleReference {
    pub module_name: String,  // The resolved module name (last part of path)
    pub original_path: String,  // The original import path
}

// Variable Declarations
#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclaration {
    pub name: Identifier,
    pub type_annotation: Option<TypeAnnotation>,
    pub initializer: Option<Expression>,
    pub is_public: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstantDeclaration {
    pub name: Identifier,
    pub type_annotation: Option<TypeAnnotation>,
    pub initializer: Expression,
    pub is_public: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeAliasDeclaration {
    pub name: Identifier,
    pub target_type: TypeAnnotation,
    pub is_public: bool,
}

// Function Declaration
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclaration {
    pub name: Identifier,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<TypeAnnotation>,
    pub body: BlockStatement,
    pub is_public: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: Identifier,
    pub type_annotation: Option<TypeAnnotation>,
}

// Data Structures
#[derive(Debug, Clone, PartialEq)]
pub struct StructDeclaration {
    pub name: Identifier,
    pub fields: Vec<StructField>,
    pub is_public: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructField {
    pub name: Identifier,
    pub type_annotation: TypeAnnotation,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumDeclaration {
    pub name: Identifier,
    pub variants: Vec<EnumVariant>,
    pub is_public: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumVariant {
    pub name: Identifier,
    pub fields: Option<Vec<TypeAnnotation>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImplBlock {
    pub target_type: Identifier,
    pub methods: Vec<MethodDeclaration>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MethodDeclaration {
    pub name: Identifier,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<TypeAnnotation>,
    pub body: BlockStatement,
    pub is_static: bool, // true for associated functions (no self), false for methods (with self)
}

// Control Flow
#[derive(Debug, Clone, PartialEq)]
pub struct IfStatement {
    pub condition: Expression,
    pub then_branch: Box<Statement>,
    pub elif_branches: Vec<ElifBranch>,
    pub else_branch: Option<Box<Statement>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ElifBranch {
    pub condition: Expression,
    pub body: Box<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhileStatement {
    pub condition: Expression,
    pub body: Box<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForStatement {
    pub variable: Identifier,
    pub iterable: Expression,
    pub body: Box<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchStatement {
    pub expression: Expression,
    pub arms: Vec<MatchArm>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub body: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Literal(Expression),
    Identifier(Identifier),
    Wildcard,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TryStatement {
    pub body: BlockStatement,
    pub catch_clause: Option<CatchClause>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CatchClause {
    pub parameter: Option<Identifier>,
    pub body: BlockStatement,
}

// Jump Statements
#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStatement {
    pub value: Option<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BreakStatement;

#[derive(Debug, Clone, PartialEq)]
pub struct ContinueStatement;

#[derive(Debug, Clone, PartialEq)]
pub struct ThrowStatement {
    pub value: Expression,
}

// Other Statements
#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionStatement {
    pub expression: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockStatement {
    pub statements: Vec<Statement>,
}

// Expressions
#[derive(Debug, Clone, PartialEq)]
pub struct IntegerLiteral {
    pub value: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FloatLiteral {
    pub value: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StringLiteral {
    pub value: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BooleanLiteral {
    pub value: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NullLiteral;

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub name: String,
    pub span: Option<crate::frontend::diagnostics::Span>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpression {
    pub left: Box<Expression>,
    pub operator: BinaryOperator,
    pub right: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    // Arithmetic
    Add, Subtract, Multiply, Divide, Modulo, Power,
    
    // Comparison
    Equal, NotEqual, Less, Greater, LessEqual, GreaterEqual,
    
    // Logical
    And, Or,
    
    // Bitwise
    BitwiseAnd, BitwiseOr, BitwiseXor, LeftShift, RightShift,
    
    // Special
    Range, // for 1..10
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExpression {
    pub operator: UnaryOperator,
    pub operand: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    Not, Minus, Plus, BitwiseNot, PreIncrement, PostIncrement, PreDecrement, PostDecrement,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AssignmentExpression {
    pub left: Box<Expression>,
    pub operator: AssignmentOperator,
    pub right: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AssignmentOperator {
    Assign, AddAssign, SubtractAssign, MultiplyAssign, DivideAssign, ModuloAssign,
    BitwiseAndAssign, BitwiseOrAssign, BitwiseXorAssign, LeftShiftAssign, RightShiftAssign,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallExpression {
    pub callee: Box<Expression>,
    pub arguments: Vec<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MemberExpression {
    pub object: Box<Expression>,
    pub property: Identifier,
    pub computed: bool, // true for obj[prop], false for obj.prop
}

#[derive(Debug, Clone, PartialEq)]
pub struct MethodCallExpression {
    pub object: Box<Expression>,
    pub method: Identifier,
    pub arguments: Vec<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelfExpression;

#[derive(Debug, Clone, PartialEq)]
pub struct IndexExpression {
    pub object: Box<Expression>,
    pub index: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayLiteral {
    pub elements: Vec<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MapLiteral {
    pub pairs: Vec<MapPair>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MapPair {
    pub key: Expression,
    pub value: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructInstantiation {
    pub name: Identifier,
    pub fields: Vec<StructFieldInit>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructFieldInit {
    pub name: Identifier,
    pub value: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub struct QualifiedStructInstantiation {
    pub qualified_name: Box<Expression>,  // The qualified type name (e.g., module.Type as MemberExpression)
    pub fields: Vec<StructFieldInit>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InterpolatedString {
    pub parts: Vec<InterpolationPart>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InterpolationPart {
    Text(String),
    Expression(Expression),
}

#[derive(Debug, Clone, PartialEq)]
pub struct RangeExpression {
    pub start: Box<Expression>,
    pub end: Box<Expression>,
    pub inclusive: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModuleCallExpression {
    pub module: Identifier,    // The module name (e.g., "utils")
    pub function: Identifier,  // The function name (e.g., "Function")
    pub arguments: Vec<Expression>, // Function arguments
}

#[derive(Debug, Clone, PartialEq)]
pub struct GroupingExpression {
    pub expression: Box<Expression>,
}

// Type System
#[derive(Debug, Clone, PartialEq)]
pub enum TypeAnnotation {
    Int,
    Float,
    String,
    Bool,
    Char,
    Array(Box<TypeAnnotation>),
    Map(Box<TypeAnnotation>, Box<TypeAnnotation>),
    Custom(Identifier),
    Any,
}

// Utility functions for creating AST nodes
impl Program {
    pub fn new(statements: Vec<Statement>) -> Self {
        Program { statements }
    }
}

impl Identifier {
    pub fn new(name: String) -> Self {
        Identifier { name, span: None }
    }
    
    pub fn with_span(name: String, span: crate::frontend::diagnostics::Span) -> Self {
        Identifier { name, span: Some(span) }
    }
}

impl IntegerLiteral {
    pub fn new(value: i64) -> Self {
        IntegerLiteral { value }
    }
}

impl FloatLiteral {
    pub fn new(value: f64) -> Self {
        FloatLiteral { value }
    }
}

impl StringLiteral {
    pub fn new(value: String) -> Self {
        StringLiteral { value }
    }
}

impl BooleanLiteral {
    pub fn new(value: bool) -> Self {
        BooleanLiteral { value }
    }
}

impl BlockStatement {
    pub fn new(statements: Vec<Statement>) -> Self {
        BlockStatement { statements }
    }
}

impl ImplBlock {
    pub fn new(target_type: Identifier, methods: Vec<MethodDeclaration>) -> Self {
        ImplBlock { target_type, methods }
    }
}

impl MethodDeclaration {
    pub fn new(name: Identifier, parameters: Vec<Parameter>, return_type: Option<TypeAnnotation>, body: BlockStatement, is_static: bool) -> Self {
        MethodDeclaration { name, parameters, return_type, body, is_static }
    }
}

impl MethodCallExpression {
    pub fn new(object: Box<Expression>, method: Identifier, arguments: Vec<Expression>) -> Self {
        MethodCallExpression { object, method, arguments }
    }
}

impl SelfExpression {
    pub fn new() -> Self {
        SelfExpression
    }
}

impl ModuleCallExpression {
    pub fn new(module: Identifier, function: Identifier, arguments: Vec<Expression>) -> Self {
        ModuleCallExpression { module, function, arguments }
    }
}