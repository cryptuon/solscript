# Build an Escrow

Create a trustless escrow contract for secure transactions.

## Prerequisites

- SolScript installed
- Understanding of state machines
- Completed previous tutorials

## What We'll Build

An escrow system with:
- Deposit funds
- Release to recipient
- Refund to depositor
- Arbiter for disputes
- Time-based expiry

## Step 1: State Machine

```solidity
contract Escrow {
    // Escrow states
    enum State {
        Created,    // Initial state
        Funded,     // Buyer deposited funds
        Released,   // Funds released to seller
        Refunded,   // Funds returned to buyer
        Disputed,   // Under arbiter review
        Resolved    // Dispute resolved
    }

    // Escrow details
    struct EscrowDetails {
        address buyer;
        address seller;
        address arbiter;
        uint256 amount;
        uint256 deadline;
        State state;
        string description;
    }

    // State
    mapping(uint256 => EscrowDetails) public escrows;
    uint256 public escrowCount;
    uint256 public arbiterFee = 100; // 1% in basis points

    // Events
    event EscrowCreated(uint256 indexed id, address buyer, address seller, uint256 amount);
    event EscrowFunded(uint256 indexed id);
    event EscrowReleased(uint256 indexed id);
    event EscrowRefunded(uint256 indexed id);
    event DisputeRaised(uint256 indexed id, address by);
    event DisputeResolved(uint256 indexed id, bool releasedToSeller);

    // Errors
    error InvalidState(State current, State required);
    error NotAuthorized();
    error InsufficientFunds();
    error DeadlineNotReached();
    error DeadlinePassed();
}
```

## Step 2: Modifiers

```solidity
modifier inState(uint256 escrowId, State required) {
    if (escrows[escrowId].state != required) {
        revert InvalidState(escrows[escrowId].state, required);
    }
    _;
}

modifier onlyBuyer(uint256 escrowId) {
    if (msg.sender != escrows[escrowId].buyer) revert NotAuthorized();
    _;
}

modifier onlySeller(uint256 escrowId) {
    if (msg.sender != escrows[escrowId].seller) revert NotAuthorized();
    _;
}

modifier onlyArbiter(uint256 escrowId) {
    if (msg.sender != escrows[escrowId].arbiter) revert NotAuthorized();
    _;
}

modifier onlyParty(uint256 escrowId) {
    EscrowDetails storage e = escrows[escrowId];
    if (msg.sender != e.buyer && msg.sender != e.seller) {
        revert NotAuthorized();
    }
    _;
}
```

## Step 3: Create Escrow

```solidity
function createEscrow(
    address seller,
    address arbiter,
    uint256 deadline,
    string memory description
) public payable returns (uint256) {
    require(seller != address(0), "Invalid seller");
    require(arbiter != address(0), "Invalid arbiter");
    require(msg.value > 0, "Must send funds");
    require(deadline > block.timestamp, "Deadline must be future");

    escrowCount++;

    escrows[escrowCount] = EscrowDetails({
        buyer: msg.sender,
        seller: seller,
        arbiter: arbiter,
        amount: msg.value,
        deadline: deadline,
        state: State.Funded,
        description: description
    });

    emit EscrowCreated(escrowCount, msg.sender, seller, msg.value);
    emit EscrowFunded(escrowCount);

    return escrowCount;
}
```

## Step 4: Release Funds

Buyer releases funds to seller:

```solidity
function release(uint256 escrowId)
    public
    onlyBuyer(escrowId)
    inState(escrowId, State.Funded)
{
    EscrowDetails storage e = escrows[escrowId];
    e.state = State.Released;

    // Transfer funds to seller
    // (Solana transfer implementation)

    emit EscrowReleased(escrowId);
}
```

## Step 5: Request Refund

Seller agrees to refund or deadline passed:

```solidity
function refund(uint256 escrowId)
    public
    inState(escrowId, State.Funded)
{
    EscrowDetails storage e = escrows[escrowId];

    // Seller can voluntarily refund
    if (msg.sender == e.seller) {
        e.state = State.Refunded;
        // Transfer funds back to buyer
        emit EscrowRefunded(escrowId);
        return;
    }

    // Buyer can claim refund after deadline
    if (msg.sender == e.buyer) {
        if (block.timestamp < e.deadline) {
            revert DeadlineNotReached();
        }
        e.state = State.Refunded;
        // Transfer funds back to buyer
        emit EscrowRefunded(escrowId);
        return;
    }

    revert NotAuthorized();
}
```

## Step 6: Dispute Handling

```solidity
function raiseDispute(uint256 escrowId)
    public
    onlyParty(escrowId)
    inState(escrowId, State.Funded)
{
    escrows[escrowId].state = State.Disputed;
    emit DisputeRaised(escrowId, msg.sender);
}

function resolveDispute(uint256 escrowId, bool releaseToSeller)
    public
    onlyArbiter(escrowId)
    inState(escrowId, State.Disputed)
{
    EscrowDetails storage e = escrows[escrowId];
    e.state = State.Resolved;

    // Calculate arbiter fee
    uint256 fee = (e.amount * arbiterFee) / 10000;
    uint256 remaining = e.amount - fee;

    if (releaseToSeller) {
        // Transfer remaining to seller
        // Transfer fee to arbiter
    } else {
        // Transfer remaining to buyer
        // Transfer fee to arbiter
    }

    emit DisputeResolved(escrowId, releaseToSeller);
}
```

## Step 7: View Functions

```solidity
function getEscrow(uint256 escrowId) public view returns (
    address buyer,
    address seller,
    address arbiter,
    uint256 amount,
    uint256 deadline,
    State state,
    string memory description
) {
    EscrowDetails storage e = escrows[escrowId];
    return (
        e.buyer,
        e.seller,
        e.arbiter,
        e.amount,
        e.deadline,
        e.state,
        e.description
    );
}

function isExpired(uint256 escrowId) public view returns (bool) {
    return block.timestamp >= escrows[escrowId].deadline;
}

function getTimeRemaining(uint256 escrowId) public view returns (uint256) {
    EscrowDetails storage e = escrows[escrowId];
    if (block.timestamp >= e.deadline) return 0;
    return e.deadline - block.timestamp;
}
```

## Complete Contract

```solidity
contract Escrow {
    enum State { Created, Funded, Released, Refunded, Disputed, Resolved }

    struct EscrowDetails {
        address buyer;
        address seller;
        address arbiter;
        uint256 amount;
        uint256 deadline;
        State state;
        string description;
    }

    mapping(uint256 => EscrowDetails) public escrows;
    uint256 public escrowCount;
    uint256 public arbiterFee = 100;

    event EscrowCreated(uint256 indexed id, address buyer, address seller, uint256 amount);
    event EscrowFunded(uint256 indexed id);
    event EscrowReleased(uint256 indexed id);
    event EscrowRefunded(uint256 indexed id);
    event DisputeRaised(uint256 indexed id, address by);
    event DisputeResolved(uint256 indexed id, bool releasedToSeller);

    error InvalidState(State current, State required);
    error NotAuthorized();
    error DeadlineNotReached();

    modifier inState(uint256 escrowId, State required) {
        if (escrows[escrowId].state != required) {
            revert InvalidState(escrows[escrowId].state, required);
        }
        _;
    }

    modifier onlyBuyer(uint256 escrowId) {
        if (msg.sender != escrows[escrowId].buyer) revert NotAuthorized();
        _;
    }

    modifier onlySeller(uint256 escrowId) {
        if (msg.sender != escrows[escrowId].seller) revert NotAuthorized();
        _;
    }

    modifier onlyArbiter(uint256 escrowId) {
        if (msg.sender != escrows[escrowId].arbiter) revert NotAuthorized();
        _;
    }

    modifier onlyParty(uint256 escrowId) {
        EscrowDetails storage e = escrows[escrowId];
        if (msg.sender != e.buyer && msg.sender != e.seller) revert NotAuthorized();
        _;
    }

    function createEscrow(
        address seller,
        address arbiter,
        uint256 deadline,
        string memory description
    ) public payable returns (uint256) {
        require(seller != address(0), "Invalid seller");
        require(arbiter != address(0), "Invalid arbiter");
        require(msg.value > 0, "Must send funds");
        require(deadline > block.timestamp, "Invalid deadline");

        escrowCount++;
        escrows[escrowCount] = EscrowDetails({
            buyer: msg.sender,
            seller: seller,
            arbiter: arbiter,
            amount: msg.value,
            deadline: deadline,
            state: State.Funded,
            description: description
        });

        emit EscrowCreated(escrowCount, msg.sender, seller, msg.value);
        emit EscrowFunded(escrowCount);
        return escrowCount;
    }

    function release(uint256 escrowId) public onlyBuyer(escrowId) inState(escrowId, State.Funded) {
        escrows[escrowId].state = State.Released;
        emit EscrowReleased(escrowId);
    }

    function refund(uint256 escrowId) public inState(escrowId, State.Funded) {
        EscrowDetails storage e = escrows[escrowId];

        if (msg.sender == e.seller) {
            e.state = State.Refunded;
            emit EscrowRefunded(escrowId);
            return;
        }

        if (msg.sender == e.buyer) {
            if (block.timestamp < e.deadline) revert DeadlineNotReached();
            e.state = State.Refunded;
            emit EscrowRefunded(escrowId);
            return;
        }

        revert NotAuthorized();
    }

    function raiseDispute(uint256 escrowId) public onlyParty(escrowId) inState(escrowId, State.Funded) {
        escrows[escrowId].state = State.Disputed;
        emit DisputeRaised(escrowId, msg.sender);
    }

    function resolveDispute(uint256 escrowId, bool releaseToSeller) public onlyArbiter(escrowId) inState(escrowId, State.Disputed) {
        escrows[escrowId].state = State.Resolved;
        emit DisputeResolved(escrowId, releaseToSeller);
    }

    function getEscrow(uint256 escrowId) public view returns (
        address buyer, address seller, address arbiter,
        uint256 amount, uint256 deadline, State state, string memory description
    ) {
        EscrowDetails storage e = escrows[escrowId];
        return (e.buyer, e.seller, e.arbiter, e.amount, e.deadline, e.state, e.description);
    }
}
```

## Build and Deploy

```bash
solscript build escrow.sol -o ./build
solscript deploy escrow.sol --network devnet
```

## Usage Flow

1. **Buyer creates escrow** with payment
2. **Seller delivers** goods/services
3. **Buyer releases** funds OR
4. **Seller refunds** OR deadline passes
5. If dispute: **Arbiter resolves**

## Next Steps

- [Token Tutorial](token.md)
- [NFT Marketplace](nft-marketplace.md)
