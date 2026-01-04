# VS Code Extension

SolScript provides a VS Code extension for an enhanced development experience.

## Installation

### From Marketplace

1. Open VS Code
2. Go to Extensions (Ctrl+Shift+X)
3. Search for "SolScript"
4. Click Install

### From VSIX

```bash
# Download the extension
curl -L -o solscript.vsix https://github.com/solscript/vscode/releases/latest/download/solscript.vsix

# Install
code --install-extension solscript.vsix
```

## Features

### Syntax Highlighting

Full syntax highlighting for SolScript files (`.sol`):

- Keywords and operators
- Types and functions
- Comments and strings
- Contract and interface names

### Code Completion

Intelligent code completion for:

- Contract members
- Function parameters
- Type names
- Built-in functions

### Error Diagnostics

Real-time error checking:

- Syntax errors
- Type mismatches
- Undefined variables
- Missing imports

### Hover Information

Hover over any symbol to see:

- Type information
- Function signatures
- Documentation

### Go to Definition

Navigate to definitions with:

- Ctrl+Click on symbols
- F12 key
- Right-click → Go to Definition

### Find References

Find all usages of:

- Functions
- Variables
- Types
- Contracts

### Code Formatting

Format your code with:

- Shift+Alt+F (Format Document)
- Right-click → Format Document

## Configuration

### Settings

Open Settings (Ctrl+,) and search for "SolScript":

```json
{
  "solscript.format.tabSize": 4,
  "solscript.format.useTabs": false,
  "solscript.lint.enable": true,
  "solscript.trace.server": "off"
}
```

### Recommended Settings

```json
{
  "editor.formatOnSave": true,
  "[solscript]": {
    "editor.defaultFormatter": "solscript.solscript"
  }
}
```

## Commands

Access commands via Command Palette (Ctrl+Shift+P):

| Command | Description |
|---------|-------------|
| `SolScript: Build` | Build current file |
| `SolScript: Check` | Type-check current file |
| `SolScript: Format Document` | Format current file |
| `SolScript: Restart Language Server` | Restart LSP |

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| Ctrl+Shift+B | Build |
| F12 | Go to Definition |
| Shift+F12 | Find References |
| Shift+Alt+F | Format |
| Ctrl+Space | Trigger Completion |

## Snippets

Built-in snippets for common patterns:

### Contract

Type `contract` + Tab:

```solidity
contract ${1:Name} {
    ${0}
}
```

### Function

Type `function` + Tab:

```solidity
function ${1:name}(${2:params}) public ${3:returns (${4:type})} {
    ${0}
}
```

### Modifier

Type `modifier` + Tab:

```solidity
modifier ${1:name}() {
    ${2:require(condition, "message");}
    _;
}
```

### Event

Type `event` + Tab:

```solidity
event ${1:Name}(${2:params});
```

### Mapping

Type `mapping` + Tab:

```solidity
mapping(${1:keyType} => ${2:valueType}) public ${3:name};
```

## Troubleshooting

### Extension Not Working

1. Check that SolScript CLI is installed:
   ```bash
   solscript --version
   ```

2. Restart the language server:
   - Command Palette → "SolScript: Restart Language Server"

3. Check the Output panel:
   - View → Output
   - Select "SolScript" from dropdown

### Slow Performance

1. Check file size and complexity
2. Try disabling other extensions
3. Increase VS Code memory limit

### Formatting Issues

1. Check format settings
2. Ensure file is valid SolScript
3. Try reformatting after fixing errors

## See Also

- [Language Server](lsp.md)
- [CLI Reference](../reference/cli.md)
