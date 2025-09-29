"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.deactivate = exports.activate = void 0;
const vscode = require("vscode");
const path = require("path");
function activate(context) {
    console.log('Razen Language Support extension is now active!');
    // Show welcome message on first activation
    const isFirstTime = context.globalState.get('razen.firstTime', true);
    if (isFirstTime) {
        vscode.window.showInformationMessage('Welcome to Razen Language Support! Create a .rzn file to get started.', 'Create Sample File', 'Learn More').then(selection => {
            if (selection === 'Create Sample File') {
                createSampleFile();
            }
            else if (selection === 'Learn More') {
                vscode.env.openExternal(vscode.Uri.parse('https://github.com/razen-lang/razen'));
            }
        });
        context.globalState.update('razen.firstTime', false);
    }
    // Register completion provider
    const completionProvider = vscode.languages.registerCompletionItemProvider('razen', {
        provideCompletionItems(document, position) {
            const completionItems = [];
            // Get current line context for smarter completions
            const lineText = document.lineAt(position).text;
            const linePrefix = lineText.substring(0, position.character);
            // Keywords with enhanced information
            const keywords = [
                { name: 'const', detail: 'Declare immutable constant', kind: vscode.CompletionItemKind.Keyword },
                { name: 'var', detail: 'Declare mutable variable', kind: vscode.CompletionItemKind.Keyword },
                { name: 'fun', detail: 'Declare function', kind: vscode.CompletionItemKind.Keyword },
                { name: 'struct', detail: 'Define structure', kind: vscode.CompletionItemKind.Keyword },
                { name: 'enum', detail: 'Define enumeration', kind: vscode.CompletionItemKind.Keyword },
                { name: 'impl', detail: 'Implementation block', kind: vscode.CompletionItemKind.Keyword },
                { name: 'self', detail: 'Self reference', kind: vscode.CompletionItemKind.Keyword },
                { name: 'new', detail: 'Constructor method', kind: vscode.CompletionItemKind.Method },
                { name: 'if', detail: 'Conditional statement', kind: vscode.CompletionItemKind.Keyword },
                { name: 'else', detail: 'Alternative condition', kind: vscode.CompletionItemKind.Keyword },
                { name: 'elif', detail: 'Else if condition', kind: vscode.CompletionItemKind.Keyword },
                { name: 'while', detail: 'While loop', kind: vscode.CompletionItemKind.Keyword },
                { name: 'for', detail: 'For loop', kind: vscode.CompletionItemKind.Keyword },
                { name: 'in', detail: 'Iterator keyword', kind: vscode.CompletionItemKind.Keyword },
                { name: 'return', detail: 'Return from function', kind: vscode.CompletionItemKind.Keyword },
                { name: 'break', detail: 'Break from loop', kind: vscode.CompletionItemKind.Keyword },
                { name: 'continue', detail: 'Continue loop iteration', kind: vscode.CompletionItemKind.Keyword },
                { name: 'match', detail: 'Pattern matching', kind: vscode.CompletionItemKind.Keyword },
                { name: 'try', detail: 'Exception handling', kind: vscode.CompletionItemKind.Keyword },
                { name: 'catch', detail: 'Catch exceptions', kind: vscode.CompletionItemKind.Keyword },
                { name: 'throw', detail: 'Throw exception', kind: vscode.CompletionItemKind.Keyword },
                { name: 'mod', detail: 'Module declaration', kind: vscode.CompletionItemKind.Module },
                { name: 'use', detail: 'Import module', kind: vscode.CompletionItemKind.Module },
                { name: 'pub', detail: 'Public visibility', kind: vscode.CompletionItemKind.Keyword },
                { name: 'from', detail: 'Import from module', kind: vscode.CompletionItemKind.Module },
                { name: 'as', detail: 'Alias import', kind: vscode.CompletionItemKind.Module }
            ];
            keywords.forEach(keyword => {
                const item = new vscode.CompletionItem(keyword.name, keyword.kind);
                item.detail = keyword.detail;
                item.documentation = new vscode.MarkdownString(`**${keyword.name}** - ${keyword.detail}`);
                completionItems.push(item);
            });
            // Types
            const types = ['int', 'str', 'bool', 'char', 'array', 'map', 'any', 'float'];
            types.forEach(type => {
                const item = new vscode.CompletionItem(type, vscode.CompletionItemKind.TypeParameter);
                item.detail = `Razen type: ${type}`;
                completionItems.push(item);
            });
            // Built-in functions
            const builtins = [
                { name: 'print', detail: 'Print without newline', params: '(value: any)' },
                { name: 'println', detail: 'Print with newline', params: '(value: any)' },
                { name: 'printc', detail: 'Print with color', params: '(value: any, color: str)' },
                { name: 'printlnc', detail: 'Print with color and newline', params: '(value: any, color: str)' },
                { name: 'input', detail: 'Get user input', params: '(prompt?: str) -> str' },
                { name: 'read', detail: 'Read file contents', params: '(filename: str) -> str' },
                { name: 'write', detail: 'Write to file', params: '(filename: str, content: str) -> bool' },
                { name: 'open', detail: 'Open file handle', params: '(filename: str)' },
                { name: 'close', detail: 'Close file handle', params: '(handle)' }
            ];
            builtins.forEach(builtin => {
                const item = new vscode.CompletionItem(builtin.name, vscode.CompletionItemKind.Function);
                item.detail = builtin.detail;
                item.documentation = new vscode.MarkdownString(`**${builtin.name}**${builtin.params}\n\n${builtin.detail}`);
                item.insertText = new vscode.SnippetString(`${builtin.name}($1)`);
                completionItems.push(item);
            });
            return completionItems;
        }
    }, '.' // Trigger completion on dot
    );
    // Register hover provider
    const hoverProvider = vscode.languages.registerHoverProvider('razen', {
        provideHover(document, position, token) {
            const range = document.getWordRangeAtPosition(position);
            const word = document.getText(range);
            // Provide hover information for keywords and built-ins
            const hoverInfo = {
                'fun': 'Declares a function in Razen',
                'var': 'Declares a mutable variable',
                'const': 'Declares an immutable constant',
                'struct': 'Defines a structured data type',
                'enum': 'Defines an enumeration type',
                'impl': 'Implementation block for methods',
                'self': 'Reference to current instance',
                'new': 'Constructor method',
                'pub': 'Public visibility modifier',
                'use': 'Import module or library',
                'if': 'Conditional statement',
                'while': 'Loop statement',
                'for': 'Iteration statement',
                'match': 'Pattern matching statement',
                'try': 'Exception handling block',
                'println': 'Built-in function to print with newline',
                'print': 'Built-in function to print without newline',
                'printc': 'Built-in function to print with color',
                'printlnc': 'Built-in function to print with color and newline',
                'input': 'Built-in function to get user input',
                'int': 'Integer type',
                'str': 'String type',
                'bool': 'Boolean type',
                'char': 'Character type',
                'float': 'Floating point number type',
                'array': 'Array/list type',
                'map': 'Hash map/dictionary type',
                'any': 'Any type (accepts all values)'
            };
            if (hoverInfo[word]) {
                return new vscode.Hover(new vscode.MarkdownString(`**${word}**: ${hoverInfo[word]}`));
            }
            return null;
        }
    });
    // Register document symbol provider
    const symbolProvider = vscode.languages.registerDocumentSymbolProvider('razen', {
        provideDocumentSymbols(document) {
            const symbols = [];
            const text = document.getText();
            const lines = text.split('\n');
            for (let i = 0; i < lines.length; i++) {
                const line = lines[i];
                // Function declarations
                const funMatch = line.match(/^\s*(pub\s+)?fun\s+(\w+)\s*\(/);
                if (funMatch) {
                    const name = funMatch[2];
                    const range = new vscode.Range(i, 0, i, line.length);
                    const symbol = new vscode.DocumentSymbol(name, 'Function', vscode.SymbolKind.Function, range, range);
                    symbols.push(symbol);
                }
                // Struct declarations
                const structMatch = line.match(/^\s*struct\s+(\w+)/);
                if (structMatch) {
                    const name = structMatch[1];
                    const range = new vscode.Range(i, 0, i, line.length);
                    const symbol = new vscode.DocumentSymbol(name, 'Struct', vscode.SymbolKind.Struct, range, range);
                    symbols.push(symbol);
                }
                // Enum declarations
                const enumMatch = line.match(/^\s*enum\s+(\w+)/);
                if (enumMatch) {
                    const name = enumMatch[1];
                    const range = new vscode.Range(i, 0, i, line.length);
                    const symbol = new vscode.DocumentSymbol(name, 'Enum', vscode.SymbolKind.Enum, range, range);
                    symbols.push(symbol);
                }
                // Variable declarations
                const varMatch = line.match(/^\s*(var|const)\s+(\w+)/);
                if (varMatch) {
                    const name = varMatch[2];
                    const kind = varMatch[1] === 'const' ? vscode.SymbolKind.Constant : vscode.SymbolKind.Variable;
                    const range = new vscode.Range(i, 0, i, line.length);
                    const symbol = new vscode.DocumentSymbol(name, varMatch[1], kind, range, range);
                    symbols.push(symbol);
                }
            }
            return symbols;
        }
    });
    // Register formatting provider
    const formattingProvider = vscode.languages.registerDocumentFormattingEditProvider('razen', {
        provideDocumentFormattingEdits(document) {
            const edits = [];
            const text = document.getText();
            const lines = text.split('\n');
            let indentLevel = 0;
            const indentSize = 4; // 4 spaces
            for (let i = 0; i < lines.length; i++) {
                const line = lines[i];
                const trimmedLine = line.trim();
                if (trimmedLine === '')
                    continue;
                // Decrease indent for closing braces
                if (trimmedLine.startsWith('}')) {
                    indentLevel = Math.max(0, indentLevel - 1);
                }
                const expectedIndent = ' '.repeat(indentLevel * indentSize);
                const currentIndent = line.match(/^\s*/)?.[0] || '';
                if (currentIndent !== expectedIndent) {
                    const range = new vscode.Range(i, 0, i, currentIndent.length);
                    edits.push(vscode.TextEdit.replace(range, expectedIndent));
                }
                // Increase indent for opening braces
                if (trimmedLine.endsWith('{')) {
                    indentLevel++;
                }
            }
            return edits;
        }
    });
    // Add all providers to context
    context.subscriptions.push(completionProvider, hoverProvider, symbolProvider, formattingProvider);
    // Register commands
    const compileCommand = vscode.commands.registerCommand('razen.compile', () => {
        const activeEditor = vscode.window.activeTextEditor;
        if (!activeEditor) {
            vscode.window.showErrorMessage('No active Razen file to compile');
            return;
        }
        const document = activeEditor.document;
        if (document.languageId !== 'razen') {
            vscode.window.showErrorMessage('Active file is not a Razen file');
            return;
        }
        // Save the file first
        document.save().then(() => {
            const inputFile = document.fileName;
            const outputFile = inputFile.replace(/\.(rzn|razen)$/i, '') + '.exe';
            const terminal = vscode.window.createTerminal('Razen Compiler');
            terminal.show();
            terminal.sendText(`razen compile "${inputFile}" -o "${outputFile}"`);
        });
    });
    const runCommand = vscode.commands.registerCommand('razen.run', () => {
        const activeEditor = vscode.window.activeTextEditor;
        if (!activeEditor) {
            vscode.window.showErrorMessage('No active Razen file to run');
            return;
        }
        const document = activeEditor.document;
        if (document.languageId !== 'razen') {
            vscode.window.showErrorMessage('Active file is not a Razen file');
            return;
        }
        // Save the file first
        document.save().then(() => {
            const terminal = vscode.window.createTerminal('Razen Runner');
            terminal.show();
            terminal.sendText(`razen run "${document.fileName}"`);
        });
    });
    const buildCommand = vscode.commands.registerCommand('razen.build', () => {
        const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
        if (!workspaceFolder) {
            vscode.window.showErrorMessage('No workspace folder open. Build requires a Razen project with razen.toml');
            return;
        }
        // Check if razen.toml exists
        const razenTomlPath = vscode.Uri.joinPath(workspaceFolder.uri, 'razen.toml');
        vscode.workspace.fs.stat(razenTomlPath).then(() => {
            const terminal = vscode.window.createTerminal('Razen Builder');
            terminal.show();
            terminal.sendText(`cd "${workspaceFolder.uri.fsPath}" && razen build`);
        }, () => {
            vscode.window.showErrorMessage('No razen.toml found. Use "razen init" to create a new project or open a Razen project directory.');
        });
    });
    // Register additional commands
    const createSampleCommand = vscode.commands.registerCommand('razen.createSample', createSampleFile);
    const showDocumentationCommand = vscode.commands.registerCommand('razen.showDocumentation', () => {
        vscode.env.openExternal(vscode.Uri.parse('https://github.com/razen-lang/razen'));
    });
    context.subscriptions.push(completionProvider, hoverProvider, symbolProvider, formattingProvider, compileCommand, runCommand, buildCommand, createSampleCommand, showDocumentationCommand);
    // Simple status bar for Razen files
    const statusBarItem = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Right, 100);
    statusBarItem.command = 'razen.showDocumentation';
    statusBarItem.text = 'Razen';
    statusBarItem.tooltip = 'Razen Language Documentation';
    // Show status bar item only for Razen files
    const updateStatusBar = () => {
        const activeEditor = vscode.window.activeTextEditor;
        if (activeEditor && activeEditor.document.languageId === 'razen') {
            statusBarItem.show();
        }
        else {
            statusBarItem.hide();
        }
    };
    vscode.window.onDidChangeActiveTextEditor(updateStatusBar);
    updateStatusBar();
    context.subscriptions.push(statusBarItem);
}
exports.activate = activate;
// Helper function to create a sample Razen file
async function createSampleFile() {
    const sampleContent = `// Welcome to Razen Programming Language v0.1-beta.7
// This is a comprehensive sample file showcasing Razen features

// Struct definition with fields
struct Person {
    name: str,
    age: int
}

// Implementation block with methods
impl Person {
    // Constructor method
    fun new(name: str, age: int) {
        return Person { name: name, age: age }
    }

    // Instance method with self parameter
    fun greet(self) {
        printlnc(f"Hello, I'm {self.name}!", "green")
    }

    // Method with parameters
    fun introduce(self, to: str) {
        printlnc(f"Hi {to}, I'm {self.name} and I'm {self.age} years old.", "cyan")
    }
}

/// Main function - entry point of the program
fun main() {
    // Colored output demonstration
    printlnc("=== Razen Language Demo ===", "yellow")

    // Variable declarations with type inference
    var name = "Developer"
    const version = "0.1-beta.7"

    // F-string interpolation
    println(f"Welcome {name} to Razen v{version}")

    // Object-oriented programming demonstration
    var person = Person.new("Alice", 25)
    person.greet()
    person.introduce("Bob")

    // Function calls
    var result = add(10, 20)
    printlnc(f"10 + 20 = {result}", "blue")

    // Control flow
    if result > 25 {
        printlnc("Result is greater than 25!", "green")
    }

    // Different loop types
    printlnc("Range loops:", "magenta")
    for i in 1..4 {
        print(f"{i} ")
    }
    println("")

    // Inclusive range
    printlnc("Inclusive range (1..=3):", "magenta")
    for i in 1..=3 {
        print(f"{i} ")
    }
    println("")

    // Array iteration
    printlnc("Array iteration:", "magenta")
    for item in [10, 20, 30] {
        print(f"{item} ")
    }
    println("")

    // String array iteration
    for person_name in ["Alice", "Bob", "Charlie"] {
        printlnc(f"Hello, {person_name}!", "cyan")
    }

    printlnc("=== Demo Complete! ===", "yellow")
}

/// Add two numbers together
fun add(a: int, b: int) {
    return a + b
}

// Try these features:
// 1. Type 'main' + Tab for main function snippet
// 2. Type 'impl' + Tab for implementation block
// 3. Type 'struct' + Tab for struct definition
// 4. Use Ctrl+Shift+P and search for "Razen" commands
// 5. Hover over keywords for documentation
// 6. Right-click for context menu commands
`;
    try {
        const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
        const fileName = 'sample.rzn';
        let filePath;
        if (workspaceFolder) {
            filePath = vscode.Uri.joinPath(workspaceFolder.uri, fileName);
        }
        else {
            // If no workspace, create in temp directory
            const tempDir = require('os').tmpdir();
            filePath = vscode.Uri.file(path.join(tempDir, fileName));
        }
        await vscode.workspace.fs.writeFile(filePath, Buffer.from(sampleContent, 'utf8'));
        const document = await vscode.workspace.openTextDocument(filePath);
        await vscode.window.showTextDocument(document);
        vscode.window.showInformationMessage('Sample Razen file created! Try the features mentioned in the comments.');
    }
    catch (error) {
        vscode.window.showErrorMessage(`Failed to create sample file: ${error}`);
    }
}
function deactivate() {
    console.log('Razen Language Support extension is now deactivated');
}
exports.deactivate = deactivate;
//# sourceMappingURL=extension.js.map