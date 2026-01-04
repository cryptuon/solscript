# Language Overview

SolScript is a high-level language designed for writing Solana smart contracts. It combines Solidity-style syntax with Solana's unique features.

## Design Philosophy

1. **Familiarity** - Solidity-like syntax for easy adoption
2. **Safety** - Strong typing and compile-time checks
3. **Efficiency** - Optimized for Solana's runtime
4. **Simplicity** - Hide complexity, expose power

## Basic Structure

A SolScript file consists of:

```solidity
// Imports (optional)
import { IERC20 } from "@solana/token";

// Custom errors
error InsufficientFunds(uint256 available, uint256 required);

// Events
event Transfer(address indexed from, address indexed to, uint256 amount);

// Structs
struct User {
    address wallet;
    uint256 balance;
    bool active;
}

// Enums
enum Status { Pending, Active, Completed }

// Interfaces
interface IToken {
    function transfer(address to, uint256 amount) external returns (bool);
}

// Contract
contract MyContract {
    // State variables
    uint256 public totalSupply;
    mapping(address => uint256) public balances;

    // Constructor
    constructor(uint256 initialSupply) {
        totalSupply = initialSupply;
    }

    // Functions
    function transfer(address to, uint256 amount) public {
        // Implementation
    }
}
```

## Key Concepts

### Contracts

Contracts are the main building blocks. Each contract:

- Has state variables (on-chain storage)
- Has functions (entry points)
- Can emit events
- Can inherit from other contracts

```solidity
contract Token is IERC20 {
    // ...
}
```

### State Variables

State is stored on-chain in Solana accounts:

```solidity
uint256 public totalSupply;           // Simple value
mapping(address => uint256) balances; // Key-value store (PDA)
User[] public users;                  // Dynamic array
```

### Functions

Functions define contract behavior:

```solidity
// Public - callable by anyone
function transfer(address to, uint256 amount) public { }

// External - only callable from outside
function deposit() external payable { }

// View - read-only, no state changes
function balanceOf(address account) public view returns (uint256) { }

// Pure - no state access at all
function add(uint256 a, uint256 b) public pure returns (uint256) { }
```

### Built-in Objects

SolScript provides several built-in objects:

| Object | Description |
|--------|-------------|
| `msg.sender` | Transaction signer |
| `msg.value` | SOL amount sent |
| `block.timestamp` | Current block timestamp |
| `block.number` | Current slot number |

### Control Flow

Standard control flow statements:

```solidity
// If-else
if (condition) {
    // ...
} else if (other) {
    // ...
} else {
    // ...
}

// For loop
for (uint256 i = 0; i < 10; i++) {
    // ...
}

// While loop
while (condition) {
    // ...
}

// Ternary
uint256 max = a > b ? a : b;
```

### Error Handling

```solidity
// Require - revert with message
require(amount > 0, "Amount must be positive");

// Custom errors
error InsufficientBalance(uint256 available, uint256 required);

// Revert with custom error
if (balance < amount) {
    revert InsufficientBalance(balance, amount);
}
```

## Solana-Specific Features

### Accounts and PDAs

SolScript automatically handles Solana's account model:

```solidity
// Mappings become PDAs
mapping(address => uint256) public balances;

// Access like normal
balances[user] = 100;
```

### Cross-Program Invocation (CPI)

Call other Solana programs:

```solidity
interface ITokenProgram {
    function transfer(address from, address to, uint256 amount) external;
}

contract MyContract {
    ITokenProgram token;

    function sendTokens(address to, uint256 amount) public {
        token.transfer(msg.sender, to, amount);
    }
}
```

### Signers

Require transaction signatures:

```solidity
function withdraw(signer authority, uint256 amount) public {
    require(authority == owner, "Unauthorized");
    // ...
}
```

## Type System

SolScript has a strong type system:

| Category | Types |
|----------|-------|
| Integers | `uint8` to `uint256`, `int8` to `int256` |
| Boolean | `bool` |
| Address | `address` |
| String | `string` |
| Bytes | `bytes`, `bytes32` |
| Arrays | `T[]`, `T[N]` |
| Mappings | `mapping(K => V)` |

## Visibility

| Modifier | Accessible From |
|----------|-----------------|
| `public` | Anywhere |
| `private` | Same contract only |
| `internal` | Same contract + derived |
| `external` | Outside only |

## Modifiers

Reusable access control:

```solidity
modifier onlyOwner() {
    require(msg.sender == owner, "Not owner");
    _;
}

function withdraw() public onlyOwner {
    // Only owner can call
}
```

## Next Steps

- [Contracts](contracts.md) - Contract structure in depth
- [Types](types.md) - Complete type reference
- [Functions](functions.md) - Function patterns
