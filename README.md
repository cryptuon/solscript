# SolScript: High-Level Language for Solana Development

SolScript is a high-level language designed to simplify Solana smart contract development. Write contracts in a familiar, Solidity-like syntax and compile them to Solana BPF programs.

## Features

- **Intuitive Contract Syntax**: Familiar Solidity-like syntax with Rust-inspired features
- **Multiple Compilation Modes**: Anchor/Rust codegen or direct LLVM-to-BPF compilation
- **Type Safety**: Full type checking with inference and generics
- **Built-in Security**: Automatic overflow checks, signer verification
- **Language Server**: IDE support with go-to-definition, autocomplete, and inline errors
- **Testing Framework**: Built-in test support with assertions

## Quick Start

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/solscript.git
cd solscript

# Build the compiler
cargo build --release

# Or install globally
cargo install --path crates/solscript-cli
```

### Write Your First Contract

Create `counter.sol`:

```solscript
contract Counter {
    @state count: u64;

    fn init() {
        self.count = 0;
    }

    @public
    fn increment() {
        self.count += 1;
    }

    @public
    fn decrement() {
        self.count -= 1;
    }

    @public
    @view
    fn get_count(): u64 {
        return self.count;
    }
}
```

### Compile

```bash
# Type check only (fast feedback)
solscript check counter.sol

# Generate Rust/Anchor code
solscript build counter.sol -o target/anchor

# Compile to BPF (via Anchor)
solscript build-bpf counter.sol -o target/deploy

# Compile to BPF directly via LLVM (faster, requires LLVM 18)
solscript build-bpf --llvm counter.sol -o target/deploy
```

## Compilation Modes

### 1. Anchor Mode (Default)

Generates Rust/Anchor code, then compiles using `cargo build-sbf`:

```bash
solscript build-bpf counter.sol
```

**Requirements:** Solana CLI, Anchor framework

### 2. Direct LLVM Mode

Compiles directly to BPF bytecode using LLVM:

```bash
solscript build-bpf --llvm counter.sol
```

**Requirements:** LLVM 18 with BPF target

**Setup LLVM 18:**
```bash
# Ubuntu/Debian
wget https://apt.llvm.org/llvm.sh
chmod +x llvm.sh
sudo ./llvm.sh 18
sudo apt install llvm-18-dev libpolly-18-dev
export LLVM_SYS_180_PREFIX=/usr/lib/llvm-18

# macOS
brew install llvm@18
export LLVM_SYS_180_PREFIX=$(brew --prefix llvm@18)

# Build with LLVM feature
cargo build -p solscript-bpf --features llvm
```

## Language Features

### State Variables

```solscript
contract MyContract {
    @state owner: Address;
    @state balances: Map<Address, u64>;
    @state data: Vec<u8>;
}
```

### Functions

```solscript
contract MyContract {
    // Initialization (called once on deploy)
    fn init(initial_owner: Address) {
        self.owner = initial_owner;
    }

    // Public function (callable externally)
    @public
    fn transfer(to: Address, amount: u64) {
        // ...
    }

    // View function (read-only, cannot modify state)
    @public
    @view
    fn get_balance(account: Address): u64 {
        return self.balances.get(account).unwrap_or(0);
    }
}
```

### Events and Errors

```solscript
event Transfer(from: Address, to: Address, amount: u64);
error InsufficientBalance(available: u64, required: u64);

contract Token {
    @public
    fn transfer(to: Address, amount: u64): Result<(), Error> {
        let balance = self.balances.get(tx.sender).unwrap_or(0);
        if balance < amount {
            return Err(InsufficientBalance(balance, amount));
        }
        // ... transfer logic
        emit Transfer(tx.sender, to, amount);
        return Ok(());
    }
}
```

### Structs and Enums

```solscript
#[derive(Clone, Serialize, Deserialize)]
struct TokenMetadata {
    name: string;
    symbol: string;
    decimals: u8;
}

enum OrderStatus {
    Pending,
    Filled,
    Cancelled,
}
```

### Cross-Program Invocation (CPI)

```solscript
import { Token } from "@solana/token";

contract TokenInteractor {
    @public
    fn transfer_tokens(from: Address, to: Address, amount: u64) {
        Token.transfer(from, to, amount);
    }
}
```

## CLI Commands

| Command | Description |
|---------|-------------|
| `solscript init <name>` | Create a new project |
| `solscript build <file>` | Generate Rust/Anchor code |
| `solscript build-bpf <file>` | Compile to BPF bytecode |
| `solscript build-bpf --llvm <file>` | Compile directly via LLVM |
| `solscript check <file>` | Type check without compiling |
| `solscript test` | Run tests |
| `solscript fmt <file>` | Format source code |
| `solscript lsp` | Start Language Server |

## Project Structure

```
solscript/
├── crates/
│   ├── solscript-ast/       # AST definitions
│   ├── solscript-parser/    # Parser (pest grammar)
│   ├── solscript-typeck/    # Type checker
│   ├── solscript-codegen/   # Rust/Anchor code generation
│   ├── solscript-bpf/       # Direct LLVM BPF compilation
│   ├── solscript-lsp/       # Language Server Protocol
│   └── solscript-cli/       # CLI tool
├── examples/                # Example contracts
├── editors/
│   └── vscode/              # VS Code extension
└── documentation/           # mdBook documentation
```

## Examples

See the `examples/` directory for sample contracts:

- `examples/counter/` - Simple counter contract
- `examples/token/` - Token contract with transfer and mint
- `examples/simple/` - Basic examples

## IDE Support

### VS Code

Install the SolScript extension from `editors/vscode/`:

```bash
cd editors/vscode
npm install
npm run package
code --install-extension solscript-*.vsix
```

Features:
- Syntax highlighting
- Go to definition
- Hover information
- Autocomplete
- Inline error diagnostics

## Documentation

- [Language Specification](docs/specs.md)
- [Implementation Roadmap](docs/roadmap.md)
- [Full Documentation](documentation/docs/)

## Contributing

Contributions are welcome! Please see the roadmap for planned features.

## License

MIT License - see [LICENSE](LICENSE) for details.

---

SolScript: Simplifying Solana development without sacrificing power!
