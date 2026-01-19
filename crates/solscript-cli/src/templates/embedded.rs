//! Embedded template files
//!
//! This module contains all template files embedded at compile time
//! using include_str! macro.

// Simple template
pub const SIMPLE_MAIN: &str = include_str!("../../templates/simple/main.sol");
pub const SIMPLE_CONFIG: &str = include_str!("../../templates/simple/solscript.toml.template");
pub const SIMPLE_README: &str = include_str!("../../templates/simple/README.md.template");

// Counter template
pub const COUNTER_MAIN: &str = include_str!("../../templates/counter/main.sol");
pub const COUNTER_CONFIG: &str = include_str!("../../templates/counter/solscript.toml.template");
pub const COUNTER_README: &str = include_str!("../../templates/counter/README.md.template");

// Token template
pub const TOKEN_MAIN: &str = include_str!("../../templates/token/main.sol");
pub const TOKEN_CONFIG: &str = include_str!("../../templates/token/solscript.toml.template");
pub const TOKEN_README: &str = include_str!("../../templates/token/README.md.template");

// Voting template
pub const VOTING_MAIN: &str = include_str!("../../templates/voting/main.sol");
pub const VOTING_CONFIG: &str = include_str!("../../templates/voting/solscript.toml.template");
pub const VOTING_README: &str = include_str!("../../templates/voting/README.md.template");

// Escrow template
pub const ESCROW_MAIN: &str = include_str!("../../templates/escrow/main.sol");
pub const ESCROW_CONFIG: &str = include_str!("../../templates/escrow/solscript.toml.template");
pub const ESCROW_README: &str = include_str!("../../templates/escrow/README.md.template");

// NFT template
pub const NFT_MAIN: &str = include_str!("../../templates/nft/main.sol");
pub const NFT_CONFIG: &str = include_str!("../../templates/nft/solscript.toml.template");
pub const NFT_README: &str = include_str!("../../templates/nft/README.md.template");

// Shared gitignore for all templates
pub const GITIGNORE: &str = r#"# Build outputs
/output/
/target/

# IDE
.idea/
.vscode/
*.swp
*.swo

# OS
.DS_Store
Thumbs.db

# Anchor
.anchor/
node_modules/
"#;
