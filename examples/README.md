# SolScript Examples

Example contracts demonstrating SolScript features, organized from beginner to advanced.

## Beginner

Start here if you're new to SolScript or smart contract development.

| Example | Description | Key Features |
|---------|-------------|--------------|
| [hello-world](hello-world/) | Your first contract | State variables, constructor, functions |
| [simple](simple/) | Basic state management | Public variables, view functions |
| [counter](counter/) | Counter with ownership | Events, errors, modifiers, access control |
| [storage](storage/) | Storage patterns | Data types, mappings, arrays, loops |

## Intermediate

Build on the basics with more complex patterns.

| Example | Description | Key Features |
|---------|-------------|--------------|
| [token](token/) | ERC20-style token | Transfers, approvals, minting, burning, pausable |
| [voting](voting/) | Voting system | Structs, enums, time-based logic, weighted votes |
| [multisig](multisig/) | Multi-signature wallet | Multi-party coordination, transaction lifecycle |

## Advanced

Complex DeFi and NFT patterns.

| Example | Description | Key Features |
|---------|-------------|--------------|
| [escrow](escrow/) | Trustless escrow | State machine, disputes, deadlines |
| [nft](nft/) | NFT collection | ERC721 pattern, metadata, minting |
| [staking](staking/) | Staking pool | Time-based rewards, APR calculation, compounding |
| [amm](amm/) | Automated market maker | Constant product formula, LP tokens, swaps |

## Running Examples

Each example can be built and tested with:

```bash
# Navigate to example
cd <example-name>

# Check the contract
solscript check <name>.sol

# Build to BPF
solscript build <name>.sol

# Deploy to devnet
solscript deploy <name>.sol --cluster devnet
```

## Creating New Projects

Use `solscript new` to create a new project from a template:

```bash
# List available templates
solscript new --list

# Create from template
solscript new my-project --template counter
```

Available templates: `simple`, `counter`, `token`, `voting`, `escrow`, `nft`

## Learning Path

**Week 1: Foundations**
1. hello-world - Understand contract structure
2. simple - Learn state management
3. counter - Add events and access control
4. storage - Master data types

**Week 2: Token Patterns**
5. token - Build a fungible token
6. nft - Create an NFT collection

**Week 3: Governance & Security**
7. voting - Implement governance
8. multisig - Multi-party security
9. escrow - Trustless transactions

**Week 4: DeFi**
10. staking - Yield generation
11. amm - Decentralized exchange
