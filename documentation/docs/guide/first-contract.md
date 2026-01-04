# Your First Contract

Let's build a simple counter contract step by step.

## The Complete Contract

```solidity
contract Counter {
    // State variables
    uint64 public count;
    address public owner;

    // Events
    event CountChanged(address indexed changer, uint64 oldValue, uint64 newValue);
    event OwnerChanged(address indexed oldOwner, address indexed newOwner);

    // Errors
    error Unauthorized(address caller);
    error InvalidAmount(uint64 amount);

    // Constructor
    constructor() {
        owner = msg.sender;
        count = 0;
    }

    // Increment the counter
    function increment() public {
        uint64 oldValue = count;
        count += 1;
        emit CountChanged(msg.sender, oldValue, count);
    }

    // Increment by a specific amount
    function incrementBy(uint64 amount) public {
        require(amount > 0, "Amount must be positive");
        uint64 oldValue = count;
        count += amount;
        emit CountChanged(msg.sender, oldValue, count);
    }

    // Decrement the counter
    function decrement() public {
        require(count > 0, "Counter cannot go below zero");
        uint64 oldValue = count;
        count -= 1;
        emit CountChanged(msg.sender, oldValue, count);
    }

    // Reset counter (owner only)
    function reset() public {
        require(msg.sender == owner, "Only owner can reset");
        uint64 oldValue = count;
        count = 0;
        emit CountChanged(msg.sender, oldValue, 0);
    }

    // Transfer ownership
    function transferOwnership(address newOwner) public {
        require(msg.sender == owner, "Only owner can transfer");
        require(newOwner != address(0), "Invalid new owner");

        address oldOwner = owner;
        owner = newOwner;
        emit OwnerChanged(oldOwner, newOwner);
    }

    // View functions
    function getCount() public view returns (uint64) {
        return count;
    }

    function getOwner() public view returns (address) {
        return owner;
    }
}
```

## Breaking It Down

### State Variables

```solidity
uint64 public count;
address public owner;
```

State variables are stored on-chain. The `public` keyword automatically generates getter functions.

- `uint64` - 64-bit unsigned integer
- `address` - Solana public key (32 bytes)

### Events

```solidity
event CountChanged(address indexed changer, uint64 oldValue, uint64 newValue);
```

Events are logged to the transaction and can be monitored by clients. The `indexed` keyword allows efficient filtering.

### Errors

```solidity
error Unauthorized(address caller);
error InvalidAmount(uint64 amount);
```

Custom errors provide meaningful failure messages with associated data.

### Constructor

```solidity
constructor() {
    owner = msg.sender;
    count = 0;
}
```

The constructor runs once when the contract is deployed. `msg.sender` is the account that deployed the contract.

### Functions

#### Public Functions

```solidity
function increment() public {
    count += 1;
}
```

Public functions can be called by anyone.

#### View Functions

```solidity
function getCount() public view returns (uint64) {
    return count;
}
```

View functions don't modify state and are read-only.

### Access Control

```solidity
require(msg.sender == owner, "Only owner can reset");
```

Use `require` to enforce conditions. If the condition is false, the transaction reverts with the error message.

### Emitting Events

```solidity
emit CountChanged(msg.sender, oldValue, count);
```

Use `emit` to log events to the blockchain.

## Build and Test

### Check for Errors

```bash
solscript check src/main.sol
```

### Add Tests

```solidity
#[test]
function testIncrement() {
    // Initial state
    assertEq(count, 0, "Initial count should be 0");

    // Increment
    increment();
    assertEq(count, 1, "Count should be 1 after increment");

    // Increment again
    increment();
    assertEq(count, 2, "Count should be 2 after second increment");
}

#[test]
function testIncrementBy() {
    incrementBy(5);
    assertEq(count, 5, "Count should be 5");

    incrementBy(10);
    assertEq(count, 15, "Count should be 15");
}

#[test]
#[should_fail]
function testDecrementUnderflow() {
    // This should fail - can't decrement below 0
    decrement();
}
```

### Run Tests

```bash
solscript test src/main.sol
```

## Deploy

### To Localnet

```bash
# Start local validator
solana-test-validator

# Deploy
solscript deploy src/main.sol --cluster localnet
```

### To Devnet

```bash
# Get devnet SOL
solana airdrop 2 --url devnet

# Deploy
solscript deploy src/main.sol --cluster devnet
```

## Interact with Your Contract

After deployment, you can interact with your contract using:

1. **Solana CLI** - For basic operations
2. **Anchor Client** - For TypeScript/JavaScript
3. **Web3.js** - For web applications

Example using Anchor client:

```typescript
import * as anchor from "@coral-xyz/anchor";

const program = anchor.workspace.Counter;

// Call increment
await program.methods.increment().rpc();

// Read count
const count = await program.account.counter.fetch(counterPDA);
console.log("Count:", count.count.toString());
```

## Next Steps

- [Contracts](contracts.md) - Learn more about contract structure
- [Types](types.md) - Explore all available types
- [Functions](functions.md) - Function modifiers and patterns
