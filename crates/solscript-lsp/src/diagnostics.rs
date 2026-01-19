//! Diagnostics generation for the language server

use crate::Document;
use tower_lsp::lsp_types::*;

/// Get diagnostics for a document
pub fn get_diagnostics(doc: &Document) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    // Add parse errors
    for error in &doc.parse_errors {
        diagnostics.push(Diagnostic {
            range: Range {
                start: Position::new(0, 0),
                end: Position::new(0, 0),
            },
            severity: Some(DiagnosticSeverity::ERROR),
            code: Some(NumberOrString::String("parse-error".to_string())),
            source: Some("solscript".to_string()),
            message: error.clone(),
            ..Default::default()
        });
    }

    // Add type errors - convert to strings since we can't easily access internal spans
    for error in &doc.type_errors {
        let message = format!("{}", error);
        diagnostics.push(Diagnostic {
            range: Range {
                start: Position::new(0, 0),
                end: Position::new(0, 0),
            },
            severity: Some(DiagnosticSeverity::ERROR),
            code: Some(NumberOrString::String("type-error".to_string())),
            source: Some("solscript".to_string()),
            message,
            ..Default::default()
        });
    }

    diagnostics
}
