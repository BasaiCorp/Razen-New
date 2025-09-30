# Razen Zed Extension Development Guide

This guide explains how to develop and test the Razen extension for Zed.

## Prerequisites

- Rust installed via rustup (required by Zed for extensions)
- Node.js and npm (for Tree-sitter grammar)
- Zed editor installed

## Project Structure

```
razen-zed/
├── extension.toml              # Extension metadata
├── languages/
│   └── razen/
│       ├── config.toml         # Language configuration
│       ├── highlights.scm      # Syntax highlighting rules
│       ├── brackets.scm        # Bracket matching
│       ├── outline.scm         # Code outline/structure
│       ├── indents.scm         # Auto-indentation rules
│       └── injections.scm      # Code injections
└── tree-sitter-razen/          # Tree-sitter grammar
    ├── grammar.js              # Grammar definition
    ├── package.json            # npm package config
    └── Cargo.toml              # Rust package config
```

## Development Workflow

### 1. Building the Tree-sitter Grammar

```bash
cd tree-sitter-razen

# Install dependencies
npm install

# Generate the parser
npx tree-sitter generate

# Test the grammar
npx tree-sitter test

# Parse a sample file
npx tree-sitter parse ../test.rzn
```

### 2. Testing in Zed

#### Install as Dev Extension

1. Open Zed
2. Press `Cmd+Shift+P` (Mac) or `Ctrl+Shift+P` (Linux/Windows)
3. Type "Install Dev Extension"
4. Select the `razen-zed` directory

#### View Logs

To see debug output:

```bash
# Open Zed with foreground logging
zed --foreground
```

Or use the command palette:
- Press `Cmd+Shift+P` / `Ctrl+Shift+P`
- Type "Open Log"
- Select "Zed: Open Log"

### 3. Making Changes

#### Modifying Syntax Highlighting

Edit `languages/razen/highlights.scm`:

```scheme
; Add new highlight rule
(your_node_name) @your_highlight_type
```

Available highlight types:
- `@keyword`, `@keyword.control`
- `@function`, `@function.call`
- `@variable`, `@variable.parameter`
- `@type`, `@type.builtin`
- `@string`, `@string.special`
- `@number`, `@number.float`
- `@comment`
- `@operator`
- `@punctuation.bracket`, `@punctuation.delimiter`
- `@constant`, `@constant.builtin`
- `@property`

#### Modifying Grammar

Edit `tree-sitter-razen/grammar.js`:

```javascript
// Add new rule
your_rule: $ => seq(
  'keyword',
  field('name', $.identifier),
  // ...
),
```

After changes:
```bash
npx tree-sitter generate
```

#### Testing Grammar Changes

Create test files in `tree-sitter-razen/test/corpus/`:

```
================================================================================
Test name
================================================================================

fun main() {
    println("Hello")
}

--------------------------------------------------------------------------------

(source_file
  (function_declaration
    name: (identifier)
    parameters: (parameter_list)
    body: (block_statement
      (expression_statement
        (call_expression
          function: (identifier)
          arguments: (argument_list
            (string)))))))
```

Run tests:
```bash
npx tree-sitter test
```

### 4. Debugging

#### Check Grammar Parsing

```bash
# Parse a file and see the syntax tree
npx tree-sitter parse test.rzn

# Highlight a file
npx tree-sitter highlight test.rzn
```

#### Check Queries

```bash
# Test highlight queries
npx tree-sitter highlight --query-path languages/razen/highlights.scm test.rzn
```

## Common Issues

### Extension Not Loading

1. Check `extension.toml` syntax
2. Verify grammar repository URL
3. Check Zed logs for errors
4. Ensure Rust is installed via rustup

### Syntax Highlighting Not Working

1. Verify Tree-sitter grammar generates correctly
2. Check `highlights.scm` syntax
3. Ensure node names match grammar
4. Test queries with `tree-sitter highlight`

### Indentation Issues

1. Check `indents.scm` rules
2. Verify `@indent`, `@start`, `@end` captures
3. Test with various code samples

## Publishing Checklist

Before publishing:

- [ ] Grammar generates without errors
- [ ] All tests pass
- [ ] Syntax highlighting works correctly
- [ ] Bracket matching works
- [ ] Code outline shows correctly
- [ ] Auto-indentation works
- [ ] README is complete
- [ ] Version numbers are updated
- [ ] License file is included

## Resources

- [Zed Extension Docs](https://zed.dev/docs/extensions)
- [Tree-sitter Documentation](https://tree-sitter.github.io)
- [Tree-sitter Grammar Writing](https://tree-sitter.github.io/tree-sitter/creating-parsers)
- [Zed Extensions Repository](https://github.com/zed-industries/extensions)

## Tips

1. **Start Simple**: Begin with basic syntax highlighting, then add complexity
2. **Test Incrementally**: Test each change before moving to the next
3. **Use Examples**: Look at other Zed extensions for reference
4. **Check Logs**: Always check Zed logs when debugging
5. **Grammar First**: Get the grammar right before worrying about queries

## Getting Help

- Check Zed Discord community
- Review existing extensions in zed-industries/extensions
- Open issues on GitHub
- Read Tree-sitter documentation

---

Happy developing! 🚀
