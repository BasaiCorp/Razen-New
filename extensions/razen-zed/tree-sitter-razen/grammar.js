/**
 * @file Razen grammar for tree-sitter
 * @author Razen Language Team
 * @license MIT
 * Based on actual Razen parser implementation
 */

/// <reference types="tree-sitter-cli/dsl" />

// Operator precedence levels (based on Razen parser)
const PREC = {
  call: 15,
  member: 15,
  index: 15,
  unary: 13,
  power: 12,
  multiplicative: 11,
  additive: 10,
  shift: 9,
  bitwise_and: 9,
  comparison: 8,
  equality: 7,
  bitwise_xor: 6,
  bitwise_or: 6,
  logical_and: 5,
  logical_or: 4,
  range: 3,
  assignment: 2,
};

module.exports = grammar({
  name: 'razen',

  extras: $ => [
    /\s/,
    $.comment,
  ],

  word: $ => $.identifier,

  conflicts: $ => [
    // Intentional ambiguities
    [$.struct_instantiation],
    [$.map_literal, $.block_statement],
    [$.method_call_expression, $.member_expression],
    [$.call_expression, $.member_expression],
  ],

  rules: {
    source_file: $ => repeat($._statement),

    // Comments
    comment: $ => token(choice(
      seq('//', /.*/),
      seq('/*', /[^*]*\*+([^/*][^*]*\*+)*/, '/')
    )),

    // Statements
    _statement: $ => choice(
      $.use_statement,
      $.variable_declaration,
      $.constant_declaration,
      $.function_declaration,
      $.struct_declaration,
      $.enum_declaration,
      $.impl_block,
      $.if_statement,
      $.while_statement,
      $.for_statement,
      $.match_statement,
      $.return_statement,
      $.break_statement,
      $.continue_statement,
      $.expression_statement,
      $.block_statement,
    ),

    // Module System
    use_statement: $ => seq(
      'use',
      field('path', $.string),
      optional(seq('as', field('alias', $.identifier))),
    ),

    // Variable Declaration
    variable_declaration: $ => seq(
      optional('pub'),
      'var',
      field('name', $.identifier),
      optional(seq(':', field('type', $.type_annotation))),
      optional(seq('=', field('value', $._expression))),
    ),

    // Constant Declaration
    constant_declaration: $ => seq(
      optional('pub'),
      'const',
      field('name', $.identifier),
      optional(seq(':', field('type', $.type_annotation))),
      '=',
      field('value', $._expression),
    ),

    // Function Declaration
    function_declaration: $ => seq(
      optional('pub'),
      'fun',
      field('name', $.identifier),
      field('parameters', $.parameter_list),
      optional(seq('->', field('return_type', $.type_annotation))),
      field('body', $.block_statement),
    ),

    parameter_list: $ => seq(
      '(',
      optional(seq(
        $.parameter,
        repeat(seq(',', $.parameter)),
        optional(',')
      )),
      ')'
    ),

    parameter: $ => seq(
      field('name', $.identifier),
      optional(seq(':', field('type', $.type_annotation))),
    ),

    // Struct Declaration
    struct_declaration: $ => seq(
      optional('pub'),
      'struct',
      field('name', $.identifier),
      '{',
      repeat($.struct_field),
      '}',
    ),

    struct_field: $ => seq(
      field('name', $.identifier),
      ':',
      field('type', $.type_annotation),
      optional(','),
    ),

    // Enum Declaration
    enum_declaration: $ => seq(
      optional('pub'),
      'enum',
      field('name', $.identifier),
      '{',
      repeat(seq($.identifier, optional(','))),
      '}',
    ),

    // Impl Block
    impl_block: $ => seq(
      'impl',
      field('type', $.identifier),
      '{',
      repeat($.method_declaration),
      '}',
    ),

    method_declaration: $ => seq(
      optional('pub'),
      'fun',
      field('name', $.identifier),
      field('parameters', $.parameter_list),
      optional(seq('->', field('return_type', $.type_annotation))),
      field('body', $.block_statement),
    ),

    // Control Flow Statements
    if_statement: $ => prec.right(seq(
      'if',
      field('condition', $._expression),
      field('consequence', $.block_statement),
      repeat($.elif_clause),
      optional($.else_clause),
    )),

    elif_clause: $ => seq(
      'elif',
      field('condition', $._expression),
      field('consequence', $.block_statement),
    ),

    else_clause: $ => seq(
      'else',
      field('alternative', $.block_statement),
    ),

    while_statement: $ => seq(
      'while',
      field('condition', $._expression),
      field('body', $.block_statement),
    ),

    for_statement: $ => seq(
      'for',
      field('variable', $.identifier),
      'in',
      field('iterable', $._expression),
      field('body', $.block_statement),
    ),

    match_statement: $ => seq(
      'match',
      field('value', $._expression),
      '{',
      repeat($.match_arm),
      '}',
    ),

    match_arm: $ => seq(
      field('pattern', choice(
        '_',
        $._expression
      )),
      '=>',
      field('body', $._expression),
      optional(','),
    ),

    // Jump Statements
    return_statement: $ => prec.right(seq(
      'return',
      optional($._expression),
    )),

    break_statement: $ => 'break',

    continue_statement: $ => 'continue',

    // Block Statement
    block_statement: $ => seq(
      '{',
      repeat($._statement),
      '}',
    ),

    // Expression Statement
    expression_statement: $ => prec.right($._expression),

    // Expressions
    _expression: $ => choice(
      $.assignment_expression,
      $.binary_expression,
      $.unary_expression,
      $.call_expression,
      $.method_call_expression,
      $.member_expression,
      $.index_expression,
      $.range_expression,
      $.array_literal,
      $.map_literal,
      $.struct_instantiation,
      $.interpolated_string,
      $.grouped_expression,
      $.identifier,
      $.self,
      $.integer,
      $.float,
      $.string,
      $.boolean,
      $.null,
    ),

    // Assignment Expression
    assignment_expression: $ => prec.right(PREC.assignment, seq(
      field('left', choice($.identifier, $.member_expression, $.index_expression)),
      field('operator', choice('=', '+=', '-=', '*=', '/=', '%=', '&=', '|=', '^=', '<<=', '>>=')),
      field('right', $._expression),
    )),

    // Binary Expression
    binary_expression: $ => choice(
      // Arithmetic
      prec.left(PREC.additive, seq($._expression, '+', $._expression)),
      prec.left(PREC.additive, seq($._expression, '-', $._expression)),
      prec.left(PREC.multiplicative, seq($._expression, '*', $._expression)),
      prec.left(PREC.multiplicative, seq($._expression, '/', $._expression)),
      prec.left(PREC.multiplicative, seq($._expression, '%', $._expression)),
      prec.left(PREC.power, seq($._expression, '**', $._expression)),
      
      // Comparison
      prec.left(PREC.equality, seq($._expression, '==', $._expression)),
      prec.left(PREC.equality, seq($._expression, '!=', $._expression)),
      prec.left(PREC.comparison, seq($._expression, '<', $._expression)),
      prec.left(PREC.comparison, seq($._expression, '>', $._expression)),
      prec.left(PREC.comparison, seq($._expression, '<=', $._expression)),
      prec.left(PREC.comparison, seq($._expression, '>=', $._expression)),
      
      // Logical
      prec.left(PREC.logical_and, seq($._expression, '&&', $._expression)),
      prec.left(PREC.logical_or, seq($._expression, '||', $._expression)),
      
      // Bitwise
      prec.left(PREC.bitwise_and, seq($._expression, '&', $._expression)),
      prec.left(PREC.bitwise_or, seq($._expression, '|', $._expression)),
      prec.left(PREC.bitwise_xor, seq($._expression, '^', $._expression)),
      prec.left(PREC.shift, seq($._expression, '<<', $._expression)),
      prec.left(PREC.shift, seq($._expression, '>>', $._expression)),
    ),

    // Unary Expression
    unary_expression: $ => prec(PREC.unary, choice(
      seq('-', $._expression),
      seq('!', $._expression),
      seq('~', $._expression),
      seq('++', $._expression),
      seq('--', $._expression),
    )),

    // Call Expression
    call_expression: $ => prec(PREC.call, seq(
      field('function', choice($.identifier, $.member_expression)),
      field('arguments', $.argument_list),
    )),

    argument_list: $ => seq(
      '(',
      optional(seq(
        $._expression,
        repeat(seq(',', $._expression)),
        optional(',')
      )),
      ')'
    ),

    // Method Call Expression
    method_call_expression: $ => prec(PREC.call, seq(
      field('object', $._expression),
      '.',
      field('method', $.identifier),
      field('arguments', $.argument_list),
    )),

    // Member Expression
    member_expression: $ => prec(PREC.member, seq(
      field('object', $._expression),
      '.',
      field('property', $.identifier),
    )),

    // Index Expression
    index_expression: $ => prec(PREC.index, seq(
      field('object', $._expression),
      '[',
      field('index', $._expression),
      ']',
    )),

    // Range Expression
    range_expression: $ => prec.left(PREC.range, choice(
      seq($._expression, '..', $._expression),
      seq($._expression, '..=', $._expression),
    )),

    // Array Literal
    array_literal: $ => seq(
      '[',
      optional(seq(
        $._expression,
        repeat(seq(',', $._expression)),
        optional(',')
      )),
      ']',
    ),

    // Map Literal
    map_literal: $ => seq(
      '{',
      optional(seq(
        $.map_entry,
        repeat(seq(',', $.map_entry)),
        optional(',')
      )),
      '}',
    ),

    map_entry: $ => seq(
      field('key', $._expression),
      ':',
      field('value', $._expression),
    ),

    // Struct Instantiation
    struct_instantiation: $ => prec(1, seq(
      field('type', $.identifier),
      '{',
      optional(seq(
        $.struct_field_init,
        repeat(seq(',', $.struct_field_init)),
        optional(',')
      )),
      '}',
    )),

    struct_field_init: $ => seq(
      field('name', $.identifier),
      ':',
      field('value', $._expression),
    ),

    // Interpolated String (f-string)
    interpolated_string: $ => seq(
      'f',
      '"',
      repeat(choice(
        $.string_content,
        $.interpolation,
      )),
      '"',
    ),

    string_content: $ => token(prec(-1, /[^"{\\]+/)),

    interpolation: $ => seq(
      '{',
      $._expression,
      '}',
    ),

    // Grouped Expression
    grouped_expression: $ => seq(
      '(',
      $._expression,
      ')',
    ),

    // Type Annotation
    type_annotation: $ => choice(
      'int',
      'float',
      'str',
      'bool',
      'char',
      'any',
      seq('[', $.type_annotation, ']'), // Array type
      seq('{', $.type_annotation, ':', $.type_annotation, '}'), // Map type
      $.identifier, // Custom type
    ),

    // Literals
    identifier: $ => token(/[a-zA-Z_][a-zA-Z0-9_]*/),

    wildcard: $ => '_',

    self: $ => 'self',

    integer: $ => /\d+/,

    float: $ => /\d+\.\d+/,

    string: $ => token(seq(
      '"',
      repeat(choice(
        /[^"\\\n]/,
        seq('\\', /./)
      )),
      '"',
    )),

    boolean: $ => choice('true', 'false'),

    null: $ => 'null',
  }
});
