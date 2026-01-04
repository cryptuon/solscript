// SolScript Token Contract Example
// A basic fungible token implementation for Solana

// Events for token operations
event Transfer(address indexed from, address indexed to, uint256 amount);
event Mint(address indexed to, uint256 amount);
event Burn(address indexed from, uint256 amount);

// Custom errors
error InsufficientBalance(uint256 available, uint256 required);
error Unauthorized;

// Main token contract
contract Token {
    uint256 public totalSupply;
    mapping(address => uint256) public balances;
    address public owner;

    modifier onlyOwner() {
        require(msg.sender == owner, "Not owner");
        _;
    }

    // Initialize the token
    constructor(uint256 initialSupply) {
        owner = msg.sender;
        totalSupply = initialSupply;
        balances[msg.sender] = initialSupply;
        emit Mint(msg.sender, initialSupply);
    }

    // Get balance of an account
    function balanceOf(address account) public view returns (uint256) {
        return balances[account];
    }

    // Get total supply
    function getTotalSupply() public view returns (uint256) {
        return totalSupply;
    }

    // Transfer tokens to another account
    function transfer(address to, uint256 amount) public returns (bool) {
        uint256 senderBalance = balances[msg.sender];
        require(senderBalance >= amount, "Insufficient balance");

        balances[msg.sender] = senderBalance - amount;
        balances[to] = balances[to] + amount;

        emit Transfer(msg.sender, to, amount);
        return true;
    }

    // Mint new tokens (owner only)
    function mint(address to, uint256 amount) public onlyOwner {
        totalSupply = totalSupply + amount;
        balances[to] = balances[to] + amount;
        emit Mint(to, amount);
    }

    // Burn tokens
    function burn(uint256 amount) public {
        uint256 balance = balances[msg.sender];
        require(balance >= amount, "Insufficient balance");

        balances[msg.sender] = balance - amount;
        totalSupply = totalSupply - amount;

        emit Burn(msg.sender, amount);
    }
}
