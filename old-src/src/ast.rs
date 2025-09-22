use std::fmt;

// Node represents a node in the AST
#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Program(Program),
    Statement(Statement),
    Expression(Expression),
}

// Program is the root node of the AST
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            statements: Vec::new(),
        }
    }
}

// Statement represents a statement in the program
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    VariableDeclaration {
        var_type: String,     // let, take, hold, put
        name: String,
        value: Option<Expression>,
    },
    FunctionDeclaration {
        name: String,
        parameters: Vec<String>,
        body: Vec<Statement>,
    },
    ReturnStatement {
        value: Option<Expression>,
    },
    ExpressionStatement {
        expression: Expression,
    },
    BlockStatement {
        statements: Vec<Statement>,
    },
    IfStatement {
        condition: Expression,
        consequence: Vec<Statement>,
        alternative: Option<Vec<Statement>>,
    },
    WhileStatement {
        condition: Expression,
        body: Vec<Statement>,
    },
    ForStatement {
        iterator: String,
        iterable: Expression,
        body: Vec<Statement>,
    },
    BreakStatement,
    ContinueStatement,
    ShowStatement {
        value: Expression,
        color: Option<String>, // Optional color parameter
    },
    TryStatement {
        try_block: Vec<Statement>,
        catch_param: Option<String>,  // Parameter name for the caught error
        catch_block: Option<Vec<Statement>>,
        finally_block: Option<Vec<Statement>>,
    },
    ThrowStatement {
        value: Expression,
    },
    ReadStatement {
        name: String,
    },
    ExitStatement,
    DocumentTypeDeclaration {
        doc_type: String,  // web, script, cli
    },
    // Module system
    ModuleImport {
        names: Vec<String>,         // Names to import
        alias: Option<String>,      // Optional namespace alias
        source: String,             // Module source path
    },
    ModuleExport {
        name: String,               // Name to export
    },
    // Debug and developer tools
    DebugStatement {
        value: Expression,
    },
    AssertStatement {
        condition: Expression,
        message: Option<Expression>,
    },
    TraceStatement {
        value: Expression,
    },
    // OOP (Section 12)
    ClassDeclaration {
        name: String,
        body: Vec<Statement>,
    },
    // API Integration (Section 13)
    ApiDeclaration {
        name: String,
        url: String,
    },
    ApiCall {
        name: String,
        body: Vec<Statement>,
    },
    // Connect and From (Section 14)
    ConnectStatement {
        name: String,
        url: String,
        options: Vec<(String, Expression)>,  // For auth, timeout, etc.
    },
    // Import/Export (Section 15)
    ImportStatement {
        imports: Vec<String>,
        path: String,
    },
    // Libraries (Section 16)
    LibStatement {
        name: String,
    },
    LoadStatement {
        cycles: Expression,
        block: Vec<Statement>,
    },
    // Compiler Construction (Section 17)
    GrammarStatement {
        name: String,
        properties: Vec<(String, Expression)>,
    },
    TokenStatement {
        name: String,
        pattern: String,
    },
    LexerStatement {
        name: String,
        config: Vec<(String, Expression)>,
    },
    ParserStatement {
        name: String,
        config: Vec<(String, Expression)>,
    },
    NodeStatement {
        name: String,
        properties: Vec<(String, Expression)>,
    },
    RuleStatement {
        name: String,
        production: String,
        node_type: Option<String>,
    },
    VisitorStatement {
        name: String,
        methods: Vec<String>,
    },
    SymbolStatement {
        name: String,
        attributes: Vec<String>,
    },
    ScopeStatement {
        name: String,
        parent: Option<String>,
    },
    TypeStatement {
        name: String,
        operations: Vec<String>,
    },
    IRStatement {
        name: String,
        opcode: String,
        operands: Vec<String>,
    },
    CodeGenStatement {
        name: String,
        target: String,
        instructions: Vec<(String, Expression)>,
    },
    OptimizeStatement {
        name: String,
        description: String,
        passes: Vec<String>,
    },
    TargetStatement {
        name: String,
        properties: Vec<(String, Expression)>,
    },
    AttributeStatement {
        name: String,
        values: Vec<(String, Expression)>,
    },
    // Performance and Type Safety Keywords
    ConstDeclaration {
        name: String,
        value: Expression,
    },
    EnumDeclaration {
        name: String,
        variants: Vec<(String, Option<Expression>)>,
    },
    InlineFunctionDeclaration {
        name: String,
        parameters: Vec<String>,
        body: Vec<Statement>,
    },
    FinalClassDeclaration {
        name: String,
        body: Vec<Statement>,
    },
    VolatileDeclaration {
        var_type: String,     // let, hold, etc.
        name: String,
        value: Option<Expression>,
    },
}

// Expression represents an expression in the program
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Identifier(String),
    StringLiteral(String),
    NumberLiteral(f64),
    BooleanLiteral(bool),
    NullLiteral,
    PrefixExpression {
        operator: String,
        right: Box<Expression>,
    },
    InfixExpression {
        left: Box<Expression>,
        operator: String,
        right: Box<Expression>,
    },
    AssignmentExpression {
        left: Box<Expression>,
        operator: String,  // =, +=, -=, *=, /=, %=
        right: Box<Expression>,
    },
    CallExpression {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
    ArrayLiteral {
        elements: Vec<Expression>,
    },
    IndexExpression {
        left: Box<Expression>,
        index: Box<Expression>,
    },
    MapLiteral {
        pairs: Vec<(Expression, Expression)>,
    },
    LibraryCall {
        library: Box<Expression>,
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
    NamespaceCall {
        namespace: String,
        function: String,
        arguments: Vec<Expression>,
    },
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Node::Program(program) => {
                let mut result = String::new();
                for stmt in &program.statements {
                    result.push_str(&format!("{}", Node::Statement(stmt.clone())));
                }
                write!(f, "{}", result)
            },
            Node::Statement(stmt) => {
                match stmt {
                    Statement::VariableDeclaration { var_type, name, value } => {
                        if let Some(val) = value {
                            write!(f, "{} {} = {};", var_type, name, Node::Expression(val.clone()))
                        } else {
                            write!(f, "{} {};", var_type, name)
                        }
                    },
                    // Compiler Construction Statements
                    Statement::GrammarStatement { name, properties } => {
                        write!(f, "grammar {} = {{ ... }};", name)
                    },
                    Statement::TokenStatement { name, pattern } => {
                        write!(f, "token {} = \"{}\";", name, pattern)
                    },
                    Statement::LexerStatement { name, config } => {
                        write!(f, "lexer {} = {{ ... }};", name)
                    },
                    Statement::ParserStatement { name, config } => {
                        write!(f, "parser {} = {{ ... }};", name)
                    },
                    Statement::NodeStatement { name, properties } => {
                        write!(f, "node {} = {{ ... }};", name)
                    },
                    Statement::RuleStatement { name, production, node_type } => {
                        if let Some(node) = node_type {
                            write!(f, "rule {} = {{ production: \"{}\", astNode: {} }};", name, production, node)
                        } else {
                            write!(f, "rule {} = \"{}\";", name, production)
                        }
                    },
                    Statement::VisitorStatement { name, methods } => {
                        write!(f, "visitor {} = {{ methods: [...] }};", name)
                    },
                    Statement::SymbolStatement { name, attributes } => {
                        write!(f, "symbol {} = {{ attributes: [...] }};", name)
                    },
                    Statement::ScopeStatement { name, parent } => {
                        if let Some(p) = parent {
                            write!(f, "scope {} = {{ parent: {} }};", name, p)
                        } else {
                            write!(f, "scope {} = {{ parent: null }};", name)
                        }
                    },
                    Statement::TypeStatement { name, operations } => {
                        write!(f, "typesys {} = {{ operations: [...] }};", name)
                    },
                    Statement::IRStatement { name, opcode, operands } => {
                        write!(f, "ir {} = {{ opcode: \"{}\", operands: [...] }};", name, opcode)
                    },
                    Statement::CodeGenStatement { name, target, instructions } => {
                        write!(f, "codegen {} = {{ architecture: \"{}\", ... }};", name, target)
                    },
                    Statement::OptimizeStatement { name, description, passes } => {
                        write!(f, "optimize {} = {{ description: \"{}\", passes: [...] }};", name, description)
                    },
                    Statement::TargetStatement { name, properties } => {
                        write!(f, "target {} = {{ ... }};", name)
                    },
                    Statement::AttributeStatement { name, values } => {
                        write!(f, "attribute {} = {{ ... }};", name)
                    },
                    Statement::FunctionDeclaration { name, parameters, body } => {
                        let params = parameters.join(", ");
                        let mut body_str = String::new();
                        for stmt in body {
                            body_str.push_str(&format!("{}", Node::Statement(stmt.clone())));
                        }
                        write!(f, "fun {}({}) {{
{}
}}", name, params, body_str)
                    },
                    Statement::ReturnStatement { value } => {
                        if let Some(val) = value {
                            write!(f, "return {};", Node::Expression(val.clone()))
                        } else {
                            write!(f, "return;")
                        }
                    },
                    Statement::ExpressionStatement { expression } => {
                        write!(f, "{};", Node::Expression(expression.clone()))
                    },
                    Statement::BlockStatement { statements } => {
                        let mut result = String::new();
                        for stmt in statements {
                            result.push_str(&format!("{}", Node::Statement(stmt.clone())));
                        }
                        write!(f, "{{
{}
}}", result)
                    },
                    Statement::IfStatement { condition, consequence, alternative } => {
                        let mut result = format!("if ({}) {{
", Node::Expression(condition.clone()));
                        for stmt in consequence {
                            result.push_str(&format!("{}", Node::Statement(stmt.clone())));
                        }
                        result.push_str("}");
                        
                        if let Some(alt) = alternative {
                            result.push_str(" else {
");
                            for stmt in alt {
                                result.push_str(&format!("{}", Node::Statement(stmt.clone())));
                            }
                            result.push_str("}");
                        }
                        
                        write!(f, "{}", result)
                    },
                    Statement::WhileStatement { condition, body } => {
                        let mut result = format!("while ({}) {{
", Node::Expression(condition.clone()));
                        for stmt in body {
                            result.push_str(&format!("{}", Node::Statement(stmt.clone())));
                        }
                        result.push_str("}");
                        
                        write!(f, "{}", result)
                    },
                    Statement::ForStatement { iterator, iterable, body } => {
                        let mut result = format!("for ({} in {}) {{
", iterator, Node::Expression(iterable.clone()));
                        for stmt in body {
                            result.push_str(&format!("{}", Node::Statement(stmt.clone())));
                        }
                        result.push_str("}");
                        
                        write!(f, "{}", result)
                    },
                    Statement::BreakStatement => write!(f, "break;"),
                    Statement::ContinueStatement => write!(f, "continue;"),
                    Statement::ShowStatement { value, color } => {
                        if let Some(c) = color {
                            write!(f, "show({}) {};", c, Node::Expression(value.clone()))
                        } else {
                            write!(f, "show {};", Node::Expression(value.clone()))
                        }
                    },
                    Statement::TryStatement { try_block, catch_param, catch_block, finally_block } => {
                        let mut result = String::from("try {
");
                        for stmt in try_block {
                            result.push_str(&format!("{}", Node::Statement(stmt.clone())));
                        }
                        result.push_str("}");
                        
                        if let Some(catch) = catch_block {
                            result.push_str(" catch {
");
                            for stmt in catch {
                                result.push_str(&format!("{}", Node::Statement(stmt.clone())));
                            }
                            result.push_str("}");
                        }
                        
                        if let Some(finally) = finally_block {
                            result.push_str(" finally {
");
                            for stmt in finally {
                                result.push_str(&format!("{}", Node::Statement(stmt.clone())));
                            }
                            result.push_str("}");
                        }
                        
                        write!(f, "{}", result)
                    },
                    Statement::ThrowStatement { value } => {
                        write!(f, "throw {};", Node::Expression(value.clone()))
                    },
                    Statement::ReadStatement { name } => {
                        write!(f, "read {};", name)
                    },
                    Statement::ExitStatement => {
                        write!(f, "exit;")
                    },
                    Statement::DocumentTypeDeclaration { doc_type } => {
                        write!(f, "type {};", doc_type)
                    },
                    Statement::ModuleImport { names, alias, source } => {
                        let names_str = names.join(", ");
                        if let Some(alias_name) = alias {
                            write!(f, "use {} as {} from \"{}\";", names_str, alias_name, source)
                        } else {
                            write!(f, "use {} from \"{}\";", names_str, source)
                        }
                    },
                    Statement::ModuleExport { name } => {
                        write!(f, "export {};", name)
                    },
                    Statement::DebugStatement { value } => {
                        write!(f, "debug {};", Node::Expression(value.clone()))
                    },
                    Statement::AssertStatement { condition, message } => {
                        if let Some(msg) = message {
                            write!(f, "assert({}, {});", Node::Expression(condition.clone()), Node::Expression(msg.clone()))
                        } else {
                            write!(f, "assert({});", Node::Expression(condition.clone()))
                        }
                    },
                    Statement::TraceStatement { value } => {
                        write!(f, "trace {};", Node::Expression(value.clone()))
                    },
                    // OOP (Section 12)
                    Statement::ClassDeclaration { name, body } => {
                        let mut body_str = String::new();
                        for stmt in body {
                            body_str.push_str(&format!("{}", Node::Statement(stmt.clone())));
                        }
                        write!(f, "class {} {{
{}
}}", name, body_str)
                    },
                    // API Integration (Section 13)
                    Statement::ApiDeclaration { name, url } => {
                        write!(f, "api {} = from(\"{}\");", name, url)
                    },
                    Statement::ApiCall { name, body } => {
                        let mut body_str = String::new();
                        for stmt in body {
                            body_str.push_str(&format!("{}", Node::Statement(stmt.clone())));
                        }
                        write!(f, "call {} {{
{}
}}", name, body_str)
                    },
                    // Connect and From (Section 14)
                    Statement::ConnectStatement { name, url, options } => {
                        let mut options_str = String::new();
                        if !options.is_empty() {
                            options_str.push_str(" {\n");
                            for (option_name, option_value) in options {
                                options_str.push_str(&format!("    {} {};\n", option_name, Node::Expression(option_value.clone())));
                            }
                            options_str.push_str("}");
                        }
                        write!(f, "connect {} = from(\"{}\"){}", name, url, options_str)
                    },
                    // Import/Export (Section 15)
                    Statement::ImportStatement { imports, path } => {
                        let imports_str = imports.join(", ");
                        write!(f, "import {{{}}} from({});", imports_str, path)
                    },
                    // Libraries (Section 16)
                    Statement::LibStatement { name } => {
                        write!(f, "lib {};", name)
                    },
                    Statement::LoadStatement { cycles, block } => {
                        let mut result = format!("load ({}) {{\n", Node::Expression(cycles.clone()));
                        for stmt in block {
                            result.push_str(&format!("    {}", Node::Statement(stmt.clone())));
                        }
                        result.push_str("\n}");
                        write!(f, "{}", result)
                    },
                    // Performance and Type Safety Keywords
                    Statement::ConstDeclaration { name, value } => {
                        write!(f, "const {} = {};", name, Node::Expression(value.clone()))
                    },
                    Statement::EnumDeclaration { name, variants } => {
                        let mut result = format!("enum {} {{\n", name);
                        for (variant_name, variant_value) in variants {
                            if let Some(value) = variant_value {
                                result.push_str(&format!("    {} = {},\n", variant_name, Node::Expression(value.clone())));
                            } else {
                                result.push_str(&format!("    {},\n", variant_name));
                            }
                        }
                        result.push_str("}");
                        write!(f, "{}", result)
                    },
                    Statement::InlineFunctionDeclaration { name, parameters, body } => {
                        let params = parameters.join(", ");
                        let mut result = format!("inline fun {}({}) {{\n", name, params);
                        for stmt in body {
                            result.push_str(&format!("    {}", Node::Statement(stmt.clone())));
                        }
                        result.push_str("\n}");
                        write!(f, "{}", result)
                    },
                    Statement::FinalClassDeclaration { name, body } => {
                        let mut result = format!("final class {} {{\n", name);
                        for stmt in body {
                            result.push_str(&format!("    {}", Node::Statement(stmt.clone())));
                        }
                        result.push_str("\n}");
                        write!(f, "{}", result)
                    },
                    Statement::VolatileDeclaration { var_type, name, value } => {
                        if let Some(val) = value {
                            write!(f, "volatile {} {} = {};", var_type, name, Node::Expression(val.clone()))
                        } else {
                            write!(f, "volatile {} {};", var_type, name)
                        }
                    },
                }
            },
            Node::Expression(expr) => {
                match expr {
                    Expression::Identifier(name) => write!(f, "{}", name),
                    Expression::StringLiteral(value) => write!(f, "\"{}\"", value),
                    Expression::NumberLiteral(value) => write!(f, "{}", value),
                    Expression::BooleanLiteral(value) => write!(f, "{}", value),
                    Expression::NullLiteral => write!(f, "null"),
                    Expression::PrefixExpression { operator, right } => {
                        write!(f, "({}{})", operator, Node::Expression(*right.clone()))
                    },
                    Expression::InfixExpression { left, operator, right } => {
                        write!(f, "({} {} {})", Node::Expression(*left.clone()), operator, Node::Expression(*right.clone()))
                    },
                    Expression::AssignmentExpression { left, operator, right } => {
                        write!(f, "({} {} {})", Node::Expression(*left.clone()), operator, Node::Expression(*right.clone()))
                    },
                    Expression::CallExpression { function, arguments } => {
                        let mut args = Vec::new();
                        for arg in arguments {
                            args.push(format!("{}", Node::Expression(arg.clone())));
                        }
                        let args_str = args.join(", ");
                        write!(f, "{}({})", Node::Expression(*function.clone()), args_str)
                    },
                    Expression::ArrayLiteral { elements } => {
                        let mut elems = Vec::new();
                        for elem in elements {
                            elems.push(format!("{}", Node::Expression(elem.clone())));
                        }
                        let elems_str = elems.join(", ");
                        write!(f, "[{}]", elems_str)
                    },
                    Expression::IndexExpression { left, index } => {
                        write!(f, "({}[{}])", Node::Expression(*left.clone()), Node::Expression(*index.clone()))
                    },
                    Expression::MapLiteral { pairs } => {
                        let mut pairs_vec = Vec::new();
                        for (key, value) in pairs {
                            pairs_vec.push(format!("{}: {}", Node::Expression(key.clone()), Node::Expression(value.clone())));
                        }
                        let pairs_str = pairs_vec.join(", ");
                        write!(f, "{{{}}}", pairs_str)
                    },
                    Expression::LibraryCall { library, function, arguments } => {
                        let mut args = Vec::new();
                        for arg in arguments {
                            args.push(format!("{}", Node::Expression(arg.clone())));
                        }
                        let args_str = args.join(", ");
                        write!(f, "{}[{}].call({})", Node::Expression(*library.clone()), Node::Expression(*function.clone()), args_str)
                    },
                    Expression::NamespaceCall { namespace, function, arguments } => {
                        let mut args = Vec::new();
                        for arg in arguments {
                            args.push(format!("{}", Node::Expression(arg.clone())));
                        }
                        let args_str = args.join(", ");
                        write!(f, "{}::{}({})", namespace, function, args_str)
                    },
                }
            },
        }
    }
}