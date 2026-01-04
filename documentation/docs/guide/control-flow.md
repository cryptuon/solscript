# Control Flow

SolScript provides standard control flow statements.

## Conditionals

### If Statement

```solidity
function checkValue(uint256 value) public pure returns (string memory) {
    if (value > 100) {
        return "High";
    }
    return "Low";
}
```

### If-Else

```solidity
function checkValue(uint256 value) public pure returns (string memory) {
    if (value > 100) {
        return "High";
    } else {
        return "Low";
    }
}
```

### If-Else If-Else

```solidity
function classify(uint256 value) public pure returns (string memory) {
    if (value == 0) {
        return "Zero";
    } else if (value < 10) {
        return "Small";
    } else if (value < 100) {
        return "Medium";
    } else {
        return "Large";
    }
}
```

### Ternary Operator

```solidity
function max(uint256 a, uint256 b) public pure returns (uint256) {
    return a > b ? a : b;
}

function abs(int256 value) public pure returns (uint256) {
    return value >= 0 ? uint256(value) : uint256(-value);
}
```

## Loops

### For Loop

```solidity
function sum(uint256[] memory values) public pure returns (uint256) {
    uint256 total = 0;
    for (uint256 i = 0; i < values.length; i++) {
        total += values[i];
    }
    return total;
}
```

### While Loop

```solidity
function factorial(uint256 n) public pure returns (uint256) {
    uint256 result = 1;
    uint256 i = n;
    while (i > 1) {
        result *= i;
        i--;
    }
    return result;
}
```

### Do-While Loop

```solidity
function countDigits(uint256 n) public pure returns (uint256) {
    uint256 count = 0;
    do {
        count++;
        n /= 10;
    } while (n > 0);
    return count;
}
```

## Loop Control

### Break

Exit the loop early:

```solidity
function findIndex(uint256[] memory arr, uint256 target) public pure returns (int256) {
    for (uint256 i = 0; i < arr.length; i++) {
        if (arr[i] == target) {
            return int256(i);  // Found, exit loop
        }
    }
    return -1;  // Not found
}
```

### Continue

Skip to next iteration:

```solidity
function sumEven(uint256[] memory values) public pure returns (uint256) {
    uint256 total = 0;
    for (uint256 i = 0; i < values.length; i++) {
        if (values[i] % 2 != 0) {
            continue;  // Skip odd numbers
        }
        total += values[i];
    }
    return total;
}
```

## Early Returns

```solidity
function validate(uint256 value) public pure returns (bool) {
    if (value == 0) {
        return false;  // Early return
    }

    if (value > 1000) {
        return false;  // Early return
    }

    // More validation...
    return true;
}
```

## Require Statements

Stop execution if condition fails:

```solidity
function withdraw(uint256 amount) public {
    require(amount > 0, "Amount must be positive");
    require(amount <= balance, "Insufficient balance");
    require(msg.sender == owner, "Not authorized");

    balance -= amount;
    // Transfer...
}
```

## Assert Statements

For invariant checking:

```solidity
function transfer(address to, uint256 amount) public {
    uint256 totalBefore = balances[msg.sender] + balances[to];

    balances[msg.sender] -= amount;
    balances[to] += amount;

    // Invariant: total should remain the same
    assert(balances[msg.sender] + balances[to] == totalBefore);
}
```

## Revert Statements

Explicit revert with custom errors:

```solidity
error InvalidAmount(uint256 provided, uint256 minimum);
error Unauthorized(address caller);

function deposit(uint256 amount) public {
    if (amount < minimumDeposit) {
        revert InvalidAmount(amount, minimumDeposit);
    }

    if (msg.sender != authorized) {
        revert Unauthorized(msg.sender);
    }

    // Process deposit...
}
```

## Patterns

### Guard Pattern

Check conditions early:

```solidity
function transfer(address to, uint256 amount) public {
    // Guards at the top
    require(to != address(0), "Invalid recipient");
    require(amount > 0, "Invalid amount");
    require(balances[msg.sender] >= amount, "Insufficient balance");

    // Logic after guards pass
    balances[msg.sender] -= amount;
    balances[to] += amount;

    emit Transfer(msg.sender, to, amount);
}
```

### State Machine

```solidity
enum State { Created, Active, Completed, Cancelled }

State public state;

modifier inState(State expected) {
    require(state == expected, "Invalid state");
    _;
}

function activate() public inState(State.Created) {
    state = State.Active;
}

function complete() public inState(State.Active) {
    state = State.Completed;
}

function cancel() public {
    require(state != State.Completed, "Cannot cancel completed");
    state = State.Cancelled;
}
```

### Bounded Loops

Always bound your loops:

```solidity
// Good - bounded by array length (which should be reasonable)
function sumAll(uint256[] memory values) public pure returns (uint256) {
    uint256 total = 0;
    for (uint256 i = 0; i < values.length; i++) {
        total += values[i];
    }
    return total;
}

// Good - explicit maximum
uint256 constant MAX_ITERATIONS = 100;

function processItems() public {
    uint256 processed = 0;
    uint256 i = startIndex;

    while (i < items.length && processed < MAX_ITERATIONS) {
        // Process item
        processed++;
        i++;
    }

    startIndex = i;  // Save progress for next call
}
```

### Short-Circuit Evaluation

```solidity
// Second condition not evaluated if first is false
if (user != address(0) && balances[user] > 0) {
    // Safe to access balances
}

// Second condition not evaluated if first is true
if (isAdmin || msg.sender == owner) {
    // Authorized
}
```

## Common Mistakes

### Unbounded Loops

```solidity
// BAD - could run out of gas
function sendToAll() public {
    for (uint256 i = 0; i < recipients.length; i++) {
        // If recipients grows large, this will fail
        send(recipients[i], amount);
    }
}

// GOOD - process in batches
function sendBatch(uint256 start, uint256 count) public {
    uint256 end = start + count;
    if (end > recipients.length) end = recipients.length;

    for (uint256 i = start; i < end; i++) {
        send(recipients[i], amount);
    }
}
```

### Off-by-One Errors

```solidity
// BAD - skips last element
for (uint256 i = 0; i < arr.length - 1; i++) { }

// GOOD - includes all elements
for (uint256 i = 0; i < arr.length; i++) { }
```

### Integer Overflow in Loops

```solidity
// Be careful with uint256 underflow
for (uint256 i = arr.length - 1; i >= 0; i--) {
    // i will underflow to max uint256 when it reaches 0!
}

// GOOD - safe reverse iteration
for (uint256 i = arr.length; i > 0; i--) {
    uint256 index = i - 1;
    // Use index
}
```

## Next Steps

- [Events & Errors](events-errors.md) - Logging and error handling
- [Modifiers](modifiers.md) - Reusable conditions
- [Interfaces](interfaces.md) - Contract interfaces
