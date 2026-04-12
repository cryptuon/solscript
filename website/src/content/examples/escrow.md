---
title: "Escrow"
description: "A trustless escrow contract for conditional token transfers between buyer and seller. Demonstrates state machines and multi-party authorization."
tags: ["escrow", "payments", "trustless"]
difficulty: intermediate
order: 7
sourceFile: "escrow/escrow.sol"
---

## Overview

The escrow contract holds funds from a buyer until conditions are met, then releases them to the seller. This is a foundational pattern for marketplaces and peer-to-peer trading.

Key patterns:
- **State machine**: Created -> Deposited -> Confirmed -> Released/Refunded
- **Multi-party authorization**: Buyer and seller have different permissions
- **Conditional execution**: Funds release only after buyer confirmation
- **Refund mechanism**: Buyer can reclaim before confirming
