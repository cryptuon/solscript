---
title: "Multi-Sig Wallet Tutorial"
description: "Build a multi-signature wallet on Solana with SolScript. Require multiple approvals before executing transactions."
pubDate: 2026-04-05
author: "SolScript Team"
difficulty: advanced
duration: "60 minutes"
tags: ["multisig", "advanced", "security", "wallet"]
order: 3
---

A multi-signature wallet requires multiple owners to approve a transaction before it executes. This is essential for treasury management, DAO governance, and team-controlled funds.

## What You'll Build

A multi-sig wallet where:
- Multiple owners are registered at creation
- Any owner can propose a transaction
- A configurable threshold of owners must approve
- The transaction executes when the threshold is met

## The Contract

```solidity
contract MultiSig {
    address[] public owners;
    uint256 public required;
    uint256 public transactionCount;

    mapping(uint256 => address) public transactionTo;
    mapping(uint256 => uint256) public transactionValue;
    mapping(uint256 => bool) public transactionExecuted;
    mapping(uint256 => uint256) public confirmationCount;
    mapping(uint256 => mapping(address => bool)) public confirmations;
    mapping(address => bool) public isOwner;

    event Submitted(uint256 indexed txId, address indexed to, uint256 value);
    event Confirmed(uint256 indexed txId, address indexed owner);
    event Executed(uint256 indexed txId);

    error NotOwner();
    error AlreadyConfirmed();
    error AlreadyExecuted();
    error NotEnoughConfirmations();

    modifier onlyOwner() {
        if (!isOwner[msg.sender]) revert NotOwner();
        _;
    }

    constructor(address[] memory _owners, uint256 _required) {
        required = _required;
        for (uint256 i = 0; i < _owners.length; i++) {
            owners.push(_owners[i]);
            isOwner[_owners[i]] = true;
        }
    }

    function submit(address to, uint256 value) public onlyOwner {
        uint256 txId = transactionCount;
        transactionTo[txId] = to;
        transactionValue[txId] = value;
        transactionCount += 1;
        emit Submitted(txId, to, value);
    }

    function confirm(uint256 txId) public onlyOwner {
        if (confirmations[txId][msg.sender]) revert AlreadyConfirmed();
        if (transactionExecuted[txId]) revert AlreadyExecuted();

        confirmations[txId][msg.sender] = true;
        confirmationCount[txId] += 1;
        emit Confirmed(txId, msg.sender);
    }

    function execute(uint256 txId) public onlyOwner {
        if (transactionExecuted[txId]) revert AlreadyExecuted();
        if (confirmationCount[txId] < required) revert NotEnoughConfirmations();

        transactionExecuted[txId] = true;
        emit Executed(txId);
    }
}
```

## How It Works

### 1. Initialization

The constructor takes an array of owner addresses and a required confirmation threshold. Each owner is registered in the `isOwner` mapping.

### 2. Submit a Transaction

Any owner calls `submit()` with the destination address and value. This creates a new transaction proposal with a unique ID.

### 3. Confirm

Each owner calls `confirm()` to approve a transaction. The contract tracks which owners have confirmed and prevents double-confirmation.

### 4. Execute

Once enough owners confirm (meeting the `required` threshold), any owner can call `execute()` to finalize the transaction.

## PDA Structure

SolScript converts the multiple mappings into efficient PDA structures:
- `transactionTo[id]` and `transactionValue[id]` share a PDA per transaction
- `confirmations[id][owner]` creates a PDA per (transaction, owner) pair
- `isOwner[addr]` creates a PDA per owner

## Security Considerations

- Always set `required` to at least 2 for meaningful multi-sig security
- Consider adding a mechanism to add/remove owners
- Add a timelock for high-value transactions
- The contract should be audited before managing real funds

## Try It

Open the [SolScript Playground](/playground) and compile this contract. Review the generated Anchor code to see how SolScript handles the complex PDA derivations for nested mappings.
