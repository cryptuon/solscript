# CLI Reference

The SolScript command-line interface for compiling and managing projects.

## Installation

```bash
cargo install solscript
```

## Commands

### `solscript build`

Compile a SolScript file to Anchor/Rust.

```bash
solscript build <FILE> [OPTIONS]
```

**Arguments:**
- `<FILE>` - Path to the `.sol` source file

**Options:**
- `-o, --output <DIR>` - Output directory (default: `./output`)
- `--no-color` - Disable colored output

**Example:**
```bash
solscript build counter.sol -o ./build
```

**Output Structure:**
```
./build/
├── Anchor.toml
├── Cargo.toml
├── programs/
│   └── counter/
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs
└── tests/
    └── counter.ts
```

---

### `solscript check`

Type-check a SolScript file without generating output.

```bash
solscript check <FILE>
```

**Arguments:**
- `<FILE>` - Path to the `.sol` source file

**Example:**
```bash
solscript check token.sol
```

**Output:**
- Success: No output, exit code 0
- Errors: Displays type errors with source locations

---

### `solscript parse`

Parse a file and display the AST.

```bash
solscript parse <FILE> [OPTIONS]
```

**Arguments:**
- `<FILE>` - Path to the `.sol` source file

**Options:**
- `--json` - Output AST as JSON (default: debug format)

**Example:**
```bash
solscript parse counter.sol --json > ast.json
```

---

### `solscript build-bpf`

Compile to Solana BPF bytecode.

```bash
solscript build-bpf <FILE> [OPTIONS]
```

**Arguments:**
- `<FILE>` - Path to the `.sol` source file

**Options:**
- `-o, --output <DIR>` - Output directory (default: `./target`)
- `--opt-level <LEVEL>` - Optimization level: 0, 1, 2, 3 (default: 2)
- `--keep-intermediate` - Keep intermediate Anchor files

**Example:**
```bash
solscript build-bpf token.sol -o ./deploy --opt-level 3
```

**Requirements:**
- Solana CLI tools installed
- `cargo-build-sbf` available

---

### `solscript deploy`

Deploy a compiled program to Solana.

```bash
solscript deploy <FILE> [OPTIONS]
```

**Arguments:**
- `<FILE>` - Path to the `.sol` source file

**Options:**
- `-o, --output <DIR>` - Build output directory (default: `./target`)
- `--network <NETWORK>` - Target network: devnet, testnet, mainnet (default: devnet)
- `--keypair <PATH>` - Path to keypair file
- `--skip-build` - Skip the build step

**Example:**
```bash
solscript deploy token.sol --network devnet --keypair ~/.config/solana/id.json
```

---

### `solscript test`

Run tests for a SolScript project.

```bash
solscript test <FILE> [OPTIONS]
```

**Arguments:**
- `<FILE>` - Path to the `.sol` source file

**Options:**
- `-o, --output <DIR>` - Build output directory (default: `./target`)
- `--skip-build` - Skip the build step

**Example:**
```bash
solscript test counter.sol
```

---

### `solscript init`

Initialize a new SolScript project.

```bash
solscript init <NAME> [OPTIONS]
```

**Arguments:**
- `<NAME>` - Project name

**Options:**
- `--template <TEMPLATE>` - Project template: counter, token, nft (default: counter)

**Example:**
```bash
solscript init my-project --template token
```

**Generated Structure:**
```
my-project/
├── solscript.toml
├── src/
│   └── lib.sol
└── tests/
    └── test.ts
```

---

### `solscript add`

Add a dependency to the project.

```bash
solscript add <PACKAGE> [OPTIONS]
```

**Arguments:**
- `<PACKAGE>` - Package name or GitHub URL

**Options:**
- `--version <VERSION>` - Package version
- `--git <URL>` - Git repository URL
- `--branch <BRANCH>` - Git branch name

**Examples:**
```bash
# Add from registry
solscript add @solana/spl-token

# Add from GitHub
solscript add --git https://github.com/user/lib --branch main
```

---

### `solscript doctor`

Check the development environment.

```bash
solscript doctor
```

**Output:**
```
SolScript Environment Check
============================
Solana CLI:     ✓ installed (1.18.0)
Anchor CLI:     ✓ installed (0.30.0)
cargo-build-sbf: ✓ installed
Node.js:        ✓ installed (20.0.0)

All tools available!
```

---

### `solscript lsp`

Start the Language Server Protocol server.

```bash
solscript lsp [OPTIONS]
```

**Options:**
- `--stdio` - Use stdio for communication (default)
- `--tcp <PORT>` - Use TCP on specified port

**Example:**
```bash
solscript lsp --stdio
```

Used by IDE extensions for code intelligence.

---

## Configuration

### `solscript.toml`

Project configuration file:

```toml
[package]
name = "my-project"
version = "0.1.0"
description = "My SolScript project"

[solana]
cluster = "devnet"
program_id = "11111111111111111111111111111111"

[build]
output_dir = "./target"
optimization = 2

[dependencies]
spl-token = { git = "https://github.com/solana-labs/solana-program-library", branch = "master" }
```

---

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `SOLSCRIPT_OUTPUT` | Default output directory | `./output` |
| `SOLANA_KEYPAIR` | Path to keypair file | `~/.config/solana/id.json` |
| `SOLANA_CLUSTER` | Target cluster | `devnet` |

---

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | Compilation error |
| 2 | Type checking error |
| 3 | Runtime/deployment error |
| 4 | Configuration error |

---

## See Also

- [Installation Guide](../guide/installation.md)
- [Quick Start](../guide/quickstart.md)
- [Build System](../guide/first-contract.md)
