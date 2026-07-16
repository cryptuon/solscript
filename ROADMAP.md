# SolScript Roadmap

SolScript is a Solidity-to-Solana compiler framework: write contracts in familiar Solidity syntax and compile them to **native Solana BPF programs** with full Anchor compatibility and automatic PDA derivation. This document is the public, product-level roadmap — where SolScript is going and, concretely, the cheapest path to a production-grade compiler. For the granular per-crate implementation checklist, see [`docs/roadmap.md`](docs/roadmap.md).

> **Related:** [Site](https://solscript.cryptuon.com/) · [Docs](https://docs.cryptuon.com/solscript/) · [crates.io](https://crates.io/crates/solscript-cli) · [README](README.md)

## Vision

The smart-contract world is consolidating onto a few high-performance chains, and Solana is one of them — but the developer base, the audited patterns, and the tooling still live overwhelmingly in Solidity. The industry's answer is **chain abstraction: one language, many chains.** SolScript is the Solidity-side of that answer for Solana.

The goal is a compiler where a Solidity-fluent team can:

1. Write contracts in Solidity syntax they already know.
2. Compile to a **native** Solana program — not an emulated EVM, no interpreter in the hot path.
3. Read, audit, and eject to idiomatic Anchor/Rust at any time.
4. Trust that the same source can target EVM chains and Solana as a normal multi-chain deployment.

Success looks like: a team ships a Solidity codebase to Solana mainnet, their auditors review the generated Anchor code as if it were hand-written, and the deploy costs a few SOL — not a quarter of engineering rewrite time.

---

## Cheapest path to production

SolScript is a **developer tool**, so "production" does not mean running our own chain or standing up infrastructure. For a compiler, **production means two things**:

1. **A published, spec-conformant compiler** — installable from a package registry, versioned, with a documented, stable subset of Solidity it provably compiles correctly.
2. **Compiled programs that deploy to real Solana** — the artifacts SolScript emits go from **devnet → mainnet** and behave as specified on-chain.

That framing makes the cheapest path clear, because both halves are inherently low-cost:

- **Publish the compiler.** Ship `solscript-cli` to [crates.io](https://crates.io/crates/solscript-cli) (and an npm wrapper for JS/TS teams). Distribution is essentially free — no servers, no hosting bill. The cost is purely engineering discipline: a stable CLI surface, semver, and release automation.
- **Target Solana, which is cheap to deploy to.** Solana program deploys and transactions cost fractions of a cent to a few SOL. Proving the pipeline end-to-end — compile an example, deploy to **devnet** (free faucet SOL), exercise it, then promote the *same artifact* to **mainnet** — costs almost nothing. There is no L1/L2 infrastructure for us to run.

So the cheapest path to production is: **publish the compiler to public registries and prove compiled programs deploy devnet → mainnet.** The spend is measured in engineering hours and a handful of SOL, not infrastructure.

### Production-viability checklist

Cheap to *deploy* is not the same as *ready*. A compiler people trust with on-chain value has to clear these gates. This is the bar for calling SolScript production-grade:

| Gate | What "done" means | Why it matters |
|------|-------------------|----------------|
| **Solidity language-feature coverage matrix** | A published, tested table of exactly which Solidity features compile, which are partial, and which are unsupported — kept honest and in sync with the code | Users must know precisely what will and won't compile before they commit a port. Ambiguity here is how mainnet bugs happen. |
| **Conformance test suite** | A golden corpus of Solidity contracts with asserted on-chain behavior, run in CI on every commit; coverage tracked against the matrix | The only way to claim "spec-conformant" and defend it against regressions as the language surface grows. |
| **Audited codegen** | The Anchor/BPF code generator reviewed by a third party, with the emitted account/PDA/CPI patterns validated against known-good Anchor idioms | The generated program holds real value. A codegen bug is a vulnerability in *every* contract it produces. |
| **Deterministic builds** | Byte-for-byte reproducible artifacts from a given source + compiler version, so an auditor can independently reproduce the deployed `.so` | Reproducibility is the foundation of auditability and supply-chain trust for on-chain programs. |
| **Documentation** | Complete language guide, type/builtin reference, migration notes for Solidity devs, and clear "here's what's different on Solana" guidance | Adoption is gated by docs as much as by the compiler. A tool nobody can learn isn't in production. |
| **Versioning & stability** | Semver on the compiler and language, a documented deprecation policy, and pinned compiler versions per project | Contracts are long-lived. Teams need to pin a compiler version and know their build won't silently change. |

Meeting these gates — not merely getting *a* program onto mainnet — is what turns "it compiles" into "you can build on it."

---

## Milestones

Status reflects the compiler as of `v0.1.x`. The core pipeline (parse → typecheck → Anchor/BPF codegen) is working end-to-end; the near-term work is coverage, conformance, and distribution.

### Now — v0.1.x → v0.2 · Foundation & distribution

- [x] End-to-end pipeline: Solidity source → typed AST → Anchor/Rust codegen → deployable `.so`
- [x] Automatic PDA derivation from `mapping` types
- [x] Direct LLVM BPF compilation mode
- [x] Basic language server (LSP) + VS Code extension
- [x] Working example contracts (token, escrow, voting, NFT, staking, AMM)
- [ ] **Publish `solscript-cli` to crates.io** (cheapest-path item #1)
- [ ] npm wrapper for JS/TS teams
- [ ] Release automation + semver policy documented
- [ ] First public **Solidity language-feature coverage matrix**

### Next — v0.3 · Conformance & trust

- [ ] Golden **conformance test suite** wired into CI, tracked against the coverage matrix
- [ ] **Deterministic / reproducible builds** — pinned toolchain, byte-stable artifacts
- [ ] Documented **devnet → mainnet** deploy flow for compiled programs, with a promotion checklist
- [ ] Expand language coverage toward the most-requested Solidity features (see [Limitations](README.md#current-limitations))
- [ ] Migration guide for Solidity developers ("what's different on Solana")

### Later — v0.4+ · Production-grade & ecosystem

- [ ] **Third-party audit of the codegen** and the emitted Anchor/PDA/CPI patterns
- [ ] Token 2022 CPI generation
- [ ] Incoming `msg.value` / native SOL handling that maps cleanly to Solana's model
- [ ] Package manager for reusable SolScript modules
- [ ] Broader Solidity coverage toward a stable, versioned language spec
- [ ] `v1.0` — stable language, audited codegen, full conformance suite green

Milestones are directional, not dated. The ordering reflects the cheapest-path priority: **publish and prove first, then broaden coverage, then audit toward `v1.0`.**

---

## How to help

Coverage, conformance tests, and documentation are the highest-leverage contributions right now — every contract that compiles correctly and every gap the coverage matrix documents moves SolScript toward production. See [Contributing](README.md#contributing) in the README and the internal [`docs/`](docs/) for language spec and design decisions.

---

*SolScript is part of [Cryptuon Research](https://www.cryptuon.com) — blockchain theory, shipped as protocols. Questions: [contact@cryptuon.com](mailto:contact@cryptuon.com).*
