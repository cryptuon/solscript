# Automated Market Maker (AMM)

A simple constant product AMM (x * y = k) for decentralized token swaps.

## Features Demonstrated

- Constant product formula (Uniswap V2 style)
- Liquidity provider (LP) tokens
- Swap fee mechanism
- Slippage protection
- Price calculation
- Square root calculation
- Internal helper functions

## How It Works

### Constant Product Formula
The AMM maintains the invariant: `reserveA * reserveB = k`

When swapping:
```
amountOut = (amountIn * reserveOut) / (reserveIn + amountIn)
```

### Liquidity Provision
LPs deposit both tokens proportionally and receive LP tokens representing their share:
```
lpTokens = min(
  (amountA * totalLP) / reserveA,
  (amountB * totalLP) / reserveB
)
```

### Fees
Swap fees (in basis points) are deducted from input:
- 30 basis points = 0.3% fee
- Fees remain in the pool, benefiting LPs

## Contract Interface

### Initialization
- `initialize(tokenA, tokenB, amountA, amountB)` - Create pool with initial liquidity

### Liquidity Management
- `addLiquidity(amountA, amountB, minLP)` - Add liquidity, receive LP tokens
- `removeLiquidity(lpTokens, minA, minB)` - Burn LP tokens, receive underlying

### Swaps
- `swapAForB(amountIn, minOut)` - Swap token A for B
- `swapBForA(amountIn, minOut)` - Swap token B for A

### View Functions
- `getReserves()` - Current pool reserves
- `getPriceAtoB()` - Spot price A/B
- `getPriceBtoA()` - Spot price B/A
- `getAmountOut(amountIn, aToB)` - Quote expected output
- `getLPBalance(account)` - LP token balance

### Admin
- `setSwapFee(fee)` - Update swap fee (max 10%)

## Example Usage

```solidity
// Deploy AMM with 0.3% fee
constructor(30)

// Initialize pool with 1000 of each token
initialize(tokenA, tokenB, 1000, 1000)

// Add more liquidity
addLiquidity(500, 500, 400)  // Expect ~500 LP tokens

// Swap 100 A for B
swapAForB(100, 80)  // Min 80 B out

// Check price
getPriceAtoB()  // Returns price with 6 decimal precision

// Remove liquidity
removeLiquidity(200, 180, 180)  // Burn 200 LP, expect ~200 of each
```

## Build & Deploy

```bash
solscript check amm.sol
solscript build amm.sol
```

## Key Concepts

### Price Impact
Larger trades relative to reserves cause more price impact (slippage).

### Impermanent Loss
LPs may experience IL when prices diverge from entry point.

### Arbitrage
Price differences with other markets create arbitrage opportunities that rebalance the pool.

## Security Considerations

- Minimum liquidity locked to prevent manipulation
- Slippage protection on all operations
- Fee caps to prevent abuse
- Consider flash loan protection for production
