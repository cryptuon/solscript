---
title: "Migrating from Ethereum to Solana with SolScript"
description: "A practical guide for Ethereum developers moving to Solana. Learn how Solidity concepts map to Solana architecture and how SolScript bridges the gap."
pubDate: 2026-04-08
author: "SolScript Team"
tags: ["ethereum", "solana", "migration", "solidity", "architecture"]
---

If you've been building on Ethereum and want to explore Solana's speed and low fees, SolScript makes the transition straightforward. You keep writing in Solidity syntax while SolScript handles the Solana-specific details.

## Why Move to Solana?

The numbers speak for themselves:

- **Transaction throughput**: Solana processes 65,000+ TPS vs Ethereum's ~15 TPS
- **Fees**: Under $0.01 per transaction vs Ethereum's $1-100+
- **Finality**: ~400ms vs Ethereum's ~12 seconds (plus confirmation time)
- **Growing ecosystem**: DeFi, NFTs, DePIN, and gaming all flourishing

## The Biggest Conceptual Shift: Accounts

On Ethereum, your contract owns its storage. On Solana, data lives in **accounts** that programs read and write. This is Solana's key architectural difference -- it's what enables parallel transaction processing.

When you write this in Solidity:

```solidity
mapping(address => uint256) public balanceOf;
```

Ethereum stores this in the contract's storage trie. On Solana, each key-value pair becomes a separate **Program Derived Address (PDA)** -- a deterministic account address derived from seeds.

SolScript handles this conversion automatically. You write `mapping`, and SolScript generates the PDA derivation, account creation, and seed management code.

## What Maps to What?

| Ethereum | Solana | SolScript |
|----------|--------|-----------|
| `mapping(k => v)` | PDA accounts | Automatic conversion |
| `msg.sender` | Signer account | Automatic |
| `constructor()` | `initialize` instruction | Automatic |
| Events | Anchor events | Direct mapping |
| `require()` / `revert` | Custom errors | Direct mapping |
| ERC-20 | SPL Token | Built-in support |

## A Real Example: Token Contract

Here's an ERC-20 style token in SolScript:

```solidity
contract Token {
    string public name;
    uint256 public totalSupply;
    mapping(address => uint256) public balanceOf;

    event Transfer(address indexed from, address indexed to, uint256 value);

    function transfer(address to, uint256 amount) public returns (bool) {
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        emit Transfer(msg.sender, to, amount);
        return true;
    }
}
```

This compiles to a Solana program where:
- `balanceOf` becomes a PDA per address
- `msg.sender` maps to the transaction signer
- `Transfer` emits as an Anchor event
- All account validation is generated automatically

## Things That Don't Translate

Some Ethereum features have no Solana equivalent:

- **`delegatecall`**: Solana programs can't execute arbitrary code in their context
- **`selfdestruct`**: Solana programs can be upgraded or closed, but not self-destructed
- **Assembly blocks**: Solana uses BPF bytecode, not EVM opcodes
- **Global block info**: `block.timestamp` and `block.number` work differently on Solana

SolScript will give compile-time errors if you use unsupported features.

## Getting Started

1. Open the [SolScript Playground](/playground)
2. Write or paste your Solidity contract
3. Click Compile to see the generated Anchor code
4. Connect a wallet and deploy to devnet

The playground runs entirely in your browser via WASM -- no installation needed. For local development, install the [SolScript CLI](https://docs.cryptuon.com/solscript).
