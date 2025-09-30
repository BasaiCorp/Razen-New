# tree-sitter-razen

Tree-sitter grammar for the Razen programming language.

## Features

- Complete syntax support for Razen language
- Supports all Razen keywords and constructs
- Module system (use statements)
- Struct and impl blocks
- Control flow (if, while, for, match)
- F-string interpolation
- Range expressions
- And more!

## Installation

```bash
npm install tree-sitter-razen
```

## Usage with Tree-sitter

```javascript
const Parser = require('tree-sitter');
const Razen = require('tree-sitter-razen');

const parser = new Parser();
parser.setLanguage(Razen);

const sourceCode = `
fun main() {
    println("Hello, Razen!")
}
`;

const tree = parser.parse(sourceCode);
console.log(tree.rootNode.toString());
```

## Development

### Building

```bash
# Generate parser
npm install
npx tree-sitter generate

# Test
npx tree-sitter test
```

## License

MIT
