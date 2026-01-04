# Built-in Functions & Objects

Reference for all built-in functions and global objects.

## Global Objects

### msg

Transaction message context:

| Property | Type | Description |
|----------|------|-------------|
| `msg.sender` | `address` | Address of the caller |
| `msg.value` | `uint256` | Amount of SOL sent (in lamports) |
| `msg.data` | `bytes` | Complete calldata |

```solidity
function deposit() public payable {
    address caller = msg.sender;
    uint256 amount = msg.value;
}
```

---

### block

Current block information:

| Property | Type | Description |
|----------|------|-------------|
| `block.timestamp` | `uint256` | Current block timestamp (Unix seconds) |
| `block.number` | `uint256` | Current slot number |

```solidity
function checkExpiry(uint256 deadline) public view {
    require(block.timestamp < deadline, "Expired");
}
```

---

### tx

Transaction context:

| Property | Type | Description |
|----------|------|-------------|
| `tx.origin` | `address` | Original transaction sender |

```solidity
// Note: Avoid using tx.origin for authorization
function getOriginalSender() public view returns (address) {
    return tx.origin;
}
```

---

## Type Functions

### address

| Function | Returns | Description |
|----------|---------|-------------|
| `address(0)` | `address` | Zero address |
| `address(x)` | `address` | Convert to address |

```solidity
address zero = address(0);
require(recipient != address(0), "Invalid address");
```

---

### uint/int Conversions

```solidity
// Explicit conversions
uint8 small = uint8(256);    // Truncates to 0
uint256 large = uint256(100);
int256 signed = int256(unsignedValue);
```

---

## String Functions

### string.concat

Concatenate strings:

```solidity
string memory greeting = string.concat("Hello, ", name, "!");
```

---

## Array Functions

### push

Append element to dynamic array:

```solidity
uint256[] storage arr;
arr.push(42);
```

### pop

Remove and return last element:

```solidity
uint256 last = arr.pop();
```

### length

Get array length:

```solidity
uint256 len = arr.length;
```

---

## Bytes Functions

### bytes.concat

Concatenate byte arrays:

```solidity
bytes memory result = bytes.concat(data1, data2);
```

### keccak256

Compute Keccak-256 hash:

```solidity
bytes32 hash = keccak256(abi.encodePacked("data"));
bytes32 hash2 = keccak256(abi.encode(value1, value2));
```

### sha256

Compute SHA-256 hash:

```solidity
bytes32 hash = sha256(abi.encodePacked("data"));
```

---

## ABI Encoding

### abi.encode

Standard ABI encoding with padding:

```solidity
bytes memory encoded = abi.encode(
    uint256(100),
    address(0x123...),
    "hello"
);
```

### abi.encodePacked

Packed encoding (no padding):

```solidity
bytes memory packed = abi.encodePacked(
    uint8(1),
    uint16(2),
    uint32(3)
);
```

### abi.encodeWithSelector

Encode with function selector:

```solidity
bytes memory data = abi.encodeWithSelector(
    IERC20.transfer.selector,
    recipient,
    amount
);
```

### abi.decode

Decode ABI-encoded data:

```solidity
(uint256 value, address addr) = abi.decode(
    data,
    (uint256, address)
);
```

---

## Control Flow

### require

Revert if condition is false:

```solidity
require(condition, "Error message");
require(amount > 0, "Amount must be positive");
require(msg.sender == owner);  // No message
```

### revert

Explicitly revert execution:

```solidity
// With message
revert("Something went wrong");

// With custom error
revert InsufficientBalance(available, required);

// Plain revert
revert();
```

### assert

Check invariants (should never fail):

```solidity
assert(totalBefore == totalAfter);
```

---

## Math Functions

### Operators

| Operator | Description |
|----------|-------------|
| `+` | Addition |
| `-` | Subtraction |
| `*` | Multiplication |
| `/` | Integer division |
| `%` | Modulo (remainder) |
| `**` | Exponentiation |

```solidity
uint256 sum = a + b;
uint256 power = base ** exponent;
uint256 remainder = dividend % divisor;
```

### Comparison

| Operator | Description |
|----------|-------------|
| `<` | Less than |
| `<=` | Less than or equal |
| `>` | Greater than |
| `>=` | Greater than or equal |
| `==` | Equal |
| `!=` | Not equal |

---

## Bitwise Operations

| Operator | Description |
|----------|-------------|
| `&` | Bitwise AND |
| `\|` | Bitwise OR |
| `^` | Bitwise XOR |
| `~` | Bitwise NOT |
| `<<` | Left shift |
| `>>` | Right shift |

```solidity
uint256 flags = a & b;
uint256 combined = a | b;
uint256 shifted = value << 8;
```

---

## Type Information

### type(T).min / type(T).max

Get type bounds:

```solidity
uint256 maxUint = type(uint256).max;
int256 minInt = type(int256).min;
```

---

## Special Values

### this

Reference to current contract:

```solidity
address contractAddr = address(this);
uint256 balance = address(this).balance;
```

### super

Reference to parent contract:

```solidity
function foo() public override {
    super.foo();  // Call parent implementation
}
```

---

## Events

### emit

Emit an event:

```solidity
event Transfer(address indexed from, address indexed to, uint256 amount);

function transfer(address to, uint256 amount) public {
    // ... transfer logic ...
    emit Transfer(msg.sender, to, amount);
}
```

---

## Memory Allocation

### new

Create new array in memory:

```solidity
uint256[] memory arr = new uint256[](10);
bytes memory data = new bytes(32);
```

### delete

Reset value to default:

```solidity
delete balances[user];  // Reset to 0
delete myStruct;        // Reset all fields
delete myArray;         // Clear array
```

---

## Solana-Specific

### Program Context

```solidity
// Access current program ID
address programId = program.id;
```

### PDA Derivation

```solidity
// Derive PDA
(address pda, uint8 bump) = findProgramAddress(
    seeds,
    programId
);
```

---

## See Also

- [Control Flow](../guide/control-flow.md)
- [Events & Errors](../guide/events-errors.md)
- [Types Reference](types.md)
