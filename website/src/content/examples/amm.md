---
title: "Automated Market Maker (AMM)"
description: "A constant-product AMM (x*y=k) for token swaps. Demonstrates liquidity pools, price calculation, and DeFi primitives on Solana."
tags: ["AMM", "DeFi", "swap", "liquidity", "DEX"]
difficulty: advanced
order: 5
sourceFile: "amm/amm.sol"
---

## Overview

This AMM implements the constant-product formula (x * y = k) popularized by Uniswap. It allows trustless token swaps between two assets using a liquidity pool.

Key DeFi concepts:
- **Constant product formula**: Price determination via x * y = k
- **Liquidity provision**: Add/remove liquidity and receive LP shares
- **Token swaps**: Exchange one token for another with automatic pricing
- **Fee collection**: Swap fees distributed to liquidity providers

This is one of the most complex SolScript examples, showing that full DeFi primitives can be built in Solidity syntax on Solana.
