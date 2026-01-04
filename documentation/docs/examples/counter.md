# Counter Example

A simple counter contract demonstrating basic state management.

## Contract

```solidity
contract Counter {
    uint256 public count;
    address public owner;

    event Incremented(address indexed by, uint256 newValue);
    event Decremented(address indexed by, uint256 newValue);
    event Reset(address indexed by);

    error Underflow();

    modifier onlyOwner() {
        require(msg.sender == owner, "Not owner");
        _;
    }

    constructor() {
        owner = msg.sender;
        count = 0;
    }

    function increment() public {
        count += 1;
        emit Incremented(msg.sender, count);
    }

    function decrement() public {
        if (count == 0) revert Underflow();
        count -= 1;
        emit Decremented(msg.sender, count);
    }

    function reset() public onlyOwner {
        count = 0;
        emit Reset(msg.sender);
    }

    function incrementBy(uint256 amount) public {
        count += amount;
        emit Incremented(msg.sender, count);
    }

    function getCount() public view returns (uint256) {
        return count;
    }
}
```

## Build

```bash
solscript build counter.sol -o ./build
```

## Test

```typescript
import * as anchor from "@coral-xyz/anchor";
import { expect } from "chai";

describe("Counter", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  it("initializes to zero", async () => {
    // Test initialization
  });

  it("increments correctly", async () => {
    // Test increment
  });

  it("decrements correctly", async () => {
    // Test decrement
  });

  it("prevents underflow", async () => {
    // Test underflow protection
  });

  it("only owner can reset", async () => {
    // Test access control
  });
});
```

## Key Concepts

- **State variables**: `count` and `owner`
- **Events**: Track state changes
- **Modifiers**: Access control with `onlyOwner`
- **Custom errors**: Gas-efficient error handling

## See Also

- [First Contract Tutorial](../guide/first-contract.md)
- [State Variables](../guide/state.md)
