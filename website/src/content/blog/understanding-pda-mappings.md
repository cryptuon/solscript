---
title: "Understanding PDA Mappings in SolScript"
description: "How SolScript automatically converts Solidity mappings to Solana Program Derived Addresses. A deep dive into the PDA transformation."
pubDate: 2026-04-05
author: "SolScript Team"
tags: ["PDA", "mappings", "solana", "architecture", "deep-dive"]
---

One of SolScript's most important features is automatic PDA (Program Derived Address) mapping. When you write `mapping(address => uint256)` in Solidity, SolScript converts it to Solana's account-based storage model. This article explains how that transformation works.

## The Problem

Ethereum has built-in key-value storage. Every contract has a 256-bit address space where it can store and retrieve values by key. Solidity `mapping` is just syntactic sugar over this storage.

Solana has no built-in key-value storage. Programs read and write to **accounts** -- separate data objects with their own addresses, owners, and balances.

## The Solution: PDAs

Program Derived Addresses are deterministic account addresses computed from:
- One or more **seeds** (arbitrary byte arrays)
- A **bump seed** (a single byte ensuring the address is off the Ed25519 curve)
- The **program ID**

The key insight: given the same seeds and program ID, you always get the same PDA. This makes them perfect for implementing mappings.

## How SolScript Transforms Mappings

When you write:

```solidity
mapping(address => uint256) public balanceOf;
```

SolScript generates Anchor code that:

1. **Derives a PDA** using the user's address as a seed:
   ```rust
   seeds = [b"balanceOf", user_key.as_ref()]
   ```

2. **Creates an account** at that PDA (if it doesn't exist) to store the `uint256` value

3. **Reads/writes** the account data when you access `balanceOf[someAddress]`

## Nested Mappings

Nested mappings like `mapping(address => mapping(address => uint256))` (used for token allowances) use multiple seeds:

```solidity
mapping(address => mapping(address => uint256)) public allowance;
```

Becomes:
```rust
seeds = [b"allowance", owner_key.as_ref(), spender_key.as_ref()]
```

Each unique combination of owner and spender gets its own PDA account.

## Why This Matters

Understanding PDAs helps you:
- **Debug**: When a transaction fails, you can inspect the PDA account
- **Optimize**: Fewer mappings mean fewer accounts to create (and less rent to pay)
- **Design**: Knowing the PDA structure helps you design efficient data layouts

## Try It Yourself

Open the [SolScript Playground](/playground) and compile a contract with a mapping. Check the generated Rust code in the output panel -- you'll see the PDA derivation logic SolScript generates.
