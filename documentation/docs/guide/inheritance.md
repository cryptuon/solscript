# Inheritance

Contracts can inherit from other contracts to reuse code.

## Basic Inheritance

```solidity
contract Ownable {
    address public owner;

    constructor() {
        owner = msg.sender;
    }

    modifier onlyOwner() {
        require(msg.sender == owner, "Not owner");
        _;
    }

    function transferOwnership(address newOwner) public onlyOwner {
        require(newOwner != address(0), "Invalid address");
        owner = newOwner;
    }
}

contract Token is Ownable {
    mapping(address => uint256) public balances;

    // Inherits owner, onlyOwner, transferOwnership
    function mint(address to, uint256 amount) public onlyOwner {
        balances[to] += amount;
    }
}
```

## Constructor Inheritance

### Calling Parent Constructor

```solidity
contract Base {
    uint256 public value;

    constructor(uint256 _value) {
        value = _value;
    }
}

// Method 1: In inheritance list
contract Derived1 is Base(100) {
    // Base constructor called with 100
}

// Method 2: In derived constructor
contract Derived2 is Base {
    constructor(uint256 _value) Base(_value) {
        // Additional initialization
    }
}

// Method 3: With own parameters
contract Derived3 is Base {
    string public name;

    constructor(string memory _name, uint256 _value) Base(_value) {
        name = _name;
    }
}
```

## Function Overriding

### Virtual and Override

```solidity
contract Base {
    function getValue() public view virtual returns (uint256) {
        return 10;
    }

    function getMessage() public pure virtual returns (string memory) {
        return "Base";
    }
}

contract Derived is Base {
    function getValue() public view override returns (uint256) {
        return 20;
    }

    function getMessage() public pure override returns (string memory) {
        return "Derived";
    }
}
```

### Calling Parent Functions

```solidity
contract Base {
    uint256 public value;

    function increment() public virtual {
        value += 1;
    }
}

contract Derived is Base {
    function increment() public override {
        // Call parent implementation
        super.increment();
        // Then add more
        value += 1;  // Total: +2
    }
}
```

### Explicit Parent Call

```solidity
contract A {
    function foo() public virtual returns (string memory) {
        return "A";
    }
}

contract B is A {
    function foo() public virtual override returns (string memory) {
        return "B";
    }
}

contract C is A, B {
    function foo() public override(A, B) returns (string memory) {
        // Call specific parent
        return A.foo();  // Returns "A"
        // Or: return B.foo();  // Returns "B"
        // Or: return super.foo();  // Returns "B" (most derived)
    }
}
```

## Multiple Inheritance

```solidity
contract Ownable {
    address public owner;

    constructor() {
        owner = msg.sender;
    }

    modifier onlyOwner() {
        require(msg.sender == owner, "Not owner");
        _;
    }
}

contract Pausable {
    bool public paused;

    modifier whenNotPaused() {
        require(!paused, "Paused");
        _;
    }
}

// Inherit from multiple contracts
contract Token is Ownable, Pausable {
    mapping(address => uint256) public balances;

    function transfer(address to, uint256 amount)
        public
        whenNotPaused
    {
        // Uses Pausable modifier
        balances[msg.sender] -= amount;
        balances[to] += amount;
    }

    function pause() public onlyOwner {
        // Uses Ownable modifier
        paused = true;
    }
}
```

### Inheritance Order

Order matters - list from most base to most derived:

```solidity
// Correct order
contract D is A, B, C {
    // A is most base, C is most derived
}

// If B and C both override A's function,
// super.foo() calls C's implementation
```

## Abstract Contracts

```solidity
abstract contract Base {
    // Abstract function - no implementation
    function getValue() public view virtual returns (uint256);

    // Concrete function - has implementation
    function getDoubleValue() public view returns (uint256) {
        return getValue() * 2;
    }
}

contract Derived is Base {
    uint256 private value = 10;

    // Must implement abstract function
    function getValue() public view override returns (uint256) {
        return value;
    }
}
```

## Visibility in Inheritance

### Internal Access

```solidity
contract Base {
    uint256 private privateVar;    // Only in Base
    uint256 internal internalVar;  // Base + derived
    uint256 public publicVar;      // Anyone

    function _internalFunc() internal view returns (uint256) {
        return internalVar;
    }

    function _privateFunc() private view returns (uint256) {
        return privateVar;
    }
}

contract Derived is Base {
    function test() public view returns (uint256) {
        // return privateVar;      // Error - private
        // return _privateFunc();  // Error - private

        return internalVar;        // OK - internal
        // return _internalFunc(); // OK - internal
    }
}
```

### Protected Pattern

```solidity
contract Base {
    // Internal setter for derived contracts
    function _setValue(uint256 value) internal {
        _value = value;
    }

    // Public getter for everyone
    function getValue() public view returns (uint256) {
        return _value;
    }

    uint256 private _value;
}

contract Derived is Base {
    function updateValue(uint256 newValue) public {
        _setValue(newValue);  // Can call internal
    }
}
```

## State Variable Inheritance

```solidity
contract Base {
    uint256 public baseValue;
    mapping(address => uint256) internal balances;
}

contract Derived is Base {
    // Additional state
    uint256 public derivedValue;

    function getBalance(address user) public view returns (uint256) {
        return balances[user];  // Access inherited mapping
    }
}
```

### Shadowing Prevention

```solidity
contract Base {
    uint256 public value;
}

contract Derived is Base {
    // This would shadow - not allowed
    // uint256 public value;  // Error!

    // Use different name
    uint256 public derivedValue;
}
```

## Modifier Inheritance

```solidity
contract Base {
    bool internal locked;

    modifier nonReentrant() virtual {
        require(!locked, "Reentrant");
        locked = true;
        _;
        locked = false;
    }
}

contract Derived is Base {
    // Use inherited modifier
    function withdraw() public nonReentrant {
        // ...
    }

    // Or override it
    modifier nonReentrant() override {
        require(!locked, "Derived: Reentrant");
        locked = true;
        _;
        locked = false;
    }
}
```

## Interface Implementation

```solidity
interface IERC20 {
    function transfer(address to, uint256 amount) external returns (bool);
    function balanceOf(address account) external view returns (uint256);
}

abstract contract ERC20Base is IERC20 {
    mapping(address => uint256) internal _balances;

    function balanceOf(address account) public view virtual override returns (uint256) {
        return _balances[account];
    }

    // Leave transfer abstract
    function transfer(address to, uint256 amount) public virtual override returns (bool);
}

contract Token is ERC20Base {
    function transfer(address to, uint256 amount) public override returns (bool) {
        _balances[msg.sender] -= amount;
        _balances[to] += amount;
        return true;
    }
}
```

## Diamond Inheritance

```solidity
contract A {
    function foo() public pure virtual returns (string memory) {
        return "A";
    }
}

contract B is A {
    function foo() public pure virtual override returns (string memory) {
        return "B";
    }
}

contract C is A {
    function foo() public pure virtual override returns (string memory) {
        return "C";
    }
}

// Diamond: D inherits from both B and C, which both inherit from A
contract D is B, C {
    // Must override and specify all parents
    function foo() public pure override(B, C) returns (string memory) {
        return super.foo();  // Calls C.foo() (rightmost)
    }
}
```

## Best Practices

### 1. Use Composition Over Deep Inheritance

```solidity
// Good - shallow hierarchy
contract Token is Ownable, Pausable {
    // Combines focused base contracts
}

// Avoid - deep hierarchy
contract A { }
contract B is A { }
contract C is B { }
contract D is C { }
contract E is D { }  // Too deep
```

### 2. Mark Functions Virtual Explicitly

```solidity
contract Base {
    // Explicitly virtual - can be overridden
    function getValue() public view virtual returns (uint256) {
        return 10;
    }

    // Not virtual - cannot be overridden
    function getFixed() public pure returns (uint256) {
        return 100;
    }
}
```

### 3. Use Abstract Contracts for Shared Logic

```solidity
abstract contract TokenBase {
    mapping(address => uint256) internal _balances;

    function _transfer(address from, address to, uint256 amount) internal virtual {
        _balances[from] -= amount;
        _balances[to] += amount;
    }

    // Force derived to implement
    function transfer(address to, uint256 amount) public virtual returns (bool);
}
```

### 4. Document Inheritance Requirements

```solidity
/// @title Base token with ownership
/// @notice Derived contracts must implement _beforeTransfer hook
abstract contract TokenBase {
    /// @dev Override to add transfer validation
    function _beforeTransfer(address from, address to, uint256 amount) internal virtual;
}
```

### 5. Be Careful with Constructor Order

```solidity
contract A {
    constructor() { /* runs first */ }
}

contract B is A {
    constructor() { /* runs second */ }
}

contract C is A, B {
    constructor() { /* runs last */ }
}
// Order: A -> B -> C
```

## Next Steps

- [Interfaces](interfaces.md) - Contract interfaces
- [Contracts](contracts.md) - Contract fundamentals
- [Modifiers](modifiers.md) - Reusable function conditions
