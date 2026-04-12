---
title: "Creating an Escrow Smart Contract"
description: "Build a trustless escrow on Solana with SolScript. Learn conditional transfers, multi-party authorization, and state machine patterns."
pubDate: 2026-04-08
author: "SolScript Team"
difficulty: intermediate
duration: "45 minutes"
tags: ["escrow", "intermediate", "DeFi", "patterns"]
order: 2
---

An escrow holds funds until conditions are met. In this tutorial, you'll build a trustless escrow contract where a buyer deposits funds, and the seller receives them only when the buyer confirms delivery.

## What You'll Build

An escrow contract with:
- Buyer deposits tokens into escrow
- Seller can claim after buyer confirms
- Buyer can reclaim if seller doesn't deliver
- Automatic state tracking

## The Contract

```solidity
contract Escrow {
    address public buyer;
    address public seller;
    uint256 public amount;
    bool public buyerConfirmed;
    bool public completed;

    event Deposited(address indexed buyer, uint256 amount);
    event Confirmed(address indexed buyer);
    event Released(address indexed seller, uint256 amount);
    event Refunded(address indexed buyer, uint256 amount);

    error Unauthorized();
    error AlreadyCompleted();
    error NotConfirmed();
    error NotDeposited();

    modifier onlyBuyer() {
        if (msg.sender != buyer) revert Unauthorized();
        _;
    }

    modifier onlySeller() {
        if (msg.sender != seller) revert Unauthorized();
        _;
    }

    modifier notCompleted() {
        if (completed) revert AlreadyCompleted();
        _;
    }

    constructor(address _seller) {
        buyer = msg.sender;
        seller = _seller;
    }

    function deposit(uint256 _amount) public onlyBuyer notCompleted {
        amount = _amount;
        emit Deposited(msg.sender, _amount);
    }

    function confirm() public onlyBuyer notCompleted {
        if (amount == 0) revert NotDeposited();
        buyerConfirmed = true;
        emit Confirmed(msg.sender);
    }

    function release() public onlySeller notCompleted {
        if (!buyerConfirmed) revert NotConfirmed();
        completed = true;
        emit Released(seller, amount);
    }

    function refund() public onlyBuyer notCompleted {
        if (buyerConfirmed) revert AlreadyCompleted();
        completed = true;
        amount = 0;
        emit Refunded(buyer, amount);
    }
}
```

## Key Patterns

### State Machine

The escrow moves through states: `Created -> Deposited -> Confirmed -> Released/Refunded`. The `completed` flag and `buyerConfirmed` flag track the current state.

### Multi-Party Authorization

Different functions are restricted to different parties:
- `deposit()` and `confirm()` -- buyer only
- `release()` -- seller only
- `refund()` -- buyer only, before confirmation

### Conditional Execution

The `notCompleted` modifier prevents double-spending. The `release()` function checks `buyerConfirmed` before allowing funds to be released.

## Compiling

Open the [SolScript Playground](/playground), paste the contract, and compile. SolScript generates:
- Separate account structures for escrow state
- PDAs for the escrow instance
- Signer validation matching the buyer/seller addresses

## Next Steps

- Add a timeout mechanism so the buyer can reclaim after a deadline
- Add an arbiter role for dispute resolution
- Explore the [multisig example](/examples/multisig) for multi-party authorization patterns
