//! Template registry and metadata definitions

use super::embedded;

/// Template difficulty level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Advanced,
}

impl std::fmt::Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Difficulty::Beginner => write!(f, "Beginner"),
            Difficulty::Intermediate => write!(f, "Intermediate"),
            Difficulty::Advanced => write!(f, "Advanced"),
        }
    }
}

/// Metadata for a project template
#[derive(Debug, Clone)]
pub struct TemplateMetadata {
    /// Unique template identifier
    pub id: &'static str,
    /// Human-readable name
    pub name: &'static str,
    /// Short description
    pub description: &'static str,
    /// Difficulty level
    pub difficulty: Difficulty,
    /// SolScript features demonstrated
    pub features: &'static [&'static str],
}

/// A complete template with metadata and file contents
#[derive(Debug, Clone)]
pub struct Template {
    pub metadata: TemplateMetadata,
    pub main_sol: &'static str,
    pub config_template: &'static str,
    pub readme_template: &'static str,
    pub gitignore: &'static str,
}

/// All available templates
pub static TEMPLATES: &[Template] = &[
    // Beginner templates
    Template {
        metadata: TemplateMetadata {
            id: "simple",
            name: "Simple",
            description: "Minimal contract for learning",
            difficulty: Difficulty::Beginner,
            features: &["state variables", "constructor", "view functions"],
        },
        main_sol: embedded::SIMPLE_MAIN,
        config_template: embedded::SIMPLE_CONFIG,
        readme_template: embedded::SIMPLE_README,
        gitignore: embedded::GITIGNORE,
    },
    Template {
        metadata: TemplateMetadata {
            id: "counter",
            name: "Counter",
            description: "Counter with ownership and access control",
            difficulty: Difficulty::Beginner,
            features: &["events", "errors", "modifiers", "access control"],
        },
        main_sol: embedded::COUNTER_MAIN,
        config_template: embedded::COUNTER_CONFIG,
        readme_template: embedded::COUNTER_README,
        gitignore: embedded::GITIGNORE,
    },
    // Intermediate templates
    Template {
        metadata: TemplateMetadata {
            id: "token",
            name: "Token",
            description: "ERC20-style fungible token",
            difficulty: Difficulty::Intermediate,
            features: &["mappings", "transfers", "approvals", "pausable", "mintable"],
        },
        main_sol: embedded::TOKEN_MAIN,
        config_template: embedded::TOKEN_CONFIG,
        readme_template: embedded::TOKEN_README,
        gitignore: embedded::GITIGNORE,
    },
    Template {
        metadata: TemplateMetadata {
            id: "voting",
            name: "Voting",
            description: "Decentralized voting system",
            difficulty: Difficulty::Intermediate,
            features: &["structs", "enums", "time-based logic", "weighted votes"],
        },
        main_sol: embedded::VOTING_MAIN,
        config_template: embedded::VOTING_CONFIG,
        readme_template: embedded::VOTING_README,
        gitignore: embedded::GITIGNORE,
    },
    // Advanced templates
    Template {
        metadata: TemplateMetadata {
            id: "escrow",
            name: "Escrow",
            description: "Trustless escrow with dispute resolution",
            difficulty: Difficulty::Advanced,
            features: &["state machine", "multi-party", "deadlines", "dispute resolution"],
        },
        main_sol: embedded::ESCROW_MAIN,
        config_template: embedded::ESCROW_CONFIG,
        readme_template: embedded::ESCROW_README,
        gitignore: embedded::GITIGNORE,
    },
    Template {
        metadata: TemplateMetadata {
            id: "nft",
            name: "NFT",
            description: "ERC721-style NFT collection",
            difficulty: Difficulty::Advanced,
            features: &["metadata", "minting", "approvals", "operator pattern"],
        },
        main_sol: embedded::NFT_MAIN,
        config_template: embedded::NFT_CONFIG,
        readme_template: embedded::NFT_README,
        gitignore: embedded::GITIGNORE,
    },
];
