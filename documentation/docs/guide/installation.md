# Installation

This guide covers installing SolScript and its dependencies.

## Prerequisites

Before installing SolScript, ensure you have:

- **Rust** (1.83 or later) - [Install Rust](https://rustup.rs/)
- **Solana CLI** - [Install Solana](https://docs.solana.com/cli/install-solana-cli-tools)
- **Anchor** (optional, for Anchor mode) - [Install Anchor](https://www.anchor-lang.com/docs/installation)
- **LLVM 18** (optional, for direct LLVM mode) - See below

## Install SolScript

### From Cargo (Recommended)

```bash
cargo install solscript
```

### From Source

```bash
git clone https://github.com/cryptuon/solscript
cd solscript
cargo install --path crates/solscript-cli
```

### With LLVM Support (Direct BPF Compilation)

```bash
# Install LLVM 18 first (see below), then:
cargo install --path crates/solscript-cli --features llvm
```

### Verify Installation

```bash
solscript --version
```

You should see output like:

```
solscript 0.1.0
```

## Check Your Environment

Run the doctor command to verify all tools are installed:

```bash
solscript doctor
```

Expected output:

```
SolScript Build Environment

✓ cargo build-sbf: solana-cargo-build-sbf 1.18.0
✓ solana: solana-cli 1.18.0
✓ anchor: anchor-cli 0.29.0

✓ Ready to build SolScript programs
```

## Installing Dependencies

### Solana CLI

```bash
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

Add to your PATH:

```bash
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
```

### Anchor Framework

```bash
# Install AVM (Anchor Version Manager)
cargo install --git https://github.com/coral-xyz/anchor avm --locked

# Install latest Anchor
avm install latest
avm use latest
```

### LLVM 18 (For Direct BPF Compilation)

Direct LLVM compilation bypasses Anchor/Rust for faster builds. Requires LLVM 18.

**Ubuntu/Debian:**
```bash
wget https://apt.llvm.org/llvm.sh
chmod +x llvm.sh
sudo ./llvm.sh 18
sudo apt install llvm-18-dev libpolly-18-dev
export LLVM_SYS_180_PREFIX=/usr/lib/llvm-18
```

**macOS:**
```bash
brew install llvm@18
export LLVM_SYS_180_PREFIX=$(brew --prefix llvm@18)
```

**Build with LLVM feature:**
```bash
cargo build -p solscript-bpf --features llvm
```

**Verify LLVM:**
```bash
llvm-config-18 --version  # Should show 18.x
```

## IDE Setup

### VS Code

Install the SolScript extension from the VS Code marketplace, or manually:

1. Download the `.vsix` file from the releases page
2. In VS Code: Extensions → ... → Install from VSIX

The extension provides:

- Syntax highlighting
- Error diagnostics
- Go to definition
- Hover information
- Auto-completion

### Other Editors

SolScript provides a Language Server Protocol (LSP) implementation that works with any LSP-compatible editor:

```bash
# Start the language server
solscript-lsp
```

Configure your editor to use `solscript-lsp` as the language server for `.sol` files.

## Troubleshooting

### "cargo build-sbf not found"

Ensure Solana CLI is installed and in your PATH:

```bash
which solana
solana --version
```

If not found, reinstall Solana CLI and add it to your PATH.

### "anchor not found"

Install Anchor using AVM:

```bash
cargo install --git https://github.com/coral-xyz/anchor avm --locked
avm install latest
avm use latest
```

### Build Errors

If you encounter build errors, try:

1. Update Rust: `rustup update`
2. Update Solana: `solana-install update`
3. Clear cargo cache: `cargo clean`

## Next Steps

- [Quick Start](quickstart.md) - Create your first project
- [Your First Contract](first-contract.md) - Write your first smart contract
