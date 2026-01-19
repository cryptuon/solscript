# SolScript Roadmap

## Current Status: Beta

SolScript is a Solidity-to-Solana compiler that generates Anchor/Rust code. It's functional for common smart contract patterns.

## What Works

| Feature | Status |
|---------|--------|
| State variables (primitives, structs, arrays) | ✅ |
| Mappings → PDA transformation | ✅ |
| Events and custom errors | ✅ |
| Access control modifiers | ✅ |
| View/mutable functions | ✅ |
| Cross-program invocation (CPI) | ✅ |
| SPL Token operations | ✅ |
| `msg.sender`, `block.timestamp` | ✅ |
| Automatic rent exemption | ✅ |
| Structs/enums inside contracts | ✅ |
| Empty error declarations | ✅ |
| Direct SOL transfers | ✅ |
| Mapping PDA closing | ✅ |

## Known Limitations

| Limitation | Workaround | Planned Fix |
|------------|------------|-------------|
| `msg.value` = 0 for incoming payments | Use wrapped SOL | - |
| No Token 2022 | Extend generated code | v0.4.0 |
| No compute budget control | Client-side request | v0.4.0 |
| Modifiers inlined | Keep modifiers small | v0.4.0 |

## Release Plan

### v0.2.0 - Parser Improvements ✅ RELEASED
- ✅ Structs inside contracts
- ✅ Enums inside contracts
- ✅ Empty error declarations
- Better error messages (planned)

### v0.3.0 - Solana Native Features ✅ RELEASED
- ✅ Direct SOL transfers (`transfer(to, amount)`)
- ✅ Mapping PDA closing (`delete mapping[key]`)
- Optimized modifier generation (planned)

### v0.4.0 - Extended Ecosystem
- Token 2022 support
- Compute budget control
- Additional sysvars
- Metaplex integration

### v0.5.0 - Advanced Features
- Versioned transactions
- Address lookup tables
- State compression

## Contributing

Priority areas for contributions:
1. Parser grammar extensions
2. SOL transfer codegen
3. Token 2022 CPI generation
4. Integration tests

See [documentation/docs/reference/roadmap.md](documentation/docs/reference/roadmap.md) for detailed information.
