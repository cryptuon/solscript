# Internal Development Documentation

This directory contains internal documentation for SolScript compiler development.

**For user-facing documentation, see `/documentation/`.**

## Contents

| Document | Purpose |
|----------|---------|
| [specs.md](specs.md) | Language specification - syntax, semantics, type system |
| [roadmap.md](roadmap.md) | Implementation status and planned features |
| [design-decisions.md](design-decisions.md) | Resolved design choices and conventions |

## Quick Reference

### Crate Structure

```
crates/
├── solscript-ast/       # AST node definitions
├── solscript-parser/    # pest grammar + parsing
├── solscript-typeck/    # Type checking + inference
├── solscript-codegen/   # Rust/Anchor code generation
├── solscript-bpf/       # Direct LLVM BPF compilation
├── solscript-lsp/       # Language server
└── solscript-cli/       # CLI tool
```

### Build Commands

```bash
# Build all crates
cargo build

# Run tests
cargo test

# Build with LLVM support (requires LLVM 18)
cargo build -p solscript-bpf --features llvm

# Run the CLI
cargo run -p solscript-cli -- check examples/counter/counter.sol
```

### Adding New Features

1. Update grammar in `crates/solscript-parser/src/grammar.pest`
2. Add AST nodes in `crates/solscript-ast/src/`
3. Update parser in `crates/solscript-parser/src/lib.rs`
4. Add type checking in `crates/solscript-typeck/src/`
5. Add codegen in `crates/solscript-codegen/src/`
6. Update `specs.md` with the new syntax/semantics
