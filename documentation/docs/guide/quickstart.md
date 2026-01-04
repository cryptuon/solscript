# Quick Start

Get up and running with SolScript in minutes.

## Create a New Project

```bash
solscript init my-project
cd my-project
```

This creates:

```
my-project/
├── src/
│   └── main.sol        # Your contract
├── solscript.toml      # Project configuration
├── .gitignore
└── README.md
```

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

```solidity
contract MyProject {
    uint256 public counter;
    address public owner;

    event CounterIncremented(address indexed by, uint256 newValue);

    constructor() {
        owner = msg.sender;
        counter = 0;
    }

    function increment() public {
        counter += 1;
        emit CounterIncremented(msg.sender, counter);
    }

    function getCounter() public view returns (uint256) {
        return counter;
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

```bash
solscript build-bpf src/main.sol
```

This compiles directly to a deployable Solana program.

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

```solidity
contract MyProject {
    // ... contract code ...

    #[test]
    function testIncrement() {
        increment();
        assertEq(counter, 1, "Counter should be 1");
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
| `solscript init <name>` | Create new project |
| `solscript check <file>` | Type check only |
| `solscript build <file>` | Generate Anchor code |
| `solscript build-bpf <file>` | Compile to BPF |
| `solscript test <file>` | Run tests |
| `solscript deploy <file>` | Deploy to cluster |
| `solscript watch <file>` | Watch and rebuild |
| `solscript fmt <file>` | Format code |
| `solscript doctor` | Check environment |

## Next Steps

- [Your First Contract](first-contract.md) - Learn contract basics
- [Language Guide](overview.md) - Deep dive into SolScript
- [Examples](../examples/counter.md) - Browse example contracts
