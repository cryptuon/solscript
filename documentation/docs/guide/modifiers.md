# Modifiers

Modifiers are reusable code that can run before and/or after a function.

## Basic Modifiers

### Defining a Modifier

```solidity
contract Ownable {
    address public owner;

    modifier onlyOwner() {
        require(msg.sender == owner, "Not owner");
        _;  // Continue with function body
    }

    constructor() {
        owner = msg.sender;
    }

    function changeOwner(address newOwner) public onlyOwner {
        owner = newOwner;
    }
}
```

### The Placeholder `_`

The `_` represents where the function body executes:

```solidity
modifier beforeAndAfter() {
    // Code here runs BEFORE function
    doSomethingBefore();

    _;  // Function body executes here

    // Code here runs AFTER function
    doSomethingAfter();
}
```

## Modifiers with Parameters

```solidity
modifier minAmount(uint256 minimum) {
    require(msg.value >= minimum, "Amount too low");
    _;
}

modifier validAddress(address addr) {
    require(addr != address(0), "Invalid address");
    _;
}

modifier onlyRole(bytes32 role) {
    require(hasRole(role, msg.sender), "Missing role");
    _;
}

// Using parameterized modifiers
function deposit() public payable minAmount(1 ether) {
    balances[msg.sender] += msg.value;
}

function transfer(address to, uint256 amount)
    public
    validAddress(to)
{
    // ...
}
```

## Common Modifier Patterns

### Access Control

```solidity
contract AccessControl {
    address public owner;
    mapping(address => bool) public admins;
    mapping(address => bool) public operators;

    modifier onlyOwner() {
        require(msg.sender == owner, "Not owner");
        _;
    }

    modifier onlyAdmin() {
        require(admins[msg.sender], "Not admin");
        _;
    }

    modifier onlyOperator() {
        require(operators[msg.sender], "Not operator");
        _;
    }

    modifier onlyAdminOrOwner() {
        require(
            msg.sender == owner || admins[msg.sender],
            "Not authorized"
        );
        _;
    }
}
```

### Reentrancy Guard

```solidity
contract ReentrancyGuard {
    bool private locked;

    modifier nonReentrant() {
        require(!locked, "Reentrant call");
        locked = true;
        _;
        locked = false;
    }

    function withdraw(uint256 amount) public nonReentrant {
        require(balances[msg.sender] >= amount, "Insufficient");
        balances[msg.sender] -= amount;
        // Transfer SOL...
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

    function transfer(address to, uint256 amount)
        public
        whenNotPaused
    {
        // Only works when not paused
    }
}
```

### State Validation

```solidity
contract Auction {
    enum State { Created, Active, Ended }
    State public state;

    modifier inState(State expected) {
        require(state == expected, "Invalid state");
        _;
    }

    modifier notInState(State forbidden) {
        require(state != forbidden, "Invalid state");
        _;
    }

    function startAuction() public onlyOwner inState(State.Created) {
        state = State.Active;
    }

    function bid() public payable inState(State.Active) {
        // Process bid
    }

    function endAuction() public inState(State.Active) {
        state = State.Ended;
    }
}
```

### Time-Based

```solidity
contract TimeLock {
    uint256 public unlockTime;

    modifier onlyAfter(uint256 time) {
        require(block.timestamp >= time, "Too early");
        _;
    }

    modifier onlyBefore(uint256 time) {
        require(block.timestamp < time, "Too late");
        _;
    }

    function withdraw() public onlyAfter(unlockTime) {
        // Can only withdraw after unlock time
    }

    function deposit() public payable onlyBefore(unlockTime) {
        // Can only deposit before unlock time
    }
}
```

## Multiple Modifiers

Modifiers execute in order, left to right:

```solidity
function sensitiveAction()
    public
    onlyOwner           // First: check ownership
    whenNotPaused       // Second: check not paused
    nonReentrant        // Third: set reentrancy lock
{
    // Function body runs last
}
```

Execution flow:
1. `onlyOwner` checks owner, reaches `_`
2. `whenNotPaused` checks pause state, reaches `_`
3. `nonReentrant` sets lock, reaches `_`
4. Function body executes
5. `nonReentrant` post-code runs (unlock)
6. Control returns up the chain

## Modifier Inheritance

```solidity
contract Base {
    address public owner;

    modifier onlyOwner() {
        require(msg.sender == owner, "Not owner");
        _;
    }
}

contract Derived is Base {
    // Can use inherited modifier
    function adminFunction() public onlyOwner {
        // ...
    }

    // Can override modifier
    modifier onlyOwner() override {
        require(msg.sender == owner, "Derived: Not owner");
        _;
    }
}
```

## Virtual Modifiers

```solidity
contract Base {
    modifier check() virtual {
        require(condition(), "Check failed");
        _;
    }

    function condition() internal view virtual returns (bool) {
        return true;
    }
}

contract Derived is Base {
    modifier check() override {
        require(condition(), "Derived check failed");
        _;
    }

    function condition() internal view override returns (bool) {
        return someOtherCondition;
    }
}
```

## Advanced Patterns

### Modifier with Return Value Check

```solidity
modifier checkReturn() {
    _;
    // Check something after function executes
    require(lastOperationSuccessful, "Operation failed");
}
```

### Combining with Events

```solidity
modifier logged(string memory action) {
    emit ActionStarted(msg.sender, action);
    _;
    emit ActionCompleted(msg.sender, action);
}

function importantAction() public logged("importantAction") {
    // ...
}
```

### Rate Limiting

```solidity
contract RateLimited {
    mapping(address => uint256) public lastAction;
    uint256 public cooldown = 1 hours;

    modifier rateLimited() {
        require(
            block.timestamp >= lastAction[msg.sender] + cooldown,
            "Rate limited"
        );
        _;
        lastAction[msg.sender] = block.timestamp;
    }

    function limitedAction() public rateLimited {
        // Can only be called once per cooldown period
    }
}
```

## Best Practices

### 1. Keep Modifiers Simple

```solidity
// Good - single responsibility
modifier onlyOwner() {
    require(msg.sender == owner, "Not owner");
    _;
}

// Avoid - too complex
modifier complexCheck() {
    require(msg.sender == owner, "Not owner");
    require(!paused, "Paused");
    require(balance > 0, "No balance");
    // Too many checks in one modifier
    _;
}
```

### 2. Use Descriptive Names

```solidity
// Good
modifier onlyOwner() { }
modifier whenNotPaused() { }
modifier validAddress(address addr) { }

// Avoid
modifier check1() { }
modifier mod() { }
```

### 3. Order Modifiers Logically

```solidity
// Good - logical order
function action() public onlyOwner whenNotPaused nonReentrant {
    // Access -> State -> Safety
}
```

### 4. Document Modifier Behavior

```solidity
/// @notice Restricts function to contract owner
/// @dev Reverts with "Not owner" if caller is not owner
modifier onlyOwner() {
    require(msg.sender == owner, "Not owner");
    _;
}
```

### 5. Be Careful with State Changes in Modifiers

```solidity
// Caution - modifies state
modifier countCalls() {
    callCount++;  // State change in modifier
    _;
}

// Usually better to be explicit in function
function action() public {
    callCount++;  // Clear state change
    // ...
}
```

## Next Steps

- [Interfaces](interfaces.md) - Contract interfaces
- [Inheritance](inheritance.md) - Contract inheritance
- [Events & Errors](events-errors.md) - Error handling
