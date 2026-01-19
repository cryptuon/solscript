# Project Templates

SolScript includes pre-built templates to help you get started quickly with common smart contract patterns.

## Using Templates

### List Available Templates

```bash
solscript new --list
```

Output:
```
Available templates:

  simple (Beginner) - Minimal contract for learning
    Features: state variables, constructor, view functions

  counter (Beginner) - Counter with ownership [DEFAULT]
    Features: events, errors, modifiers, access control

  token (Intermediate) - ERC20-style fungible token
    Features: mappings, transfers, approvals, pausable, mintable

  voting (Intermediate) - Decentralized voting system
    Features: structs, enums, time-based logic, weighted votes

  escrow (Advanced) - Trustless escrow with dispute resolution
    Features: state machine, multi-party, deadlines, dispute resolution

  nft (Advanced) - ERC721-style NFT collection
    Features: metadata, minting, approvals, operator pattern

Usage: solscript new <project-name> --template <template>
```

### Create a Project

```bash
# Using the default template (counter)
solscript new my-project

# Using a specific template
solscript new my-token --template token
```

## Template Reference

### Simple (Beginner)

The most basic contract - perfect for learning the fundamentals.

**Features:**
- State variables with `public` visibility
- Constructor initialization
- View functions

**Use when:** Learning SolScript basics or need a minimal starting point.

```bash
solscript new my-first-contract --template simple
```

---

### Counter (Beginner) - Default

A counter with ownership and access control - demonstrates essential patterns.

**Features:**
- Events for tracking state changes
- Custom errors for validation
- Modifiers for access control
- Owner-only functions

**Use when:** Building contracts that need ownership patterns.

```bash
solscript new my-counter --template counter
```

---

### Token (Intermediate)

An ERC20-style fungible token implementation.

**Features:**
- Balance mappings
- Transfer functionality
- Approval/allowance pattern
- Minting and burning
- Pausable transfers

**Use when:** Creating a fungible token, reward system, or point system.

```bash
solscript new my-token --template token
```

---

### Voting (Intermediate)

A decentralized voting system with proposals.

**Features:**
- Struct definitions for proposals
- Enum for proposal states
- Time-based voting periods
- Weighted voting power

**Use when:** Building governance, polls, or decision-making systems.

```bash
solscript new my-dao --template voting
```

---

### Escrow (Advanced)

A trustless escrow with dispute resolution.

**Features:**
- State machine pattern
- Multi-party interactions (buyer, seller, arbiter)
- Deadline enforcement
- Dispute resolution flow

**Use when:** Building marketplaces, freelance platforms, or any trustless exchange.

```bash
solscript new my-marketplace --template escrow
```

---

### NFT (Advanced)

An ERC721-style NFT collection.

**Features:**
- Unique token tracking
- Metadata management
- Minting with supply limits
- Approval and operator patterns
- Transfer functionality

**Use when:** Creating NFT collections, digital assets, or collectibles.

```bash
solscript new my-nft --template nft
```

## Generated Project Structure

All templates create the same directory structure:

```
my-project/
├── src/
│   └── main.sol        # Your contract code
├── solscript.toml      # Project configuration
├── .gitignore          # Git ignore rules
└── README.md           # Project documentation
```

### solscript.toml

```toml
[project]
name = "my-project"
version = "0.1.0"
description = "Project description"

[contract]
main = "src/main.sol"
name = "MyProject"

[build]
output = "output"

[solana]
cluster = "devnet"
```

## Customizing Templates

After creating a project, you can:

1. **Rename the contract** - Update the contract name in `main.sol` and `solscript.toml`
2. **Add state variables** - Add your custom storage
3. **Add functions** - Implement your business logic
4. **Modify events/errors** - Customize for your use case

## Next Steps

After creating your project:

```bash
cd my-project

# Check for errors
solscript check src/main.sol

# Build to Anchor
solscript build src/main.sol

# Compile to BPF
solscript build-bpf src/main.sol

# Deploy to devnet
solscript deploy src/main.sol --cluster devnet
```

## See Also

- [Quick Start](quickstart.md) - Get started with SolScript
- [Examples](/examples/) - More example contracts in the repository
- [CLI Reference](../reference/cli.md) - Full command documentation
