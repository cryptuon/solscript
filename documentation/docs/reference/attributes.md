# Attributes Reference

Attributes provide metadata and compiler directives.

## Visibility Modifiers

### public

Accessible from anywhere, creates automatic getter:

```solidity
uint256 public count;
// Automatically creates: function count() public view returns (uint256)

function transfer(address to, uint256 amount) public {
    // Callable by anyone
}
```

### private

Only accessible within the contract:

```solidity
uint256 private _secretValue;

function _internalHelper() private {
    // Only this contract can call
}
```

### internal

Accessible within contract and derived contracts:

```solidity
uint256 internal _baseValue;

function _sharedLogic() internal {
    // This contract and children can call
}
```

### external

Only callable from outside the contract:

```solidity
function deposit() external payable {
    // Cannot be called internally with this.deposit()
    // More gas efficient for large calldata
}
```

---

## State Mutability

### view

Can read but not modify state:

```solidity
function getBalance(address account) public view returns (uint256) {
    return balances[account];  // Read-only
}
```

### pure

Cannot read or modify state:

```solidity
function add(uint256 a, uint256 b) public pure returns (uint256) {
    return a + b;  // No state access
}
```

### payable

Can receive SOL:

```solidity
function deposit() public payable {
    balances[msg.sender] += msg.value;
}

// Payable constructor
constructor() payable {
    // Can receive SOL during deployment
}
```

---

## Storage Locations

### memory

Temporary data during function execution:

```solidity
function process(uint256[] memory data) public {
    uint256[] memory temp = new uint256[](10);
    // temp only exists during this call
}
```

### storage

Reference to permanent storage:

```solidity
function updateUser(uint256 id) public {
    User storage user = users[id];  // Reference to storage
    user.balance += 100;  // Modifies storage directly
}
```

### calldata

Read-only parameter data:

```solidity
function process(bytes calldata data) external {
    // data is read-only, gas efficient
}
```

---

## Variable Modifiers

### constant

Compile-time constant:

```solidity
uint256 public constant MAX_SUPPLY = 1000000;
address public constant TREASURY = 0x1234...;
string public constant NAME = "Token";
```

**Rules:**
- Must be initialized at declaration
- Cannot be changed after compilation
- Only value types and strings

### immutable

Set once in constructor:

```solidity
address public immutable owner;
uint256 public immutable deployTime;

constructor() {
    owner = msg.sender;
    deployTime = block.timestamp;
}
```

**Rules:**
- Set in constructor or at declaration
- Cannot be changed after deployment
- More gas efficient than regular state

---

## Function Modifiers

### virtual

Function can be overridden:

```solidity
function getValue() public view virtual returns (uint256) {
    return 10;
}
```

### override

Function overrides a parent:

```solidity
function getValue() public view override returns (uint256) {
    return 20;
}

// Multiple inheritance
function foo() public override(A, B) returns (uint256) {
    return super.foo();
}
```

---

## Event Modifiers

### indexed

Parameter is searchable in logs:

```solidity
event Transfer(
    address indexed from,    // Searchable
    address indexed to,      // Searchable
    uint256 amount           // Not searchable, in data
);
```

**Rules:**
- Maximum 3 indexed parameters per event
- Indexed parameters use more gas
- Value types only (not strings or bytes)

---

## Contract Modifiers

### abstract

Contract cannot be deployed directly:

```solidity
abstract contract Base {
    function getValue() public view virtual returns (uint256);

    function getDoubleValue() public view returns (uint256) {
        return getValue() * 2;
    }
}
```

### interface

Pure interface definition:

```solidity
interface IERC20 {
    function transfer(address to, uint256 amount) external returns (bool);
    function balanceOf(address account) external view returns (uint256);
}
```

---

## Inheritance

### is

Inherit from contract or interface:

```solidity
contract Token is Ownable, Pausable, IERC20 {
    // Inherits from all listed
}
```

---

## Summary Table

| Attribute | Applies To | Description |
|-----------|------------|-------------|
| `public` | Variables, Functions | Accessible externally |
| `private` | Variables, Functions | Contract-only access |
| `internal` | Variables, Functions | Contract + derived access |
| `external` | Functions | External-only access |
| `view` | Functions | Read-only state access |
| `pure` | Functions | No state access |
| `payable` | Functions, Constructor | Can receive SOL |
| `memory` | Parameters, Variables | Temporary storage |
| `storage` | Parameters, Variables | Persistent storage reference |
| `calldata` | Parameters | Read-only input data |
| `constant` | Variables | Compile-time constant |
| `immutable` | Variables | Set once at deploy |
| `virtual` | Functions | Can be overridden |
| `override` | Functions | Overrides parent |
| `indexed` | Event Parameters | Searchable in logs |
| `abstract` | Contracts | Cannot deploy directly |

---

## See Also

- [Functions](../guide/functions.md)
- [State Variables](../guide/state.md)
- [Interfaces](../guide/interfaces.md)
