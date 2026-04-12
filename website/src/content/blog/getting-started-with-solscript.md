---
title: "Getting Started with SolScript in 5 Minutes"
description: "Write and compile your first Solana smart contract in Solidity syntax using the SolScript browser playground. No installation required."
pubDate: 2026-04-10
author: "SolScript Team"
tags: ["getting started", "tutorial", "beginner", "solana", "solidity"]
---

SolScript lets you write Solana smart contracts using Solidity syntax. If you know Solidity from Ethereum, you can start building on Solana immediately -- no Rust required.

## What You'll Build

A simple counter contract that anyone can increment, but only the owner can reset. This covers state variables, events, custom errors, modifiers, and constructors.

## Step 1: Open the Playground

Navigate to the [SolScript Playground](/playground). The browser-based IDE includes a WASM-compiled SolScript compiler, so everything runs client-side with no setup.

## Step 2: Write the Contract

```solidity
contract Counter {
    uint256 public count;
    address public owner;

    event Incremented(address indexed by, uint256 newValue);
    error Unauthorized();

    modifier onlyOwner() {
        if (msg.sender != owner) revert Unauthorized();
        _;
    }

    constructor() {
        owner = msg.sender;
        count = 0;
    }

    function increment() public {
        count += 1;
        emit Incremented(msg.sender, count);
    }

    function reset() public onlyOwner {
        count = 0;
    }
}
```

This is standard Solidity. The key difference is what happens when you compile: SolScript generates a Solana program instead of EVM bytecode.

## Step 3: Compile

Click the **Compile** button in the playground. SolScript will:

1. Parse the contract
2. Validate types and access patterns
3. Convert `mapping` types to PDA (Program Derived Address) lookups
4. Generate Anchor/Rust code

You'll see the generated Rust code in the output panel. It's a standard Anchor program that you can inspect, modify, or audit.

## Step 4: Deploy (Optional)

Connect a Solana wallet (Phantom or Solflare) and select a network (devnet recommended for testing). Click **Deploy** to send the compiled program to Solana.

## What Just Happened?

You wrote a smart contract in Solidity syntax and deployed it to Solana. Behind the scenes, SolScript:

- Converted your `address public owner` state variable into a Solana account field
- Turned the `constructor()` into an Anchor `initialize` instruction
- Mapped `msg.sender` to the transaction signer
- Generated all the Anchor account validation boilerplate

## Next Steps

- Browse the [examples](/examples) for token contracts, AMMs, and more
- Read the [SolScript vs Anchor comparison](/compare/solidity-vs-anchor) to understand the trade-offs
- Try the [token contract tutorial](/tutorials/building-a-token-contract)
