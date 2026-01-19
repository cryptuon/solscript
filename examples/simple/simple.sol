// Simple counter - minimal example for testing BPF compilation

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
