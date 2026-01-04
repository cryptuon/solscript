// Escrow - Trustless escrow contract
// Demonstrates state machines and multi-party transactions

contract Escrow {
    // Escrow states
    enum State {
        Created,
        Funded,
        Released,
        Refunded,
        Disputed,
        Resolved
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
    event EscrowCreated(
        uint256 indexed id,
        address indexed buyer,
        address indexed seller,
        uint256 amount
    );
    event EscrowFunded(uint256 indexed id);
    event EscrowReleased(uint256 indexed id);
    event EscrowRefunded(uint256 indexed id);
    event DisputeRaised(uint256 indexed id, address indexed by);
    event DisputeResolved(uint256 indexed id, bool releasedToSeller);

    // Errors
    error InvalidState(State current, State required);
    error NotAuthorized();
    error DeadlineNotReached();
    error DeadlinePassed();
    error InvalidAddress();
    error InvalidAmount();

    // Modifiers
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

    // Create and fund a new escrow
    function createEscrow(
        address seller,
        address arbiter,
        uint256 deadline,
        string memory description
    ) public payable returns (uint256) {
        if (seller == address(0)) revert InvalidAddress();
        if (arbiter == address(0)) revert InvalidAddress();
        if (msg.value == 0) revert InvalidAmount();
        require(deadline > block.timestamp, "Deadline must be in future");

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

    // Buyer releases funds to seller
    function release(uint256 escrowId)
        public
        onlyBuyer(escrowId)
        inState(escrowId, State.Funded)
    {
        EscrowDetails storage e = escrows[escrowId];
        e.state = State.Released;

        // Transfer funds to seller
        // (Implementation depends on Solana transfer mechanism)

        emit EscrowReleased(escrowId);
    }

    // Refund to buyer
    function refund(uint256 escrowId)
        public
        inState(escrowId, State.Funded)
    {
        EscrowDetails storage e = escrows[escrowId];

        // Seller can voluntarily refund anytime
        if (msg.sender == e.seller) {
            e.state = State.Refunded;
            emit EscrowRefunded(escrowId);
            return;
        }

        // Buyer can claim refund after deadline
        if (msg.sender == e.buyer) {
            if (block.timestamp < e.deadline) {
                revert DeadlineNotReached();
            }
            e.state = State.Refunded;
            emit EscrowRefunded(escrowId);
            return;
        }

        revert NotAuthorized();
    }

    // Raise a dispute
    function raiseDispute(uint256 escrowId)
        public
        onlyParty(escrowId)
        inState(escrowId, State.Funded)
    {
        escrows[escrowId].state = State.Disputed;
        emit DisputeRaised(escrowId, msg.sender);
    }

    // Arbiter resolves dispute
    function resolveDispute(uint256 escrowId, bool releaseToSeller)
        public
        onlyArbiter(escrowId)
        inState(escrowId, State.Disputed)
    {
        EscrowDetails storage e = escrows[escrowId];
        e.state = State.Resolved;

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

    // View functions
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
}
