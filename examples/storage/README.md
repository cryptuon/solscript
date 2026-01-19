# Storage Patterns

Demonstrates different data types and storage patterns in SolScript.

## Features Demonstrated

- Primitive types (uint64, int64, bool, address)
- Single-level mappings
- Nested mappings
- Events with indexed parameters
- Arrays as function parameters
- Multiple return values
- Loops and iteration

## Data Types

### Primitives
- `uint64` - Unsigned 64-bit integer
- `int64` - Signed 64-bit integer
- `bool` - Boolean (true/false)
- `address` - Solana account address

### Collections
- `mapping(K => V)` - Key-value store
- `mapping(K1 => mapping(K2 => V))` - Nested mapping
- `T[]` - Dynamic array

## Contract Interface

### State Variables
- `counter` - Simple counter value
- `signedValue` - Signed integer example
- `isActive` - Boolean flag
- `owner` - Contract owner address
- `balances` - Address to balance mapping
- `allowances` - Nested allowance mapping

### Functions
- `incrementCounter()` - Increment and emit event
- `setSignedValue(value)` - Set signed integer
- `toggleActive()` - Toggle boolean
- `setBalance(account, amount)` - Set single balance
- `setAllowance(spender, amount)` - Set allowance
- `batchSetBalances(accounts, amounts)` - Batch operation
- `getContractState()` - Return multiple values

## Build & Deploy

```bash
solscript check storage.sol
solscript build storage.sol
```

## Next Steps

- [token](../token/) - Uses mappings for balances
- [voting](../voting/) - Uses structs and enums
