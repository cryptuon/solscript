# Contracts

Contracts are the fundamental building blocks of SolScript programs.

## Basic Contract

```solidity
contract MyContract {
    // State variables
    uint256 public value;

    // Constructor
    constructor() {
        value = 0;
    }

    // Functions
    function setValue(uint256 newValue) public {
        value = newValue;
    }
}
```

## Contract Members

### State Variables

State variables are stored on-chain:

```solidity
contract Storage {
    // Simple types
    uint256 public count;
    bool public active;
    address public owner;
    string public name;

    // Complex types
    uint256[] public numbers;
    mapping(address => uint256) public balances;

    // Visibility
    uint256 public publicVar;     // Readable by anyone
    uint256 private privateVar;   // Only this contract
    uint256 internal internalVar; // This + derived contracts
}
```

### Constructor

The constructor runs once at deployment:

```solidity
contract Token {
    string public name;
    uint256 public totalSupply;
    address public owner;

    constructor(string memory _name, uint256 _supply) {
        name = _name;
        totalSupply = _supply;
        owner = msg.sender;
    }
}
```

### Functions

```solidity
contract Functions {
    uint256 public value;

    // State-changing function
    function setValue(uint256 _value) public {
        value = _value;
    }

    // View function (read-only)
    function getValue() public view returns (uint256) {
        return value;
    }

    // Pure function (no state access)
    function add(uint256 a, uint256 b) public pure returns (uint256) {
        return a + b;
    }

    // Payable function (receives SOL)
    function deposit() public payable {
        // msg.value contains the SOL amount
    }

    // External function
    function externalOnly() external {
        // Can only be called from outside
    }
}
```

### Events

Events log data to the blockchain:

```solidity
contract Events {
    event Transfer(
        address indexed from,
        address indexed to,
        uint256 amount
    );

    event Log(string message);

    function transfer(address to, uint256 amount) public {
        // ... transfer logic ...
        emit Transfer(msg.sender, to, amount);
    }
}
```

### Errors

Custom errors for better error handling:

```solidity
contract Errors {
    error InsufficientBalance(uint256 available, uint256 required);
    error Unauthorized(address caller);
    error InvalidAddress();
    error TransferFailed();  // Empty errors work too!

    function withdraw(uint256 amount) public {
        if (balances[msg.sender] < amount) {
            revert InsufficientBalance(balances[msg.sender], amount);
        }
        // ...
    }
}
```

### Structs and Enums Inside Contracts

You can define structs and enums directly inside contracts:

```solidity
contract Token {
    // Struct inside contract
    struct UserBalance {
        uint256 amount;
        uint64 lastUpdate;
        bool frozen;
    }

    // Enum inside contract
    enum TokenStatus {
        Active,
        Paused,
        Deprecated
    }

    // Use them in state
    mapping(address => UserBalance) public balances;
    TokenStatus public status;

    function freeze(address user) public onlyOwner {
        balances[user].frozen = true;
    }

    function pause() public onlyOwner {
        status = TokenStatus.Paused;
    }
}
```

!!! tip "Struct and Enum Placement"
    Structs and enums can be defined either inside or outside contracts.
    Inside contracts provides better encapsulation, while outside allows
    sharing between multiple contracts.

### Modifiers

Reusable function modifiers:

```solidity
contract Modifiers {
    address public owner;

    modifier onlyOwner() {
        require(msg.sender == owner, "Not owner");
        _;
    }

    modifier validAddress(address addr) {
        require(addr != address(0), "Invalid address");
        _;
    }

    function changeOwner(address newOwner)
        public
        onlyOwner
        validAddress(newOwner)
    {
        owner = newOwner;
    }
}
```

## Abstract Contracts

Abstract contracts cannot be deployed directly:

```solidity
abstract contract Base {
    function getValue() public view virtual returns (uint256);

    function doubleValue() public view returns (uint256) {
        return getValue() * 2;
    }
}

contract Derived is Base {
    uint256 private value = 10;

    function getValue() public view override returns (uint256) {
        return value;
    }
}
```

## Inheritance

Contracts can inherit from other contracts:

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

contract Token is Ownable {
    mapping(address => uint256) public balances;

    function mint(address to, uint256 amount) public onlyOwner {
        balances[to] += amount;
    }
}
```

### Multiple Inheritance

```solidity
contract A {
    function foo() public pure virtual returns (string memory) {
        return "A";
    }
}

contract B {
    function bar() public pure virtual returns (string memory) {
        return "B";
    }
}

contract C is A, B {
    function foo() public pure override returns (string memory) {
        return "C";
    }
}
```

## Contract Interactions

### Interface Calls

```solidity
interface IToken {
    function transfer(address to, uint256 amount) external returns (bool);
    function balanceOf(address account) external view returns (uint256);
}

contract Wallet {
    function sendTokens(
        IToken token,
        address to,
        uint256 amount
    ) public {
        require(token.transfer(to, amount), "Transfer failed");
    }
}
```

### Creating Contracts

```solidity
contract Factory {
    Token[] public tokens;

    function createToken(string memory name) public returns (Token) {
        Token token = new Token(name, 1000000);
        tokens.push(token);
        return token;
    }
}
```

## Best Practices

### 1. Use Modifiers for Access Control

```solidity
modifier onlyOwner() {
    require(msg.sender == owner, "Unauthorized");
    _;
}
```

### 2. Emit Events for Important Actions

```solidity
function transfer(address to, uint256 amount) public {
    // ... logic ...
    emit Transfer(msg.sender, to, amount);
}
```

### 3. Use Custom Errors

```solidity
error InsufficientBalance(uint256 available, uint256 required);

// Better than: require(balance >= amount, "Insufficient balance");
if (balance < amount) {
    revert InsufficientBalance(balance, amount);
}
```

### 4. Validate Inputs

```solidity
function setOwner(address newOwner) public onlyOwner {
    require(newOwner != address(0), "Invalid address");
    owner = newOwner;
}
```

### 5. Keep Functions Focused

Each function should do one thing well.

## Next Steps

- [Types](types.md) - Learn about all available types
- [Functions](functions.md) - Deep dive into functions
- [Modifiers](modifiers.md) - Creating reusable modifiers
