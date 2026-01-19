// Hello World - Your first SolScript contract
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
