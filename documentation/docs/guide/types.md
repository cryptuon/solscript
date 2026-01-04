# Types

SolScript provides a rich type system optimized for Solana development.

## Integer Types

### Unsigned Integers

| Type | Size | Range |
|------|------|-------|
| `uint8` | 8 bits | 0 to 255 |
| `uint16` | 16 bits | 0 to 65,535 |
| `uint32` | 32 bits | 0 to 4,294,967,295 |
| `uint64` | 64 bits | 0 to 18,446,744,073,709,551,615 |
| `uint128` | 128 bits | Very large numbers |
| `uint256` | 256 bits | Maximum precision |

```solidity
uint8 small = 255;
uint64 medium = 1000000;
uint256 large = 10**18;
```

### Signed Integers

| Type | Size | Range |
|------|------|-------|
| `int8` | 8 bits | -128 to 127 |
| `int16` | 16 bits | -32,768 to 32,767 |
| `int32` | 32 bits | -2,147,483,648 to 2,147,483,647 |
| `int64` | 64 bits | Large signed numbers |
| `int128` | 128 bits | Very large signed numbers |
| `int256` | 256 bits | Maximum precision signed |

```solidity
int8 temperature = -20;
int256 balance = -1000;
```

### Integer Operations

```solidity
uint256 a = 10;
uint256 b = 3;

uint256 sum = a + b;        // 13
uint256 diff = a - b;       // 7
uint256 prod = a * b;       // 30
uint256 quot = a / b;       // 3 (integer division)
uint256 rem = a % b;        // 1 (remainder)

// Comparisons
bool eq = a == b;           // false
bool ne = a != b;           // true
bool lt = a < b;            // false
bool le = a <= b;           // false
bool gt = a > b;            // true
bool ge = a >= b;           // true

// Bitwise
uint256 and = a & b;        // Bitwise AND
uint256 or = a | b;         // Bitwise OR
uint256 xor = a ^ b;        // Bitwise XOR
uint256 shl = a << 2;       // Left shift
uint256 shr = a >> 2;       // Right shift
```

## Boolean

```solidity
bool active = true;
bool inactive = false;

// Logical operations
bool and = active && inactive;  // false
bool or = active || inactive;   // true
bool not = !active;             // false
```

## Address

The `address` type represents a Solana public key (32 bytes):

```solidity
address owner = msg.sender;
address zero = address(0);  // Zero address

// Comparison
bool isOwner = owner == msg.sender;

// Address literals (hex)
address treasury = 0x1234567890abcdef...;
```

## String

Strings are UTF-8 encoded text:

```solidity
string name = "SolScript";
string message = "Hello, World!";

// String concatenation (in memory)
string greeting = string.concat("Hello, ", name);
```

## Bytes

### Fixed-Size Bytes

```solidity
bytes32 hash;           // 32 bytes (common for hashes)
bytes20 shortHash;      // 20 bytes
```

### Dynamic Bytes

```solidity
bytes data;             // Dynamic byte array

// Operations
data.push(0x01);        // Append byte
uint256 len = data.length;
bytes1 first = data[0];
```

## Arrays

### Fixed-Size Arrays

```solidity
uint256[5] fixed;       // Array of 5 uint256

fixed[0] = 1;
fixed[4] = 5;
uint256 len = fixed.length;  // 5
```

### Dynamic Arrays

```solidity
uint256[] dynamic;

// Operations
dynamic.push(1);        // Append element
dynamic.push(2);
uint256 len = dynamic.length;  // 2
uint256 last = dynamic.pop();  // Remove and return last
```

### Array Literals

```solidity
uint256[] memory arr = [1, 2, 3, 4, 5];
```

## Mappings

Key-value storage (implemented as PDAs on Solana):

```solidity
mapping(address => uint256) public balances;
mapping(address => mapping(address => uint256)) public allowances;
mapping(uint256 => User) public users;

// Usage
balances[msg.sender] = 100;
uint256 balance = balances[msg.sender];

allowances[owner][spender] = amount;
```

!!! note "Mapping Behavior"
    - Keys are not enumerable
    - All values default to zero/false/empty
    - Cannot get the size of a mapping

## Structs

Custom data structures:

```solidity
struct User {
    address wallet;
    uint256 balance;
    bool active;
    string name;
}

// Declaration
User public admin;
User[] public users;
mapping(address => User) public userByAddress;

// Initialization
User memory newUser = User({
    wallet: msg.sender,
    balance: 0,
    active: true,
    name: "Alice"
});

// Or positional
User memory user2 = User(msg.sender, 0, true, "Bob");

// Access
admin.balance = 100;
string memory name = admin.name;
```

## Enums

Named constants:

```solidity
enum Status {
    Pending,    // 0
    Active,     // 1
    Completed,  // 2
    Cancelled   // 3
}

Status public status = Status.Pending;

function activate() public {
    status = Status.Active;
}

function isActive() public view returns (bool) {
    return status == Status.Active;
}
```

## Tuples

Multiple return values:

```solidity
function getValues() public pure returns (uint256, bool, string memory) {
    return (42, true, "hello");
}

// Destructuring
(uint256 num, bool flag, string memory text) = getValues();

// Partial destructuring
(uint256 num, , ) = getValues();  // Ignore some values
```

## Type Conversions

### Implicit Conversions

Smaller types can convert to larger types:

```solidity
uint8 small = 100;
uint256 large = small;  // OK
```

### Explicit Conversions

```solidity
uint256 large = 1000;
uint8 small = uint8(large);  // Explicit cast (may overflow!)

int256 signed = -100;
uint256 unsigned = uint256(signed);  // Be careful with negative values!
```

### Address Conversions

```solidity
// From bytes32
bytes32 data = 0x1234...;
address addr = address(uint160(uint256(data)));
```

## Storage Locations

### Storage

Permanent on-chain storage:

```solidity
uint256 public value;  // Storage by default for state variables
```

### Memory

Temporary during function execution:

```solidity
function process(uint256[] memory data) public pure {
    uint256[] memory temp = new uint256[](10);
    // temp exists only during this call
}
```

### Calldata

Read-only function parameters:

```solidity
function process(uint256[] calldata data) external pure {
    // data is read-only, more gas efficient
}
```

## Type Aliases

Create readable type names:

```solidity
type TokenId is uint256;
type Amount is uint128;

function transfer(TokenId id, Amount amount) public {
    // ...
}
```

## Next Steps

- [Functions](functions.md) - Function definitions and patterns
- [State Variables](state.md) - Managing on-chain state
- [Control Flow](control-flow.md) - Control flow statements
