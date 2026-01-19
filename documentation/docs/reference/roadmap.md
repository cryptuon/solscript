# Roadmap

This document outlines SolScript's current capabilities, known limitations, and planned improvements.

## Current Status: Beta

SolScript is functional for common smart contract patterns. The compiler generates valid Anchor/Rust code that compiles and deploys to Solana.

## Feature Matrix

| Feature | Status | Notes |
|---------|--------|-------|
| State variables | ✅ Complete | All primitive types, structs, arrays |
| Mappings → PDAs | ✅ Complete | Automatic transformation with seeds |
| Events | ✅ Complete | Full Anchor event support |
| Custom errors | ✅ Complete | Error codes with parameters, empty errors supported |
| Modifiers | ✅ Complete | Inlined into functions |
| View functions | ✅ Complete | Read-only account access |
| Access control | ✅ Complete | Via modifiers and require |
| CPI (interfaces) | ✅ Complete | Cross-program calls |
| SPL Token | ✅ Complete | Transfer, mint, burn |
| `msg.sender` | ✅ Complete | Signer account |
| `block.timestamp` | ✅ Complete | Clock sysvar |
| Rent handling | ✅ Complete | Auto rent-exempt accounts |
| Structs in contracts | ✅ Complete | Define inside or outside contracts |
| Enums in contracts | ✅ Complete | Define inside or outside contracts |
| Direct SOL transfers | ✅ Complete | `transfer(to, amount)` built-in |
| PDA account closing | ✅ Complete | `delete mapping[key]` closes PDAs |
| Token 2022 | ❌ Not implemented | Only SPL Token |
| Compute budget | ❌ Not implemented | Uses defaults |

---

## Known Limitations & Planned Remediations

### 1. ~~No Direct SOL/Lamport Transfers~~ ✅ IMPLEMENTED in v0.3.0

**Status:** Implemented!

Use the `transfer(to, amount)` built-in function to transfer SOL:

```solidity
function withdraw(address to, uint64 amount) public {
    require(msg.sender == owner, "Unauthorized");
    transfer(to, amount);  // Transfers SOL to recipient
}
```

**How it works:**
- Generates Anchor `system_program::transfer` CPI
- Automatically adds `recipient` account to context
- Validates recipient matches the `to` address
- Rent is deducted from signer's account

**Note:** `msg.value` still returns 0 for incoming payments. Use SPL Token (wrapped SOL) for receiving payments.

---

### 2. No Token 2022 Support

**Current behavior:** Only SPL Token program is supported.

**Impact:** Cannot use Token 2022 features (transfer fees, interest-bearing, etc.)

**Workaround:** Extend generated code manually for Token 2022.

**Planned remediation:**
- Add Token 2022 interface definitions
- Support extension detection
- Generate proper Token 2022 CPI calls
- Target: v0.4.0

---

### 3. ~~Structs/Enums Inside Contracts~~ ✅ IMPLEMENTED in v0.2.0

**Status:** Implemented!

You can now define structs and enums inside contracts:

```solidity
contract Token {
    // Struct inside contract
    struct Balance {
        uint256 amount;
        uint64 lastUpdate;
    }

    // Enum inside contract
    enum Status { Active, Paused, Closed }

    mapping(address => Balance) public balances;
    Status public status;
}
```

Both inside-contract and outside-contract definitions work seamlessly.

---

### 4. ~~Empty Error Declarations~~ ✅ IMPLEMENTED in v0.2.0

**Status:** Implemented!

You can now define errors with empty parameter lists:

```solidity
// All of these work
error Unauthorized();           // Empty parentheses
error NotOwner;                 // No parentheses
error InsufficientBalance(uint256 available, uint256 required);  // With params

function withdraw() public {
    if (msg.sender != owner) {
        revert Unauthorized();
    }
}
```

---

### 5. ~~PDA Account Closing for Mappings~~ ✅ IMPLEMENTED in v0.3.0

**Status:** Implemented!

Use `delete mapping[key]` to close a mapping PDA and reclaim rent:

```solidity
contract UserRegistry {
    mapping(address => uint64) public scores;

    function removeUser(address user) public {
        delete scores[user];  // Closes PDA, refunds rent to signer
    }
}
```

**How it works:**
- Generates `close = signer` account constraint
- PDA is closed automatically by Anchor
- Lamports (rent) returned to the transaction signer
- Works with nested mappings too

---

### 6. No Compute Budget Control

**Current behavior:** Uses Solana's default compute budget.

**Impact:** Complex operations may exceed limits.

**Workaround:** Request additional compute units client-side.

**Planned remediation:**
- Add `@computeBudget(units)` function attribute
- Generate compute budget instructions
- Target: v0.4.0

---

### 7. Modifiers Are Inlined

**Current behavior:** Modifier bodies are copied into each function.

**Impact:** Code duplication, larger program size.

**Workaround:** Keep modifiers small.

**Planned remediation:**
- Generate separate validation functions
- Call validation before main logic
- Share common checks across functions
- Target: v0.3.0

---

### 8. No Versioned Transactions

**Current behavior:** Standard transaction format only.

**Impact:** Cannot use address lookup tables for account compression.

**Workaround:** Build versioned transactions client-side.

**Planned remediation:**
- Support lookup table references
- Generate v0 transaction metadata
- Target: v0.5.0

---

### 9. Limited Sysvar Access

**Current behavior:** Only Clock and Rent sysvars.

**Impact:** Cannot access recent blockhashes, stake history, etc.

**Workaround:** Access via CPI to system program.

**Planned remediation:**
- Add `blockhash`, `epochSchedule`, `fees` built-ins
- Generate proper sysvar account includes
- Target: v0.4.0

---

## Release Timeline

### v0.2.0 - Parser Improvements ✅ RELEASED
- [x] Structs inside contracts
- [x] Enums inside contracts
- [x] Empty error declarations
- [ ] Improved error messages

### v0.3.0 - Solana Native Features ✅ RELEASED
- [x] Direct SOL transfers (`transfer(to, amount)`)
- [x] PDA account closing for mappings (`delete mapping[key]`)
- [ ] Modifier function generation (not inlining)
- [ ] Account constraints improvements

### v0.4.0 - Extended Ecosystem
- [ ] Token 2022 support
- [ ] Compute budget control
- [ ] Additional sysvars
- [ ] Metaplex integration

### v0.5.0 - Advanced Features
- [ ] Versioned transactions
- [ ] Address lookup tables
- [ ] State compression
- [ ] Cross-chain messaging (Wormhole)

---

## Contributing

We welcome contributions! Priority areas:

1. **Parser improvements** - Grammar extensions for in-contract types
2. **Codegen** - SOL transfer and Token 2022 CPI generation
3. **Testing** - Integration tests for all features
4. **Documentation** - Examples and tutorials

See [CONTRIBUTING.md](https://github.com/cryptuon/solscript/blob/main/CONTRIBUTING.md) for guidelines.

---

## Feedback

Found a bug or have a feature request?

- [GitHub Issues](https://github.com/cryptuon/solscript/issues) - Bug reports and features
- [Discussions](https://github.com/cryptuon/solscript/discussions) - Questions and ideas
