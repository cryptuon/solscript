export interface ExampleContract {
  name: string;
  description: string;
  source: string;
}

export const EXAMPLES: ExampleContract[] = [
  {
    name: "Hello World",
    description:
      "Your first SolScript contract. The simplest possible contract that stores and returns a greeting message.",
    source: `// Hello World - Your first SolScript contract
//
// This is the simplest possible contract that stores
// and returns a greeting message.

contract HelloWorld {
    // A public string stored on-chain
    string public greeting;

    // Constructor runs once when the contract is deployed
    constructor() {
        greeting = "Hello, Solana!";
    }

    // View function to read the greeting
    function getGreeting() public view returns (string) {
        return greeting;
    }

    // Function to update the greeting
    function setGreeting(string newGreeting) public {
        greeting = newGreeting;
    }
}
`,
  },
  {
    name: "Counter",
    description:
      "A simple counter contract. Demonstrates basic state management and access control.",
    source: `// Counter - A simple counter contract
// Demonstrates basic state management and access control

contract Counter {
    // State variables
    uint256 public count;
    address public owner;

    // Events
    event Incremented(address indexed by, uint256 newValue);
    event Decremented(address indexed by, uint256 newValue);
    event Reset(address indexed by);

    // Custom errors
    error Underflow();
    error Unauthorized();

    // Modifier for owner-only functions
    modifier onlyOwner() {
        if (msg.sender != owner) revert Unauthorized();
        _;
    }

    // Constructor - runs once at deployment
    constructor() {
        owner = msg.sender;
        count = 0;
    }

    // Increment the counter
    function increment() public {
        count += 1;
        emit Incremented(msg.sender, count);
    }

    // Decrement the counter (with underflow protection)
    function decrement() public {
        if (count == 0) revert Underflow();
        count -= 1;
        emit Decremented(msg.sender, count);
    }

    // Increment by a specific amount
    function incrementBy(uint256 amount) public {
        count += amount;
        emit Incremented(msg.sender, count);
    }

    // Reset to zero (owner only)
    function reset() public onlyOwner {
        count = 0;
        emit Reset(msg.sender);
    }

    // Read the current count
    function getCount() public view returns (uint256) {
        return count;
    }
}
`,
  },
  {
    name: "Simple",
    description:
      "Simple counter - minimal example for testing BPF compilation.",
    source: `// Simple counter - minimal example for testing BPF compilation

contract Simple {
    // State variables
    uint64 public count;

    // Constructor
    constructor() {
        count = 0;
    }

    // Increment the counter
    function increment() public {
        count = count + 1;
    }

    // Read the current count
    function getCount() public view returns (uint64) {
        return count;
    }
}
`,
  },
  {
    name: "Token",
    description:
      "ERC20-style fungible token. Demonstrates mappings, events, and transfers.",
    source: `// Token - ERC20-style fungible token
// Demonstrates mappings, events, and transfers

contract Token {
    // Token metadata
    string public name;
    string public symbol;
    uint8 public decimals = 9;

    // Token state
    uint256 public totalSupply;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    // Ownership
    address public owner;
    bool public paused;

    // Events
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);
    event Mint(address indexed to, uint256 amount);
    event Burn(address indexed from, uint256 amount);
    event Paused(address account);
    event Unpaused(address account);

    // Errors
    error InsufficientBalance(uint256 available, uint256 required);
    error InsufficientAllowance(uint256 available, uint256 required);
    error ContractPaused();
    error Unauthorized();
    error InvalidAddress();

    // Modifiers
    modifier onlyOwner() {
        if (msg.sender != owner) revert Unauthorized();
        _;
    }

    modifier whenNotPaused() {
        if (paused) revert ContractPaused();
        _;
    }

    modifier validAddress(address addr) {
        if (addr == address(0)) revert InvalidAddress();
        _;
    }

    // Constructor
    constructor(string memory _name, string memory _symbol, uint256 _initialSupply) {
        name = _name;
        symbol = _symbol;
        owner = msg.sender;

        if (_initialSupply > 0) {
            _mint(msg.sender, _initialSupply);
        }
    }

    // Transfer tokens
    function transfer(address to, uint256 amount)
        public
        whenNotPaused
        validAddress(to)
        returns (bool)
    {
        _transfer(msg.sender, to, amount);
        return true;
    }

    // Approve spender
    function approve(address spender, uint256 amount)
        public
        validAddress(spender)
        returns (bool)
    {
        allowance[msg.sender][spender] = amount;
        emit Approval(msg.sender, spender, amount);
        return true;
    }

    // Transfer from (using allowance)
    function transferFrom(address from, address to, uint256 amount)
        public
        whenNotPaused
        validAddress(to)
        returns (bool)
    {
        uint256 currentAllowance = allowance[from][msg.sender];

        if (currentAllowance < amount) {
            revert InsufficientAllowance(currentAllowance, amount);
        }

        allowance[from][msg.sender] = currentAllowance - amount;
        _transfer(from, to, amount);
        return true;
    }

    // Mint new tokens (owner only)
    function mint(address to, uint256 amount) public onlyOwner validAddress(to) {
        _mint(to, amount);
        emit Mint(to, amount);
    }

    // Burn tokens
    function burn(uint256 amount) public {
        if (balanceOf[msg.sender] < amount) {
            revert InsufficientBalance(balanceOf[msg.sender], amount);
        }

        balanceOf[msg.sender] -= amount;
        totalSupply -= amount;
        emit Burn(msg.sender, amount);
        emit Transfer(msg.sender, address(0), amount);
    }

    // Pause transfers (owner only)
    function pause() public onlyOwner {
        paused = true;
        emit Paused(msg.sender);
    }

    // Unpause transfers (owner only)
    function unpause() public onlyOwner {
        paused = false;
        emit Unpaused(msg.sender);
    }

    // Increase allowance
    function increaseAllowance(address spender, uint256 addedValue) public returns (bool) {
        allowance[msg.sender][spender] += addedValue;
        emit Approval(msg.sender, spender, allowance[msg.sender][spender]);
        return true;
    }

    // Decrease allowance
    function decreaseAllowance(address spender, uint256 subtractedValue) public returns (bool) {
        uint256 currentAllowance = allowance[msg.sender][spender];
        require(currentAllowance >= subtractedValue, "Decreased below zero");
        allowance[msg.sender][spender] = currentAllowance - subtractedValue;
        emit Approval(msg.sender, spender, allowance[msg.sender][spender]);
        return true;
    }

    // Internal transfer
    function _transfer(address from, address to, uint256 amount) internal {
        if (balanceOf[from] < amount) {
            revert InsufficientBalance(balanceOf[from], amount);
        }

        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        emit Transfer(from, to, amount);
    }

    // Internal mint
    function _mint(address to, uint256 amount) internal {
        totalSupply += amount;
        balanceOf[to] += amount;
        emit Transfer(address(0), to, amount);
    }
}
`,
  },
  {
    name: "Voting",
    description:
      "Decentralized voting system. Demonstrates enums, structs, and time-based logic.",
    source: `// Voting - Decentralized voting system
// Demonstrates enums, structs, and time-based logic

contract Voting {
    // Proposal structure
    struct Proposal {
        string description;
        uint256 forVotes;
        uint256 againstVotes;
        uint256 deadline;
        bool executed;
    }

    // Voter structure
    struct Voter {
        uint256 weight;
        bool registered;
    }

    // Vote choice
    enum Vote { For, Against }

    // State
    mapping(uint256 => Proposal) public proposals;
    mapping(address => Voter) public voters;
    mapping(uint256 => mapping(address => bool)) public hasVoted;

    uint256 public proposalCount;
    address public admin;
    uint256 public totalVoters;
    uint256 public quorum = 50; // 50% quorum

    // Events
    event ProposalCreated(
        uint256 indexed id,
        string description,
        uint256 deadline
    );
    event Voted(
        uint256 indexed proposalId,
        address indexed voter,
        Vote vote,
        uint256 weight
    );
    event VoterRegistered(address indexed voter, uint256 weight);
    event ProposalExecuted(uint256 indexed id, bool passed);

    // Errors
    error NotAdmin();
    error NotRegistered();
    error AlreadyVoted();
    error VotingEnded();
    error VotingNotEnded();
    error AlreadyExecuted();
    error ProposalNotFound();
    error AlreadyRegistered();

    // Modifiers
    modifier onlyAdmin() {
        if (msg.sender != admin) revert NotAdmin();
        _;
    }

    modifier onlyRegistered() {
        if (!voters[msg.sender].registered) revert NotRegistered();
        _;
    }

    // Constructor
    constructor() {
        admin = msg.sender;
    }

    // Register a voter with voting weight
    function registerVoter(address voter, uint256 weight) public onlyAdmin {
        require(weight > 0, "Weight must be positive");
        if (voters[voter].registered) revert AlreadyRegistered();

        voters[voter] = Voter({
            weight: weight,
            registered: true
        });

        totalVoters++;
        emit VoterRegistered(voter, weight);
    }

    // Create a new proposal
    function createProposal(
        string memory description,
        uint256 durationSeconds
    ) public onlyAdmin returns (uint256) {
        proposalCount++;

        proposals[proposalCount] = Proposal({
            description: description,
            forVotes: 0,
            againstVotes: 0,
            deadline: block.timestamp + durationSeconds,
            executed: false
        });

        emit ProposalCreated(
            proposalCount,
            description,
            block.timestamp + durationSeconds
        );

        return proposalCount;
    }

    // Vote on a proposal
    function vote(uint256 proposalId, Vote choice) public onlyRegistered {
        Proposal storage p = proposals[proposalId];

        if (bytes(p.description).length == 0) revert ProposalNotFound();
        if (block.timestamp >= p.deadline) revert VotingEnded();
        if (hasVoted[proposalId][msg.sender]) revert AlreadyVoted();

        hasVoted[proposalId][msg.sender] = true;
        uint256 weight = voters[msg.sender].weight;

        if (choice == Vote.For) {
            p.forVotes += weight;
        } else {
            p.againstVotes += weight;
        }

        emit Voted(proposalId, msg.sender, choice, weight);
    }

    // Execute a proposal after voting ends
    function execute(uint256 proposalId) public onlyAdmin {
        Proposal storage p = proposals[proposalId];

        if (bytes(p.description).length == 0) revert ProposalNotFound();
        if (block.timestamp < p.deadline) revert VotingNotEnded();
        if (p.executed) revert AlreadyExecuted();

        p.executed = true;
        bool passed = p.forVotes > p.againstVotes;

        emit ProposalExecuted(proposalId, passed);
    }

    // View functions
    function getProposal(uint256 proposalId) public view returns (
        string memory description,
        uint256 forVotes,
        uint256 againstVotes,
        uint256 deadline,
        bool executed
    ) {
        Proposal storage p = proposals[proposalId];
        return (
            p.description,
            p.forVotes,
            p.againstVotes,
            p.deadline,
            p.executed
        );
    }

    function isVotingActive(uint256 proposalId) public view returns (bool) {
        Proposal storage p = proposals[proposalId];
        return block.timestamp < p.deadline && !p.executed;
    }

    function getTimeRemaining(uint256 proposalId) public view returns (uint256) {
        Proposal storage p = proposals[proposalId];
        if (block.timestamp >= p.deadline) return 0;
        return p.deadline - block.timestamp;
    }

    function getVoter(address voter) public view returns (uint256 weight, bool registered) {
        Voter storage v = voters[voter];
        return (v.weight, v.registered);
    }
}
`,
  },
  {
    name: "NFT",
    description:
      "Non-Fungible Token collection. Demonstrates ERC721-style NFT with metadata.",
    source: `// NFT - Non-Fungible Token collection
// Demonstrates ERC721-style NFT with metadata

contract NFT {
    // Token metadata
    string public name;
    string public symbol;
    string public baseURI;

    // Token state
    uint256 public totalSupply;
    uint256 public maxSupply;
    uint256 public mintPrice;

    mapping(uint256 => address) public ownerOf;
    mapping(address => uint256) public balanceOf;
    mapping(uint256 => address) public getApproved;
    mapping(address => mapping(address => bool)) public isApprovedForAll;
    mapping(uint256 => string) private _tokenURIs;

    // Ownership
    address public owner;
    bool public mintingEnabled;

    // Events
    event Transfer(address indexed from, address indexed to, uint256 indexed tokenId);
    event Approval(address indexed owner, address indexed approved, uint256 indexed tokenId);
    event ApprovalForAll(address indexed owner, address indexed operator, bool approved);
    event Minted(address indexed to, uint256 indexed tokenId);
    event BaseURIUpdated(string newBaseURI);

    // Errors
    error Unauthorized();
    error TokenNotFound();
    error AlreadyMinted();
    error MaxSupplyReached();
    error MintingDisabled();
    error InsufficientPayment();
    error InvalidAddress();
    error NotApproved();

    // Modifiers
    modifier onlyOwner() {
        if (msg.sender != owner) revert Unauthorized();
        _;
    }

    modifier tokenExists(uint256 tokenId) {
        if (ownerOf[tokenId] == address(0)) revert TokenNotFound();
        _;
    }

    // Constructor
    constructor(
        string memory _name,
        string memory _symbol,
        uint256 _maxSupply,
        uint256 _mintPrice
    ) {
        name = _name;
        symbol = _symbol;
        maxSupply = _maxSupply;
        mintPrice = _mintPrice;
        owner = msg.sender;
        mintingEnabled = true;
    }

    // Mint a new NFT
    function mint() public payable returns (uint256) {
        if (!mintingEnabled) revert MintingDisabled();
        if (totalSupply >= maxSupply) revert MaxSupplyReached();
        if (msg.value < mintPrice) revert InsufficientPayment();

        totalSupply++;
        uint256 tokenId = totalSupply;

        ownerOf[tokenId] = msg.sender;
        balanceOf[msg.sender]++;

        emit Transfer(address(0), msg.sender, tokenId);
        emit Minted(msg.sender, tokenId);

        return tokenId;
    }

    // Owner can mint for free
    function ownerMint(address to) public onlyOwner returns (uint256) {
        if (to == address(0)) revert InvalidAddress();
        if (totalSupply >= maxSupply) revert MaxSupplyReached();

        totalSupply++;
        uint256 tokenId = totalSupply;

        ownerOf[tokenId] = to;
        balanceOf[to]++;

        emit Transfer(address(0), to, tokenId);
        emit Minted(to, tokenId);

        return tokenId;
    }

    // Transfer NFT
    function transferFrom(address from, address to, uint256 tokenId) public tokenExists(tokenId) {
        if (to == address(0)) revert InvalidAddress();

        address tokenOwner = ownerOf[tokenId];
        if (from != tokenOwner) revert Unauthorized();

        // Check authorization
        if (msg.sender != tokenOwner &&
            getApproved[tokenId] != msg.sender &&
            !isApprovedForAll[tokenOwner][msg.sender]) {
            revert NotApproved();
        }

        // Clear approval
        getApproved[tokenId] = address(0);

        // Transfer
        balanceOf[from]--;
        balanceOf[to]++;
        ownerOf[tokenId] = to;

        emit Transfer(from, to, tokenId);
    }

    // Safe transfer (with callback check)
    function safeTransferFrom(address from, address to, uint256 tokenId) public {
        transferFrom(from, to, tokenId);
        // In production, check if \`to\` is a contract and call onERC721Received
    }

    // Approve single token
    function approve(address to, uint256 tokenId) public tokenExists(tokenId) {
        address tokenOwner = ownerOf[tokenId];
        if (msg.sender != tokenOwner && !isApprovedForAll[tokenOwner][msg.sender]) {
            revert Unauthorized();
        }

        getApproved[tokenId] = to;
        emit Approval(tokenOwner, to, tokenId);
    }

    // Approve all tokens
    function setApprovalForAll(address operator, bool approved) public {
        if (operator == address(0)) revert InvalidAddress();
        isApprovedForAll[msg.sender][operator] = approved;
        emit ApprovalForAll(msg.sender, operator, approved);
    }

    // Get token URI
    function tokenURI(uint256 tokenId) public view tokenExists(tokenId) returns (string memory) {
        string memory _tokenURI = _tokenURIs[tokenId];
        if (bytes(_tokenURI).length > 0) {
            return _tokenURI;
        }
        return string.concat(baseURI, toString(tokenId));
    }

    // Set token-specific URI
    function setTokenURI(uint256 tokenId, string memory _tokenURI) public onlyOwner tokenExists(tokenId) {
        _tokenURIs[tokenId] = _tokenURI;
    }

    // Set base URI
    function setBaseURI(string memory _baseURI) public onlyOwner {
        baseURI = _baseURI;
        emit BaseURIUpdated(_baseURI);
    }

    // Toggle minting
    function setMintingEnabled(bool enabled) public onlyOwner {
        mintingEnabled = enabled;
    }

    // Update mint price
    function setMintPrice(uint256 _price) public onlyOwner {
        mintPrice = _price;
    }

    // Withdraw funds
    function withdraw() public onlyOwner {
        // Transfer contract balance to owner
    }

    // Helper: uint to string
    function toString(uint256 value) internal pure returns (string memory) {
        if (value == 0) {
            return "0";
        }
        uint256 temp = value;
        uint256 digits;
        while (temp != 0) {
            digits++;
            temp /= 10;
        }
        bytes memory buffer = new bytes(digits);
        while (value != 0) {
            digits -= 1;
            buffer[digits] = bytes1(uint8(48 + uint256(value % 10)));
            value /= 10;
        }
        return string(buffer);
    }
}
`,
  },
  {
    name: "Escrow",
    description:
      "Trustless escrow contract. Demonstrates state machines and multi-party transactions.",
    source: `// Escrow - Trustless escrow contract
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
`,
  },
  {
    name: "Staking",
    description:
      "A DeFi staking contract where users deposit tokens to earn rewards over time. Demonstrates time-based calculations and reward distribution.",
    source: `// Staking Pool
//
// A DeFi staking contract where users deposit tokens to earn rewards
// over time. Demonstrates time-based calculations and reward distribution.

contract StakingPool {
    // Configuration
    address public owner;
    uint64 public rewardRate;           // Rewards per second per staked token
    uint64 public lastUpdateTime;       // Last time rewards were calculated
    uint64 public rewardPerTokenStored; // Accumulated rewards per token

    uint64 public totalStaked;          // Total tokens staked in the pool
    uint64 public minimumStake;         // Minimum stake amount
    uint64 public lockDuration;         // Time tokens must be locked

    bool public paused;

    // User State
    mapping(address => uint64) public stakedBalance;
    mapping(address => uint64) public stakingTimestamp;
    mapping(address => uint64) public userRewardPerTokenPaid;
    mapping(address => uint64) public pendingRewards;

    // Events
    event Staked(address indexed user, uint64 amount, uint64 timestamp);
    event Unstaked(address indexed user, uint64 amount);
    event RewardsClaimed(address indexed user, uint64 amount);
    event RewardRateUpdated(uint64 oldRate, uint64 newRate);
    event PoolPaused(bool isPaused);

    // Constructor
    constructor(uint64 _rewardRate) {
        owner = msg.sender;
        rewardRate = _rewardRate;
        lastUpdateTime = block.timestamp;
        minimumStake = 100;      // Minimum 100 tokens
        lockDuration = 86400;    // 1 day in seconds
        paused = false;
        totalStaked = 0;
        rewardPerTokenStored = 0;
    }

    // Stake tokens in the pool
    function stake(uint64 amount) public {
        require(!paused, "Pool is paused");
        require(amount >= minimumStake, "Below minimum stake");

        // Update rewards before changing balances
        _updateReward(msg.sender);

        stakedBalance[msg.sender] = stakedBalance[msg.sender] + amount;
        stakingTimestamp[msg.sender] = block.timestamp;
        totalStaked = totalStaked + amount;

        emit Staked(msg.sender, amount, block.timestamp);
    }

    // Unstake tokens from the pool
    function unstake(uint64 amount) public {
        require(stakedBalance[msg.sender] >= amount, "Insufficient balance");

        // Check lock duration
        uint64 unlockTime = stakingTimestamp[msg.sender] + lockDuration;
        require(block.timestamp >= unlockTime, "Still locked");

        // Update rewards before changing balances
        _updateReward(msg.sender);

        stakedBalance[msg.sender] = stakedBalance[msg.sender] - amount;
        totalStaked = totalStaked - amount;

        emit Unstaked(msg.sender, amount);
    }

    // Claim accumulated rewards
    function claimRewards() public {
        _updateReward(msg.sender);

        uint64 reward = pendingRewards[msg.sender];
        require(reward > 0, "No rewards to claim");

        pendingRewards[msg.sender] = 0;

        emit RewardsClaimed(msg.sender, reward);
    }

    // Compound rewards - add to staked balance
    function compoundRewards() public {
        require(!paused, "Pool is paused");

        _updateReward(msg.sender);

        uint64 reward = pendingRewards[msg.sender];
        if (reward > 0) {
            pendingRewards[msg.sender] = 0;

            stakedBalance[msg.sender] = stakedBalance[msg.sender] + reward;
            totalStaked = totalStaked + reward;

            emit Staked(msg.sender, reward, block.timestamp);
        }
    }

    // Internal reward update
    function _updateReward(address account) internal {
        rewardPerTokenStored = rewardPerToken();
        lastUpdateTime = block.timestamp;

        if (account != address(0)) {
            pendingRewards[account] = earned(account);
            userRewardPerTokenPaid[account] = rewardPerTokenStored;
        }
    }

    // Calculate current reward per token
    function rewardPerToken() public view returns (uint64) {
        if (totalStaked == 0) {
            return rewardPerTokenStored;
        }

        uint64 timeDelta = block.timestamp - lastUpdateTime;
        uint64 rewardAccrued = timeDelta * rewardRate;

        return rewardPerTokenStored + (rewardAccrued * 1000000 / totalStaked);
    }

    // Calculate rewards earned by an account
    function earned(address account) public view returns (uint64) {
        uint64 balance = stakedBalance[account];
        uint64 rewardDelta = rewardPerToken() - userRewardPerTokenPaid[account];

        return (balance * rewardDelta / 1000000) + pendingRewards[account];
    }

    // Get time until tokens can be unstaked
    function timeUntilUnlock(address account) public view returns (uint64) {
        if (stakedBalance[account] == 0) {
            return 0;
        }

        uint64 unlockTime = stakingTimestamp[account] + lockDuration;

        if (block.timestamp >= unlockTime) {
            return 0;
        }

        return unlockTime - block.timestamp;
    }

    // Admin: Set reward rate
    function setRewardRate(uint64 newRate) public {
        require(msg.sender == owner, "Not owner");
        _updateReward(address(0));
        uint64 oldRate = rewardRate;
        rewardRate = newRate;
        emit RewardRateUpdated(oldRate, newRate);
    }

    // Admin: Set minimum stake
    function setMinimumStake(uint64 amount) public {
        require(msg.sender == owner, "Not owner");
        minimumStake = amount;
    }

    // Admin: Set lock duration
    function setLockDuration(uint64 duration) public {
        require(msg.sender == owner, "Not owner");
        lockDuration = duration;
    }

    // Admin: Pause/unpause pool
    function setPaused(bool _paused) public {
        require(msg.sender == owner, "Not owner");
        paused = _paused;
        emit PoolPaused(_paused);
    }

    // View: Get pool stats
    function getPoolStats() public view returns (uint64, uint64, uint64, uint64) {
        return (totalStaked, rewardRate, minimumStake, lockDuration);
    }

    // View: Get user info
    function getUserInfo(address user) public view returns (uint64, uint64, uint64, uint64) {
        return (
            stakedBalance[user],
            earned(user),
            stakingTimestamp[user],
            timeUntilUnlock(user)
        );
    }
}
`,
  },
  {
    name: "AMM",
    description:
      "A simple constant product AMM (x * y = k) for token swaps. Demonstrates DeFi primitives: liquidity pools, swaps, and LP tokens.",
    source: `// Automated Market Maker (AMM)
//
// A simple constant product AMM (x * y = k) for token swaps.
// Demonstrates DeFi primitives: liquidity pools, swaps, and LP tokens.

contract SimpleAMM {
    // Pool State
    address public tokenA;
    address public tokenB;
    uint64 public reserveA;
    uint64 public reserveB;

    // LP Token tracking
    uint64 public totalLPSupply;
    mapping(address => uint64) public lpBalances;

    // Configuration
    address public owner;
    uint64 public swapFee;      // Fee in basis points (e.g., 30 = 0.3%)

    bool public initialized;

    // Events
    event PoolInitialized(address indexed tokenA, address indexed tokenB);
    event LiquidityAdded(address indexed provider, uint64 amountA, uint64 amountB, uint64 lpTokens);
    event LiquidityRemoved(address indexed provider, uint64 amountA, uint64 amountB, uint64 lpTokens);
    event Swap(address indexed user, address tokenIn, uint64 amountIn, address tokenOut, uint64 amountOut);
    event FeeUpdated(uint64 oldFee, uint64 newFee);

    // Constructor
    constructor(uint64 _swapFee) {
        owner = msg.sender;
        swapFee = _swapFee;
        initialized = false;
        totalLPSupply = 0;
        reserveA = 0;
        reserveB = 0;
    }

    // Initialize the pool with initial liquidity
    function initialize(
        address _tokenA,
        address _tokenB,
        uint64 amountA,
        uint64 amountB
    ) public {
        require(msg.sender == owner, "Not owner");
        require(!initialized, "Already initialized");
        require(amountA > 1000, "Insufficient initial liquidity A");
        require(amountB > 1000, "Insufficient initial liquidity B");

        tokenA = _tokenA;
        tokenB = _tokenB;
        reserveA = amountA;
        reserveB = amountB;

        // Calculate initial LP tokens using geometric mean approximation
        uint64 lpTokens = sqrt(amountA * amountB);

        // Lock minimum liquidity to prevent manipulation
        lpBalances[address(0)] = 1000;
        lpBalances[msg.sender] = lpTokens - 1000;
        totalLPSupply = lpTokens;

        initialized = true;

        emit PoolInitialized(_tokenA, _tokenB);
        emit LiquidityAdded(msg.sender, amountA, amountB, lpTokens - 1000);
    }

    // Add liquidity to the pool
    function addLiquidity(
        uint64 amountA,
        uint64 amountB,
        uint64 minLPTokens
    ) public returns (uint64) {
        require(initialized, "Not initialized");
        require(amountA > 0 && amountB > 0, "Zero amount");

        // Calculate optimal amounts based on current ratio
        uint64 optimalB = (amountA * reserveB) / reserveA;
        uint64 actualA = amountA;
        uint64 actualB = amountB;

        if (optimalB <= amountB) {
            actualB = optimalB;
        } else {
            uint64 optimalA = (amountB * reserveA) / reserveB;
            actualA = optimalA;
        }

        // Calculate LP tokens to mint
        uint64 lpFromA = (actualA * totalLPSupply) / reserveA;
        uint64 lpFromB = (actualB * totalLPSupply) / reserveB;
        uint64 lpTokens = lpFromA;
        if (lpFromB < lpFromA) {
            lpTokens = lpFromB;
        }

        require(lpTokens >= minLPTokens, "Slippage exceeded");

        // Update state
        reserveA = reserveA + actualA;
        reserveB = reserveB + actualB;
        lpBalances[msg.sender] = lpBalances[msg.sender] + lpTokens;
        totalLPSupply = totalLPSupply + lpTokens;

        emit LiquidityAdded(msg.sender, actualA, actualB, lpTokens);

        return lpTokens;
    }

    // Remove liquidity from the pool
    function removeLiquidity(
        uint64 lpTokens,
        uint64 minAmountA,
        uint64 minAmountB
    ) public returns (uint64, uint64) {
        require(initialized, "Not initialized");
        require(lpTokens > 0, "Zero amount");
        require(lpBalances[msg.sender] >= lpTokens, "Insufficient LP balance");

        // Calculate token amounts to return
        uint64 amountA = (lpTokens * reserveA) / totalLPSupply;
        uint64 amountB = (lpTokens * reserveB) / totalLPSupply;

        require(amountA >= minAmountA && amountB >= minAmountB, "Slippage exceeded");

        // Update state
        lpBalances[msg.sender] = lpBalances[msg.sender] - lpTokens;
        totalLPSupply = totalLPSupply - lpTokens;
        reserveA = reserveA - amountA;
        reserveB = reserveB - amountB;

        emit LiquidityRemoved(msg.sender, amountA, amountB, lpTokens);

        return (amountA, amountB);
    }

    // Swap exact amount of tokenA for tokenB
    function swapAForB(uint64 amountIn, uint64 minAmountOut) public returns (uint64) {
        require(initialized, "Not initialized");
        require(amountIn > 0, "Zero amount");

        // Apply fee (10000 basis points = 100%)
        uint64 amountInWithFee = amountIn * (10000 - swapFee);

        // Constant product formula
        uint64 numerator = amountInWithFee * reserveB;
        uint64 denominator = (reserveA * 10000) + amountInWithFee;
        uint64 amountOut = numerator / denominator;

        require(amountOut >= minAmountOut, "Slippage exceeded");
        require(amountOut < reserveB, "Insufficient liquidity");

        // Update reserves
        reserveA = reserveA + amountIn;
        reserveB = reserveB - amountOut;

        emit Swap(msg.sender, tokenA, amountIn, tokenB, amountOut);

        return amountOut;
    }

    // Swap exact amount of tokenB for tokenA
    function swapBForA(uint64 amountIn, uint64 minAmountOut) public returns (uint64) {
        require(initialized, "Not initialized");
        require(amountIn > 0, "Zero amount");

        // Apply fee
        uint64 amountInWithFee = amountIn * (10000 - swapFee);

        // Constant product formula
        uint64 numerator = amountInWithFee * reserveA;
        uint64 denominator = (reserveB * 10000) + amountInWithFee;
        uint64 amountOut = numerator / denominator;

        require(amountOut >= minAmountOut, "Slippage exceeded");
        require(amountOut < reserveA, "Insufficient liquidity");

        // Update reserves
        reserveB = reserveB + amountIn;
        reserveA = reserveA - amountOut;

        emit Swap(msg.sender, tokenB, amountIn, tokenA, amountOut);

        return amountOut;
    }

    // Get current price of tokenA in terms of tokenB (6 decimal precision)
    function getPriceAtoB() public view returns (uint64) {
        if (reserveA == 0) {
            return 0;
        }
        return (reserveB * 1000000) / reserveA;
    }

    // Get current price of tokenB in terms of tokenA (6 decimal precision)
    function getPriceBtoA() public view returns (uint64) {
        if (reserveB == 0) {
            return 0;
        }
        return (reserveA * 1000000) / reserveB;
    }

    // Calculate expected output for a given input
    function getAmountOut(uint64 amountIn, bool aToB) public view returns (uint64) {
        uint64 reserveIn = reserveA;
        uint64 reserveOut = reserveB;
        if (!aToB) {
            reserveIn = reserveB;
            reserveOut = reserveA;
        }

        uint64 amountInWithFee = amountIn * (10000 - swapFee);
        uint64 numerator = amountInWithFee * reserveOut;
        uint64 denominator = (reserveIn * 10000) + amountInWithFee;

        return numerator / denominator;
    }

    // Get pool reserves
    function getReserves() public view returns (uint64, uint64) {
        return (reserveA, reserveB);
    }

    // Get LP token balance
    function getLPBalance(address account) public view returns (uint64) {
        return lpBalances[account];
    }

    // Admin: Set swap fee
    function setSwapFee(uint64 newFee) public {
        require(msg.sender == owner, "Not owner");
        require(newFee <= 1000, "Fee too high");  // Max 10%
        uint64 oldFee = swapFee;
        swapFee = newFee;
        emit FeeUpdated(oldFee, newFee);
    }

    // Integer square root (Babylonian method)
    function sqrt(uint64 x) internal pure returns (uint64) {
        if (x == 0) {
            return 0;
        }

        uint64 z = (x + 1) / 2;
        uint64 y = x;

        while (z < y) {
            y = z;
            z = (x / z + z) / 2;
        }

        return y;
    }
}
`,
  },
  {
    name: "Multisig",
    description:
      "A wallet that requires multiple owners to approve transactions before they can be executed. Demonstrates multi-party coordination.",
    source: `// Multi-Signature Wallet
//
// A wallet that requires multiple owners to approve transactions
// before they can be executed. Demonstrates multi-party coordination.

contract MultiSigWallet {
    // Configuration
    uint64 public requiredApprovals;
    uint64 public ownerCount;
    uint64 public transactionCount;

    // Owner tracking
    mapping(address => bool) public isOwner;

    // Transaction state (flattened - no struct support)
    mapping(uint64 => address) public txDestination;
    mapping(uint64 => uint64) public txValue;
    mapping(uint64 => bool) public txExecuted;
    mapping(uint64 => uint64) public txApprovalCount;

    // Approval tracking: txId => owner => approved
    mapping(uint64 => mapping(address => bool)) public approvals;

    // Events
    event TransactionSubmitted(uint64 indexed txId, address indexed submitter, address destination, uint64 value);
    event TransactionApproved(uint64 indexed txId, address indexed approver);
    event ApprovalRevoked(uint64 indexed txId, address indexed revoker);
    event TransactionExecuted(uint64 indexed txId);
    event OwnerAdded(address indexed owner);

    // Constructor
    constructor(uint64 required) {
        requiredApprovals = required;
        ownerCount = 1;
        transactionCount = 0;
        // Deployer is first owner
        isOwner[msg.sender] = true;
    }

    // Submit a new transaction for approval
    function submitTransaction(address destination, uint64 value) public returns (uint64) {
        require(isOwner[msg.sender], "Not an owner");

        uint64 txId = transactionCount;

        txDestination[txId] = destination;
        txValue[txId] = value;
        txExecuted[txId] = false;
        txApprovalCount[txId] = 1;  // Auto-approve by submitter

        approvals[txId][msg.sender] = true;
        transactionCount = transactionCount + 1;

        emit TransactionSubmitted(txId, msg.sender, destination, value);
        emit TransactionApproved(txId, msg.sender);

        return txId;
    }

    // Approve a pending transaction
    function approve(uint64 txId) public {
        require(isOwner[msg.sender], "Not an owner");
        require(txId < transactionCount, "Transaction not found");
        require(!txExecuted[txId], "Already executed");
        require(!approvals[txId][msg.sender], "Already approved");

        approvals[txId][msg.sender] = true;
        txApprovalCount[txId] = txApprovalCount[txId] + 1;

        emit TransactionApproved(txId, msg.sender);
    }

    // Revoke approval for a transaction
    function revokeApproval(uint64 txId) public {
        require(isOwner[msg.sender], "Not an owner");
        require(txId < transactionCount, "Transaction not found");
        require(!txExecuted[txId], "Already executed");
        require(approvals[txId][msg.sender], "Not approved");

        approvals[txId][msg.sender] = false;
        txApprovalCount[txId] = txApprovalCount[txId] - 1;

        emit ApprovalRevoked(txId, msg.sender);
    }

    // Execute a transaction that has enough approvals
    function executeTransaction(uint64 txId) public {
        require(isOwner[msg.sender], "Not an owner");
        require(txId < transactionCount, "Transaction not found");
        require(!txExecuted[txId], "Already executed");
        require(txApprovalCount[txId] >= requiredApprovals, "Insufficient approvals");

        txExecuted[txId] = true;

        emit TransactionExecuted(txId);
    }

    // Add a new owner
    function addOwner(address newOwner) public {
        require(isOwner[msg.sender], "Not an owner");
        require(!isOwner[newOwner], "Already an owner");

        isOwner[newOwner] = true;
        ownerCount = ownerCount + 1;

        emit OwnerAdded(newOwner);
    }

    // View functions
    function getTransaction(uint64 txId) public view returns (address, uint64, bool, uint64) {
        return (txDestination[txId], txValue[txId], txExecuted[txId], txApprovalCount[txId]);
    }

    function hasApproved(uint64 txId, address owner) public view returns (bool) {
        return approvals[txId][owner];
    }

    function isConfirmed(uint64 txId) public view returns (bool) {
        return txApprovalCount[txId] >= requiredApprovals;
    }
}
`,
  },
  {
    name: "Storage",
    description:
      "Demonstrating different data types and storage. Shows how to work with various data types and common storage patterns in SolScript.",
    source: `// Storage Patterns - Demonstrating different data types and storage
//
// This contract shows how to work with various data types
// and common storage patterns in SolScript.

contract StorageDemo {
    // === Primitive Types ===
    uint64 public counter;
    int64 public signedValue;
    bool public isActive;
    address public owner;

    // === Mappings ===
    // Single mapping: address -> balance
    mapping(address => uint64) public balances;

    // Nested mapping: owner -> spender -> allowance
    mapping(address => mapping(address => uint64)) public allowances;

    // === Events ===
    event ValueUpdated(string field, uint64 oldValue, uint64 newValue);
    event BalanceSet(address indexed account, uint64 amount);

    // === Constructor ===
    constructor() {
        owner = msg.sender;
        counter = 0;
        signedValue = -100;
        isActive = true;
    }

    // === Primitive Operations ===

    function incrementCounter() public {
        uint64 oldValue = counter;
        counter = counter + 1;
        emit ValueUpdated("counter", oldValue, counter);
    }

    function setSignedValue(int64 value) public {
        signedValue = value;
    }

    function toggleActive() public {
        isActive = !isActive;
    }

    // === Mapping Operations ===

    function setBalance(address account, uint64 amount) public {
        balances[account] = amount;
        emit BalanceSet(account, amount);
    }

    function getBalance(address account) public view returns (uint64) {
        return balances[account];
    }

    function setAllowance(address spender, uint64 amount) public {
        allowances[msg.sender][spender] = amount;
    }

    function getAllowance(address tokenOwner, address spender) public view returns (uint64) {
        return allowances[tokenOwner][spender];
    }

    // === Batch Operations ===

    function batchSetBalances(address[] accounts, uint64[] amounts) public {
        require(accounts.length == amounts.length, "Arrays must match");

        for (uint64 i = 0; i < accounts.length; i = i + 1) {
            balances[accounts[i]] = amounts[i];
            emit BalanceSet(accounts[i], amounts[i]);
        }
    }

    // === View Functions ===

    function getContractState() public view returns (uint64, int64, bool, address) {
        return (counter, signedValue, isActive, owner);
    }
}
`,
  },
];
