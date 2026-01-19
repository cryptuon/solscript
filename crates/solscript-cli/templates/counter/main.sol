// Counter - A simple counter contract
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
