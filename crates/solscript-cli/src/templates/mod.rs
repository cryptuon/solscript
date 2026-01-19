//! Template system for SolScript projects
//!
//! This module provides pre-defined project templates that can be used
//! to quickly scaffold new SolScript projects with working example code.

mod embedded;
mod registry;

pub use registry::{Difficulty, Template, TemplateMetadata, TEMPLATES};

/// Get a template by its ID
pub fn get_template(id: &str) -> Option<&'static Template> {
    TEMPLATES.iter().find(|t| t.metadata.id == id)
}

/// Get all available template IDs
pub fn template_ids() -> Vec<&'static str> {
    TEMPLATES.iter().map(|t| t.metadata.id).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_template() {
        assert!(get_template("counter").is_some());
        assert!(get_template("nonexistent").is_none());
    }

    #[test]
    fn test_all_templates_exist() {
        for id in ["simple", "counter", "token", "voting", "escrow", "nft"] {
            assert!(get_template(id).is_some(), "Template '{}' should exist", id);
        }
    }

    #[test]
    fn test_template_ids() {
        let ids = template_ids();
        assert!(ids.contains(&"counter"));
        assert!(ids.contains(&"token"));
    }
}
