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

## Known Limitations

| Limitation | Workaround | Planned Fix |
|------------|------------|-------------|
| No SOL transfers (`msg.value` = 0) | Use wrapped SOL | v0.3.0 |
| No Token 2022 | Extend generated code | v0.4.0 |
| No structs/enums inside contracts | Define outside | v0.2.0 |
| No empty errors `error Foo()` | Add dummy parameter | v0.2.0 |
| Can't close mapping PDAs | Manual Anchor code | v0.3.0 |
| No compute budget control | Client-side request | v0.4.0 |

## Release Plan

### v0.2.0 - Parser Improvements
- Structs inside contracts
- Enums inside contracts
- Empty error declarations
- Better error messages

### v0.3.0 - Solana Native Features
- Direct SOL transfers
- `msg.value` support
- Mapping PDA closing
- Optimized modifier generation

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
