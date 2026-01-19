# SolScript

**A high-level language for Solana smart contract development**

SolScript brings Solidity-style syntax to Solana, making it easier for developers to write secure and efficient smart contracts for the Solana blockchain.

## Features

- **Familiar Syntax** - Solidity-style syntax that's easy to learn
- **Automatic PDA Handling** - Mappings become PDAs automatically
- **Solana Native** - Compiles to optimized Solana BPF programs
- **Type Safe** - Strong static typing catches errors at compile time
- **SPL Token Support** - Built-in token transfer, mint, burn operations
- **IDE Support** - Language server for VS Code and other editors
- **Fast Compilation** - Quick feedback during development

!!! info "Beta Status"
    SolScript is in beta. Most features work well, but some Solana-specific
    capabilities are still in development. See the [Roadmap](reference/roadmap.md)
    for current limitations and planned improvements.

## Quick Example

```solscript
contract Counter {
    @state count: u64;
    @state owner: Address;

    event CountChanged(by: Address, newValue: u64);

    fn init() {
        self.owner = tx.sender;
        self.count = 0;
    }

    @public
    fn increment() {
        self.count += 1;
        emit CountChanged(tx.sender, self.count);
    }

    @public
    @view
    fn get_count(): u64 {
        return self.count;
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

### Key Advantages

- **Automatic PDAs**: Mappings become PDA-based storage automatically
- **Readable**: Code that's easy to understand and audit
- **Safe**: Built-in overflow protection and access control
- **Fast**: Generates optimized Anchor/Rust code
- **Extensible**: Generated code can be customized when needed

### Current Limitations

SolScript handles most common patterns, but some Solana features require workarounds:

- **No incoming SOL payments** - `msg.value` returns 0, use wrapped SOL for receiving payments
- **No Token 2022** - Only SPL Token supported
- **See [Roadmap](reference/roadmap.md)** for full details

!!! success "New in v0.3.0"
    - **SOL transfers** - Use `transfer(to, amount)` to send SOL
    - **Mapping cleanup** - Use `delete mapping[key]` to close PDAs and reclaim rent
    - **Structs/enums in contracts** - Define types inside contract bodies

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
