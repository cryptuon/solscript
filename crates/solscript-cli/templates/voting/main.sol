// Voting - Decentralized voting system
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

    // Events
    event ProposalCreated(uint256 indexed id, string description, uint256 deadline);
    event Voted(uint256 indexed proposalId, address indexed voter, Vote vote, uint256 weight);
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
    function createProposal(string memory description, uint256 durationSeconds)
        public
        onlyAdmin
        returns (uint256)
    {
        proposalCount++;

        proposals[proposalCount] = Proposal({
            description: description,
            forVotes: 0,
            againstVotes: 0,
            deadline: block.timestamp + durationSeconds,
            executed: false
        });

        emit ProposalCreated(proposalCount, description, block.timestamp + durationSeconds);
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
        return (p.description, p.forVotes, p.againstVotes, p.deadline, p.executed);
    }

    function isVotingActive(uint256 proposalId) public view returns (bool) {
        Proposal storage p = proposals[proposalId];
        return block.timestamp < p.deadline && !p.executed;
    }
}
