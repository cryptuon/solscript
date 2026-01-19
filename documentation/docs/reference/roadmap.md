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
| Custom errors | ✅ Complete | Error codes with parameters |
| Modifiers | ✅ Complete | Inlined into functions |
| View functions | ✅ Complete | Read-only account access |
| Access control | ✅ Complete | Via modifiers and require |
| CPI (interfaces) | ✅ Complete | Cross-program calls |
| SPL Token | ✅ Complete | Transfer, mint, burn |
| `msg.sender` | ✅ Complete | Signer account |
| `block.timestamp` | ✅ Complete | Clock sysvar |
| Rent handling | ✅ Complete | Auto rent-exempt accounts |
| Direct SOL transfers | ❌ Not implemented | `msg.value` returns 0 |
| Token 2022 | ❌ Not implemented | Only SPL Token |
| Structs in contracts | ❌ Parser limitation | Define outside contract |
| Enums in contracts | ❌ Parser limitation | Define outside contract |
| PDA account closing | ⚠️ Partial | State accounts only, not mappings |
| Compute budget | ❌ Not implemented | Uses defaults |

---

## Known Limitations & Planned Remediations

### 1. No Direct SOL/Lamport Transfers

**Current behavior:** `msg.value` returns 0. Payable functions include `system_program` but don't transfer lamports.

**Impact:** Cannot accept SOL payments or transfer SOL between accounts.

**Workaround:** Use wrapped SOL (SPL Token) or extend generated Anchor code manually.

**Planned remediation:**
- Add `transfer(address to, uint256 lamports)` built-in function
- Generate proper `system_program::transfer` CPI
- Support `msg.value` for payable functions with lamport amount
- Target: v0.3.0

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

### 3. Structs/Enums Inside Contracts

**Current behavior:** Parser rejects struct and enum definitions inside contract bodies.

**Impact:** Must define types outside contracts, less encapsulation.

**Workaround:** Define structs and enums before the contract:

```solidity
// Works
struct User {
    address wallet;
    uint256 balance;
}

contract MyContract {
    mapping(address => User) users;
}
```

**Planned remediation:**
- Update parser grammar to allow in-contract definitions
- Generate nested Rust types
- Target: v0.2.0

---

### 4. Empty Error Declarations

**Current behavior:** `error Foo();` without parameters fails to parse.

**Impact:** Cannot define simple marker errors.

**Workaround:** Add a dummy parameter:

```solidity
// Instead of: error Unauthorized();
error Unauthorized(string reason);

// Usage
revert Unauthorized("Not owner");
```

**Planned remediation:**
- Update parser to allow empty error parameters
- Generate unit-variant error codes
- Target: v0.2.0

---

### 5. PDA Account Closing for Mappings

**Current behavior:** `selfdestruct` closes state accounts but not mapping entries.

**Impact:** Cannot reclaim rent from mapping PDAs.

**Workaround:** Manually extend generated code with close constraints.

**Planned remediation:**
- Add `delete mappingName[key]` syntax
- Generate `close = signer` constraint for mapping entries
- Refund rent to specified account
- Target: v0.3.0

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

### v0.2.0 - Parser Improvements
- [ ] Structs inside contracts
- [ ] Enums inside contracts
- [ ] Empty error declarations
- [ ] Improved error messages

### v0.3.0 - Solana Native Features
- [ ] Direct SOL transfers (`msg.value`)
- [ ] PDA account closing for mappings
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

See [CONTRIBUTING.md](https://github.com/solscript/solscript/blob/main/CONTRIBUTING.md) for guidelines.

---

## Feedback

Found a bug or have a feature request?

- [GitHub Issues](https://github.com/solscript/solscript/issues) - Bug reports and features
- [Discussions](https://github.com/solscript/solscript/discussions) - Questions and ideas
