# Interfaces

Interfaces define contracts without implementation, enabling interoperability.

## Defining Interfaces

```solidity
interface IERC20 {
    function totalSupply() external view returns (uint256);
    function balanceOf(address account) external view returns (uint256);
    function transfer(address to, uint256 amount) external returns (bool);
    function allowance(address owner, address spender) external view returns (uint256);
    function approve(address spender, uint256 amount) external returns (bool);
    function transferFrom(address from, address to, uint256 amount) external returns (bool);

    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);
}
```

## Interface Rules

Interfaces can contain:

- Function signatures (no implementation)
- Events
- Errors
- Struct and enum definitions

Interfaces cannot contain:

- Function implementations
- State variables
- Constructors
- Modifiers

```solidity
interface IVault {
    // Events - allowed
    event Deposit(address indexed user, uint256 amount);
    event Withdrawal(address indexed user, uint256 amount);

    // Errors - allowed
    error InsufficientBalance(uint256 available, uint256 required);

    // Structs - allowed
    struct Position {
        uint256 amount;
        uint256 timestamp;
    }

    // Enums - allowed
    enum Status { Active, Paused, Closed }

    // Function signatures - allowed (no body)
    function deposit(uint256 amount) external;
    function withdraw(uint256 amount) external;
    function getPosition(address user) external view returns (Position memory);
}
```

## Implementing Interfaces

```solidity
contract Token is IERC20 {
    string public name;
    uint256 public override totalSupply;
    mapping(address => uint256) private balances;
    mapping(address => mapping(address => uint256)) private allowances;

    constructor(string memory _name, uint256 _supply) {
        name = _name;
        totalSupply = _supply;
        balances[msg.sender] = _supply;
    }

    function balanceOf(address account) external view override returns (uint256) {
        return balances[account];
    }

    function transfer(address to, uint256 amount) external override returns (bool) {
        require(balances[msg.sender] >= amount, "Insufficient balance");
        balances[msg.sender] -= amount;
        balances[to] += amount;
        emit Transfer(msg.sender, to, amount);
        return true;
    }

    function allowance(address owner, address spender) external view override returns (uint256) {
        return allowances[owner][spender];
    }

    function approve(address spender, uint256 amount) external override returns (bool) {
        allowances[msg.sender][spender] = amount;
        emit Approval(msg.sender, spender, amount);
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external override returns (bool) {
        require(allowances[from][msg.sender] >= amount, "Not allowed");
        require(balances[from] >= amount, "Insufficient balance");

        allowances[from][msg.sender] -= amount;
        balances[from] -= amount;
        balances[to] += amount;

        emit Transfer(from, to, amount);
        return true;
    }
}
```

## Using Interfaces

### Calling External Contracts

```solidity
contract Wallet {
    function sendTokens(
        IERC20 token,
        address to,
        uint256 amount
    ) public {
        bool success = token.transfer(to, amount);
        require(success, "Transfer failed");
    }

    function checkBalance(
        IERC20 token,
        address account
    ) public view returns (uint256) {
        return token.balanceOf(account);
    }
}
```

### Interface as Parameter Type

```solidity
contract Exchange {
    function swap(
        IERC20 tokenIn,
        IERC20 tokenOut,
        uint256 amountIn
    ) public returns (uint256 amountOut) {
        // Transfer tokens in
        tokenIn.transferFrom(msg.sender, address(this), amountIn);

        // Calculate output
        amountOut = calculateOutput(amountIn);

        // Transfer tokens out
        tokenOut.transfer(msg.sender, amountOut);
    }
}
```

### Interface as Return Type

```solidity
contract Factory {
    mapping(uint256 => IERC20) public tokens;

    function getToken(uint256 id) external view returns (IERC20) {
        return tokens[id];
    }
}
```

## Interface Inheritance

```solidity
interface IERC20Basic {
    function totalSupply() external view returns (uint256);
    function balanceOf(address account) external view returns (uint256);
    function transfer(address to, uint256 amount) external returns (bool);
}

interface IERC20 is IERC20Basic {
    function allowance(address owner, address spender) external view returns (uint256);
    function approve(address spender, uint256 amount) external returns (bool);
    function transferFrom(address from, address to, uint256 amount) external returns (bool);
}

interface IERC20Metadata is IERC20 {
    function name() external view returns (string memory);
    function symbol() external view returns (string memory);
    function decimals() external view returns (uint8);
}
```

## Multiple Interface Implementation

```solidity
interface IOwnable {
    function owner() external view returns (address);
    function transferOwnership(address newOwner) external;
}

interface IPausable {
    function paused() external view returns (bool);
    function pause() external;
    function unpause() external;
}

contract Token is IERC20, IOwnable, IPausable {
    // Implement all interface functions
    address public override owner;
    bool public override paused;

    function transferOwnership(address newOwner) external override {
        require(msg.sender == owner, "Not owner");
        owner = newOwner;
    }

    function pause() external override {
        require(msg.sender == owner, "Not owner");
        paused = true;
    }

    function unpause() external override {
        require(msg.sender == owner, "Not owner");
        paused = false;
    }

    // ... IERC20 implementations
}
```

## Common Interface Patterns

### Callback Interface

```solidity
interface ICallback {
    function onTokenReceived(
        address sender,
        uint256 amount,
        bytes calldata data
    ) external returns (bytes4);
}

contract Token {
    function safeTransfer(
        address to,
        uint256 amount,
        bytes memory data
    ) public {
        transfer(to, amount);

        // Check if recipient is a contract
        if (isContract(to)) {
            bytes4 response = ICallback(to).onTokenReceived(
                msg.sender,
                amount,
                data
            );
            require(
                response == ICallback.onTokenReceived.selector,
                "Invalid callback"
            );
        }
    }
}
```

### Factory Interface

```solidity
interface IFactory {
    event Created(address indexed instance);

    function create(bytes calldata params) external returns (address);
    function getInstance(uint256 id) external view returns (address);
    function getInstanceCount() external view returns (uint256);
}
```

### Registry Interface

```solidity
interface IRegistry {
    function register(address instance) external;
    function unregister(address instance) external;
    function isRegistered(address instance) external view returns (bool);
    function getAll() external view returns (address[] memory);
}
```

### Oracle Interface

```solidity
interface IPriceOracle {
    function getPrice(address token) external view returns (uint256);
    function updatePrice(address token, uint256 price) external;

    event PriceUpdated(address indexed token, uint256 price);
}
```

## Type Casting with Interfaces

```solidity
contract Example {
    function interact(address tokenAddress) public {
        // Cast address to interface
        IERC20 token = IERC20(tokenAddress);

        // Now can call interface methods
        uint256 balance = token.balanceOf(msg.sender);
    }

    function getAddress(IERC20 token) public pure returns (address) {
        // Cast interface to address
        return address(token);
    }
}
```

## Best Practices

### 1. Use Standard Interfaces

```solidity
// Good - use established standards
interface IERC20 { ... }
interface IERC721 { ... }
interface IERC1155 { ... }
```

### 2. Keep Interfaces Focused

```solidity
// Good - single purpose interfaces
interface IOwnable {
    function owner() external view returns (address);
    function transferOwnership(address newOwner) external;
}

interface IPausable {
    function paused() external view returns (bool);
    function pause() external;
    function unpause() external;
}

// Avoid - kitchen sink interface
interface IEverything {
    // Too many unrelated functions
}
```

### 3. Document Interface Functions

```solidity
interface IVault {
    /// @notice Deposits tokens into the vault
    /// @param amount The amount to deposit
    /// @return shares The number of shares minted
    function deposit(uint256 amount) external returns (uint256 shares);

    /// @notice Withdraws tokens from the vault
    /// @param shares The number of shares to burn
    /// @return amount The amount of tokens returned
    function withdraw(uint256 shares) external returns (uint256 amount);
}
```

### 4. Version Interfaces

```solidity
interface IVaultV1 {
    function deposit(uint256 amount) external;
}

interface IVaultV2 is IVaultV1 {
    function depositWithReferral(uint256 amount, address referrer) external;
}
```

### 5. Include Events and Errors

```solidity
interface IMarket {
    // Events for off-chain tracking
    event Listed(uint256 indexed id, address indexed seller, uint256 price);
    event Sold(uint256 indexed id, address indexed buyer);

    // Errors for clear failure reasons
    error NotListed(uint256 id);
    error InsufficientPayment(uint256 required, uint256 provided);

    // Functions
    function list(uint256 id, uint256 price) external;
    function buy(uint256 id) external payable;
}
```

## Next Steps

- [Inheritance](inheritance.md) - Contract inheritance
- [Contracts](contracts.md) - Contract fundamentals
- [Functions](functions.md) - Function details
