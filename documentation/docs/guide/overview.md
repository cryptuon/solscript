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

| Object | Description | Status |
|--------|-------------|--------|
| `msg.sender` | Transaction signer | ✅ Works |
| `msg.value` | SOL amount sent | ⚠️ Returns 0 (see [limitations](../reference/roadmap.md)) |
| `block.timestamp` | Current Unix timestamp | ✅ Works |
| `block.number` | Current Solana slot | ✅ Works |

!!! warning "msg.value Limitation"
    `msg.value` currently returns 0. Direct SOL transfers are not yet supported.
    Use wrapped SOL (SPL Token) for value transfers. See the [roadmap](../reference/roadmap.md) for planned improvements.

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
// Mappings become PDAs automatically
mapping(address => uint256) public balances;

// Access like normal Solidity - SolScript handles PDA derivation
balances[user] = 100;  // Creates PDA with seeds [b"balances", user]
```

This is SolScript's most powerful feature - you write Solidity-style mappings and get proper Solana PDAs.

### Cross-Program Invocation (CPI)

Call other Solana programs via interfaces:

```solidity
interface ITokenProgram {
    function transfer(address from, address to, uint256 amount) external;
}

contract MyContract {
    function sendTokens(address token, address to, uint256 amount) public {
        ITokenProgram(token).transfer(msg.sender, to, amount);
    }
}
```

### SPL Token Built-ins

SolScript has native SPL Token support:

```solidity
// These generate proper CPI calls to SPL Token program
tokenTransfer(from, to, amount);
tokenMint(mint, to, amount);
tokenBurn(account, amount);
```

### Signers

Require additional transaction signatures:

```solidity
function withdraw(signer authority, uint256 amount) public {
    require(authority == owner, "Unauthorized");
    // authority becomes a Signer<'info> in generated Anchor code
}
```

### Time-Based Logic

Access Solana's clock sysvar:

```solidity
function isExpired(uint64 deadline) public view returns (bool) {
    return block.timestamp >= deadline;
}
```

For more details, see [Solana Concepts](solana-concepts.md).

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
