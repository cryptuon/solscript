---
title: "Token (ERC-20 Style)"
description: "A full-featured fungible token with transfers, allowances, minting, burning, and pause functionality. Demonstrates mappings, modifiers, and events."
tags: ["token", "DeFi", "mappings", "SPL", "ERC-20"]
difficulty: intermediate
order: 2
sourceFile: "token/token.sol"
---

## Overview

This token contract implements the ERC-20 interface pattern on Solana. It demonstrates SolScript's most important feature: **automatic PDA mapping** from Solidity `mapping` types.

Key features:
- **`balanceOf` mapping**: Converted to per-address PDA accounts
- **`allowance` nested mapping**: Two-level PDA with owner + spender seeds
- **Minting and burning**: Owner-controlled supply management
- **Pause mechanism**: Emergency stop pattern
- **Custom errors**: Rich error types with parameters

This is the most comprehensive example, showing how a real DeFi primitive looks in SolScript.
