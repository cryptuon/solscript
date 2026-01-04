# SolScript Implementation Roadmap

## Overview

This roadmap outlines the implementation plan for SolScript, a high-level language for Solana smart contract development.

### Key Decisions

| Decision | Choice |
|----------|--------|
| Implementation Language | **Rust** |
| Parser | **pest** (PEG grammar) |
| Compilation Strategy | **Hybrid** (Rust codegen → direct BPF later) |
| First Target | **Full Example** contract |
| Package Registry | **GitHub** (self-hosted) |
| Governance | **Open** |

---

## Phase 0: Specification

**Status: COMPLETE**

The language specification is documented in `specs.md` with 12 sections covering all language features. See `inconsistencies.md` for resolved design decisions.

---

## Phase 1: Core Compiler

### Milestone 1.1: Project Setup & Lexer

**Goal:** Establish project structure and tokenize SolScript source code.

#### Deliverables

```
solscript/
├── Cargo.toml
├── crates/
│   ├── solscript-lexer/        # Tokenization
│   ├── solscript-parser/       # AST generation
│   ├── solscript-ast/          # AST types
│   ├── solscript-typeck/       # Type checking
│   ├── solscript-codegen/      # Rust code generation
│   └── solscript-cli/          # CLI tool
├── grammar/
│   └── solscript.pest          # PEG grammar
└── examples/
    └── *.ss                    # Example contracts
```

#### Tasks

- [ ] Initialize Cargo workspace with crate structure
- [ ] Define pest grammar for tokens:
  - [ ] Keywords (`contract`, `fn`, `let`, `struct`, `trait`, `impl`, etc.)
  - [ ] Operators (`+`, `-`, `*`, `/`, `==`, `!=`, `&&`, `||`, etc.)
  - [ ] Delimiters (`{`, `}`, `(`, `)`, `[`, `]`, `;`, `:`, etc.)
  - [ ] Literals (integers, strings, booleans, addresses)
  - [ ] Identifiers
  - [ ] Comments (single-line `//`, multi-line `/* */`, doc `///`)
  - [ ] Decorators (`@state`, `@public`, `@view`, etc.)
  - [ ] Attributes (`#[derive]`, `#[test]`, etc.)
- [ ] Implement token span tracking for error reporting
- [ ] Write lexer tests for all token types

#### Success Criteria

- Can tokenize all examples in `specs.md`
- Tokens include source location (line, column, span)
- Comprehensive test coverage

---

### Milestone 1.2: Parser & AST

**Goal:** Parse tokens into an Abstract Syntax Tree.

#### AST Node Types

```rust
// Core AST nodes to implement
pub enum Item {
    Contract(ContractDef),
    Struct(StructDef),
    Enum(EnumDef),
    Trait(TraitDef),
    Impl(ImplBlock),
    Function(FnDef),
    Event(EventDef),
    Error(ErrorDef),
    Import(ImportStmt),
    Module(ModuleDef),
}

pub enum Stmt {
    Let(LetStmt),
    Return(ReturnStmt),
    If(IfStmt),
    Match(MatchStmt),
    While(WhileStmt),
    For(ForStmt),
    Expr(ExprStmt),
    Emit(EmitStmt),
}

pub enum Expr {
    Literal(Literal),
    Ident(Ident),
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Call(CallExpr),
    MethodCall(MethodCallExpr),
    FieldAccess(FieldAccessExpr),
    Index(IndexExpr),
    Struct(StructExpr),
    Array(ArrayExpr),
    Closure(ClosureExpr),
    Await(AwaitExpr),
    Try(TryExpr),
    Match(MatchExpr),
    Block(BlockExpr),
}
```

#### Tasks

- [ ] Define complete pest grammar for SolScript
  - [ ] Top-level items (contract, struct, trait, impl, etc.)
  - [ ] Statements
  - [ ] Expressions with precedence
  - [ ] Type annotations
  - [ ] Generics and constraints
  - [ ] Patterns for match/let
- [ ] Define AST types in `solscript-ast` crate
- [ ] Implement parser that builds AST from pest pairs
- [ ] Implement pretty-printer for AST (debugging)
- [ ] Error recovery for better error messages
- [ ] Write parser tests for all language constructs

#### Success Criteria

- Parses all examples from `specs.md` into valid AST
- Error messages include source location and context
- AST can be pretty-printed back to valid SolScript

---

### Milestone 1.3: Symbol Table & Name Resolution

**Goal:** Resolve all names and build symbol tables.

#### Tasks

- [ ] Define symbol table structure
  - [ ] Scopes (global, contract, function, block)
  - [ ] Symbol types (type, function, variable, field)
  - [ ] Visibility tracking
- [ ] Implement name resolution pass
  - [ ] Resolve type references
  - [ ] Resolve function calls
  - [ ] Resolve variable references
  - [ ] Handle imports
- [ ] Detect undefined/duplicate symbol errors
- [ ] Resolve trait implementations to types
- [ ] Handle generic type parameters

#### Success Criteria

- All names resolved to their definitions
- Clear errors for undefined/ambiguous names
- Import resolution working

---

### Milestone 1.4: Type System & Type Checking

**Goal:** Implement full type checking.

#### Type System Components

```rust
pub enum Type {
    // Primitives
    U8, U16, U32, U64, U128,
    I8, I16, I32, I64, I128,
    Bool, String, Address,

    // Compound
    Array(Box<Type>, usize),      // [T; N]
    Vec(Box<Type>),               // Vec<T>
    Map(Box<Type>, Box<Type>),    // Map<K, V>
    Option(Box<Type>),            // Option<T>
    Result(Box<Type>, Box<Type>), // Result<T, E>
    Tuple(Vec<Type>),             // (T, U, ...)

    // User-defined
    Struct(StructId),
    Enum(EnumId),
    Contract(ContractId),
    Trait(TraitId),

    // Generics
    Generic(GenericParam),
    Applied(Box<Type>, Vec<Type>), // T<A, B>

    // Special
    Unit,       // ()
    Never,      // !
    Infer,      // _ (to be inferred)
    Error,      // Type error placeholder
}
```

#### Tasks

- [ ] Implement type representation
- [ ] Implement type inference engine
  - [ ] Hindley-Milner style inference
  - [ ] Constraint generation
  - [ ] Constraint solving
- [ ] Type check all expressions
- [ ] Type check all statements
- [ ] Validate function signatures
- [ ] Check trait bounds
- [ ] Validate generic instantiations
- [ ] Check `Result` and `?` operator usage
- [ ] Implement type coercion rules
- [ ] Validate decorator usage (`@state`, `@public`, etc.)

#### Success Criteria

- Catches all type errors
- Generic functions/types work correctly
- Trait bounds enforced
- Good error messages for type mismatches

---

### Milestone 1.5: Semantic Analysis

**Goal:** Validate program semantics beyond type checking.

#### Tasks

- [ ] Validate contract structure
  - [ ] Single `init` function per contract
  - [ ] State variables have correct decorators
  - [ ] Public functions are valid entry points
- [ ] Control flow analysis
  - [ ] All paths return a value
  - [ ] No unreachable code warnings
  - [ ] Break/continue in valid contexts
- [ ] Mutability checking
  - [ ] `@view` functions don't mutate state
  - [ ] Immutable variable reassignment errors
- [ ] Ownership/borrowing (simplified)
  - [ ] No use after move (for non-Copy types)
- [ ] Security checks
  - [ ] Overflow protection annotations
  - [ ] Signer verification requirements
- [ ] Async validation
  - [ ] `await` only in `async` functions
  - [ ] Proper CPI context

#### Success Criteria

- Catches semantic errors before codegen
- Security issues flagged
- Contract structure validated

---

## Phase 2: Code Generation

### Milestone 2.1: Rust Code Generation (Basic)

**Goal:** Generate valid Rust/Anchor code from SolScript.

#### Output Structure

```rust
// Generated from: counter.ss

use anchor_lang::prelude::*;

declare_id!("...");

#[program]
pub mod counter {
    use super::*;

    pub fn init(ctx: Context<Init>) -> Result<()> {
        // Generated from SolScript init function
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        // Generated from SolScript public function
        ctx.accounts.counter.count += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Init<'info> {
    // Generated account constraints
}

#[account]
pub struct Counter {
    pub count: u64,
}
```

#### Tasks

- [ ] Generate Rust module structure
- [ ] Generate struct definitions
- [ ] Generate enum definitions
- [ ] Generate function bodies
- [ ] Generate Anchor account structs
- [ ] Generate Anchor `#[program]` module
- [ ] Map SolScript types to Rust types
- [ ] Generate error types
- [ ] Generate event emission
- [ ] Handle `self` → account access translation

#### Success Criteria

- Generated Rust compiles with `cargo build-sbf`
- Output is readable and debuggable
- Preserves SolScript semantics

---

### Milestone 2.2: Standard Library Stubs

**Goal:** Provide SolScript standard library that maps to Anchor/Solana.

#### Tasks

- [ ] `@solana/token` → SPL Token CPI wrappers
- [ ] `@solana/account` → Account creation helpers
- [ ] `@solana/pda` → PDA derivation utilities
- [ ] `@solana/cpi` → Generic CPI helpers
- [ ] `@solana/clock` → Clock sysvar access
- [ ] `@solana/rent` → Rent calculations
- [ ] `@solana/crypto` → Signature verification

#### Success Criteria

- Standard library imports resolve correctly
- Generated code uses appropriate Solana/Anchor APIs

---

### Milestone 2.3: Full Example Contract

**Goal:** Compile a complete contract using most language features.

#### Target Contract Features

```solscript
// examples/token.ss - Target full example

import { Token } from "@solana/token";
import { PDA } from "@solana/pda";

error InsufficientBalance(available: u64, required: u64);
error Unauthorized;

event Transfer(from: Address, to: Address, amount: u64);
event Mint(to: Address, amount: u64);

#[derive(Clone, Serialize, Deserialize)]
struct TokenMetadata {
  name: string;
  symbol: string;
  decimals: u8;
}

contract TokenContract {
  @state totalSupply: u64;
  @state balances: Map<Address, u64>;
  @state metadata: TokenMetadata;
  @state owner: Address;

  fn init(name: string, symbol: string, decimals: u8, initialSupply: u64) {
    self.metadata = TokenMetadata { name, symbol, decimals };
    self.totalSupply = initialSupply;
    self.balances.set(tx.sender, initialSupply);
    self.owner = tx.sender;
  }

  @public
  fn transfer(to: Address, amount: u64): Result<(), Error> {
    let senderBalance = self.balances.get(tx.sender).unwrap_or(0);

    if senderBalance < amount {
      return Err(InsufficientBalance(senderBalance, amount));
    }

    self.balances.set(tx.sender, senderBalance - amount);
    let recipientBalance = self.balances.get(to).unwrap_or(0);
    self.balances.set(to, recipientBalance + amount);

    emit Transfer(tx.sender, to, amount);
    return Ok(());
  }

  @public
  fn mint(to: Address, amount: u64): Result<(), Error> {
    if tx.sender != self.owner {
      return Err(Unauthorized);
    }

    self.totalSupply += amount;
    let balance = self.balances.get(to).unwrap_or(0);
    self.balances.set(to, balance + amount);

    emit Mint(to, amount);
    return Ok(());
  }

  @public
  @view
  fn balanceOf(account: Address): u64 {
    return self.balances.get(account).unwrap_or(0);
  }

  @public
  @view
  fn getMetadata(): TokenMetadata {
    return self.metadata.clone();
  }
}
```

#### Tasks

- [ ] Compile full token contract
- [ ] Deploy to Solana devnet
- [ ] Test all functions via CLI/client
- [ ] Verify event emission
- [ ] Verify error handling

#### Success Criteria

- Contract deploys and runs on devnet
- All functions work correctly
- Events visible in transaction logs
- Errors returned appropriately

---

## Phase 3: CLI & Developer Experience

### Milestone 3.1: CLI Tool

**Goal:** Create the `solscript` CLI.

#### Commands

```bash
solscript init <project-name>    # Create new project
solscript build                  # Compile to deployable program
solscript test                   # Run tests
solscript deploy                 # Deploy to cluster
solscript verify                 # Verify deployed program
solscript fmt                    # Format source code
solscript check                  # Type check without building
```

#### Tasks

- [ ] Implement `init` with project template
- [ ] Implement `build` pipeline
- [ ] Implement `check` for fast feedback
- [ ] Implement `fmt` code formatter
- [ ] Configuration via `solscript.toml`
- [ ] Colored error output
- [ ] Watch mode for development

---

### Milestone 3.2: Testing Framework

**Goal:** Enable testing SolScript contracts.

#### Tasks

- [ ] `#[test]` attribute support
- [ ] `#[should_fail]` attribute
- [ ] Test context/fixture setup
- [ ] Assert macros
- [ ] Test isolation (BanksClient)
- [ ] Coverage reporting

---

### Milestone 3.3: Error Messages

**Goal:** Provide excellent error messages.

#### Examples

```
error[E0001]: type mismatch
  --> src/token.ss:24:12
   |
24 |     return "hello";
   |            ^^^^^^^ expected `u64`, found `string`
   |
   = help: try converting with `.parse::<u64>()?`

error[E0002]: undefined variable
  --> src/token.ss:30:5
   |
30 |     balance = 100;
   |     ^^^^^^^ not found in this scope
   |
   = help: did you mean `self.balance`?
```

#### Tasks

- [ ] Source snippets in errors
- [ ] Colored output
- [ ] Suggestions/hints
- [ ] Error codes with documentation
- [ ] Multi-span errors

---

## Phase 4: Advanced Features

### Milestone 4.1: Direct BPF Compilation

**Goal:** Skip Rust codegen for faster compilation.

#### Tasks

- [ ] LLVM IR generation
- [ ] BPF backend integration
- [ ] Optimization passes
- [ ] Debug info generation

---

### Milestone 4.2: Language Server (LSP)

**Goal:** IDE support via LSP.

#### Features

- [ ] Go to definition
- [ ] Find references
- [ ] Hover information
- [ ] Autocomplete
- [ ] Inline errors
- [ ] Rename symbol
- [ ] Code actions

---

### Milestone 4.3: Package Manager

**Goal:** Share and reuse SolScript libraries.

#### Tasks

- [ ] Package manifest format
- [ ] GitHub registry integration
- [ ] Dependency resolution
- [ ] Version constraints
- [ ] `solscript add <package>`

---

## Phase 5: Ecosystem

### Milestone 5.1: Documentation Site

- [ ] Language guide
- [ ] API reference
- [ ] Tutorials
- [ ] Examples gallery

### Milestone 5.2: VS Code Extension

- [ ] Syntax highlighting
- [ ] LSP integration
- [ ] Snippets
- [ ] Debugger integration

### Milestone 5.3: Example Projects

- [ ] Token contract
- [ ] NFT marketplace
- [ ] DeFi AMM
- [ ] Governance DAO

---

## Implementation Order

```
Phase 1 (Core Compiler)
├── 1.1 Project Setup & Lexer
├── 1.2 Parser & AST
├── 1.3 Symbol Table
├── 1.4 Type System
└── 1.5 Semantic Analysis

Phase 2 (Code Generation)
├── 2.1 Rust Code Generation
├── 2.2 Standard Library Stubs
└── 2.3 Full Example Contract  ← FIRST TARGET

Phase 3 (Developer Experience)
├── 3.1 CLI Tool
├── 3.2 Testing Framework
└── 3.3 Error Messages

Phase 4 (Advanced)
├── 4.1 Direct BPF Compilation
├── 4.2 Language Server
└── 4.3 Package Manager

Phase 5 (Ecosystem)
├── 5.1 Documentation
├── 5.2 VS Code Extension
└── 5.3 Example Projects
```

---

## Success Criteria (Overall)

| Milestone | Criteria |
|-----------|----------|
| **MVP** | Full example contract compiles and deploys to devnet |
| **Alpha** | CLI usable, basic error messages, 3 example contracts |
| **Beta** | LSP working, package manager, comprehensive tests |
| **1.0** | Production-ready, audited, documented |

---

*Last updated: January 2, 2026*
