# Events & Errors

Events log data to the blockchain, while errors handle failure conditions.

## Events

### Declaring Events

```solidity
contract Token {
    event Transfer(
        address indexed from,
        address indexed to,
        uint256 amount
    );

    event Approval(
        address indexed owner,
        address indexed spender,
        uint256 amount
    );

    event Log(string message);
}
```

### Indexed Parameters

Indexed parameters are searchable:

```solidity
event Transfer(
    address indexed from,    // Can filter by sender
    address indexed to,      // Can filter by recipient
    uint256 amount           // Not indexed, in data
);

// Up to 3 indexed parameters per event
```

### Emitting Events

```solidity
function transfer(address to, uint256 amount) public {
    require(balances[msg.sender] >= amount, "Insufficient balance");

    balances[msg.sender] -= amount;
    balances[to] += amount;

    emit Transfer(msg.sender, to, amount);
}

function approve(address spender, uint256 amount) public {
    allowances[msg.sender][spender] = amount;
    emit Approval(msg.sender, spender, amount);
}
```

### Event Patterns

#### Activity Logging

```solidity
event ActivityLog(
    address indexed user,
    string action,
    uint256 timestamp
);

function performAction(string memory action) public {
    // ... action logic ...
    emit ActivityLog(msg.sender, action, block.timestamp);
}
```

#### State Changes

```solidity
event OwnershipTransferred(
    address indexed previousOwner,
    address indexed newOwner
);

function transferOwnership(address newOwner) public onlyOwner {
    address oldOwner = owner;
    owner = newOwner;
    emit OwnershipTransferred(oldOwner, newOwner);
}
```

## Errors

### Custom Errors

More gas-efficient than string messages:

```solidity
// Define custom errors
error InsufficientBalance(uint256 available, uint256 required);
error Unauthorized(address caller);
error InvalidAddress();
error TransferFailed();
error AmountTooLow(uint256 minimum);
```

### Using Revert

```solidity
function withdraw(uint256 amount) public {
    if (balances[msg.sender] < amount) {
        revert InsufficientBalance(balances[msg.sender], amount);
    }

    if (msg.sender != authorized) {
        revert Unauthorized(msg.sender);
    }

    balances[msg.sender] -= amount;
    // ... transfer logic
}
```

### Using Require

Traditional error handling with strings:

```solidity
function transfer(address to, uint256 amount) public {
    require(to != address(0), "Invalid recipient");
    require(amount > 0, "Amount must be positive");
    require(balances[msg.sender] >= amount, "Insufficient balance");

    balances[msg.sender] -= amount;
    balances[to] += amount;
}
```

### Using Assert

For invariant checking (should never fail):

```solidity
function transfer(address to, uint256 amount) public {
    uint256 totalBefore = balances[msg.sender] + balances[to];

    balances[msg.sender] -= amount;
    balances[to] += amount;

    // This should always be true
    assert(balances[msg.sender] + balances[to] == totalBefore);
}
```

## Error Patterns

### Guard Clauses

Check conditions at function start:

```solidity
function deposit(uint256 amount) public payable {
    // Guards first
    require(msg.value == amount, "Value mismatch");
    require(amount >= minimumDeposit, "Below minimum");
    require(!paused, "Contract paused");

    // Main logic after guards
    balances[msg.sender] += amount;
    emit Deposit(msg.sender, amount);
}
```

### Custom Error with Data

```solidity
error SlippageExceeded(
    uint256 expected,
    uint256 actual,
    uint256 maxSlippage
);

function swap(uint256 amountIn, uint256 minAmountOut) public {
    uint256 amountOut = calculateOutput(amountIn);

    if (amountOut < minAmountOut) {
        revert SlippageExceeded(minAmountOut, amountOut, maxSlippage);
    }

    // ... swap logic
}
```

### Error Hierarchies

```solidity
// Base errors
error Unauthorized(address caller);

// Specific errors
error NotOwner(address caller, address owner);
error NotAdmin(address caller);
error NotOperator(address caller);

function ownerAction() public {
    if (msg.sender != owner) {
        revert NotOwner(msg.sender, owner);
    }
    // ...
}
```

## Combined Example

```solidity
contract Vault {
    // Events
    event Deposit(address indexed user, uint256 amount);
    event Withdrawal(address indexed user, uint256 amount);
    event EmergencyWithdraw(address indexed user, uint256 amount);

    // Errors
    error InsufficientBalance(uint256 available, uint256 requested);
    error WithdrawalLocked(uint256 unlockTime);
    error InvalidAmount();
    error ContractPaused();

    // State
    mapping(address => uint256) public balances;
    mapping(address => uint256) public lockTime;
    bool public paused;

    function deposit() public payable {
        if (paused) revert ContractPaused();
        if (msg.value == 0) revert InvalidAmount();

        balances[msg.sender] += msg.value;
        lockTime[msg.sender] = block.timestamp + 1 days;

        emit Deposit(msg.sender, msg.value);
    }

    function withdraw(uint256 amount) public {
        if (paused) revert ContractPaused();
        if (amount == 0) revert InvalidAmount();
        if (balances[msg.sender] < amount) {
            revert InsufficientBalance(balances[msg.sender], amount);
        }
        if (block.timestamp < lockTime[msg.sender]) {
            revert WithdrawalLocked(lockTime[msg.sender]);
        }

        balances[msg.sender] -= amount;

        // Transfer SOL...

        emit Withdrawal(msg.sender, amount);
    }
}
```

## Best Practices

### 1. Use Custom Errors for Gas Efficiency

```solidity
// Good - gas efficient
error Unauthorized();
if (msg.sender != owner) revert Unauthorized();

// Less efficient (but more readable for users)
require(msg.sender == owner, "Not authorized");
```

### 2. Include Relevant Data in Errors

```solidity
// Good - provides context
error InsufficientBalance(uint256 available, uint256 required);
revert InsufficientBalance(balance, amount);

// Less helpful
error InsufficientBalance();
revert InsufficientBalance();
```

### 3. Emit Events for All State Changes

```solidity
function updateConfig(uint256 newValue) public onlyOwner {
    uint256 oldValue = config;
    config = newValue;
    emit ConfigUpdated(oldValue, newValue);  // Always emit
}
```

### 4. Use Indexed Parameters Wisely

```solidity
// Good - commonly filtered fields indexed
event Transfer(
    address indexed from,
    address indexed to,
    uint256 amount  // Rarely filtered, not indexed
);
```

### 5. Document Error Conditions

```solidity
/// @notice Withdraws funds from the vault
/// @dev Reverts if balance insufficient or still locked
/// @param amount The amount to withdraw
function withdraw(uint256 amount) public {
    // Implementation with clear error conditions
}
```

## Next Steps

- [Modifiers](modifiers.md) - Reusable function conditions
- [Interfaces](interfaces.md) - Contract interfaces
- [Inheritance](inheritance.md) - Contract inheritance
