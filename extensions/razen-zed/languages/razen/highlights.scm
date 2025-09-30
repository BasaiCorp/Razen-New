; Razen syntax highlighting

(comment) @comment

(parameter
  name: (identifier) @parameter)

(function_declaration
  name: (identifier) @function)

(method_declaration
  name: (identifier) @function)

(call_expression
  function: (identifier) @function)

(method_call_expression
  method: (identifier) @function)

(struct_declaration
  name: (identifier) @type)

(enum_declaration
  name: (identifier) @type)

(impl_block
  type: (identifier) @type)

(struct_instantiation
  type: (identifier) @type)

(struct_field
  name: (identifier) @property)

(struct_field_init
  name: (identifier) @property)

(member_expression
  property: (identifier) @property)

(variable_declaration
  name: (identifier) @variable)

(constant_declaration
  name: (identifier) @constant)

(integer) @number
(float) @number
(string) @string
(interpolated_string) @string
(string_content) @string
(interpolation) @embedded

; Keywords as statements
(break_statement) @keyword
(continue_statement) @keyword
(return_statement) @keyword

; Keywords as string literals (anonymous tokens)
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
 "if"
 "else"
 "elif"
 "while"
 "for"
 "in"
 "match"
 "try"
 "catch"
 "throw"
 "self"
] @keyword

[
 "true"
 "false"
] @boolean

[
 "."
 ","
 ":"
 ";"
] @punctuation.delimiter

[
 "("
 ")"
 "{"
 "}"
 "["
 "]"
] @punctuation.bracket

[
 "+"
 "-"
 "*"
 "/"
 "%"
 "="
 "=="
 "!="
 "<"
 ">"
 "<="
 ">="
 "!"
 "&&"
 "||"
 "&"
 "|"
 "^"
 "~"
] @operator

(identifier) @variable
