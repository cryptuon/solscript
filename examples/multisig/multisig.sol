// Multi-Signature Wallet
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
