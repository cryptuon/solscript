# Quick Start

Get up and running with SolScript in minutes.

## Create a New Project

```bash
solscript new my-project
cd my-project
```

This creates a new project using the default template (counter):

```
my-project/
├── src/
│   └── main.sol        # Your contract
├── solscript.toml      # Project configuration
├── .gitignore
└── README.md
```

### Using Templates

SolScript includes templates for common patterns:

```bash
# List available templates
solscript new --list

# Create with a specific template
solscript new my-token --template token
solscript new my-nft --template nft
```

**Available templates:**

| Template | Difficulty | Description |
|----------|------------|-------------|
| `simple` | Beginner | Minimal contract for learning |
| `counter` | Beginner | Counter with ownership (default) |
| `token` | Intermediate | ERC20-style fungible token |
| `voting` | Intermediate | Decentralized voting system |
| `escrow` | Advanced | Trustless escrow with disputes |
| `nft` | Advanced | ERC721-style NFT collection |

## Project Structure

### solscript.toml

```toml
[project]
name = "my-project"
version = "0.1.0"

[contract]
main = "src/main.sol"
name = "MyProject"

[build]
output = "output"

[solana]
cluster = "devnet"

[dependencies]
# Add packages here
```

### src/main.sol

The generated contract template:

```solscript
contract MyProject {
    @state counter: u64;
    @state owner: Address;

    event CounterIncremented(by: Address, newValue: u64);

    fn init() {
        self.owner = tx.sender;
        self.counter = 0;
    }

    @public
    fn increment() {
        self.counter += 1;
        emit CounterIncremented(tx.sender, self.counter);
    }

    @public
    @view
    fn get_counter(): u64 {
        return self.counter;
    }
}
```

## Build Your Contract

### Type Check Only

```bash
solscript check src/main.sol
```

### Generate Anchor Code

```bash
solscript build src/main.sol
```

This creates an Anchor project in `output/`:

```
output/
├── Anchor.toml
├── Cargo.toml
├── programs/
│   └── solscript_program/
│       └── src/
│           ├── lib.rs
│           ├── state.rs
│           ├── instructions.rs
│           ├── error.rs
│           └── events.rs
└── tests/
```

### Compile to BPF

**Via Anchor (default):**
```bash
solscript build-bpf src/main.sol
```

**Via Direct LLVM (faster, requires LLVM 18):**
```bash
solscript build-bpf --llvm src/main.sol
```

This compiles to a deployable Solana program (.so file).

## Development Workflow

### Watch Mode

Automatically rebuild on file changes:

```bash
solscript watch src/main.sol
```

### Format Code

```bash
solscript fmt src/main.sol
```

### Run Tests

Add test functions to your contract:

```solscript
contract MyProject {
    // ... contract code ...

    #[test]
    fn test_increment() {
        increment();
        assert_eq(self.counter, 1, "Counter should be 1");
    }
}
```

Run tests:

```bash
solscript test src/main.sol
```

## Deploy to Solana

### Local Development

Start a local validator:

```bash
solana-test-validator
```

Deploy:

```bash
solscript deploy src/main.sol --cluster localnet
```

### Devnet

```bash
# Ensure you have devnet SOL
solana airdrop 2 --url devnet

# Deploy
solscript deploy src/main.sol --cluster devnet
```

## Common Commands

| Command | Description |
|---------|-------------|
| `solscript new <name>` | Create new project from template |
| `solscript new --list` | List available templates |
| `solscript check <file>` | Type check only |
| `solscript build <file>` | Generate Anchor code |
| `solscript build-bpf <file>` | Compile to BPF (via Anchor) |
| `solscript build-bpf --llvm <file>` | Compile to BPF (direct LLVM) |
| `solscript test <file>` | Run tests |
| `solscript deploy <file>` | Deploy to cluster |
| `solscript watch <file>` | Watch and rebuild |
| `solscript fmt <file>` | Format code |
| `solscript doctor` | Check environment |
| `solscript lsp` | Start Language Server |

## Next Steps

- [Your First Contract](first-contract.md) - Learn contract basics
- [Language Guide](overview.md) - Deep dive into SolScript
- [Examples](../examples/counter.md) - Browse example contracts
