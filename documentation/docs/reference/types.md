# Type Reference

Complete reference for all SolScript types.

## Primitive Types

### Integer Types

#### Unsigned Integers

| Type | Size | Minimum | Maximum |
|------|------|---------|---------|
| `uint8` | 8 bits | 0 | 255 |
| `uint16` | 16 bits | 0 | 65,535 |
| `uint32` | 32 bits | 0 | 4,294,967,295 |
| `uint64` | 64 bits | 0 | 18,446,744,073,709,551,615 |
| `uint128` | 128 bits | 0 | 2^128 - 1 |
| `uint256` | 256 bits | 0 | 2^256 - 1 |

**Aliases:**
- `uint` is an alias for `uint256`

#### Signed Integers

| Type | Size | Minimum | Maximum |
|------|------|---------|---------|
| `int8` | 8 bits | -128 | 127 |
| `int16` | 16 bits | -32,768 | 32,767 |
| `int32` | 32 bits | -2,147,483,648 | 2,147,483,647 |
| `int64` | 64 bits | -2^63 | 2^63 - 1 |
| `int128` | 128 bits | -2^127 | 2^127 - 1 |
| `int256` | 256 bits | -2^255 | 2^255 - 1 |

**Aliases:**
- `int` is an alias for `int256`

---

### Boolean

```solidity
bool flag = true;
bool other = false;
```

**Operations:**
- `&&` - Logical AND
- `||` - Logical OR
- `!` - Logical NOT
- `==`, `!=` - Equality

---

### Address

32-byte Solana public key:

```solidity
address owner = msg.sender;
address zero = address(0);
```

**Literals:**
```solidity
address treasury = 0x1234567890abcdef1234567890abcdef12345678;
```

**Operations:**
- `==`, `!=` - Equality comparison
- Conversion from `bytes32`

---

### String

UTF-8 encoded text:

```solidity
string name = "SolScript";
string empty = "";
```

**Operations:**
- `string.concat(a, b)` - Concatenate strings

**Storage:**
- In storage: stored on-chain
- In memory: temporary during execution

---

### Bytes

#### Fixed-Size Bytes

| Type | Size |
|------|------|
| `bytes1` | 1 byte |
| `bytes2` | 2 bytes |
| ... | ... |
| `bytes32` | 32 bytes |

```solidity
bytes32 hash = keccak256("data");
bytes4 selector = bytes4(keccak256("transfer(address,uint256)"));
```

#### Dynamic Bytes

```solidity
bytes data;

data.push(0x01);       // Append byte
uint256 len = data.length;
bytes1 first = data[0];
```

---

## Complex Types

### Arrays

#### Fixed-Size Arrays

```solidity
uint256[5] fixed;

fixed[0] = 1;
fixed[4] = 5;
uint256 len = fixed.length;  // Always 5
```

#### Dynamic Arrays

```solidity
uint256[] dynamic;

dynamic.push(1);           // Append
dynamic.push(2);
uint256 val = dynamic[0];  // Access
uint256 last = dynamic.pop();  // Remove last
uint256 len = dynamic.length;
delete dynamic[0];         // Reset to 0
```

**Array Literals:**
```solidity
uint256[] memory arr = [1, 2, 3, 4, 5];
```

---

### Mappings

Key-value storage:

```solidity
mapping(address => uint256) balances;
mapping(address => mapping(address => uint256)) allowances;
mapping(uint256 => User) users;
```

**Characteristics:**
- Keys are not enumerable
- All values default to zero/false/empty
- Cannot iterate over mappings
- Cannot get the size

**Access:**
```solidity
balances[msg.sender] = 100;
uint256 bal = balances[msg.sender];
```

---

### Structs

Custom data structures:

```solidity
struct User {
    address wallet;
    uint256 balance;
    bool active;
    string name;
}
```

**Declaration:**
```solidity
User public admin;
User[] public users;
mapping(address => User) public userByAddress;
```

**Initialization:**
```solidity
// Named fields
User memory u1 = User({
    wallet: msg.sender,
    balance: 0,
    active: true,
    name: "Alice"
});

// Positional
User memory u2 = User(msg.sender, 0, true, "Bob");
```

**Access:**
```solidity
admin.balance = 100;
string memory name = admin.name;
```

---

### Enums

Named constants:

```solidity
enum Status {
    Pending,    // 0
    Active,     // 1
    Completed,  // 2
    Cancelled   // 3
}
```

**Usage:**
```solidity
Status public state = Status.Pending;

function activate() public {
    state = Status.Active;
}

function isActive() public view returns (bool) {
    return state == Status.Active;
}
```

**Conversion:**
```solidity
uint8 value = uint8(Status.Active);  // 1
Status s = Status(1);  // Status.Active
```

---

## Storage Locations

### Storage

Permanent on-chain storage:

```solidity
// State variables are storage by default
uint256 public value;
mapping(address => uint256) public balances;
```

### Memory

Temporary during function execution:

```solidity
function process() public {
    uint256[] memory temp = new uint256[](10);
    // temp only exists during this call
}
```

### Calldata

Read-only function parameters:

```solidity
function process(uint256[] calldata data) external {
    // data is read-only, more gas efficient
}
```

---

## Type Conversions

### Implicit Conversions

Smaller to larger types:

```solidity
uint8 small = 100;
uint256 large = small;  // OK - implicit
```

### Explicit Conversions

```solidity
uint256 large = 1000;
uint8 small = uint8(large);  // Explicit cast (may truncate!)

int256 signed = -100;
uint256 unsigned = uint256(signed);  // Be careful!
```

### Address Conversions

```solidity
// From bytes32
bytes32 data = 0x1234...;
address addr = address(uint160(uint256(data)));
```

---

## Type Aliases

Create custom type names:

```solidity
type TokenId is uint256;
type Amount is uint128;
type Percentage is uint8;

function transfer(TokenId id, Amount amount) public {
    // Improved readability
}
```

---

## Default Values

| Type | Default Value |
|------|---------------|
| `uint*` | 0 |
| `int*` | 0 |
| `bool` | false |
| `address` | address(0) |
| `string` | "" |
| `bytes` | empty |
| Arrays | empty |
| Mappings | all keys → default value |
| Structs | all fields → default |
| Enums | first member (0) |

---

## Type Comparison

| Solidity | SolScript | Solana/Rust |
|----------|-----------|-------------|
| `uint256` | `uint256` | `u128` (or custom) |
| `uint64` | `uint64` | `u64` |
| `int64` | `int64` | `i64` |
| `bool` | `bool` | `bool` |
| `address` | `address` | `Pubkey` |
| `string` | `string` | `String` |
| `bytes` | `bytes` | `Vec<u8>` |
| `mapping` | `mapping` | PDA |

---

## See Also

- [Language Guide: Types](../guide/types.md)
- [Contracts](../guide/contracts.md)
- [State Variables](../guide/state.md)
