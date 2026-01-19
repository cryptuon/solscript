// Storage Patterns - Demonstrating different data types and storage
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
