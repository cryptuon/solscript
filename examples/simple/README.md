# Simple Counter

A minimal SolScript contract demonstrating basic state management.

## Features Demonstrated

- Contract definition
- State variables
- Constructor
- Public functions
- View functions

## Contract Interface

### State Variables
- `count` - A public counter value (uint64)

### Functions
- `increment()` - Increase counter by 1
- `getCount()` - Read the current count

## Build & Deploy

```bash
solscript check simple.sol
solscript build simple.sol
```

## Next Steps

- [counter](../counter/) - Adds events, errors, and access control
- [storage](../storage/) - Learn about different data types
