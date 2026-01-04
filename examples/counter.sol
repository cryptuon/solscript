// Simple counter contract example

event CountChanged(address caller, uint64 oldValue, uint64 newValue);

error CounterOverflow;
error CounterUnderflow;

contract Counter {
    uint64 public count;
    address public owner;

    constructor() {
        count = 0;
        owner = msg.sender;
    }

    function increment() public {
        uint64 oldValue = count;
        count += 1;
        emit CountChanged(msg.sender, oldValue, count);
    }

    function decrement() public {
        require(count > 0, "Counter underflow");
        uint64 oldValue = count;
        count -= 1;
        emit CountChanged(msg.sender, oldValue, count);
    }

    function getCount() public view returns (uint64) {
        return count;
    }

    function add(uint64 value) public {
        uint64 oldValue = count;
        count += value;
        emit CountChanged(msg.sender, oldValue, count);
    }

    function reset() public {
        require(msg.sender == owner, "Only owner");
        uint64 oldValue = count;
        count = 0;
        emit CountChanged(msg.sender, oldValue, 0);
    }
}
