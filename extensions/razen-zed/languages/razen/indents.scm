; Indentation rules for Razen

; Increase indent for blocks
(block_statement
  "{" @start
  "}" @end) @indent

; Increase indent for function bodies
(function_declaration
  body: (block_statement) @indent)

(method_declaration
  body: (block_statement) @indent)

; Increase indent for control flow
(if_statement
  consequence: (block_statement) @indent)

(elif_clause
  consequence: (block_statement) @indent)

(else_clause
  alternative: (block_statement) @indent)

(while_statement
  body: (block_statement) @indent)

(for_statement
  body: (block_statement) @indent)

(match_statement
  "{" @start
  "}" @end) @indent

; Increase indent for struct/enum/impl
(struct_declaration
  "{" @start
  "}" @end) @indent

(enum_declaration
  "{" @start
  "}" @end) @indent

(impl_block
  "{" @start
  "}" @end) @indent

; Increase indent for arrays and maps
(array_literal
  "[" @start
  "]" @end) @indent

(map_literal
  "{" @start
  "}" @end) @indent
