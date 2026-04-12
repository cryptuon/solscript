---
title: "SolScript vs Solang vs Neon EVM: 3 Ways to Use Solidity on Solana"
description: "Compare the three tools for writing Solidity on Solana. SolScript generates Anchor code, Solang compiles to BPF, and Neon EVM emulates the full EVM. Here's how to choose."
pubDate: 2026-04-12
author: "SolScript Team"
tags: ["comparison", "solang", "neon evm", "solidity", "solana", "compiler"]
---

If you search for "Solidity on Solana," you'll find three tools: SolScript, Solang, and Neon EVM. All three let Ethereum developers use their Solidity skills on Solana, but they work in fundamentally different ways.

## The Three Approaches

**SolScript** transpiles Solidity to readable Anchor/Rust source code. You write Solidity, and SolScript generates the equivalent Anchor program — complete with PDA derivations, account validations, and event emissions. The generated code is human-readable and auditable.

**Solang** (by Hyperledger) compiles Solidity directly to Solana BPF bytecode using LLVM. It's a traditional compiler that outputs binary, not source code. Solang supports Solidity 0.8 syntax but requires you to understand Solana's account model.

**Neon EVM** runs a full Ethereum Virtual Machine on Solana. Your existing Ethereum contracts work with almost no changes — just switch the RPC endpoint. But there's an emulation layer between your code and Solana's native runtime.

## Key Differences

### Output format

This is the biggest practical difference. SolScript generates **Rust source code** you can read, modify, and audit. Solang generates **binary bytecode**. Neon runs **EVM bytecode** in an emulator.

For security-conscious teams, auditable source output is a significant advantage. You can verify exactly what SolScript's compiler produced before deploying.

### PDA handling

Solidity `mapping` types don't exist on Solana. They need to be converted to Program Derived Addresses (PDAs).

- **SolScript**: Handles this automatically. Write `mapping(address => uint256)`, get PDA code.
- **Solang**: You need to understand and work with Solana's account model manually.
- **Neon EVM**: Uses EVM storage model (no PDAs), but adds emulation overhead.

### Browser IDE

SolScript offers a [browser-based playground](/playground) with a WASM compiler that runs entirely client-side. You can write, compile, and deploy without installing anything. Neither Solang nor Neon EVM offer a comparable browser experience.

### Existing contract compatibility

If you have existing Ethereum contracts you want to move:
- **Neon EVM**: Easiest path — minimal code changes needed
- **Solang**: Some modifications required for Solana's account model
- **SolScript**: Best for writing new contracts, not porting old ones

## When to Use Each

**Building new contracts on Solana?** Use SolScript for the fastest development experience with readable output.

**Need strict Solidity 0.8 compatibility?** Use Solang for standards-compliant compilation.

**Porting an existing Ethereum dApp quickly?** Use Neon EVM to minimize code changes.

**Want the best of both worlds?** Start with SolScript's playground for prototyping, then review the generated Anchor code before production deployment.

## Read More

For a detailed feature-by-feature comparison table, see our [full comparison page](/compare/solscript-vs-solang-vs-neon).
