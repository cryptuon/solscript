# SolScript VS Code Extension

Language support for SolScript - Solidity-style syntax for Solana.

## Features

- Syntax highlighting
- Code completion
- Error diagnostics
- Go to definition
- Find references
- Code formatting
- Snippets

## Requirements

- SolScript CLI installed and in PATH
- VS Code 1.75.0 or higher

## Installation

### From Marketplace

Search for "SolScript" in the VS Code Extensions view.

### From VSIX

```bash
code --install-extension solscript-0.1.0.vsix
```

## Usage

1. Open a `.sol` file
2. The extension activates automatically
3. Use Ctrl+Shift+B to build

## Commands

- `SolScript: Build` - Build the current file
- `SolScript: Check` - Type-check the current file
- `SolScript: Restart Language Server` - Restart the LSP

## Configuration

```json
{
  "solscript.server.path": "solscript",
  "solscript.format.tabSize": 4,
  "solscript.format.useTabs": false,
  "solscript.lint.enable": true,
  "solscript.trace.server": "off"
}
```

## Building

```bash
npm install
npm run compile
npm run package
```

## License

MIT
