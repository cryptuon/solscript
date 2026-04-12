---
title: "Counter"
description: "A simple counter contract demonstrating basic state management, events, custom errors, and owner-only access control."
tags: ["beginner", "state", "events", "access control"]
difficulty: beginner
order: 1
sourceFile: "counter/counter.sol"
---

## Overview

The counter example is the simplest SolScript contract. It demonstrates:

- **State variables**: `uint256` for the count and `address` for the owner
- **Events**: Emitted on increment, decrement, and reset
- **Custom errors**: `Underflow()` and `Unauthorized()`
- **Modifiers**: `onlyOwner` restricts reset to the deployer
- **Constructor**: Sets the initial owner

This is a great starting point for understanding how SolScript maps Solidity concepts to Solana programs.
