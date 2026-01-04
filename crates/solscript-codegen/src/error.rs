//! Code generation errors

use thiserror::Error;

/// Errors that can occur during code generation
#[derive(Error, Debug)]
pub enum CodegenError {
    #[error("Unsupported feature: {0}")]
    UnsupportedFeature(String),

    #[error("Invalid state: {0}")]
    InvalidState(String),

    #[error("Type conversion error: {0}")]
    TypeConversion(String),

    #[error("Missing required element: {0}")]
    MissingElement(String),

    #[error("Code generation failed: {0}")]
    GenerationFailed(String),
}
