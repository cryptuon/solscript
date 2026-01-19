//! Type checking error types

use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

use crate::types::Type;

/// A type checking error
#[derive(Error, Debug, Diagnostic)]
pub enum TypeError {
    #[error("Type mismatch: expected `{expected}`, found `{found}`")]
    #[diagnostic(
        code(solscript::typeck::mismatch),
        help("ensure the value type matches the expected type")
    )]
    TypeMismatch {
        expected: String,
        found: String,
        #[label("expected `{expected}`, found `{found}`")]
        span: SourceSpan,
        #[source_code]
        src: String,
    },

    #[error("Undefined variable: `{name}`")]
    #[diagnostic(
        code(solscript::typeck::undefined_var),
        help("check spelling, or declare the variable before use")
    )]
    UndefinedVariable {
        name: String,
        #[label("not found in this scope")]
        span: SourceSpan,
        #[source_code]
        src: String,
    },

    #[error("Undefined type: `{name}`")]
    #[diagnostic(
        code(solscript::typeck::undefined_type),
        help("check spelling, or define the type (struct/enum/contract)")
    )]
    UndefinedType {
        name: String,
        #[label("unknown type")]
        span: SourceSpan,
        #[source_code]
        src: String,
    },

    #[error("Undefined function: `{name}`")]
    #[diagnostic(
        code(solscript::typeck::undefined_fn),
        help("check spelling, or define the function")
    )]
    UndefinedFunction {
        name: String,
        #[label("not found")]
        span: SourceSpan,
        #[source_code]
        src: String,
    },

    #[error("Undefined field: `{field}` on type `{ty}`")]
    #[diagnostic(
        code(solscript::typeck::undefined_field),
        help("check the struct definition for available fields")
    )]
    UndefinedField {
        field: String,
        ty: String,
        #[label("no field `{field}` on type `{ty}`")]
        span: SourceSpan,
        #[source_code]
        src: String,
    },

    #[error("Undefined method: `{method}` on type `{ty}`")]
    #[diagnostic(
        code(solscript::typeck::undefined_method),
        help("check available methods for this type")
    )]
    UndefinedMethod {
        method: String,
        ty: String,
        #[label("no method `{method}` on type `{ty}`")]
        span: SourceSpan,
        #[source_code]
        src: String,
    },

    #[error("Cannot call non-function type `{ty}`")]
    #[diagnostic(
        code(solscript::typeck::not_callable),
        help("only functions and interface methods can be called")
    )]
    NotCallable {
        ty: String,
        #[label("this is not a function")]
        span: SourceSpan,
        #[source_code]
        src: String,
    },

    #[error("Wrong number of arguments: expected {expected}, found {found}")]
    #[diagnostic(
        code(solscript::typeck::wrong_arg_count),
        help("check the function signature for required parameters")
    )]
    WrongArgCount {
        expected: usize,
        found: usize,
        #[label("expected {expected} argument(s), found {found}")]
        span: SourceSpan,
        #[source_code]
        src: String,
    },

    #[error("Cannot index type `{ty}`")]
    #[diagnostic(
        code(solscript::typeck::not_indexable),
        help("only arrays, mappings, and dynamic arrays can be indexed")
    )]
    NotIndexable {
        ty: String,
        #[label("cannot use [] on this type")]
        span: SourceSpan,
        #[source_code]
        src: String,
    },

    #[error("Cannot apply operator `{op}` to type `{ty}`")]
    #[diagnostic(
        code(solscript::typeck::invalid_unary_op),
        help("check that the operator is valid for this type")
    )]
    InvalidUnaryOp {
        op: String,
        ty: String,
        #[label("operator `{op}` cannot be applied to `{ty}`")]
        span: SourceSpan,
        #[source_code]
        src: String,
    },

    #[error("Cannot apply operator `{op}` to types `{left}` and `{right}`")]
    #[diagnostic(
        code(solscript::typeck::invalid_binary_op),
        help("ensure both operands have compatible types")
    )]
    InvalidBinaryOp {
        op: String,
        left: String,
        right: String,
        #[label("cannot apply `{op}` to `{left}` and `{right}`")]
        span: SourceSpan,
        #[source_code]
        src: String,
    },

    #[error("Duplicate definition: `{name}`")]
    #[diagnostic(
        code(solscript::typeck::duplicate),
        help("rename one of the definitions to avoid conflict")
    )]
    DuplicateDefinition {
        name: String,
        #[label("`{name}` is already defined")]
        span: SourceSpan,
        #[source_code]
        src: String,
    },

    #[error("Missing return value")]
    #[diagnostic(
        code(solscript::typeck::missing_return),
        help("add a return statement with the expected type")
    )]
    MissingReturn {
        expected: String,
        #[label("function expects to return `{expected}`")]
        span: SourceSpan,
        #[source_code]
        src: String,
    },

    #[error("Undefined event: `{name}`")]
    #[diagnostic(
        code(solscript::typeck::undefined_event),
        help("define the event before using it: event {name}(...);")
    )]
    UndefinedEvent {
        name: String,
        #[label("event `{name}` is not defined")]
        span: SourceSpan,
        #[source_code]
        src: String,
    },

    #[error("Undefined modifier: `{name}`")]
    #[diagnostic(
        code(solscript::typeck::undefined_modifier),
        help("define the modifier before using it")
    )]
    UndefinedModifier {
        name: String,
        #[label("modifier `{name}` is not defined")]
        span: SourceSpan,
        #[source_code]
        src: String,
    },

    #[error("Undefined error: `{name}`")]
    #[diagnostic(
        code(solscript::typeck::undefined_error),
        help("define the error before using it: error {name}(...);")
    )]
    UndefinedError {
        name: String,
        #[label("error `{name}` is not defined")]
        span: SourceSpan,
        #[source_code]
        src: String,
    },
}

impl TypeError {
    pub fn type_mismatch(expected: &Type, found: &Type, span: (usize, usize), src: &str) -> Self {
        Self::TypeMismatch {
            expected: expected.to_string(),
            found: found.to_string(),
            span: SourceSpan::new(span.0.into(), span.1 - span.0),
            src: src.to_string(),
        }
    }

    pub fn undefined_variable(name: &str, span: (usize, usize), src: &str) -> Self {
        Self::UndefinedVariable {
            name: name.to_string(),
            span: SourceSpan::new(span.0.into(), span.1 - span.0),
            src: src.to_string(),
        }
    }

    pub fn undefined_type(name: &str, span: (usize, usize), src: &str) -> Self {
        Self::UndefinedType {
            name: name.to_string(),
            span: SourceSpan::new(span.0.into(), span.1 - span.0),
            src: src.to_string(),
        }
    }

    pub fn undefined_field(field: &str, ty: &Type, span: (usize, usize), src: &str) -> Self {
        Self::UndefinedField {
            field: field.to_string(),
            ty: ty.to_string(),
            span: SourceSpan::new(span.0.into(), span.1 - span.0),
            src: src.to_string(),
        }
    }

    pub fn undefined_method(method: &str, ty: &Type, span: (usize, usize), src: &str) -> Self {
        Self::UndefinedMethod {
            method: method.to_string(),
            ty: ty.to_string(),
            span: SourceSpan::new(span.0.into(), span.1 - span.0),
            src: src.to_string(),
        }
    }

    pub fn not_callable(ty: &Type, span: (usize, usize), src: &str) -> Self {
        Self::NotCallable {
            ty: ty.to_string(),
            span: SourceSpan::new(span.0.into(), span.1 - span.0),
            src: src.to_string(),
        }
    }

    pub fn wrong_arg_count(expected: usize, found: usize, span: (usize, usize), src: &str) -> Self {
        Self::WrongArgCount {
            expected,
            found,
            span: SourceSpan::new(span.0.into(), span.1 - span.0),
            src: src.to_string(),
        }
    }

    pub fn invalid_binary_op(
        op: &str,
        left: &Type,
        right: &Type,
        span: (usize, usize),
        src: &str,
    ) -> Self {
        Self::InvalidBinaryOp {
            op: op.to_string(),
            left: left.to_string(),
            right: right.to_string(),
            span: SourceSpan::new(span.0.into(), span.1 - span.0),
            src: src.to_string(),
        }
    }
}
