# SolScript

**A high-level language for Solana smart contract development**

SolScript brings Solidity-style syntax to Solana, making it easier for developers to write secure and efficient smart contracts for the Solana blockchain.

## Features

- **Familiar Syntax** - Solidity-style syntax that's easy to learn
- **Solana Native** - Compiles to optimized Solana BPF programs
- **Type Safe** - Strong static typing catches errors at compile time
- **Built-in Testing** - Write and run tests directly in your contracts
- **IDE Support** - Language server for VS Code and other editors
- **Fast Compilation** - Quick feedback during development

## Quick Example

```solidity
contract Counter {
    uint64 public count;
    address public owner;

    event CountChanged(address indexed by, uint64 newValue);

    constructor() {
        owner = msg.sender;
        count = 0;
    }

    function increment() public {
        count += 1;
        emit CountChanged(msg.sender, count);
    }

    function getCount() public view returns (uint64) {
        return count;
    }
}
```

## Getting Started

<div class="grid cards" markdown>

-   :material-download:{ .lg .middle } **Installation**

    ---

    Install SolScript and set up your development environment

    [:octicons-arrow-right-24: Install now](guide/installation.md)

-   :material-rocket-launch:{ .lg .middle } **Quick Start**

    ---

    Create your first SolScript project in minutes

    [:octicons-arrow-right-24: Get started](guide/quickstart.md)

-   :material-book-open-variant:{ .lg .middle } **Language Guide**

    ---

    Learn the SolScript language from basics to advanced

    [:octicons-arrow-right-24: Read the guide](guide/overview.md)

-   :material-code-tags:{ .lg .middle } **Examples**

    ---

    Browse example contracts and tutorials

    [:octicons-arrow-right-24: View examples](examples/counter.md)

</div>

## Why SolScript?

### For Solidity Developers

If you're coming from Ethereum, SolScript feels like home. Use familiar syntax while targeting the high-performance Solana blockchain.

### For Solana Developers

Skip the boilerplate of Anchor/Rust. Write cleaner, more maintainable code with automatic account management and PDA derivation.

### For Everyone

- **Readable**: Code that's easy to understand and audit
- **Safe**: Built-in overflow protection and access control
- **Fast**: Optimized compilation to Solana BPF
- **Tested**: Comprehensive testing framework included

## Installation

```bash
# Install via cargo
cargo install solscript

# Or build from source
git clone https://github.com/solscript/solscript
cd solscript
cargo install --path crates/solscript-cli
```

## Community

- [GitHub](https://github.com/solscript/solscript) - Source code and issues
- [Discord](#) - Community chat
- [Twitter](https://twitter.com/solscript) - Updates and announcements

## License

SolScript is open source, licensed under MIT or Apache-2.0.
