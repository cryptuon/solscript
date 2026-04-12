---
title: "Building Your First Token Contract with SolScript"
description: "Create a full-featured token on Solana using SolScript's Solidity syntax. Covers minting, transfers, allowances, burning, and access control."
pubDate: 2026-04-10
author: "SolScript Team"
difficulty: beginner
duration: "30 minutes"
tags: ["token", "SPL", "beginner", "DeFi"]
order: 1
---

In this tutorial, you'll build a complete token contract on Solana using SolScript. The contract supports minting, transfers, allowances, burning, and owner-only access control.

## Prerequisites

- Basic understanding of Solidity syntax
- A web browser (no installation needed)

## Step 1: Set Up the Contract

Open the [SolScript Playground](/playground) and create a new file called `token.sol`.

Start with the contract shell and state variables:

```solidity
contract Token {
    string public name;
    string public symbol;
    uint8 public decimals = 9;
    uint256 public totalSupply;
    mapping(address => uint256) public balanceOf;
    address public owner;
}
```

These state variables define your token's metadata and storage. The `balanceOf` mapping tracks how many tokens each address holds -- SolScript will convert this to PDA-based accounts on Solana.

## Step 2: Add Events and Errors

Events let off-chain applications track what happens on-chain. Custom errors provide clear failure messages.

```solidity
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Mint(address indexed to, uint256 amount);
    event Burn(address indexed from, uint256 amount);

    error InsufficientBalance(uint256 available, uint256 required);
    error Unauthorized();
```

## Step 3: Add Access Control

Use a modifier to restrict minting to the contract owner:

```solidity
    modifier onlyOwner() {
        if (msg.sender != owner) revert Unauthorized();
        _;
    }
```

## Step 4: Add the Constructor

The constructor initializes the token and optionally mints an initial supply:

```solidity
    constructor(string memory _name, string memory _symbol, uint256 _initialSupply) {
        name = _name;
        symbol = _symbol;
        owner = msg.sender;
        if (_initialSupply > 0) {
            totalSupply = _initialSupply;
            balanceOf[msg.sender] = _initialSupply;
            emit Transfer(address(0), msg.sender, _initialSupply);
        }
    }
```

## Step 5: Implement Transfers

The core transfer function moves tokens between addresses:

```solidity
    function transfer(address to, uint256 amount) public returns (bool) {
        if (balanceOf[msg.sender] < amount) {
            revert InsufficientBalance(balanceOf[msg.sender], amount);
        }
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        emit Transfer(msg.sender, to, amount);
        return true;
    }
```

## Step 6: Add Minting and Burning

Owner-only minting and public burning:

```solidity
    function mint(address to, uint256 amount) public onlyOwner {
        totalSupply += amount;
        balanceOf[to] += amount;
        emit Mint(to, amount);
        emit Transfer(address(0), to, amount);
    }

    function burn(uint256 amount) public {
        if (balanceOf[msg.sender] < amount) {
            revert InsufficientBalance(balanceOf[msg.sender], amount);
        }
        balanceOf[msg.sender] -= amount;
        totalSupply -= amount;
        emit Burn(msg.sender, amount);
    }
```

## Step 7: Compile and Deploy

1. Click **Compile** in the playground
2. Review the generated Anchor/Rust code in the output panel
3. Connect your Solana wallet and select **Devnet**
4. Click **Deploy**

Your token is now live on Solana devnet.

## What SolScript Generated

Behind the scenes, SolScript created:
- A PDA account structure for each user's balance
- An `initialize` instruction from your constructor
- Anchor account validation for every instruction
- Signer checks for the `onlyOwner` modifier
- Event emission via Anchor's event system

## Next Steps

- Add `approve` and `transferFrom` for allowance support (see the [full token example](/examples/token))
- Add a `pause` mechanism for emergency stops
- Explore the [escrow tutorial](/tutorials/creating-an-escrow-contract) for more advanced patterns
