# SolScript

[![CI](https://github.com/cryptuon/solscript/actions/workflows/ci.yml/badge.svg)](https://github.com/cryptuon/solscript/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/solscript-cli.svg)](https://crates.io/crates/solscript-cli)
[![Documentation](https://docs.rs/solscript-cli/badge.svg)](https://docs.rs/solscript-cli)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust Version](https://img.shields.io/badge/rust-1.82%2B-blue.svg)](https://www.rust-lang.org)

**Write Solidity. Deploy to Solana.**

SolScript lets you write smart contracts in familiar Solidity syntax and compile them to native Solana programs. No Rust required. No Anchor boilerplate. Just your contract logic.

```solidity
contract Token {
    mapping(address => uint256) public balanceOf;

    function transfer(address to, uint256 amount) public {
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        emit Transfer(msg.sender, to, amount);
    }
}
```

This compiles to a fully functional Solana program with automatic PDA derivation, account validation, and Anchor compatibility.

## Why SolScript?

| Challenge | SolScript Solution |
|-----------|-------------------|
| Rust learning curve | Write in Solidity syntax you already know |
| Anchor boilerplate | Auto-generated account structs and constraints |
| PDA complexity | `mapping` automatically becomes PDAs |
| Account validation | Derived from your contract's type system |
| Ecosystem lock-in | Output is standard Anchor/Rust - eject anytime |

### Before: Raw Anchor (70+ lines)

```rust
use anchor_lang::prelude::*;

#[program]
pub mod token {
    pub fn transfer(ctx: Context<Transfer>, amount: u64) -> Result<()> {
        let from = &mut ctx.accounts.from_balance;
        let to = &mut ctx.accounts.to_balance;
        require!(from.amount >= amount, TokenError::InsufficientBalance);
        from.amount -= amount;
        to.amount += amount;
        emit!(TransferEvent { from: ctx.accounts.from.key(), to: ctx.accounts.to.key(), amount });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(mut)]
    pub from: Signer<'info>,
    #[account(mut, seeds = [b"balance", from.key().as_ref()], bump)]
    pub from_balance: Account<'info, Balance>,
    #[account(mut, seeds = [b"balance", to.key().as_ref()], bump)]
    pub to_balance: Account<'info, Balance>,
    /// CHECK: recipient
    pub to: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
// ... plus error definitions, events, account structs
```

### After: SolScript (12 lines)

```solidity
contract Token {
    mapping(address => uint256) public balanceOf;
    event Transfer(address indexed from, address indexed to, uint256 value);
    error InsufficientBalance();

    function transfer(address to, uint256 amount) public {
        if (balanceOf[msg.sender] < amount) revert InsufficientBalance();
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        emit Transfer(msg.sender, to, amount);
    }
}
```

## Quick Start

```bash
# Install
cargo install --git https://github.com/cryptuon/solscript solscript-cli

# Create a project
solscript init my-token
cd my-token

# Build and deploy
solscript build-bpf
solana program deploy target/deploy/my_token.so
```

## What You Can Build

SolScript supports the contract patterns you need for real DeFi and NFT applications:

### Tokens & DeFi

```solidity
contract AMM {
    mapping(address => uint256) public reserves;

    function swap(address tokenIn, address tokenOut, uint256 amountIn) public {
        uint256 amountOut = getAmountOut(amountIn, reserves[tokenIn], reserves[tokenOut]);
        Token(tokenIn).transferFrom(msg.sender, address(this), amountIn);
        Token(tokenOut).transfer(msg.sender, amountOut);
        reserves[tokenIn] += amountIn;
        reserves[tokenOut] -= amountOut;
    }
}
```

### Escrow & Marketplaces

```solidity
contract Escrow {
    enum State { Funded, Released, Refunded, Disputed }

    struct Deal {
        address buyer;
        address seller;
        uint256 amount;
        State state;
    }

    mapping(uint256 => Deal) public deals;

    function release(uint256 dealId) public {
        Deal storage deal = deals[dealId];
        require(msg.sender == deal.buyer);
        require(deal.state == State.Funded);
        deal.state = State.Released;
        transfer(deal.seller, deal.amount);
    }
}
```

### Access Control

```solidity
contract Governed {
    address public owner;
    bool public paused;

    modifier onlyOwner() {
        require(msg.sender == owner);
        _;
    }

    modifier whenNotPaused() {
        require(!paused);
        _;
    }

    function pause() public onlyOwner {
        paused = true;
    }
}
```

## Features

| Feature | Status |
|---------|--------|
| State variables (primitives, structs, arrays) | Supported |
| Mappings to PDA transformation | Supported |
| Nested mappings | Supported |
| Events and custom errors | Supported |
| Access control modifiers | Supported |
| View/pure functions | Supported |
| Cross-program invocation (CPI) | Supported |
| SPL Token operations | Supported |
| Direct SOL transfers | Supported |
| `msg.sender`, `block.timestamp` | Supported |
| Structs and enums | Supported |

## Compilation Modes

### Anchor Mode (Default)

Generates Rust/Anchor code, then compiles with `cargo build-sbf`. Full Anchor ecosystem compatibility.

```bash
solscript build-bpf contract.sol
```

### Direct LLVM Mode

Compiles directly to BPF bytecode via LLVM. Faster iteration, smaller output.

```bash
solscript build-bpf --llvm contract.sol
```

Requires LLVM 18:
```bash
# Ubuntu/Debian
sudo apt install llvm-18-dev
export LLVM_SYS_180_PREFIX=/usr/lib/llvm-18

# macOS
brew install llvm@18
export LLVM_SYS_180_PREFIX=$(brew --prefix llvm@18)
```

## CLI Reference

```bash
solscript init <name>           # Create new project
solscript build <file>          # Generate Rust/Anchor code
solscript build-bpf <file>      # Compile to deployable .so
solscript build-bpf --llvm      # Direct LLVM compilation
solscript check <file>          # Type check (fast feedback)
solscript test                  # Run tests
solscript fmt <file>            # Format code
solscript lsp                   # Start language server
```

## IDE Support

**VS Code Extension** with full language server support:
- Syntax highlighting
- Go to definition
- Autocomplete
- Inline error diagnostics
- Hover documentation

```bash
cd vscode-extension && npm install && npm run package
code --install-extension solscript-*.vsix
```

## Examples

| Example | Description |
|---------|-------------|
| [counter](examples/counter/) | Simple state management |
| [token](examples/token/) | ERC20-style fungible token |
| [escrow](examples/escrow/) | Multi-party trustless escrow |
| [voting](examples/voting/) | On-chain governance |
| [nft](examples/nft/) | NFT minting and transfers |
| [staking](examples/staking/) | Token staking with rewards |
| [amm](examples/amm/) | Automated market maker |

## How It Works

```
┌─────────────────┐
│  Solidity-like  │
│   Source Code   │
└────────┬────────┘
         │ parse
┌────────▼────────┐
│       AST       │
└────────┬────────┘
         │ type check
┌────────▼────────┐
│   Typed AST     │
└────────┬────────┘
         │
    ┌────┴────┐
    │         │
┌───▼───┐ ┌───▼───┐
│Anchor │ │ LLVM  │
│Codegen│ │Codegen│
└───┬───┘ └───┬───┘
    │         │
┌───▼───┐ ┌───▼───┐
│ Rust  │ │  BPF  │
│Source │ │Bytecode│
└───┬───┘ └───────┘
    │
┌───▼────────┐
│cargo build-│
│    sbf     │
└───┬────────┘
    │
┌───▼───┐
│  .so  │
│Program│
└───────┘
```

## Documentation

- [Language Guide](documentation/docs/guide/overview.md)
- [Type Reference](documentation/docs/reference/types.md)
- [Built-in Functions](documentation/docs/reference/builtins.md)
- [CLI Reference](documentation/docs/reference/cli.md)

**Internal dev docs:** [docs/](docs/) - Language spec, roadmap, design decisions

## Current Limitations

| Limitation | Notes |
|------------|-------|
| No `msg.value` for incoming SOL | Use wrapped SOL or explicit transfer |
| No Token 2022 | Coming in v0.4 |
| Modifiers are inlined | Keep modifiers small |

## Contributing

We welcome contributions. Priority areas:

1. Parser grammar extensions
2. Token 2022 CPI generation
3. Integration tests
4. Documentation improvements

See [docs/](docs/) for internal development documentation.

## License

MIT License - see [LICENSE](LICENSE)

---

**SolScript**: Solidity syntax. Solana performance. Ship faster.
