# SolScript Language Specification

## 1. Features

### 1.1 Contract Structure

Contracts are defined using the `contract` keyword:

```solscript
contract TokenContract {
  // Contract code here
}
```

### 1.2 State Variables

State variables are declared using the `@state` decorator:

```solscript
contract TokenContract {
  @state totalSupply: u64;
  @state balances: Map<Address, u64>;
  @state allowances: Map<Address, Map<Address, u64>>;
}
```

### 1.3 Functions

Functions are defined within the contract using the `fn` keyword:

```solscript
contract TokenContract {
  @public
  fn transfer(to: Address, amount: u64): bool {
    // Function body
  }

  @view
  fn balanceOf(owner: Address): u64 {
    // Function body
  }
}
```

Function modifiers:
- `@public`: Can be called externally
- `@private`: Only callable within the contract (default if no modifier)
- `@view`: Read-only function (doesn't modify state)
- `@payable`: Can receive SOL

### 1.4 Constructor

The constructor is defined using the `init` function:

```solscript
contract TokenContract {
  fn init(initialSupply: u64) {
    // Initialization code
  }
}
```

### 1.5 Events

Events are defined using the `event` keyword and emitted using `emit`:

```solscript
event Transfer(from: Address, to: Address, amount: u64);

contract TokenContract {
  @public
  fn transfer(to: Address, amount: u64): bool {
    // Transfer logic
    emit Transfer(tx.sender, to, amount);
    return true;
  }
}
```

### 1.6 Error Handling

SolScript uses `Result<T, E>` for error handling with custom error types:

```solscript
error InsufficientBalance(available: u64, required: u64);
error Unauthorized;

contract TokenContract {
  @public
  fn transfer(to: Address, amount: u64): Result<bool, Error> {
    let balance = self.balances.get(tx.sender);
    if balance < amount {
      return Err(InsufficientBalance(balance, amount));
    }
    // Transfer logic
    return Ok(true);
  }
}
```

The `?` operator can be used for error propagation:

```solscript
@public
fn transferAll(to: Address): Result<u64, Error> {
  let balance = self.getBalance(tx.sender)?;  // Propagates error if Result is Err
  self.transfer(to, balance)?;
  return Ok(balance);
}
```

### 1.7 Built-in Security Features

- Automatic checks for integer overflow/underflow
- Reentrancy protection
- Checks-Effects-Interactions pattern enforced by default

### 1.8 Interacting with Other Contracts

```solscript
interface OtherContract {
  fn doSomething(): Result<(), Error>;
}

contract MyContract {
  @public
  fn interactWithOther(otherContractAddress: Address): Result<(), Error> {
    let other = OtherContract.at(otherContractAddress);
    other.doSomething()?;
    return Ok(());
  }
}
```

## 2. Escape Hatches

### 2.1 Inline Solana Instructions

For advanced use cases, allow inline Solana instructions:

```solscript
contract AdvancedContract {
  @public
  fn complexOperation() {
    @solana_inline {
      // Raw Solana instructions here
    }
  }
}
```

### 2.2 Custom Serialization

Allow custom serialization for complex data structures via trait implementation:

```solscript
struct ComplexStruct {
  data: Vec<u8>;
  flags: u32;
}

impl Serialize for ComplexStruct {
  fn serialize(self): Buffer {
    // Custom serialization logic
    let buf = Buffer.new();
    buf.writeU32(self.flags);
    buf.writeBytes(self.data);
    return buf;
  }
}

impl Deserialize for ComplexStruct {
  fn deserialize(data: Buffer): Result<Self, Error> {
    // Custom deserialization logic
    let flags = data.readU32()?;
    let bytes = data.readRemaining();
    return Ok(ComplexStruct { data: bytes, flags });
  }
}
```

### 2.3 Direct Memory Access

Provide a safe API for direct memory access when needed:

```solscript
contract LowLevelContract {
  @public
  fn rawOperation() {
    let data = Memory.read(0x1000, 32);
    Memory.write(0x2000, data);
  }
}
```

## 3. Standard Library

### 3.1 Token Operations

```solscript
import { Token } from "@solana/token";

contract TokenWrapper {
  @public
  fn transfer(token: Address, to: Address, amount: u64): Result<(), Error> {
    Token.transfer(token, to, amount)?;
    return Ok(());
  }
}
```

### 3.2 Account Management

```solscript
import { Account } from "@solana/account";

contract AccountManager {
  @public
  fn createAccount(owner: Address, space: u64): Result<Address, Error> {
    let newAccount = Account.create(owner, space)?;
    return Ok(newAccount.address);
  }
}
```

### 3.3 Program Derived Addresses (PDAs)

```solscript
import { PDA } from "@solana/pda";

contract PDAUser {
  @public
  fn usePDA(): Result<Address, Error> {
    let (pda, bump) = PDA.find(["seed"])?;
    return Ok(pda);
  }
}
```

### 3.4 Cross-Program Invocation (CPI)

```solscript
import { CPI } from "@solana/cpi";

contract CPIUser {
  @public
  fn invokeCPI(program: Address, accounts: Vec<AccountMeta>, data: Buffer): Result<(), Error> {
    CPI.invoke(program, accounts, data)?;
    return Ok(());
  }
}
```

### 3.5 Solana Program Library (SPL) Integration

```solscript
import { SPL } from "@solana/spl";

contract SPLTokenUser {
  @public
  fn createMint(decimals: u8, mintAuthority: Address): Result<Address, Error> {
    let mint = SPL.Token.createMint(decimals, mintAuthority)?;
    return Ok(mint);
  }
}
```

### 3.6 Cryptographic Operations

```solscript
import { Crypto } from "@solana/crypto";

contract CryptoUser {
  @public
  @view
  fn verifySignature(message: Buffer, signature: Buffer, signer: Address): bool {
    return Crypto.verify(message, signature, signer);
  }
}
```

### 3.7 Time and Slot Operations

```solscript
import { Clock } from "@solana/clock";

contract TimeUser {
  @public
  @view
  fn getCurrentSlot(): u64 {
    return Clock.slot;
  }
}
```

### 3.8 Rent and Space Management

```solscript
import { Rent } from "@solana/rent";

contract RentExemption {
  @public
  @view
  fn calculateRentExemption(space: u64): u64 {
    return Rent.exemption(space);
  }
}
```

### 3.9 Low-Level Operations

```solscript
import { LowLevel } from "@solana/low-level";

contract LowLevelUser {
  @public
  fn createRawAccount(payer: Address, space: u64): Result<Address, Error> {
    let account = LowLevel.createAccount({
      payer: payer,
      space: space,
      owner: self.programId
    })?;
    return Ok(account);
  }
}
```

### 3.10 Testing Framework

```solscript
import { Test, assert } from "@solana/testing";

#[test]
fn test_transfer(): Result<(), Error> {
  let ctx = Test.context();
  let token = TokenContract.deploy(ctx, 1000)?;

  token.transfer(ctx.accounts.user1, 100)?;

  assert.eq(token.balanceOf(ctx.accounts.user1), 100);
  return Ok(());
}

#[test]
#[should_fail(expected = "InsufficientBalance")]
fn test_overdraw(): Result<(), Error> {
  let ctx = Test.context();
  let token = TokenContract.deploy(ctx, 100)?;

  token.transfer(ctx.accounts.user1, 200)?;  // Should fail
  return Ok(());
}
```

This specification provides a comprehensive framework for SolScript, a high-level Solana-focused language. It covers the main features needed for contract development, provides escape hatches for advanced use cases, and includes a robust standard library to handle common Solana operations.

The language aims to simplify Solana development while maintaining the flexibility to handle complex scenarios. By providing high-level abstractions and built-in security features, it reduces the risk of common vulnerabilities while still allowing developers to leverage the full power of the Solana blockchain when needed.

## 4. Generics

SolScript supports generic programming to enable type-safe, reusable code.

### 4.1 Generic Type Parameters

Generic types are declared using angle bracket syntax:

```solscript
// Generic function
fn swap<T>(a: T, b: T): (T, T) {
  return (b, a);
}

// Generic struct
struct Pair<T, U> {
  first: T;
  second: U;
}

// Usage
let pair = Pair<u64, string> { first: 100, second: "hello" };
let (x, y) = swap<u64>(10, 20);
```

### 4.2 Generic Constraints

Type parameters can be constrained using trait bounds:

```solscript
// Single constraint
fn print_balance<T: Serialize>(account: T) {
  log(account.serialize());
}

// Multiple constraints
fn transfer<T: Serialize + Deserialize>(data: T): T {
  let bytes = data.serialize();
  return T.deserialize(bytes);
}

// Where clause for complex constraints
fn complex_operation<T, U>(a: T, b: U): T
where
  T: Serialize + Clone,
  U: Into<T>
{
  return b.into();
}
```

### 4.3 Generic Contracts

Contracts can be parameterized with generic types:

```solscript
contract Vault<T: Token> {
  @state token: T;
  @state balance: u64;

  @public
  fn deposit(amount: u64): Result<(), Error> {
    self.token.transferFrom(tx.sender, self.address, amount)?;
    self.balance += amount;
    return Ok(());
  }
}

// Instantiation
let usdcVault = Vault<USDC>.deploy()?;
```

### 4.4 Type Inference

The compiler infers generic types when unambiguous:

```solscript
let pair = Pair { first: 100, second: "hello" }; // Inferred as Pair<u64, string>
let result = swap(10, 20); // Inferred as swap<u64>

// Explicit annotation required when ambiguous
let zero: Option<u64> = Option.None; // Type annotation needed
```

### 4.5 Built-in Generic Types

```solscript
// Option - represents optional values
Option<T>.Some(value: T)
Option<T>.None

// Result - represents success or failure
Result<T, E>.Ok(value: T)
Result<T, E>.Err(error: E)

// Map - key-value mapping
Map<K, V>

// Vec - dynamic array
Vec<T>

// Fixed-size array
[T; N]
```

## 5. Traits

Traits define shared behavior that types can implement.

### 5.1 Trait Definition

```solscript
trait Serialize {
  fn serialize(self): Buffer;
  fn size(): u64;
}

trait Deserialize {
  fn deserialize(data: Buffer): Result<Self, Error>;
}

// Trait with default implementation
trait Display {
  fn display(self): string;

  fn log(self) {
    log(self.display()); // Default implementation
  }
}
```

### 5.2 Implementing Traits

```solscript
struct TokenAccount {
  balance: u64;
  owner: Address;
}

impl Serialize for TokenAccount {
  fn serialize(self): Buffer {
    return Buffer.concat([
      self.balance.toBytes(),
      self.owner.toBytes()
    ]);
  }

  fn size(): u64 {
    return 8 + 32; // u64 + Address
  }
}

impl Display for TokenAccount {
  fn display(self): string {
    return `TokenAccount(balance: ${self.balance})`;
  }
}
```

### 5.3 Trait Inheritance

```solscript
trait Account: Serialize + Deserialize {
  fn owner(self): Address;
  fn lamports(self): u64;
}

// Types implementing Account must also implement Serialize and Deserialize
impl Account for TokenAccount {
  fn owner(self): Address {
    return self.owner;
  }

  fn lamports(self): u64 {
    return Rent.minimumBalance(Self.size());
  }
}
```

### 5.4 Associated Types

```solscript
trait Iterator {
  type Item;

  fn next(self): Option<Self.Item>;
  fn hasNext(self): bool;
}

struct RangeIterator {
  current: u64;
  end: u64;
}

impl Iterator for RangeIterator {
  type Item = u64;

  fn next(self): Option<u64> {
    if self.current < self.end {
      let value = self.current;
      self.current += 1;
      return Option.Some(value);
    }
    return Option.None;
  }

  fn hasNext(self): bool {
    return self.current < self.end;
  }
}
```

### 5.5 Built-in Traits

```solscript
// Core traits
trait Clone {
  fn clone(self): Self;
}

trait Copy: Clone {} // Marker trait for copy semantics

trait Default {
  fn default(): Self;
}

trait PartialEq {
  fn eq(self, other: Self): bool;
}

trait Eq: PartialEq {} // Marker for full equality

// Solana-specific traits
trait AccountSerialize {
  fn trySerialize(self, writer: Buffer): Result<(), Error>;
}

trait AccountDeserialize {
  fn tryDeserialize(data: Buffer): Result<Self, Error>;
}

trait Owner {
  fn owner(): Address;
}
```

### 5.6 Derive Macros

Common traits can be automatically implemented:

```solscript
#[derive(Clone, Serialize, Deserialize, Default)]
struct UserProfile {
  name: string;
  score: u64;
  active: bool;
}
```

## 6. Async/Await

SolScript supports async syntax for cross-program invocations and asynchronous patterns.

### 6.1 Async Functions

```solscript
contract AsyncExample {
  @public
  async fn fetchAndProcess(tokenMint: Address): Result<u64, Error> {
    // Await CPI call
    let balance = await Token.balanceOf(tokenMint, tx.sender)?;

    // Process result
    return Ok(balance * 2);
  }
}
```

### 6.2 Async CPI

```solscript
import { Token } from "@solana/token";

contract TokenSwap {
  @public
  async fn swap(
    fromToken: Address,
    toToken: Address,
    amount: u64
  ): Result<u64, Error> {
    // Sequential CPI calls with await
    await Token.transfer(fromToken, tx.sender, self.address, amount)?;

    let rate = self.getExchangeRate(fromToken, toToken);
    let outputAmount = amount * rate;

    await Token.transfer(toToken, self.address, tx.sender, outputAmount)?;

    return Ok(outputAmount);
  }
}
```

### 6.3 Concurrent Operations

```solscript
contract MultiOperation {
  @public
  async fn batchQuery(accounts: Vec<Address>): Result<Vec<u64>, Error> {
    // Execute multiple CPIs - compiler optimizes into single transaction
    let results = await Future.all(
      accounts.map(|addr| Token.balanceOf(addr))
    )?;

    return Ok(results);
  }
}
```

### 6.4 Error Handling in Async

```solscript
contract SafeAsync {
  @public
  async fn safeTransfer(to: Address, amount: u64): Result<(), Error> {
    match await Token.transfer(to, amount) {
      Ok(_) => return Ok(()),
      Err(e: InsufficientFunds) => {
        emit TransferFailed(to, amount, "Insufficient funds");
        return Err(e);
      },
      Err(e) => return Err(e),
    }
  }
}
```

### 6.5 Execution Model

Async in SolScript is designed for Solana's execution model:

- **No true concurrency**: Solana executes instructions sequentially within a transaction
- **CPI batching**: Multiple awaits in sequence are batched when possible
- **Atomic execution**: All awaited operations in a function succeed or all fail
- **Compute budget**: Async operations consume compute units like synchronous code

```solscript
// This compiles to a single transaction with multiple instructions
async fn atomicSwap(): Result<(), Error> {
  await tokenA.transfer(user, pool, amountA)?;  // Instruction 1
  await tokenB.transfer(pool, user, amountB)?;  // Instruction 2
  // Both succeed or both fail
  return Ok(());
}
```

## 7. Macros

SolScript supports compile-time metaprogramming through macros.

### 7.1 Declarative Macros

Simple pattern-based macros for code generation:

```solscript
macro_rules! vec {
  ($($elem:expr),*) => {
    {
      let v = Vec.new();
      $(v.push($elem);)*
      v
    }
  }
}

// Usage
let numbers = vec![1, 2, 3, 4, 5];
```

### 7.2 Procedural Macros

More powerful macros that operate on the AST:

```solscript
#[derive(Serialize)]  // Procedural derive macro
struct MyStruct {
  field1: u64;
  field2: string;
}

// Custom procedural macro
#[proc_macro]
fn generateAccessors(struct_def: TokenStream): TokenStream {
  // Generate getter/setter for each field
  // ... macro implementation
}
```

### 7.3 Attribute Macros

Macros that transform items:

```solscript
// Define an attribute macro
#[proc_macro_attribute]
fn route(args: TokenStream, item: TokenStream): TokenStream {
  // Add routing metadata to function
  // ... macro implementation
}

// Usage
contract API {
  #[route("/users")]
  @public
  fn getUsers(): Vec<User> {
    // ...
  }
}
```

### 7.4 Built-in Macros

```solscript
// Debug printing (removed in release builds)
debug!("Value: {}", someValue);

// Compile-time assertions
static_assert!(size_of::<TokenAccount>() == 40);

// Include external files
const ABI: string = include_str!("./abi.json");
const BYTECODE: Buffer = include_bytes!("./program.so");

// Stringify
const NAME: string = stringify!(MyContract);

// Conditional compilation
#[cfg(feature = "devnet")]
const PROGRAM_ID: Address = Address("DevNet1111111111111111111111111111111111");
```

### 7.5 Macro Hygiene

Macros use hygienic expansion to prevent naming conflicts:

```solscript
macro_rules! increment {
  ($x:expr) => {
    {
      let temp = $x;  // 'temp' is hygienic - won't conflict
      temp + 1
    }
  }
}

let temp = 10;
let result = increment!(temp);  // Works correctly, no conflict
```

### 7.6 Macro Export/Import

```solscript
// In math_macros.ss
#[macro_export]
macro_rules! min {
  ($a:expr, $b:expr) => {
    if $a < $b { $a } else { $b }
  }
}

// In another file
use math_macros::min;

let smallest = min!(x, y);
```

## 8. Program Upgradability

SolScript provides first-class support for upgradeable programs.

### 8.1 Upgradeable Contract Declaration

```solscript
#[upgradeable]
contract MyContract {
  @state version: u8;
  @state data: Map<Address, u64>;

  // ...
}
```

### 8.2 Upgrade Authority

```solscript
#[upgradeable(authority = "AUTHORITY_PUBKEY")]
contract GovernedContract {
  @state upgradeAuthority: Address;

  @public
  #[only_authority]
  fn setUpgradeAuthority(newAuthority: Address) {
    self.upgradeAuthority = newAuthority;
  }
}

// Multi-sig upgrade authority
#[upgradeable(
  authority = multisig("KEY1", "KEY2", "KEY3"),
  threshold = 2
)]
contract MultiSigUpgradeable {
  // Requires 2 of 3 signatures to upgrade
}
```

### 8.3 State Migration

```solscript
#[upgradeable]
contract VersionedContract {
  #[version(1)]
  @state oldField: u64;

  #[version(2)]
  @state newField: u128;  // Added in v2

  #[migration(from = 1, to = 2)]
  fn migrateV1ToV2(oldState: V1State): V2State {
    return V2State {
      oldField: oldState.oldField,
      newField: oldState.oldField as u128 * 1000
    };
  }
}
```

### 8.4 Upgrade Hooks

```solscript
#[upgradeable]
contract HookedContract {
  #[before_upgrade]
  fn validateUpgrade(newProgramHash: Hash): bool {
    // Custom validation before upgrade
    return self.upgradeAuthority == tx.sender;
  }

  #[after_upgrade]
  fn onUpgraded(previousVersion: u8) {
    emit Upgraded(previousVersion, self.version);
    self.runMigrations(previousVersion);
  }
}
```

### 8.5 Data Layout Versioning

```solscript
// Explicit layout control for safe upgrades
#[upgradeable]
#[layout(version = 3)]
contract StableLayout {
  #[offset(0)]
  @state field1: u64;      // Fixed at byte 0

  #[offset(8)]
  @state field2: Address;  // Fixed at byte 8

  #[offset(40)]
  @state field3: u128;     // Fixed at byte 40

  // New fields must use new offsets
  #[offset(56)]
  #[version(2)]
  @state newField: u64;
}
```

### 8.6 Immutable Contracts

```solscript
// Explicitly non-upgradeable
#[immutable]
contract ImmutableContract {
  // Cannot be upgraded after deployment
  // Provides security guarantees to users
}

// Freeze capability - upgradeable until frozen
#[freezable]
contract FreezableContract {
  @state frozen: bool;

  @public
  #[only_authority]
  fn freeze() {
    self.frozen = true;
    // No more upgrades possible
  }
}
```

## 9. Anchor/Rust Interoperability

SolScript provides seamless interoperability with Anchor and native Rust programs.

### 9.1 Calling Anchor Programs from SolScript

```solscript
// Import Anchor program IDL
import { MarinadeFinance } from "@anchor/marinade-finance";

contract StakingWrapper {
  @public
  async fn stakeWithMarinade(amount: u64): Result<(), Error> {
    // Type-safe call to Anchor program
    await MarinadeFinance.deposit({
      state: MARINADE_STATE,
      msolMint: MSOL_MINT,
      liqPoolMsolLeg: MSOL_LEG,
      liqPoolSolLegPda: SOL_LEG_PDA,
      transferFrom: tx.sender,
      mintTo: self.msolAccount,
      amount: amount
    })?;
    return Ok(());
  }
}
```

### 9.2 Exposing SolScript to Anchor/Rust

```solscript
// Generate Anchor-compatible IDL
#[anchor_compatible]
contract MyProgram {
  @state counter: u64;

  @public
  fn increment() {
    self.counter += 1;
  }
}

// Generates IDL that Anchor clients can consume
// Also generates Rust types for native integration
```

### 9.3 Account Compatibility

```solscript
// SolScript account that matches Anchor layout
#[account]
#[anchor_layout]
struct CompatibleAccount {
  discriminator: [u8; 8];  // Anchor discriminator
  authority: Address;
  data: u64;
}

// Reading Anchor accounts
import { AnchorAccount } from "@solana/anchor";

fn readAnchorData(accountAddress: Address): Result<u64, Error> {
  let account = AnchorAccount::<MarinadeState>::load(accountAddress)?;
  return Ok(account.data.totalStaked);
}
```

### 9.4 FFI for Native Rust

```solscript
// Declare external Rust function
#[extern("my_rust_lib")]
fn compute_heavy_operation(input: Buffer): Buffer;

// Inline Rust code
#[rust_inline]
fn optimized_hash(data: Buffer): Hash {
  r#"
  use solana_program::hash::hash;
  hash(&data)
  "#
}

contract HybridContract {
  @public
  fn process(data: Buffer): Hash {
    // Call Rust implementation for performance
    return optimized_hash(data);
  }
}
```

### 9.5 Type Mapping

| SolScript | Rust | Anchor |
|-----------|------|--------|
| `u8`...`u128` | `u8`...`u128` | `u8`...`u128` |
| `i8`...`i128` | `i8`...`i128` | `i8`...`i128` |
| `bool` | `bool` | `bool` |
| `string` | `String` | `String` |
| `Address` | `Pubkey` | `Pubkey` |
| `Buffer` | `Vec<u8>` | `Vec<u8>` |
| `Vec<T>` | `Vec<T>` | `Vec<T>` |
| `Map<K,V>` | `BTreeMap<K,V>` | `BTreeMap<K,V>` |
| `Option<T>` | `Option<T>` | `Option<T>` |
| `Result<T,E>` | `Result<T,E>` | `Result<T,E>` |
| `[T; N]` | `[T; N]` | `[T; N]` |

### 9.6 IDL Generation

```solscript
// Automatic IDL generation for cross-language use
#[generate_idl]
contract TokenVault {
  @state totalDeposits: u64;
  @state depositors: Map<Address, u64>;

  @public
  fn deposit(amount: u64): Result<(), Error> { /* ... */ }

  @public
  @view
  fn getBalance(user: Address): u64 { /* ... */ }
}

// Generates:
// - target/idl/token_vault.json (Anchor-compatible IDL)
// - target/types/token_vault.rs (Rust types)
// - target/types/token_vault.ts (TypeScript types)
```

### 9.7 Shared Crate Pattern

```solscript
// Define shared types in SolScript
#[shared_crate("my_protocol_types")]
mod types {
  #[derive(Clone, Serialize, Deserialize)]
  struct PoolState {
    tokenA: Address;
    tokenB: Address;
    reserves: (u64, u64);
  }

  enum PoolError {
    InsufficientLiquidity,
    SlippageExceeded,
    InvalidToken,
  }
}

// Generates Rust crate that both SolScript and Rust programs can depend on
// Ensures type compatibility across the ecosystem
```

## 10. Built-in Globals

SolScript provides built-in global objects and functions available in all contracts.

### 10.1 Transaction Context

```solscript
// tx - Transaction context
tx.sender      // Address: The account that signed the transaction
tx.programId   // Address: The current program's address
tx.accounts    // Vec<AccountInfo>: All accounts passed to the instruction
tx.data        // Buffer: Raw instruction data

// self - Contract instance context
self.address   // Address: The contract's program address
self.programId // Address: Alias for self.address
```

### 10.2 Logging

```solscript
// Log messages (visible in transaction logs)
log("Simple message");
log("Value: {}", someValue);
log("Multiple: {} and {}", val1, val2);

// Debug logging (removed in release builds)
debug!("Debug info: {}", data);
```

### 10.3 Assertions

```solscript
// Runtime assertions
assert(condition, "Error message");
assert(balance >= amount, "Insufficient balance");

// Require with custom error
require(condition, CustomError::Variant);
```

## 11. Enums

SolScript supports algebraic data types through enums.

### 11.1 Simple Enums

```solscript
enum Status {
  Pending,
  Active,
  Completed,
  Cancelled,
}

let status = Status::Active;
```

### 11.2 Enums with Data

```solscript
enum TokenEvent {
  Transfer { from: Address, to: Address, amount: u64 },
  Mint { to: Address, amount: u64 },
  Burn { from: Address, amount: u64 },
}

let event = TokenEvent::Transfer {
  from: sender,
  to: recipient,
  amount: 100
};
```

### 11.3 Pattern Matching

```solscript
fn handleEvent(event: TokenEvent): Result<(), Error> {
  match event {
    TokenEvent::Transfer { from, to, amount } => {
      log("Transfer {} from {} to {}", amount, from, to);
    },
    TokenEvent::Mint { to, amount } => {
      log("Mint {} to {}", amount, to);
    },
    TokenEvent::Burn { from, amount } => {
      log("Burn {} from {}", amount, from);
    },
  }
  return Ok(());
}
```

## 12. Closures and Iterators

### 12.1 Closure Syntax

```solscript
// Basic closure
let add = |a: u64, b: u64| -> u64 { a + b };
let result = add(1, 2);

// Closure with type inference
let double = |x| x * 2;

// Closure capturing environment
let multiplier = 3;
let multiply = |x| x * multiplier;
```

### 12.2 Iterator Methods

```solscript
let numbers = vec![1, 2, 3, 4, 5];

// Map
let doubled = numbers.map(|x| x * 2);

// Filter
let evens = numbers.filter(|x| x % 2 == 0);

// Reduce/Fold
let sum = numbers.fold(0, |acc, x| acc + x);

// Chaining
let result = numbers
  .filter(|x| x > 2)
  .map(|x| x * 2)
  .collect::<Vec<u64>>();
```
