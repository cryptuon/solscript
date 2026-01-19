# Solana Concepts in SolScript

SolScript abstracts many of Solana's complexities, but understanding the underlying concepts helps you write better contracts and debug issues.

## How SolScript Maps to Solana

| Solidity Concept | SolScript | Solana/Anchor |
|------------------|-----------|---------------|
| Contract | `contract MyContract { }` | `#[program]` module |
| State variables | `uint256 public count;` | `#[account]` struct |
| Mappings | `mapping(address => uint256)` | PDA-based accounts |
| Functions | `function foo() public` | Instruction handlers |
| Events | `event Transfer(...)` | `#[event]` + `emit!()` |
| Errors | `error Unauthorized()` | `#[error_code]` enum |
| `msg.sender` | `msg.sender` | `ctx.accounts.signer.key()` |

## Accounts

Solana stores all data in accounts. SolScript handles account management automatically:

### State Accounts

Your contract's state variables become a single state account:

```solidity
contract Counter {
    uint256 public count;      // Stored in state account
    address public owner;      // Stored in state account
}
```

**Generated Anchor code:**
```rust
#[account]
pub struct CounterState {
    pub count: u64,
    pub owner: Pubkey,
}
```

SolScript automatically:
- Calculates account space via `InitSpace` derive
- Handles initialization with rent exemption
- Generates proper account constraints

### Signer Accounts

Transaction signers are inferred from function parameters:

```solidity
// msg.sender is automatically a signer
function transfer(address to, uint256 amount) public {
    require(balances[msg.sender] >= amount);
}

// Explicit additional signer
function withdraw(signer authority, uint256 amount) public {
    require(authority == owner);
}
```

## Program Derived Addresses (PDAs)

PDAs are Solana's way of creating deterministic account addresses owned by programs. **SolScript's killer feature is automatic mapping-to-PDA transformation.**

### Mappings Become PDAs

```solidity
mapping(address => uint256) public balances;

// Usage looks like Solidity
balances[user] = 100;
uint256 bal = balances[user];
```

**What SolScript generates:**
```rust
// A PDA account for each mapping entry
#[account]
pub struct BalancesEntry {
    pub value: u64,
}

// Seeds derived from the key
seeds = [b"balances", user.as_ref()]
```

### Nested Mappings

```solidity
mapping(address => mapping(address => uint256)) public allowances;

// Multi-key access
allowances[owner][spender] = amount;
```

**Generated seeds:**
```rust
seeds = [b"allowances", owner.as_ref(), spender.as_ref()]
```

### How It Works

| Operation | SolScript | Anchor Constraint |
|-----------|-----------|-------------------|
| First write | `balances[user] = 100` | `init_if_needed` |
| Read | `balances[user]` | Seeds only |
| Update | `balances[user] += 50` | `mut` + seeds |
| Delete | `delete balances[user]` | `close = signer` |

### Closing Mapping PDAs

Use `delete` to close a mapping PDA and reclaim rent:

```solidity
function removeUser(address user) public {
    delete balances[user];  // Closes PDA, rent returned to signer
}
```

When you delete a mapping entry:
1. The PDA account is closed by Anchor's `close = signer` constraint
2. Lamports (rent) are returned to the transaction signer
3. The account data is zeroed and marked for garbage collection

## Instructions

Every `public` function becomes a Solana instruction:

```solidity
function increment() public {
    count += 1;
}
```

**Generated:**
```rust
pub fn increment(ctx: Context<Increment>) -> Result<()> {
    ctx.accounts.state.count += 1;
    Ok(())
}
```

### View Functions

`view` functions generate read-only account access:

```solidity
function getCount() public view returns (uint256) {
    return count;
}
```

**Generated context:**
```rust
#[derive(Accounts)]
pub struct GetCount<'info> {
    pub state: Account<'info, State>,  // NOT mutable
}
```

### Mutable Functions

Non-view functions get mutable access:

```solidity
function setCount(uint256 newCount) public {
    count = newCount;
}
```

**Generated context:**
```rust
#[derive(Accounts)]
pub struct SetCount<'info> {
    #[account(mut)]
    pub state: Account<'info, State>,  // Mutable
}
```

## Rent

Solana charges rent for storing data. SolScript handles this automatically:

### Automatic Rent Exemption

All accounts are created rent-exempt:

```rust
// Generated initialization
#[account(
    init,
    payer = signer,
    space = 8 + State::INIT_SPACE  // Discriminator + data
)]
pub state: Account<'info, State>,
```

### Rent Calculations

SolScript provides rent utilities (used internally):

```solidity
// Available but rarely needed directly
uint64 minBalance = rent.minimumBalance(dataSize);
bool isExempt = rent.isExempt(lamports, dataSize);
```

## Transactions

Solana groups instructions into transactions. SolScript relies on Anchor's handling:

- Each function call = one instruction
- Client builds transactions from instructions
- Anchor handles serialization/deserialization

### Transaction Limits

Be aware of Solana's limits:
- **1232 bytes** max transaction size
- **200,000** compute units default (can request more)
- **64** max accounts per transaction

## Cross-Program Invocation (CPI)

Call other Solana programs using interfaces:

```solidity
interface IERC20 {
    function transfer(address to, uint256 amount) external returns (bool);
}

contract MyContract {
    function sendTokens(address token, address to, uint256 amount) public {
        IERC20(token).transfer(to, amount);
    }
}
```

### SPL Token Operations

SolScript has built-in support for SPL Token:

```solidity
// These generate proper CPI calls
token.transfer(from, to, authority, amount);
token.mint(mint, to, authority, amount);
token.burn(from, mint, authority, amount);
```

### Direct SOL Transfers

Transfer SOL (lamports) using the built-in `transfer` function:

```solidity
function withdraw(address to, uint64 amount) public {
    require(msg.sender == owner, "Unauthorized");
    transfer(to, amount);  // Transfers lamports to recipient
}

function payBounty(address hunter, uint64 reward) public {
    require(bounties[hunter] >= reward, "Insufficient bounty");
    bounties[hunter] -= reward;
    transfer(hunter, reward);
}
```

**How it works:**
- Generates Anchor `system_program::transfer` CPI
- Automatically adds `recipient` account to the instruction context
- Validates that `recipient.key() == to` for security
- Lamports are deducted from the signer's account

!!! warning "Recipient Account Required"
    The recipient address must be passed as an account to the transaction.
    The generated Anchor code includes a `recipient: UncheckedAccount` that
    must match the `to` address parameter.

## Built-in Objects

### msg.sender

The transaction signer:

```solidity
require(msg.sender == owner, "Unauthorized");
```

**Maps to:** `ctx.accounts.signer.key()`

### block.timestamp

Current Unix timestamp:

```solidity
require(block.timestamp >= deadline, "Too early");
```

**Maps to:** `Clock::get()?.unix_timestamp as u64`

### block.number (slot)

Current Solana slot:

```solidity
uint64 currentSlot = block.number;
```

**Maps to:** `Clock::get()?.slot`

## Current Limitations

Understanding these helps you work around them:

### No Incoming SOL Payments (msg.value)

```solidity
// msg.value returns 0 - incoming payments not tracked
function deposit() public payable {
    // msg.value is always 0
}
```

**Workaround:** Use SPL Token (wrapped SOL) for receiving payments.

!!! tip "Outgoing Transfers Work"
    While `msg.value` returns 0, you can still **send** SOL using `transfer(to, amount)`.
    See [Direct SOL Transfers](#direct-sol-transfers) above.

### No Token 2022

Only SPL Token is supported, not Token 2022 extensions (transfer fees, interest-bearing, etc.).

### Modifiers Are Inlined

Modifiers work but are inlined into each function, not generated as reusable validation functions. Keep modifiers small to avoid code bloat.

## Best Practices

### 1. Prefer Mappings for User Data

```solidity
// Good - each user gets their own PDA
mapping(address => UserData) public users;

// Avoid - single account grows unbounded
UserData[] public allUsers;
```

### 2. Use View Functions

```solidity
// Explicitly mark read-only functions
function getBalance(address user) public view returns (uint256) {
    return balances[user];
}
```

### 3. Validate Early

```solidity
function withdraw(uint256 amount) public {
    // Check first, modify state last
    require(balances[msg.sender] >= amount, "Insufficient");
    balances[msg.sender] -= amount;
}
```

### 4. Emit Events for Indexing

```solidity
event Transfer(address indexed from, address indexed to, uint256 amount);

function transfer(address to, uint256 amount) public {
    // ... transfer logic ...
    emit Transfer(msg.sender, to, amount);
}
```

### 5. Clean Up Mapping Data

```solidity
// Reclaim rent when data is no longer needed
function removeUser(address user) public onlyOwner {
    delete users[user];  // Closes PDA, refunds rent
}
```

### 6. Use Structs for Organization

```solidity
contract Token {
    // Define structs inside contracts for encapsulation
    struct Balance {
        uint256 amount;
        uint64 lastUpdate;
    }

    mapping(address => Balance) public balances;
}
```

## See Also

- [Roadmap](../reference/roadmap.md) - Planned features and improvements
- [Types](types.md) - Complete type reference
- [Functions](functions.md) - Function patterns
