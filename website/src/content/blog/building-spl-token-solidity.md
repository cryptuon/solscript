---
title: "Building an SPL Token in Solidity Syntax"
description: "Create a fully functional Solana SPL token using SolScript's Solidity syntax. Complete walkthrough with minting, transfers, burning, and access control."
pubDate: 2026-04-01
author: "SolScript Team"
tags: ["token", "SPL", "tutorial", "DeFi", "solana"]
---

Tokens are the building block of DeFi. On Ethereum, you write ERC-20 contracts. On Solana, tokens use the SPL Token standard. With SolScript, you write token contracts in Solidity syntax and the compiler generates the correct SPL Token interactions.

## The Token Contract

Here's a complete token contract in SolScript:

```solidity
contract Token {
    string public name;
    string public symbol;
    uint8 public decimals = 9;
    uint256 public totalSupply;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    address public owner;

    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);

    error InsufficientBalance(uint256 available, uint256 required);
    error InsufficientAllowance(uint256 available, uint256 required);
    error Unauthorized();

    modifier onlyOwner() {
        if (msg.sender != owner) revert Unauthorized();
        _;
    }

    constructor(string memory _name, string memory _symbol, uint256 _initialSupply) {
        name = _name;
        symbol = _symbol;
        owner = msg.sender;
        if (_initialSupply > 0) {
            _mint(msg.sender, _initialSupply);
        }
    }

    function transfer(address to, uint256 amount) public returns (bool) {
        _transfer(msg.sender, to, amount);
        return true;
    }

    function approve(address spender, uint256 amount) public returns (bool) {
        allowance[msg.sender][spender] = amount;
        emit Approval(msg.sender, spender, amount);
        return true;
    }

    function mint(address to, uint256 amount) public onlyOwner {
        _mint(to, amount);
    }

    function burn(uint256 amount) public {
        if (balanceOf[msg.sender] < amount) {
            revert InsufficientBalance(balanceOf[msg.sender], amount);
        }
        balanceOf[msg.sender] -= amount;
        totalSupply -= amount;
    }

    function _transfer(address from, address to, uint256 amount) internal {
        if (balanceOf[from] < amount) {
            revert InsufficientBalance(balanceOf[from], amount);
        }
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        emit Transfer(from, to, amount);
    }

    function _mint(address to, uint256 amount) internal {
        totalSupply += amount;
        balanceOf[to] += amount;
        emit Transfer(address(0), to, amount);
    }
}
```

## What SolScript Generates

When compiled, SolScript produces:

1. **Account structures** for token state and per-user balance PDAs
2. **Anchor instructions** for initialize, transfer, approve, mint, and burn
3. **PDA derivation** for the `balanceOf` and `allowance` mappings
4. **Access control** via signer validation for the `onlyOwner` modifier
5. **Events** emitted as Anchor events

## Deploying the Token

1. Open the [SolScript Playground](/playground)
2. Paste the token contract
3. Click **Compile** to verify it builds
4. Connect your Solana wallet (Phantom or Solflare)
5. Select **Devnet** as the network
6. Click **Deploy**

Your token is now live on Solana devnet. You can interact with it using any Solana wallet or SDK.

## Next Steps

- Add a **pause** mechanism for emergency stops
- Implement **transferFrom** with allowance checking
- Browse the [full token example](/examples/token) for the complete implementation
