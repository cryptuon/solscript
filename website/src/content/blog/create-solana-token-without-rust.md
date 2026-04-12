---
title: "Create a Solana Token Without Learning Rust"
description: "Build and deploy an SPL token on Solana using Solidity syntax with SolScript. No Rust knowledge required. Step-by-step guide with browser-based compilation."
pubDate: 2026-04-11
author: "SolScript Team"
tags: ["token", "SPL", "no rust", "beginner", "solana", "tutorial"]
---

Creating a token on Solana traditionally requires writing Rust — a language with a steep learning curve. With SolScript, you can create a fully functional SPL token using Solidity syntax and deploy it from your browser.

## What You'll Build

A token with:
- Configurable name, symbol, and decimals
- Minting (owner-only)
- Transfers between addresses
- Balance tracking
- Burn functionality

All written in Solidity. No Rust.

## Step 1: Open the Playground

Go to the [SolScript Playground](/playground). It runs a WASM-compiled SolScript compiler in your browser — no installation, no CLI, no local Rust toolchain.

## Step 2: Write the Token Contract

```solidity
contract MyToken {
    string public name;
    string public symbol;
    uint8 public decimals = 9;
    uint256 public totalSupply;
    mapping(address => uint256) public balanceOf;
    address public owner;

    event Transfer(address indexed from, address indexed to, uint256 value);
    event Mint(address indexed to, uint256 amount);

    error InsufficientBalance(uint256 available, uint256 required);
    error Unauthorized();

    modifier onlyOwner() {
        if (msg.sender != owner) revert Unauthorized();
        _;
    }

    constructor(string memory _name, string memory _symbol) {
        name = _name;
        symbol = _symbol;
        owner = msg.sender;
    }

    function mint(address to, uint256 amount) public onlyOwner {
        totalSupply += amount;
        balanceOf[to] += amount;
        emit Mint(to, amount);
        emit Transfer(address(0), to, amount);
    }

    function transfer(address to, uint256 amount) public returns (bool) {
        if (balanceOf[msg.sender] < amount) {
            revert InsufficientBalance(balanceOf[msg.sender], amount);
        }
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        emit Transfer(msg.sender, to, amount);
        return true;
    }

    function burn(uint256 amount) public {
        if (balanceOf[msg.sender] < amount) {
            revert InsufficientBalance(balanceOf[msg.sender], amount);
        }
        balanceOf[msg.sender] -= amount;
        totalSupply -= amount;
    }
}
```

If you've written Solidity on Ethereum before, this looks completely familiar. That's the point.

## Step 3: Compile

Click **Compile**. SolScript:
1. Parses the Solidity-style contract
2. Converts `balanceOf` mapping to Solana PDAs
3. Generates Rust/Anchor code with all account validation
4. Shows you the generated code in the output panel

## Step 4: Deploy to Devnet

1. Connect your Solana wallet (Phantom or Solflare)
2. Select **Devnet** as the target network
3. Click **Deploy**

Your token is now live on Solana devnet.

## What Happened Behind the Scenes

SolScript converted your Solidity into a native Solana program:

- `mapping(address => uint256) public balanceOf` became **PDA accounts** — one per user, derived from their public key
- `constructor()` became an Anchor **`initialize` instruction**
- `msg.sender` mapped to the **transaction signer**
- Events compiled to **Anchor events** logged on-chain
- Custom errors became **Anchor error types**

You never touched Rust. The generated code is standard Anchor that any Solana auditor can review.

## Comparison: Token Creation Methods on Solana

| Method | Language | Setup Required | Learning Curve |
|--------|----------|---------------|----------------|
| **SolScript** | Solidity | Browser only | Low (if you know Solidity) |
| Anchor (Rust) | Rust | Rust toolchain, Solana CLI | High |
| spl-token CLI | CLI commands | Solana CLI | Medium |
| Solang | Solidity | Solang compiler, Solana CLI | Medium |
| No-code tools | GUI | Browser | None |

SolScript hits the sweet spot: real smart contract code with minimal setup.

## Next Steps

- Add **allowance** support with `approve` and `transferFrom` — see the [full token example](/examples/token)
- Learn how the PDA mapping works in our [PDA deep dive](/blog/understanding-pda-mappings)
- Try the [escrow tutorial](/tutorials/creating-an-escrow-contract) for more advanced patterns
