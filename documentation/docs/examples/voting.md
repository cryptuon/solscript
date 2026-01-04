# Voting Example

A decentralized voting contract with proposals and weighted votes.

## Contract

```solidity
contract Voting {
    struct Proposal {
        string description;
        uint256 voteCount;
        uint256 deadline;
        bool executed;
        mapping(address => bool) hasVoted;
    }

    struct Voter {
        uint256 weight;
        bool registered;
    }

    mapping(uint256 => Proposal) public proposals;
    mapping(address => Voter) public voters;

    uint256 public proposalCount;
    address public admin;
    uint256 public totalVoters;

    event ProposalCreated(uint256 indexed id, string description, uint256 deadline);
    event Voted(uint256 indexed proposalId, address indexed voter, uint256 weight);
    event VoterRegistered(address indexed voter, uint256 weight);
    event ProposalExecuted(uint256 indexed id);

    error NotAdmin();
    error NotRegistered();
    error AlreadyVoted();
    error VotingEnded();
    error VotingNotEnded();
    error AlreadyExecuted();
    error ProposalNotFound();

    modifier onlyAdmin() {
        if (msg.sender != admin) revert NotAdmin();
        _;
    }

    modifier onlyRegistered() {
        if (!voters[msg.sender].registered) revert NotRegistered();
        _;
    }

    constructor() {
        admin = msg.sender;
    }

    function registerVoter(address voter, uint256 weight) public onlyAdmin {
        require(weight > 0, "Weight must be positive");
        require(!voters[voter].registered, "Already registered");

        voters[voter] = Voter({
            weight: weight,
            registered: true
        });

        totalVoters++;
        emit VoterRegistered(voter, weight);
    }

    function createProposal(string memory description, uint256 duration) public onlyAdmin returns (uint256) {
        proposalCount++;

        Proposal storage p = proposals[proposalCount];
        p.description = description;
        p.voteCount = 0;
        p.deadline = block.timestamp + duration;
        p.executed = false;

        emit ProposalCreated(proposalCount, description, p.deadline);
        return proposalCount;
    }

    function vote(uint256 proposalId) public onlyRegistered {
        Proposal storage p = proposals[proposalId];

        if (bytes(p.description).length == 0) revert ProposalNotFound();
        if (block.timestamp >= p.deadline) revert VotingEnded();
        if (p.hasVoted[msg.sender]) revert AlreadyVoted();

        p.hasVoted[msg.sender] = true;
        p.voteCount += voters[msg.sender].weight;

        emit Voted(proposalId, msg.sender, voters[msg.sender].weight);
    }

    function execute(uint256 proposalId) public onlyAdmin {
        Proposal storage p = proposals[proposalId];

        if (bytes(p.description).length == 0) revert ProposalNotFound();
        if (block.timestamp < p.deadline) revert VotingNotEnded();
        if (p.executed) revert AlreadyExecuted();

        p.executed = true;
        emit ProposalExecuted(proposalId);

        // Execute proposal logic here
    }

    function getProposal(uint256 proposalId) public view returns (
        string memory description,
        uint256 voteCount,
        uint256 deadline,
        bool executed
    ) {
        Proposal storage p = proposals[proposalId];
        return (p.description, p.voteCount, p.deadline, p.executed);
    }

    function hasVoted(uint256 proposalId, address voter) public view returns (bool) {
        return proposals[proposalId].hasVoted[voter];
    }

    function getVoter(address voter) public view returns (uint256 weight, bool registered) {
        Voter storage v = voters[voter];
        return (v.weight, v.registered);
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
}
```

## Build

```bash
solscript build voting.sol -o ./build
```

## Usage Flow

1. **Admin registers voters** with voting weights
2. **Admin creates proposals** with descriptions and deadlines
3. **Registered voters vote** on active proposals
4. **After deadline, admin executes** successful proposals

## Key Features

- **Weighted voting**: Different voters can have different weights
- **Time-limited proposals**: Voting ends at deadline
- **One vote per address**: Prevents double voting
- **Execution tracking**: Proposals can only execute once

## See Also

- [Control Flow](../guide/control-flow.md)
- [Modifiers](../guide/modifiers.md)
