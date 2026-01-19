# Hello World

Your first SolScript contract - the simplest possible example.

## Features Demonstrated

- Contract definition
- State variables with `public` visibility
- Constructor initialization
- View functions (read-only)
- State-changing functions

## Contract Interface

### State Variables
- `greeting` - A public string stored on-chain

### Functions
- `getGreeting()` - Returns the current greeting
- `setGreeting(newGreeting)` - Updates the greeting

## Build & Deploy

```bash
# Check the contract
solscript check hello.sol

# Build to BPF
solscript build hello.sol

# Deploy to devnet
solscript deploy hello.sol --cluster devnet
```

## Next Steps

Once you understand this example, try:
- [simple](../simple/) - More state management
- [counter](../counter/) - Events and access control
