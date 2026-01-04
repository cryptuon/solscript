# SolScript Examples

Example contracts demonstrating SolScript features.

## Examples

### Counter
A simple counter contract demonstrating basic state management and access control.

```bash
cd counter
solscript build counter.sol
```

### Token
ERC20-style fungible token with transfers, approvals, minting, and burning.

```bash
cd token
solscript build token.sol
```

### Voting
Decentralized voting system with weighted votes and time-limited proposals.

```bash
cd voting
solscript build voting.sol
```

### Escrow
Trustless escrow contract with buyer protection and arbiter dispute resolution.

```bash
cd escrow
solscript build escrow.sol
```

### NFT
Non-Fungible Token collection with metadata and marketplace-ready features.

```bash
cd nft
solscript build nft.sol
```

## Running Examples

1. Navigate to an example directory
2. Build the contract:
   ```bash
   solscript build <contract>.sol
   ```
3. Deploy to devnet:
   ```bash
   solscript deploy <contract>.sol --network devnet
   ```

## Features Demonstrated

| Example | Features |
|---------|----------|
| Counter | State variables, modifiers, events, errors |
| Token | Mappings, transfers, approvals, pausable |
| Voting | Enums, structs, time-based logic |
| Escrow | State machines, multi-party transactions |
| NFT | ERC721 pattern, metadata, minting |
