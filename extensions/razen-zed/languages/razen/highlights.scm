; Highlights for Razen language

; Keywords
[
  "var"
  "const"
  "fun"
  "struct"
  "enum"
  "impl"
  "use"
  "pub"
  "as"
  "from"
  "mod"
] @keyword

; Control flow keywords
[
  "if"
  "else"
  "elif"
  "while"
  "for"
  "in"
  "match"
  "return"
  "break"
  "continue"
  "try"
  "catch"
  "throw"
] @keyword.control

; Type keywords
[
  "int"
  "float"
  "str"
  "bool"
  "char"
  "any"
] @type.builtin

; Special keywords
"self" @variable.special

; Operators
[
  "+"
  "-"
  "*"
  "/"
  "%"
  "**"
  "="
  "+="
  "-="
  "*="
  "/="
  "%="
  "=="
  "!="
  "<"
  ">"
  "<="
  ">="
  "&&"
  "||"
  "!"
  "&"
  "|"
  "^"
  "~"
  "<<"
  ">>"
  "++"
  "--"
  "?"
  "??"
  ".."
  "..="
  "..."
] @operator

; Punctuation
[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

[
  ","
  "."
  ":"
  "::"
  ";"
] @punctuation.delimiter

[
  "->"
  "=>"
] @punctuation.special

; Literals
(integer) @number
(float) @number.float
(string) @string
(interpolated_string) @string.special
(boolean) @constant.builtin
(null) @constant.builtin

; Comments
(comment) @comment

; Function declarations
(function_declaration
  name: (identifier) @function)

(method_declaration
  name: (identifier) @function.method)

; Function calls
(call_expression
  function: (identifier) @function.call)

(method_call_expression
  method: (identifier) @function.method.call)

; Parameters
(parameter
  name: (identifier) @variable.parameter)

; Struct declarations
(struct_declaration
  name: (identifier) @type)

(struct_field
  name: (identifier) @property)

; Enum declarations
(enum_declaration
  name: (identifier) @type)

; Impl blocks
(impl_block
  type: (identifier) @type)

; Type annotations
(type_annotation) @type

; Variables
(variable_declaration
  name: (identifier) @variable)

(constant_declaration
  name: (identifier) @constant)

; Identifiers (general)
(identifier) @variable

; Member access
(member_expression
  property: (identifier) @property)

; Struct instantiation
(struct_instantiation
  type: (identifier) @type)

(struct_field_init
  name: (identifier) @property)

; Use statements
(use_statement
  path: (string) @string.special)

(use_statement
  alias: (identifier) @namespace)

; String interpolation
(interpolation) @embedded
