# Design Decisions

Reference for resolved design decisions and naming conventions.

## Naming

| Issue | Resolution |
|-------|------------|
| Project name | Standardized to **SolScript** |
| File extension | Changed from `.sol` to **`.ss`** |
| CLI command | Changed from `solanascript` to **`solscript`** |
| Installation | Changed from npm to **`cargo install solscript`** |

## Syntax

| Issue | Resolution |
|-------|------------|
| Function declarations | All use **`fn` keyword** |
| Self reference | Standardized to **`self`** (not `this`) |
| Data structures | Standardized to **`struct`** (not `class`) |
| Variable declarations | Use **`let`** (not `const`) |
| Code blocks | All use **`solscript`** language tag |

## Types

| Issue | Resolution |
|-------|------------|
| Address vs PublicKey | Unified to **`Address`** |
| Array syntax | Defined as **`[T; N]`** for fixed-size |
| Tuple syntax | Defined as **`(T, U)`** |

## Imports

| Issue | Resolution |
|-------|------------|
| Quote style | Standardized to **double quotes** |
| Token imports | Unified to **`@solana/token`** |
| SPL imports | Unified to **`@solana/spl`** |
| Low-level | Added **`@solana/low-level`** |
| Testing | Added **`@solana/testing`** |
| Anchor | Added **`@solana/anchor`** and **`@anchor/*`** |

## Error Handling

| Issue | Resolution |
|-------|------------|
| Primary pattern | **`Result<T, E>`** with `Ok()` and `Err()` |
| Error propagation | **`?` operator** |
| Pattern matching | **`match`** expression |
| Custom errors | **`error` keyword** for definitions |

## Attributes

| Issue | Resolution |
|-------|------------|
| Decorators | Use **`@`** for Solana-specific (e.g., `@state`, `@public`) |
| Attributes | Use **`#[]`** for Rust-like attributes (e.g., `#[derive]`, `#[test]`) |

## Conventions

### Decorator vs Attribute Usage

```solscript
// @ decorators: Solana/contract-specific modifiers
@state balance: u64;
@public fn transfer() { }
@view fn getBalance() { }
@payable fn deposit() { }

// #[] attributes: Rust-like compile-time attributes
#[derive(Clone, Serialize)]
#[test]
#[cfg(feature = "devnet")]
#[upgradeable]
```

### Standard Patterns

```solscript
// Function signature
@public
fn functionName(param: Type): Result<ReturnType, Error> {
  // Use self for instance access
  self.field;

  // Use let for variables
  let value = compute()?;

  // Return with Result
  return Ok(value);
}
```

