// Simple counter - minimal example for learning SolScript
// This is the simplest possible contract to get started

contract Simple {
    // State variables
    uint64 public count;

    // Constructor - runs once at deployment
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
