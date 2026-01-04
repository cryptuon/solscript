# Functions

Functions are the primary way to interact with contracts.

## Function Syntax

```solidity
function functionName(Type1 param1, Type2 param2)
    visibility
    stateMutability
    modifiers
    returns (ReturnType)
{
    // function body
}
```

## Visibility

### Public

Callable from anywhere:

```solidity
function transfer(address to, uint256 amount) public {
    // Anyone can call this
}
```

### External

Only callable from outside the contract:

```solidity
function deposit() external payable {
    // Only external calls
    // More gas efficient for large data
}
```

### Internal

Only callable from this contract or derived contracts:

```solidity
function _calculateFee(uint256 amount) internal pure returns (uint256) {
    return amount * 3 / 100;
}
```

### Private

Only callable from this contract:

```solidity
function _updateState() private {
    // Only this contract can call
}
```

## State Mutability

### Default (State-Changing)

Can read and modify state:

```solidity
function increment() public {
    count += 1;  // Modifies state
}
```

### View

Can read but not modify state:

```solidity
function getBalance(address account) public view returns (uint256) {
    return balances[account];  // Read-only
}
```

### Pure

Cannot read or modify state:

```solidity
function add(uint256 a, uint256 b) public pure returns (uint256) {
    return a + b;  // No state access
}
```

### Payable

Can receive SOL:

```solidity
function deposit() public payable {
    balances[msg.sender] += msg.value;
}
```

## Parameters

### Value Parameters

```solidity
function setValue(uint256 value) public {
    storedValue = value;
}
```

### Memory Parameters

For complex types:

```solidity
function processArray(uint256[] memory data) public pure returns (uint256) {
    uint256 sum = 0;
    for (uint256 i = 0; i < data.length; i++) {
        sum += data[i];
    }
    return sum;
}
```

### Calldata Parameters

Read-only, gas efficient:

```solidity
function processData(bytes calldata data) external pure returns (uint256) {
    return data.length;
}
```

## Return Values

### Single Return

```solidity
function getValue() public view returns (uint256) {
    return storedValue;
}
```

### Multiple Returns

```solidity
function getDetails() public view returns (uint256, bool, address) {
    return (count, active, owner);
}

// Named returns
function getInfo() public view returns (
    uint256 balance,
    bool isActive,
    address ownerAddress
) {
    balance = balances[msg.sender];
    isActive = active;
    ownerAddress = owner;
}
```

### Destructuring Returns

```solidity
(uint256 balance, bool active, address owner) = contract.getDetails();

// Ignore some values
(uint256 balance, , ) = contract.getDetails();
```

## Function Modifiers

Reusable function conditions:

```solidity
modifier onlyOwner() {
    require(msg.sender == owner, "Not owner");
    _;  // Continue with function
}

modifier validAddress(address addr) {
    require(addr != address(0), "Invalid address");
    _;
}

modifier nonReentrant() {
    require(!locked, "Reentrant call");
    locked = true;
    _;
    locked = false;
}

// Using modifiers
function withdraw(address to, uint256 amount)
    public
    onlyOwner
    validAddress(to)
    nonReentrant
{
    // Function body
}
```

### Modifier with Parameters

```solidity
modifier minAmount(uint256 min) {
    require(msg.value >= min, "Amount too low");
    _;
}

function deposit() public payable minAmount(1 ether) {
    // ...
}
```

## Function Overloading

Multiple functions with same name, different parameters:

```solidity
function transfer(address to, uint256 amount) public {
    _transfer(msg.sender, to, amount);
}

function transfer(address from, address to, uint256 amount) public {
    require(allowance[from][msg.sender] >= amount, "Not allowed");
    _transfer(from, to, amount);
}
```

## Virtual and Override

For inheritance:

```solidity
contract Base {
    function getValue() public view virtual returns (uint256) {
        return 10;
    }
}

contract Derived is Base {
    function getValue() public view override returns (uint256) {
        return 20;
    }
}
```

## Constructor

Special function called once at deployment:

```solidity
contract Token {
    string public name;
    address public owner;

    constructor(string memory _name) {
        name = _name;
        owner = msg.sender;
    }
}

// Deploy with: new Token("MyToken")
```

## Fallback and Receive

Handle unknown calls and plain transfers:

```solidity
contract Receiver {
    event Received(address sender, uint256 amount);

    // Receive plain SOL transfers
    receive() external payable {
        emit Received(msg.sender, msg.value);
    }

    // Fallback for unknown function calls
    fallback() external payable {
        // Handle unknown calls
    }
}
```

## Internal Function Calls

```solidity
contract Calculator {
    function add(uint256 a, uint256 b) public pure returns (uint256) {
        return _add(a, b);
    }

    function _add(uint256 a, uint256 b) internal pure returns (uint256) {
        return a + b;
    }
}
```

## External Function Calls

```solidity
interface IToken {
    function transfer(address to, uint256 amount) external returns (bool);
}

contract Wallet {
    function sendTokens(IToken token, address to, uint256 amount) public {
        bool success = token.transfer(to, amount);
        require(success, "Transfer failed");
    }
}
```

## Error Handling

### Require

```solidity
function withdraw(uint256 amount) public {
    require(amount > 0, "Amount must be positive");
    require(balances[msg.sender] >= amount, "Insufficient balance");
    // ...
}
```

### Custom Errors

```solidity
error InsufficientBalance(uint256 available, uint256 required);

function withdraw(uint256 amount) public {
    if (balances[msg.sender] < amount) {
        revert InsufficientBalance(balances[msg.sender], amount);
    }
    // ...
}
```

## Best Practices

### 1. Use Descriptive Names

```solidity
// Good
function transferTokens(address recipient, uint256 amount) public { }

// Avoid
function tt(address r, uint256 a) public { }
```

### 2. Order Parameters Consistently

```solidity
// Consistent: from, to, amount
function transfer(address from, address to, uint256 amount) public { }
function approve(address owner, address spender, uint256 amount) public { }
```

### 3. Validate All Inputs

```solidity
function setOwner(address newOwner) public onlyOwner {
    require(newOwner != address(0), "Invalid address");
    require(newOwner != owner, "Already owner");
    owner = newOwner;
}
```

### 4. Emit Events for State Changes

```solidity
function transfer(address to, uint256 amount) public {
    // ... transfer logic ...
    emit Transfer(msg.sender, to, amount);
}
```

### 5. Use View/Pure When Possible

Saves gas and clarifies intent:

```solidity
// Good - clearly read-only
function getBalance() public view returns (uint256) {
    return balance;
}

// Good - clearly no state access
function calculate(uint256 a, uint256 b) public pure returns (uint256) {
    return a * b;
}
```

## Next Steps

- [State Variables](state.md) - Managing contract state
- [Events & Errors](events-errors.md) - Logging and error handling
- [Modifiers](modifiers.md) - Creating reusable modifiers
