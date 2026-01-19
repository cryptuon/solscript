# State Variables

State variables store data permanently on the blockchain.

## Declaring State Variables

```solidity
contract Storage {
    // Simple types
    uint256 public count;
    bool public active;
    address public owner;
    string public name;

    // With initial values
    uint256 public fee = 100;
    bool public paused = false;
}
```

## Visibility

### Public

Creates automatic getter function:

```solidity
uint256 public totalSupply;
// Automatically creates: function totalSupply() public view returns (uint256)
```

### Private

Only accessible within the contract:

```solidity
uint256 private _secretValue;

function getSecret() public view returns (uint256) {
    return _secretValue;  // Only way to access
}
```

### Internal

Accessible in this contract and derived contracts:

```solidity
contract Base {
    uint256 internal _baseValue;
}

contract Derived is Base {
    function getBaseValue() public view returns (uint256) {
        return _baseValue;  // Can access internal
    }
}
```

## Mappings

Key-value storage:

```solidity
contract Balances {
    // Simple mapping
    mapping(address => uint256) public balances;

    // Nested mapping
    mapping(address => mapping(address => uint256)) public allowances;

    // Mapping to struct
    mapping(uint256 => User) public users;

    function setBalance(address account, uint256 amount) public {
        balances[account] = amount;
    }

    function setAllowance(address owner, address spender, uint256 amount) public {
        allowances[owner][spender] = amount;
    }
}
```

!!! info "Solana Implementation"
    Mappings are implemented as Program Derived Addresses (PDAs) on Solana.
    Each key derives a unique account address.

### Deleting Mapping Entries

Use `delete` to remove a mapping entry and reclaim rent:

```solidity
contract UserRegistry {
    mapping(address => uint64) public scores;
    mapping(address => mapping(address => uint256)) public allowances;

    function removeUser(address user) public {
        delete scores[user];  // Closes PDA, refunds rent to signer
    }

    function revokeAllowance(address owner, address spender) public {
        delete allowances[owner][spender];  // Works with nested mappings too
    }
}
```

!!! tip "PDA Closing"
    When you `delete` a mapping entry, the underlying PDA account is closed
    and the rent (lamports) is returned to the transaction signer. This is
    the proper way to clean up mapping data on Solana.

## Arrays

### Dynamic Arrays

```solidity
contract Arrays {
    uint256[] public numbers;
    address[] public members;

    function addNumber(uint256 num) public {
        numbers.push(num);
    }

    function removeLastNumber() public {
        numbers.pop();
    }

    function getNumbersLength() public view returns (uint256) {
        return numbers.length;
    }

    function getNumber(uint256 index) public view returns (uint256) {
        require(index < numbers.length, "Index out of bounds");
        return numbers[index];
    }
}
```

### Fixed Arrays

```solidity
contract FixedArrays {
    uint256[10] public fixedNumbers;

    function setNumber(uint256 index, uint256 value) public {
        require(index < 10, "Index out of bounds");
        fixedNumbers[index] = value;
    }
}
```

## Structs

```solidity
struct User {
    address wallet;
    uint256 balance;
    uint256 createdAt;
    bool active;
}

contract UserRegistry {
    User[] public users;
    mapping(address => User) public userByAddress;
    mapping(address => uint256) public userIndex;

    function createUser() public {
        User memory newUser = User({
            wallet: msg.sender,
            balance: 0,
            createdAt: block.timestamp,
            active: true
        });

        userIndex[msg.sender] = users.length;
        users.push(newUser);
        userByAddress[msg.sender] = newUser;
    }

    function updateBalance(address wallet, uint256 amount) public {
        userByAddress[wallet].balance = amount;
        users[userIndex[wallet]].balance = amount;
    }
}
```

## Constants and Immutables

### Constants

Compile-time constants:

```solidity
contract Constants {
    uint256 public constant MAX_SUPPLY = 1000000;
    uint256 public constant FEE_PERCENT = 3;
    address public constant TREASURY = 0x1234...;

    function calculateFee(uint256 amount) public pure returns (uint256) {
        return amount * FEE_PERCENT / 100;
    }
}
```

### Immutables

Set once in constructor:

```solidity
contract Immutables {
    address public immutable owner;
    uint256 public immutable deployTime;

    constructor() {
        owner = msg.sender;
        deployTime = block.timestamp;
    }
}
```

## State Initialization

### In Declaration

```solidity
uint256 public count = 0;
bool public active = true;
address public owner = msg.sender;  // Set at deployment
```

### In Constructor

```solidity
contract Token {
    string public name;
    uint256 public totalSupply;

    constructor(string memory _name, uint256 _supply) {
        name = _name;
        totalSupply = _supply;
    }
}
```

## Reading State

### Direct Access

```solidity
function getCount() public view returns (uint256) {
    return count;
}
```

### Mapping Access

```solidity
function getBalance(address account) public view returns (uint256) {
    return balances[account];
}
```

### Array Access

```solidity
function getUser(uint256 index) public view returns (User memory) {
    return users[index];
}
```

## Modifying State

### Simple Assignment

```solidity
function setCount(uint256 newCount) public {
    count = newCount;
}
```

### Increment/Decrement

```solidity
function increment() public {
    count += 1;
    // Or: count = count + 1;
    // Or: count++;
}

function decrement() public {
    require(count > 0, "Cannot go below zero");
    count -= 1;
}
```

### Mapping Updates

```solidity
function transfer(address to, uint256 amount) public {
    require(balances[msg.sender] >= amount, "Insufficient");
    balances[msg.sender] -= amount;
    balances[to] += amount;
}
```

## State Patterns

### Ownership

```solidity
contract Ownable {
    address public owner;

    event OwnershipTransferred(address indexed previous, address indexed next);

    constructor() {
        owner = msg.sender;
    }

    modifier onlyOwner() {
        require(msg.sender == owner, "Not owner");
        _;
    }

    function transferOwnership(address newOwner) public onlyOwner {
        require(newOwner != address(0), "Invalid address");
        emit OwnershipTransferred(owner, newOwner);
        owner = newOwner;
    }
}
```

### Pausable

```solidity
contract Pausable {
    bool public paused;

    modifier whenNotPaused() {
        require(!paused, "Contract paused");
        _;
    }

    modifier whenPaused() {
        require(paused, "Contract not paused");
        _;
    }

    function pause() public onlyOwner whenNotPaused {
        paused = true;
    }

    function unpause() public onlyOwner whenPaused {
        paused = false;
    }
}
```

### Counter

```solidity
contract Counter {
    uint256 private _counter;

    function current() public view returns (uint256) {
        return _counter;
    }

    function increment() public returns (uint256) {
        _counter += 1;
        return _counter;
    }

    function decrement() public returns (uint256) {
        require(_counter > 0, "Counter underflow");
        _counter -= 1;
        return _counter;
    }
}
```

## Gas Considerations

!!! tip "Optimize Storage"
    - Pack smaller types together (e.g., multiple `uint128` in one slot)
    - Use `uint256` for single values (no packing overhead)
    - Use mappings instead of arrays when you don't need enumeration
    - Cache storage reads in memory for multiple uses

```solidity
// Less efficient - multiple storage reads
function inefficient() public view returns (uint256) {
    return balance + balance + balance;
}

// More efficient - cache in memory
function efficient() public view returns (uint256) {
    uint256 b = balance;  // One storage read
    return b + b + b;
}
```

## Next Steps

- [Control Flow](control-flow.md) - Conditionals and loops
- [Events & Errors](events-errors.md) - Logging state changes
- [Modifiers](modifiers.md) - State access control
