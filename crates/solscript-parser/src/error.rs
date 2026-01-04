//! Parser error types

use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

/// A parsing error
#[derive(Error, Debug, Diagnostic)]
pub enum ParseError {
    #[error("Syntax error: {message}")]
    #[diagnostic(code(solscript::parse::syntax))]
    Syntax {
        message: String,
        #[label("here")]
        span: SourceSpan,
        #[source_code]
        src: String,
    },

    #[error("Unexpected token: expected {expected}, found {found}")]
    #[diagnostic(code(solscript::parse::unexpected_token))]
    UnexpectedToken {
        expected: String,
        found: String,
        #[label("unexpected token")]
        span: SourceSpan,
        #[source_code]
        src: String,
    },

    #[error("Unexpected end of input")]
    #[diagnostic(code(solscript::parse::unexpected_eof))]
    UnexpectedEof {
        #[label("end of input")]
        span: SourceSpan,
        #[source_code]
        src: String,
    },

    #[error("Invalid integer literal: {message}")]
    #[diagnostic(code(solscript::parse::invalid_int))]
    InvalidInt {
        message: String,
        #[label("invalid integer")]
        span: SourceSpan,
        #[source_code]
        src: String,
    },

    #[error("Invalid float literal: {message}")]
    #[diagnostic(code(solscript::parse::invalid_float))]
    InvalidFloat {
        message: String,
        #[label("invalid float")]
        span: SourceSpan,
        #[source_code]
        src: String,
    },

    #[error("Invalid escape sequence")]
    #[diagnostic(code(solscript::parse::invalid_escape))]
    InvalidEscape {
        #[label("invalid escape")]
        span: SourceSpan,
        #[source_code]
        src: String,
    },
}

impl ParseError {
    pub fn syntax(message: impl Into<String>, span: (usize, usize), src: &str) -> Self {
        Self::Syntax {
            message: message.into(),
            span: SourceSpan::new(span.0.into(), (span.1 - span.0).into()),
            src: src.to_string(),
        }
    }

    pub fn unexpected_token(
        expected: impl Into<String>,
        found: impl Into<String>,
        span: (usize, usize),
        src: &str,
    ) -> Self {
        Self::UnexpectedToken {
            expected: expected.into(),
            found: found.into(),
            span: SourceSpan::new(span.0.into(), (span.1 - span.0).into()),
            src: src.to_string(),
        }
    }

    pub fn unexpected_eof(pos: usize, src: &str) -> Self {
        Self::UnexpectedEof {
            span: SourceSpan::new(pos.into(), 0usize.into()),
            src: src.to_string(),
        }
    }

    pub fn invalid_int(message: impl Into<String>, span: (usize, usize), src: &str) -> Self {
        Self::InvalidInt {
            message: message.into(),
            span: SourceSpan::new(span.0.into(), (span.1 - span.0).into()),
            src: src.to_string(),
        }
    }
}

/// Convert pest error to our ParseError
impl From<pest::error::Error<crate::Rule>> for ParseError {
    fn from(err: pest::error::Error<crate::Rule>) -> Self {
        let message = err.to_string();
        let (start, end) = match err.location {
            pest::error::InputLocation::Pos(p) => (p, p + 1),
            pest::error::InputLocation::Span((s, e)) => (s, e),
        };

        ParseError::Syntax {
            message,
            span: SourceSpan::new(start.into(), (end - start).into()),
            src: String::new(), // Will be filled in by caller
        }
    }
}
