; Razen syntax highlighting

; Comments
(comment) @comment

; Literals
(integer) @number
(float) @number
(string) @string
(boolean) @constant.builtin

; String interpolation
(interpolated_string) @string
(string_content) @string
(interpolation) @embedded

; Functions
(function_declaration
  name: (identifier) @function)

(method_declaration
  name: (identifier) @function)

(call_expression
  function: (identifier) @function)

(method_call_expression
  method: (identifier) @function)

; Types
(struct_declaration
  name: (identifier) @type)

(enum_declaration
  name: (identifier) @type)

(impl_block
  type: (identifier) @type)

(struct_instantiation
  type: (identifier) @type)

; Properties
(struct_field
  name: (identifier) @property)

(struct_field_init
  name: (identifier) @property)

(member_expression
  property: (identifier) @property)

; Variables
(variable_declaration
  name: (identifier) @variable)

(constant_declaration
  name: (identifier) @constant)

(parameter
  name: (identifier) @variable.parameter)

; Identifiers
(identifier) @variable

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
  ";"
] @punctuation.delimiter

"=" @operator
