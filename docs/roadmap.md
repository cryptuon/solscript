# SolScript Implementation Roadmap

## Overview

This roadmap outlines the implementation plan for SolScript, a high-level language for Solana smart contract development.

### Key Decisions

| Decision | Choice |
|----------|--------|
| Implementation Language | **Rust** |
| Parser | **pest** (PEG grammar) |
| Compilation Strategy | **Hybrid** (Rust/Anchor codegen + Direct LLVM BPF) |
| First Target | **Full Example** contract |
| Package Registry | **GitHub** (self-hosted) |
| Governance | **Open** |

### Current Status (January 2026)

| Phase | Status |
|-------|--------|
| Phase 0: Specification | **COMPLETE** |
| Phase 1: Core Compiler | **COMPLETE** |
| Phase 2: Code Generation | **COMPLETE** |
| Phase 3: CLI & Developer Experience | **COMPLETE** |
| Phase 4.1: Direct BPF Compilation | **COMPLETE** |
| Phase 4.2: Language Server (LSP) | **COMPLETE** (basic) |
| Phase 4.3: Package Manager | Planned |
| Phase 5: Ecosystem | **IN PROGRESS** |

---

## Phase 0: Specification

**Status: COMPLETE**

The language specification is documented in `specs.md` with 12 sections covering all language features. See `inconsistencies.md` for resolved design decisions.

---

## Phase 1: Core Compiler

**Status: COMPLETE**

### Milestone 1.1: Project Setup & Lexer

**Status: COMPLETE**

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
│   ├── solscript-bpf/          # Direct LLVM BPF compilation
│   ├── solscript-lsp/          # Language Server Protocol
│   └── solscript-cli/          # CLI tool
├── grammar/
│   └── solscript.pest          # PEG grammar
└── examples/
    └── *.sol                   # Example contracts
```

#### Tasks

- [x] Initialize Cargo workspace with crate structure
- [x] Define pest grammar for tokens:
  - [x] Keywords (`contract`, `fn`, `let`, `struct`, `trait`, `impl`, etc.)
  - [x] Operators (`+`, `-`, `*`, `/`, `==`, `!=`, `&&`, `||`, etc.)
  - [x] Delimiters (`{`, `}`, `(`, `)`, `[`, `]`, `;`, `:`, etc.)
  - [x] Literals (integers, strings, booleans, addresses)
  - [x] Identifiers
  - [x] Comments (single-line `//`, multi-line `/* */`, doc `///`)
  - [x] Decorators (`@state`, `@public`, `@view`, etc.)
  - [x] Attributes (`#[derive]`, `#[test]`, etc.)
- [x] Implement token span tracking for error reporting
- [x] Write lexer tests for all token types

#### Success Criteria

- [x] Can tokenize all examples in `specs.md`
- [x] Tokens include source location (line, column, span)
- [x] Comprehensive test coverage

---

### Milestone 1.2: Parser & AST

**Status: COMPLETE**

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

- [x] Define complete pest grammar for SolScript
  - [x] Top-level items (contract, struct, trait, impl, etc.)
  - [x] Statements
  - [x] Expressions with precedence
  - [x] Type annotations
  - [x] Generics and constraints
  - [x] Patterns for match/let
- [x] Define AST types in `solscript-ast` crate
- [x] Implement parser that builds AST from pest pairs
- [x] Implement pretty-printer for AST (debugging)
- [x] Error recovery for better error messages
- [x] Write parser tests for all language constructs

#### Success Criteria

- [x] Parses all examples from `specs.md` into valid AST
- [x] Error messages include source location and context
- [x] AST can be pretty-printed back to valid SolScript

---

### Milestone 1.3: Symbol Table & Name Resolution

**Status: COMPLETE**

**Goal:** Resolve all names and build symbol tables.

#### Tasks

- [x] Define symbol table structure
  - [x] Scopes (global, contract, function, block)
  - [x] Symbol types (type, function, variable, field)
  - [x] Visibility tracking
- [x] Implement name resolution pass
  - [x] Resolve type references
  - [x] Resolve function calls
  - [x] Resolve variable references
  - [x] Handle imports
- [x] Detect undefined/duplicate symbol errors
- [x] Resolve trait implementations to types
- [x] Handle generic type parameters

#### Success Criteria

- [x] All names resolved to their definitions
- [x] Clear errors for undefined/ambiguous names
- [x] Import resolution working

---

### Milestone 1.4: Type System & Type Checking

**Status: COMPLETE**

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

- [x] Implement type representation
- [x] Implement type inference engine
  - [x] Hindley-Milner style inference
  - [x] Constraint generation
  - [x] Constraint solving
- [x] Type check all expressions
- [x] Type check all statements
- [x] Validate function signatures
- [x] Check trait bounds
- [x] Validate generic instantiations
- [x] Check `Result` and `?` operator usage
- [x] Implement type coercion rules
- [x] Validate decorator usage (`@state`, `@public`, etc.)

#### Success Criteria

- [x] Catches all type errors
- [x] Generic functions/types work correctly
- [x] Trait bounds enforced
- [x] Good error messages for type mismatches

---

### Milestone 1.5: Semantic Analysis

**Status: COMPLETE**

**Goal:** Validate program semantics beyond type checking.

#### Tasks

- [x] Validate contract structure
  - [x] Single `init` function per contract
  - [x] State variables have correct decorators
  - [x] Public functions are valid entry points
- [x] Control flow analysis
  - [x] All paths return a value
  - [x] No unreachable code warnings
  - [x] Break/continue in valid contexts
- [x] Mutability checking
  - [x] `@view` functions don't mutate state
  - [x] Immutable variable reassignment errors
- [x] Ownership/borrowing (simplified)
  - [x] No use after move (for non-Copy types)
- [x] Security checks
  - [x] Overflow protection annotations
  - [x] Signer verification requirements
- [x] Async validation
  - [x] `await` only in `async` functions
  - [x] Proper CPI context

#### Success Criteria

- [x] Catches semantic errors before codegen
- [x] Security issues flagged
- [x] Contract structure validated

---

## Phase 2: Code Generation

**Status: COMPLETE**

### Milestone 2.1: Rust Code Generation (Basic)

**Status: COMPLETE**

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

- [x] Generate Rust module structure
- [x] Generate struct definitions
- [x] Generate enum definitions
- [x] Generate function bodies
- [x] Generate Anchor account structs
- [x] Generate Anchor `#[program]` module
- [x] Map SolScript types to Rust types
- [x] Generate error types
- [x] Generate event emission
- [x] Handle `self` → account access translation

#### Success Criteria

- [x] Generated Rust compiles with `cargo build-sbf`
- [x] Output is readable and debuggable
- [x] Preserves SolScript semantics

---

### Milestone 2.2: Standard Library Stubs

**Status: COMPLETE**

**Goal:** Provide SolScript standard library that maps to Anchor/Solana.

#### Tasks

- [x] `@solana/token` → SPL Token CPI wrappers
- [x] `@solana/account` → Account creation helpers
- [x] `@solana/pda` → PDA derivation utilities
- [x] `@solana/cpi` → Generic CPI helpers
- [x] `@solana/clock` → Clock sysvar access
- [x] `@solana/rent` → Rent calculations
- [x] `@solana/crypto` → Signature verification

#### Success Criteria

- [x] Standard library imports resolve correctly
- [x] Generated code uses appropriate Solana/Anchor APIs

---

### Milestone 2.3: Full Example Contract

**Status: COMPLETE**

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

- [x] Compile full token contract
- [x] Deploy to Solana devnet
- [x] Test all functions via CLI/client
- [x] Verify event emission
- [x] Verify error handling

#### Success Criteria

- [x] Contract deploys and runs on devnet
- [x] All functions work correctly
- [x] Events visible in transaction logs
- [x] Errors returned appropriately

---

## Phase 3: CLI & Developer Experience

**Status: COMPLETE**

### Milestone 3.1: CLI Tool

**Status: COMPLETE**

**Goal:** Create the `solscript` CLI.

#### Commands

```bash
solscript init <project-name>    # Create new project
solscript build                  # Compile to Rust/Anchor
solscript build-bpf              # Compile to BPF bytecode
solscript build-bpf --llvm       # Direct LLVM compilation (fast)
solscript test                   # Run tests
solscript deploy                 # Deploy to cluster
solscript verify                 # Verify deployed program
solscript fmt                    # Format source code
solscript check                  # Type check without building
solscript lsp                    # Start Language Server
```

#### Tasks

- [x] Implement `init` with project template
- [x] Implement `build` pipeline
- [x] Implement `build-bpf` for BPF compilation
- [x] Implement `check` for fast feedback
- [x] Implement `fmt` code formatter
- [x] Configuration via `solscript.toml`
- [x] Colored error output
- [x] Watch mode for development

---

### Milestone 3.2: Testing Framework

**Status: COMPLETE**

**Goal:** Enable testing SolScript contracts.

#### Tasks

- [x] `#[test]` attribute support
- [x] `#[should_fail]` attribute
- [x] Test context/fixture setup
- [x] Assert macros
- [x] Test isolation (BanksClient)
- [ ] Coverage reporting (planned)

---

### Milestone 3.3: Error Messages

**Status: COMPLETE**

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

- [x] Source snippets in errors
- [x] Colored output
- [x] Suggestions/hints
- [x] Error codes with documentation
- [x] Multi-span errors

---

## Phase 4: Advanced Features

### Milestone 4.1: Direct BPF Compilation

**Status: COMPLETE**

**Goal:** Skip Rust codegen for faster compilation via direct LLVM-to-BPF compilation.

#### Implementation Details

The direct BPF compilation path uses LLVM 18 with inkwell bindings to generate Solana BPF bytecode directly from the AST, bypassing the Rust/Anchor intermediate step.

**Key Components:**
- `solscript-bpf` crate with LLVM feature flag
- Type mapping from SolScript types to LLVM types
- Solana syscall intrinsics (sol_log, sol_invoke, PDA derivation, etc.)
- Instruction dispatch with Anchor-compatible discriminators
- BPF-specific function attributes (nounwind, norecurse)

**Usage:**
```bash
# Requires LLVM 18 with BPF target
export LLVM_SYS_180_PREFIX=/usr/lib/llvm-18
cargo build -p solscript-bpf --features llvm

# Compile a contract
solscript build-bpf --llvm my_contract.sol -o target/deploy
```

#### Tasks

- [x] LLVM IR generation
- [x] BPF backend integration
- [x] Optimization passes (O0-O3)
- [x] Solana syscall intrinsics
- [x] Entrypoint with instruction dispatch
- [x] Anchor-compatible discriminators
- [ ] Debug info generation (planned)

---

### Milestone 4.2: Language Server (LSP)

**Status: COMPLETE (Basic)**

**Goal:** IDE support via LSP.

#### Features

- [x] Go to definition
- [x] Find references
- [x] Hover information
- [x] Autocomplete
- [x] Inline errors
- [ ] Rename symbol (planned)
- [ ] Code actions (planned)

---

### Milestone 4.3: Package Manager

**Status: Planned**

**Goal:** Share and reuse SolScript libraries.

#### Tasks

- [ ] Package manifest format
- [ ] GitHub registry integration
- [ ] Dependency resolution
- [ ] Version constraints
- [ ] `solscript add <package>`

---

## Phase 5: Ecosystem

**Status: IN PROGRESS**

### Milestone 5.1: Documentation Site

**Status: COMPLETE (Basic)**

- [x] Language guide
- [x] API reference
- [x] Tutorials
- [x] Examples gallery

Documentation available at `documentation/docs/` using mdBook.

### Milestone 5.2: VS Code Extension

**Status: COMPLETE (Basic)**

- [x] Syntax highlighting
- [x] LSP integration
- [x] Snippets
- [ ] Debugger integration (planned)

Extension available in `editors/vscode/`.

### Milestone 5.3: Example Projects

**Status: COMPLETE**

- [x] Counter contract (`examples/counter/`)
- [x] Token contract (`examples/token/`)
- [x] Simple contract (`examples/simple/`)
- [ ] NFT marketplace (planned)
- [ ] DeFi AMM (planned)
- [ ] Governance DAO (planned)

---

## Implementation Order

```
Phase 1 (Core Compiler)               [COMPLETE]
├── 1.1 Project Setup & Lexer         [COMPLETE]
├── 1.2 Parser & AST                  [COMPLETE]
├── 1.3 Symbol Table                  [COMPLETE]
├── 1.4 Type System                   [COMPLETE]
└── 1.5 Semantic Analysis             [COMPLETE]

Phase 2 (Code Generation)             [COMPLETE]
├── 2.1 Rust Code Generation          [COMPLETE]
├── 2.2 Standard Library Stubs        [COMPLETE]
└── 2.3 Full Example Contract         [COMPLETE]

Phase 3 (Developer Experience)        [COMPLETE]
├── 3.1 CLI Tool                      [COMPLETE]
├── 3.2 Testing Framework             [COMPLETE]
└── 3.3 Error Messages                [COMPLETE]

Phase 4 (Advanced)
├── 4.1 Direct BPF Compilation        [COMPLETE]
├── 4.2 Language Server               [COMPLETE - Basic]
└── 4.3 Package Manager               [PLANNED]

Phase 5 (Ecosystem)                   [IN PROGRESS]
├── 5.1 Documentation                 [COMPLETE - Basic]
├── 5.2 VS Code Extension             [COMPLETE - Basic]
└── 5.3 Example Projects              [IN PROGRESS]
```

---

## Success Criteria (Overall)

| Milestone | Criteria | Status |
|-----------|----------|--------|
| **MVP** | Full example contract compiles and deploys to devnet | **ACHIEVED** |
| **Alpha** | CLI usable, basic error messages, 3 example contracts | **ACHIEVED** |
| **Beta** | LSP working, package manager, comprehensive tests | **PARTIAL** (LSP done, package manager planned) |
| **1.0** | Production-ready, audited, documented | In Progress |

---

## Compilation Modes

SolScript supports two compilation modes:

### 1. Anchor Mode (Default)
Generates Rust/Anchor code, then uses `cargo build-sbf`:
```bash
solscript build-bpf my_contract.sol
```

**Pros:** Well-tested, full Anchor ecosystem support
**Cons:** Slower compilation, requires Rust toolchain

### 2. Direct LLVM Mode
Compiles directly to BPF bytecode via LLVM:
```bash
solscript build-bpf --llvm my_contract.sol
```

**Pros:** Faster compilation, smaller output
**Cons:** Requires LLVM 18, fewer optimizations

---

## Architecture

```
                    ┌─────────────────┐
                    │   Source Code   │
                    │   (.sol file)   │
                    └────────┬────────┘
                             │
                    ┌────────▼────────┐
                    │     Parser      │
                    │  (pest grammar) │
                    └────────┬────────┘
                             │
                    ┌────────▼────────┐
                    │       AST       │
                    │ (solscript-ast) │
                    └────────┬────────┘
                             │
                    ┌────────▼────────┐
                    │   Type Check    │
                    │(solscript-typeck│
                    └────────┬────────┘
                             │
              ┌──────────────┴──────────────┐
              │                             │
     ┌────────▼────────┐          ┌────────▼────────┐
     │  Anchor Codegen │          │  LLVM Codegen   │
     │(solscript-codegen)         │ (solscript-bpf) │
     └────────┬────────┘          └────────┬────────┘
              │                             │
     ┌────────▼────────┐          ┌────────▼────────┐
     │   Rust/Anchor   │          │    LLVM IR      │
     │     Source      │          │                 │
     └────────┬────────┘          └────────┬────────┘
              │                             │
     ┌────────▼────────┐          ┌────────▼────────┐
     │ cargo build-sbf │          │   BPF Target    │
     └────────┬────────┘          └────────┬────────┘
              │                             │
              └──────────────┬──────────────┘
                             │
                    ┌────────▼────────┐
                    │   .so Program   │
                    │  (deployable)   │
                    └─────────────────┘
```

---

*Last updated: January 18, 2026*
