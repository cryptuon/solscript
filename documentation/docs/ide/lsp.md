# Language Server Protocol

SolScript includes a built-in Language Server Protocol (LSP) implementation.

## Overview

The SolScript Language Server provides IDE features:

- Real-time diagnostics
- Code completion
- Go to definition
- Find references
- Hover information
- Code formatting

## Starting the Server

### Stdio Mode (Default)

```bash
solscript lsp --stdio
```

### TCP Mode

```bash
solscript lsp --tcp 9257
```

## Supported Features

### Text Document Synchronization

- `textDocument/didOpen`
- `textDocument/didChange`
- `textDocument/didClose`
- `textDocument/didSave`

### Diagnostics

Real-time error reporting:

- Syntax errors
- Type errors
- Semantic errors

Diagnostics are pushed automatically on file changes.

### Completion

- `textDocument/completion`

Provides suggestions for:

- Keywords
- Types
- Contract members
- Function names
- Variables in scope

### Hover

- `textDocument/hover`

Shows:

- Type information
- Function signatures
- Documentation comments

### Definition

- `textDocument/definition`

Navigate to:

- Function definitions
- Variable declarations
- Type definitions
- Contract definitions

### References

- `textDocument/references`

Find all usages of:

- Functions
- Variables
- Types

### Formatting

- `textDocument/formatting`

Format entire document with consistent style.

### Document Symbols

- `textDocument/documentSymbol`

Outline view of:

- Contracts
- Functions
- State variables
- Events

## Client Configuration

### VS Code

The VS Code extension automatically configures the client. For manual setup:

```json
{
  "solscript.server.path": "solscript",
  "solscript.server.args": ["lsp", "--stdio"]
}
```

### Neovim (nvim-lspconfig)

```lua
local lspconfig = require('lspconfig')
local configs = require('lspconfig.configs')

if not configs.solscript then
  configs.solscript = {
    default_config = {
      cmd = { 'solscript', 'lsp', '--stdio' },
      filetypes = { 'solscript', 'sol' },
      root_dir = lspconfig.util.root_pattern('solscript.toml', '.git'),
      settings = {},
    },
  }
end

lspconfig.solscript.setup {}
```

### Emacs (lsp-mode)

```elisp
(use-package lsp-mode
  :config
  (add-to-list 'lsp-language-id-configuration
    '(solscript-mode . "solscript"))

  (lsp-register-client
    (make-lsp-client
      :new-connection (lsp-stdio-connection '("solscript" "lsp" "--stdio"))
      :major-modes '(solscript-mode)
      :server-id 'solscript-ls)))
```

### Sublime Text (LSP)

In LSP.sublime-settings:

```json
{
  "clients": {
    "solscript": {
      "command": ["solscript", "lsp", "--stdio"],
      "selector": "source.solscript"
    }
  }
}
```

### Helix

In `languages.toml`:

```toml
[[language]]
name = "solscript"
scope = "source.solscript"
injection-regex = "solscript"
file-types = ["sol"]
roots = ["solscript.toml"]
language-server = { command = "solscript", args = ["lsp", "--stdio"] }
```

## Protocol Messages

### Initialize

Request:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "initialize",
  "params": {
    "capabilities": {}
  }
}
```

Response:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "capabilities": {
      "textDocumentSync": 1,
      "completionProvider": {},
      "hoverProvider": true,
      "definitionProvider": true,
      "referencesProvider": true,
      "documentFormattingProvider": true,
      "documentSymbolProvider": true
    }
  }
}
```

### Diagnostics

Notification:
```json
{
  "jsonrpc": "2.0",
  "method": "textDocument/publishDiagnostics",
  "params": {
    "uri": "file:///path/to/file.sol",
    "diagnostics": [
      {
        "range": {
          "start": { "line": 10, "character": 5 },
          "end": { "line": 10, "character": 15 }
        },
        "severity": 1,
        "message": "Type mismatch: expected uint256, got string"
      }
    ]
  }
}
```

## Debugging

### Enable Tracing

Set the trace level for debugging:

```json
{
  "solscript.trace.server": "verbose"
}
```

### Log Files

Logs are written to:

- Linux: `~/.local/share/solscript/lsp.log`
- macOS: `~/Library/Application Support/solscript/lsp.log`
- Windows: `%APPDATA%\solscript\lsp.log`

### Common Issues

#### Server Not Starting

1. Verify installation:
   ```bash
   solscript --version
   ```

2. Check PATH:
   ```bash
   which solscript
   ```

#### No Completions

1. Wait for file to be parsed
2. Check for syntax errors
3. Restart language server

#### Slow Response

1. Large files may take longer
2. Check for circular dependencies
3. Try simplifying complex types

## Performance

### Caching

The server caches:

- Parsed ASTs
- Type information
- Symbol tables

### Incremental Updates

Only changed portions of the file are re-analyzed on edits.

### Memory Usage

Typical memory usage:

- Small project: ~50MB
- Medium project: ~100MB
- Large project: ~200MB

## See Also

- [VS Code Extension](vscode.md)
- [CLI Reference](../reference/cli.md)
