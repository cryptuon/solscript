# SolScript

[![CI](https://github.com/cryptuon/solscript/actions/workflows/ci.yml/badge.svg)](https://github.com/cryptuon/solscript/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/solscript-cli.svg)](https://crates.io/crates/solscript-cli)
[![Documentation](https://docs.rs/solscript-cli/badge.svg)](https://docs.rs/solscript-cli)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust Version](https://img.shields.io/badge/rust-1.83%2B-blue.svg)](https://www.rust-lang.org)

**[🌐 Site](https://solscript.cryptuon.com/) · [📚 Docs](https://docs.cryptuon.com/solscript/) · [📦 crates.io](https://crates.io/crates/solscript-cli) · [🔬 Cryptuon Research](https://github.com/cryptuon)**

**Write Solidity. Deploy to Solana.**

SolScript is a Solidity-to-Solana compiler framework: you write smart contracts in familiar Solidity syntax and compile them to **native Solana BPF programs** with full Anchor compatibility and automatic PDA derivation. It brings the largest smart-contract developer base — the millions who already write Solidity — to Solana's high-performance runtime, without a rewrite in Rust. No Rust required. No Anchor boilerplate. Just your contract logic.

### Why this matters in 2026

On-chain activity is consolidating onto a handful of high-throughput chains, and Solana is one of them. But the developer talent, the audited patterns, the mental models, and the tooling ecosystem still overwhelmingly live in Solidity. Teams that want to ship on Solana today face a hard choice: rewrite everything in Rust/Anchor, or route through an EVM-emulation layer and give up native performance.

Chain abstraction is the answer the market is converging on — **one language, many chains.** SolScript is the Solidity-side of that story for Solana: keep the source of truth your team already understands, and compile it to a native program that runs at full Solana speed with no interpreter in the hot path. As multi-chain becomes the default deployment posture, a single Solidity codebase that can target both EVM chains *and* Solana natively is a durable engineering advantage.

See the [**ROADMAP**](ROADMAP.md) for where this is going and the cheapest path to a production-grade compiler.

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

### How SolScript compares

There are three honest ways to run Solidity-style logic on Solana. Here is where SolScript sits, without spin:

| Approach | What it is | Runtime cost on Solana | Language coverage | Eject / audit story |
|----------|------------|------------------------|-------------------|---------------------|
| **SolScript** | Solidity → **native** Anchor/BPF at compile time | Native — no interpreter in the hot path | Growing subset of Solidity (see [Limitations](#current-limitations)) | Emits standard Anchor/Rust you can read, audit, and maintain by hand |
| [Neon EVM](https://neonevm.org/) | A full EVM deployed *as* a Solana program | Interpreter overhead — you run inside an emulated EVM | Near-complete EVM/Solidity compatibility | You depend on the Neon runtime; no native Solana program to hand off |
| [Solang](https://solang.readthedocs.io/) | Solidity front-end compiling to Solana via LLVM | Native BPF | Broad Solidity, its own Solana account model | Emits BPF/IDL; less Anchor-idiomatic output |
| Rewrite in Rust/Anchor | Hand-port every contract | Native — hand-tuned | 100% (you write it) | Full control, but full cost — new language, new audits, new bugs |

**When to reach for SolScript:** you have Solidity source (or a Solidity-fluent team) and you want *native* Solana programs that look like idiomatic Anchor code your auditors can read — not an emulated EVM, and not a from-scratch Rust rewrite. If you need 100% Solidity coverage *today* and can accept interpreter overhead, Neon is the pragmatic bridge. If you want maximum control and have Rust engineers to spare, a hand rewrite still wins on tuning.

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
solscript new my-token
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
solscript new <name>            # Create new project
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

**Product roadmap:** [ROADMAP.md](ROADMAP.md) - vision, milestones, and the cheapest path to a production-grade compiler

**Internal dev docs:** [docs/](docs/) - Language spec, implementation roadmap, design decisions

## Current Limitations

| Limitation | Notes |
|------------|-------|
| No `msg.value` for incoming SOL | Use wrapped SOL or explicit transfer |
| No Token 2022 | Coming in v0.4 |
| Modifiers are inlined | Keep modifiers small |

## FAQ

### Is SolScript production-ready?

Not yet — it is an actively developed compiler at `v0.1.x`. The core pipeline (parse → typecheck → Anchor/BPF codegen) works end to end and compiles the [example contracts](#examples) to deployable Solana programs, but the Solidity language coverage is a growing subset, not the full spec. Treat it as pre-1.0: excellent for prototyping, new projects, and evaluating the workflow; not yet for porting an unaudited mainnet protocol untouched. Track progress in the [ROADMAP](ROADMAP.md).

### How is this different from Neon EVM?

Neon EVM runs a full EVM *inside* a Solana program — your contract executes in an emulated environment, which means broad compatibility but interpreter overhead. SolScript compiles your Solidity to a **native** Solana BPF program at build time, so there is no EVM in the hot path. The trade is coverage-for-speed: Neon supports more of Solidity today; SolScript gives you a native, Anchor-idiomatic program you own. See the [comparison table](#how-solscript-compares).

### How is this different from Solang?

Both compile Solidity to native BPF. SolScript is built to emit **idiomatic Anchor/Rust** you can read, audit, and eject to — and it derives Anchor accounts and PDAs automatically from your contract's type system. Solang targets its own Solana account model via LLVM and is more mature on raw language coverage. If Anchor compatibility and human-readable, hand-off-able output matter to you, that is SolScript's lane.

### Do I have to rewrite my Solidity contracts?

You write in Solidity syntax, so no rewrite into a new language — but you should expect to adapt. Solana's account model is fundamentally different from the EVM's, and some Solidity patterns (notably incoming `msg.value`, see [Limitations](#current-limitations)) map differently or aren't supported yet. Think "port and adjust," not "copy and paste."

### Can I still drop into raw Anchor when I need to?

Yes. SolScript's default mode emits standard Anchor/Rust source. You can read it, commit it, and maintain it by hand at any point — there is no proprietary runtime you're locked into. That eject path is a deliberate design goal.

### Which chains does SolScript target?

Solana today. The broader thesis is chain abstraction — one Solidity codebase, many high-performance targets — but SolScript itself compiles Solidity to native Solana programs. It pairs naturally with EVM toolchains: keep your Solidity, deploy the EVM build to EVM chains and the SolScript build to Solana.

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

---

## Part of Cryptuon Research

`solscript` is one of [20 open-source blockchain-infrastructure projects](https://www.cryptuon.com/projects) from **[Cryptuon Research](https://www.cryptuon.com)** — blockchain theory, shipped as protocols.

**Related projects:** [StxScript](https://stxscript.cryptuon.com/) · [Zig-EVM](https://zig-evm.cryptuon.com/) · [Tesseract](https://tesseract.cryptuon.com/)

Docs: [docs.cryptuon.com/solscript](https://docs.cryptuon.com/solscript/) · Contact: [contact@cryptuon.com](mailto:contact@cryptuon.com)
