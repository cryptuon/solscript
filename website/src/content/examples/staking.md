---
title: "Staking Pool"
description: "A token staking pool with reward distribution. Users stake tokens to earn rewards proportional to their share of the pool."
tags: ["staking", "DeFi", "rewards", "pool"]
difficulty: intermediate
order: 4
sourceFile: "staking/staking.sol"
---

## Overview

The staking pool contract lets users deposit tokens and earn rewards over time. Rewards are distributed proportionally based on each user's share of the total staked amount.

Key patterns:
- **Time-based rewards**: Accumulating rewards over block time
- **Proportional distribution**: Fair reward splitting among stakers
- **Deposit and withdrawal**: Stake and unstake with balance tracking
- **Reward calculation**: Per-share reward accumulation
