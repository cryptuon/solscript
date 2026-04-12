---
title: "Build a DEX on Solana Using Solidity"
description: "Create an automated market maker (AMM) on Solana using SolScript's Solidity syntax. Implements constant-product formula, liquidity pools, and token swaps."
pubDate: 2026-04-03
author: "SolScript Team"
tags: ["DEX", "AMM", "DeFi", "solana", "advanced", "liquidity pool"]
---

Decentralized exchanges are the backbone of DeFi. On Ethereum, building an AMM means writing Solidity. On Solana, it traditionally means writing Rust. With SolScript, you can build a Solana DEX using the Solidity syntax you already know.

## What You'll Build

A constant-product AMM (the same formula behind Uniswap v2) that supports:
- Adding liquidity to a two-token pool
- Removing liquidity and receiving both tokens back
- Swapping one token for another with automatic price calculation
- Fee collection on every swap

## The Constant-Product Formula

The core of any AMM is the invariant: **x * y = k**

Where:
- `x` = reserve of token A
- `y` = reserve of token B
- `k` = constant (preserved across swaps)

When a user swaps `dx` of token A, they receive `dy` of token B such that `(x + dx) * (y - dy) = k`.

## The AMM Contract

```solidity
contract AMM {
    uint256 public reserveA;
    uint256 public reserveB;
    uint256 public totalLiquidity;
    mapping(address => uint256) public liquidity;

    uint256 public constant FEE_NUMERATOR = 3;
    uint256 public constant FEE_DENOMINATOR = 1000;

    event LiquidityAdded(address indexed provider, uint256 amountA, uint256 amountB, uint256 shares);
    event LiquidityRemoved(address indexed provider, uint256 amountA, uint256 amountB);
    event Swap(address indexed user, uint256 amountIn, uint256 amountOut, bool aToB);

    error InsufficientLiquidity();
    error InvalidAmount();
    error SlippageExceeded();

    function addLiquidity(uint256 amountA, uint256 amountB) public returns (uint256 shares) {
        if (amountA == 0 || amountB == 0) revert InvalidAmount();

        if (totalLiquidity == 0) {
            shares = sqrt(amountA * amountB);
        } else {
            shares = min(
                (amountA * totalLiquidity) / reserveA,
                (amountB * totalLiquidity) / reserveB
            );
        }

        reserveA += amountA;
        reserveB += amountB;
        totalLiquidity += shares;
        liquidity[msg.sender] += shares;

        emit LiquidityAdded(msg.sender, amountA, amountB, shares);
    }

    function removeLiquidity(uint256 shares) public returns (uint256 amountA, uint256 amountB) {
        if (shares == 0 || liquidity[msg.sender] < shares) revert InvalidAmount();

        amountA = (shares * reserveA) / totalLiquidity;
        amountB = (shares * reserveB) / totalLiquidity;

        liquidity[msg.sender] -= shares;
        totalLiquidity -= shares;
        reserveA -= amountA;
        reserveB -= amountB;

        emit LiquidityRemoved(msg.sender, amountA, amountB);
    }

    function swapAForB(uint256 amountIn, uint256 minOut) public returns (uint256 amountOut) {
        if (amountIn == 0) revert InvalidAmount();

        uint256 amountInWithFee = amountIn * (FEE_DENOMINATOR - FEE_NUMERATOR);
        amountOut = (amountInWithFee * reserveB) / (reserveA * FEE_DENOMINATOR + amountInWithFee);

        if (amountOut < minOut) revert SlippageExceeded();

        reserveA += amountIn;
        reserveB -= amountOut;

        emit Swap(msg.sender, amountIn, amountOut, true);
    }

    function swapBForA(uint256 amountIn, uint256 minOut) public returns (uint256 amountOut) {
        if (amountIn == 0) revert InvalidAmount();

        uint256 amountInWithFee = amountIn * (FEE_DENOMINATOR - FEE_NUMERATOR);
        amountOut = (amountInWithFee * reserveA) / (reserveB * FEE_DENOMINATOR + amountInWithFee);

        if (amountOut < minOut) revert SlippageExceeded();

        reserveB += amountIn;
        reserveA -= amountOut;

        emit Swap(msg.sender, amountIn, amountOut, false);
    }

    function getPrice(uint256 amountIn, bool aToB) public view returns (uint256) {
        if (aToB) {
            return (amountIn * reserveB) / (reserveA + amountIn);
        }
        return (amountIn * reserveA) / (reserveB + amountIn);
    }

    function sqrt(uint256 x) internal pure returns (uint256) {
        if (x == 0) return 0;
        uint256 z = (x + 1) / 2;
        uint256 y = x;
        while (z < y) {
            y = z;
            z = (x / z + z) / 2;
        }
        return y;
    }

    function min(uint256 a, uint256 b) internal pure returns (uint256) {
        return a < b ? a : b;
    }
}
```

## What SolScript Generates

SolScript converts this to a Solana program where:

- **`liquidity` mapping** becomes PDA accounts keyed by provider address
- **`reserveA` and `reserveB`** are stored in the program's state account
- **`msg.sender`** maps to the transaction signer
- **Swap events** emit as Anchor events that indexers can track
- **Slippage protection** via `minOut` prevents front-running

## DeFi Primitives on Solana

This AMM demonstrates core DeFi patterns:
- **Liquidity provision**: Users deposit both tokens and receive LP shares
- **Constant-product pricing**: Automatic price discovery without order books
- **Fee mechanism**: 0.3% fee on each swap, accruing to liquidity providers
- **Slippage protection**: `minOut` parameter prevents sandwich attacks

## Try It

Open the [SolScript Playground](/playground) and paste the AMM contract. The generated Anchor code shows how SolScript handles the complex PDA structures for liquidity tracking.

For the complete example with additional features, see the [AMM example page](/examples/amm).

## Solana vs Ethereum for DEX Development

| Aspect | Ethereum DEX | Solana DEX (via SolScript) |
|--------|-------------|---------------------------|
| Swap cost | $5-50+ gas | Under $0.01 |
| Swap speed | ~12 seconds | ~400ms |
| MEV risk | High | Lower (faster finality) |
| Language | Solidity | Solidity (SolScript) |
| Storage model | Contract storage | PDAs (automatic) |

The economic advantages of Solana for DEX operations are significant — near-zero fees make micro-swaps viable and reduce the impact of gas-based MEV extraction.
