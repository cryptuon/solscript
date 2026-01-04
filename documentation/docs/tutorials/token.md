# Build a Token

Create a complete token contract with transfers, approvals, and minting.

## Prerequisites

- SolScript installed
- Basic understanding of tokens

## What We'll Build

A token with:
- Minting by owner
- Transfers between users
- Approval and transferFrom
- Total supply tracking
- Events for all operations

## Step 1: Contract Structure

Create `token.sol`:

```solidity
contract Token {
    // Token metadata
    string public name;
    string public symbol;
    uint8 public decimals = 9;

    // Token state
    uint256 public totalSupply;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    // Owner
    address public owner;

    // Events
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);
    event Mint(address indexed to, uint256 amount);

    // Errors
    error InsufficientBalance(uint256 available, uint256 required);
    error InsufficientAllowance(uint256 available, uint256 required);
    error Unauthorized();
    error InvalidAddress();

    modifier onlyOwner() {
        require(msg.sender == owner, "Not owner");
        _;
    }
}
```

## Step 2: Constructor

Initialize the token:

```solidity
constructor(string memory _name, string memory _symbol, uint256 initialSupply) {
    name = _name;
    symbol = _symbol;
    owner = msg.sender;

    // Mint initial supply to deployer
    if (initialSupply > 0) {
        _mint(msg.sender, initialSupply);
    }
}
```

## Step 3: Internal Mint Function

```solidity
function _mint(address to, uint256 amount) internal {
    if (to == address(0)) revert InvalidAddress();

    totalSupply += amount;
    balanceOf[to] += amount;

    emit Transfer(address(0), to, amount);
}
```

## Step 4: Public Mint (Owner Only)

```solidity
function mint(address to, uint256 amount) public onlyOwner {
    _mint(to, amount);
    emit Mint(to, amount);
}
```

## Step 5: Transfer Function

```solidity
function transfer(address to, uint256 amount) public returns (bool) {
    return _transfer(msg.sender, to, amount);
}

function _transfer(address from, address to, uint256 amount) internal returns (bool) {
    if (to == address(0)) revert InvalidAddress();
    if (balanceOf[from] < amount) {
        revert InsufficientBalance(balanceOf[from], amount);
    }

    balanceOf[from] -= amount;
    balanceOf[to] += amount;

    emit Transfer(from, to, amount);
    return true;
}
```

## Step 6: Approval System

```solidity
function approve(address spender, uint256 amount) public returns (bool) {
    allowance[msg.sender][spender] = amount;
    emit Approval(msg.sender, spender, amount);
    return true;
}

function transferFrom(address from, address to, uint256 amount) public returns (bool) {
    uint256 currentAllowance = allowance[from][msg.sender];

    if (currentAllowance < amount) {
        revert InsufficientAllowance(currentAllowance, amount);
    }

    // Decrease allowance
    allowance[from][msg.sender] = currentAllowance - amount;

    return _transfer(from, to, amount);
}
```

## Step 7: Utility Functions

```solidity
function increaseAllowance(address spender, uint256 addedValue) public returns (bool) {
    allowance[msg.sender][spender] += addedValue;
    emit Approval(msg.sender, spender, allowance[msg.sender][spender]);
    return true;
}

function decreaseAllowance(address spender, uint256 subtractedValue) public returns (bool) {
    uint256 currentAllowance = allowance[msg.sender][spender];
    require(currentAllowance >= subtractedValue, "Decreased below zero");

    allowance[msg.sender][spender] = currentAllowance - subtractedValue;
    emit Approval(msg.sender, spender, allowance[msg.sender][spender]);
    return true;
}
```

## Complete Contract

```solidity
contract Token {
    // Metadata
    string public name;
    string public symbol;
    uint8 public decimals = 9;

    // State
    uint256 public totalSupply;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    address public owner;

    // Events
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);
    event Mint(address indexed to, uint256 amount);

    // Errors
    error InsufficientBalance(uint256 available, uint256 required);
    error InsufficientAllowance(uint256 available, uint256 required);
    error Unauthorized();
    error InvalidAddress();

    modifier onlyOwner() {
        require(msg.sender == owner, "Not owner");
        _;
    }

    constructor(string memory _name, string memory _symbol, uint256 initialSupply) {
        name = _name;
        symbol = _symbol;
        owner = msg.sender;

        if (initialSupply > 0) {
            _mint(msg.sender, initialSupply);
        }
    }

    function transfer(address to, uint256 amount) public returns (bool) {
        return _transfer(msg.sender, to, amount);
    }

    function approve(address spender, uint256 amount) public returns (bool) {
        allowance[msg.sender][spender] = amount;
        emit Approval(msg.sender, spender, amount);
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) public returns (bool) {
        uint256 currentAllowance = allowance[from][msg.sender];

        if (currentAllowance < amount) {
            revert InsufficientAllowance(currentAllowance, amount);
        }

        allowance[from][msg.sender] = currentAllowance - amount;
        return _transfer(from, to, amount);
    }

    function mint(address to, uint256 amount) public onlyOwner {
        _mint(to, amount);
        emit Mint(to, amount);
    }

    function increaseAllowance(address spender, uint256 addedValue) public returns (bool) {
        allowance[msg.sender][spender] += addedValue;
        emit Approval(msg.sender, spender, allowance[msg.sender][spender]);
        return true;
    }

    function decreaseAllowance(address spender, uint256 subtractedValue) public returns (bool) {
        uint256 currentAllowance = allowance[msg.sender][spender];
        require(currentAllowance >= subtractedValue, "Decreased below zero");
        allowance[msg.sender][spender] = currentAllowance - subtractedValue;
        emit Approval(msg.sender, spender, allowance[msg.sender][spender]);
        return true;
    }

    function _transfer(address from, address to, uint256 amount) internal returns (bool) {
        if (to == address(0)) revert InvalidAddress();
        if (balanceOf[from] < amount) {
            revert InsufficientBalance(balanceOf[from], amount);
        }

        balanceOf[from] -= amount;
        balanceOf[to] += amount;

        emit Transfer(from, to, amount);
        return true;
    }

    function _mint(address to, uint256 amount) internal {
        if (to == address(0)) revert InvalidAddress();

        totalSupply += amount;
        balanceOf[to] += amount;

        emit Transfer(address(0), to, amount);
    }
}
```

## Build and Deploy

```bash
# Build
solscript build token.sol -o ./build

# Deploy to devnet
solscript deploy token.sol --network devnet
```

## Testing

```typescript
import * as anchor from "@coral-xyz/anchor";

describe("Token", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  it("mints initial supply", async () => {
    // Deploy with initial supply
    const initialSupply = 1_000_000;
    // ... deployment code
  });

  it("transfers tokens", async () => {
    // Test transfer functionality
  });

  it("handles approvals", async () => {
    // Test approve and transferFrom
  });
});
```

## Extensions

Ideas for extending this token:

1. **Burnable**: Add a `burn` function
2. **Pausable**: Add pause/unpause functionality
3. **Capped**: Add maximum supply limit
4. **Snapshot**: Track historical balances

## Next Steps

- [Build an NFT Marketplace](nft-marketplace.md)
- [Build an Escrow](escrow.md)
