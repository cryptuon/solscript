use miette::Diagnostic;
use std::time::Instant;

use crate::routes::{BuildError, BuildResponse};

/// Compile SolScript source code through the full pipeline.
/// Currently generates Anchor code only (no BPF compilation without Solana toolchain).
/// BPF compilation will be added when running inside Docker with cargo-build-sbf available.
pub async fn compile_source(source: &str) -> Result<BuildResponse, BuildResponse> {
    let start = Instant::now();

    // Parse
    let program = match solscript_parser::parse(source) {
        Ok(prog) => prog,
        Err(err) => {
            let (offset, len) = if let Some(labels) = err.labels() {
                let labels: Vec<_> = labels.collect();
                if let Some(label) = labels.first() {
                    (Some(label.offset()), Some(label.offset() + label.len()))
                } else {
                    (None, None)
                }
            } else {
                (None, None)
            };

            return Err(BuildResponse {
                success: false,
                bytecode: None,
                idl: None,
                errors: vec![BuildError {
                    phase: "parse".to_string(),
                    message: err.to_string(),
                    start: offset,
                    end: len,
                }],
                build_time_secs: Some(start.elapsed().as_secs_f64()),
            });
        }
    };

    // Type check
    if let Err(errors) = solscript_typeck::typecheck(&program, source) {
        let build_errors: Vec<BuildError> = errors
            .iter()
            .map(|err| {
                let (offset, len) = if let Some(labels) = err.labels() {
                    let labels: Vec<_> = labels.collect();
                    if let Some(label) = labels.first() {
                        (Some(label.offset()), Some(label.offset() + label.len()))
                    } else {
                        (None, None)
                    }
                } else {
                    (None, None)
                };
                BuildError {
                    phase: "typecheck".to_string(),
                    message: err.to_string(),
                    start: offset,
                    end: len,
                }
            })
            .collect();

        return Err(BuildResponse {
            success: false,
            bytecode: None,
            idl: None,
            errors: build_errors,
            build_time_secs: Some(start.elapsed().as_secs_f64()),
        });
    }

    // Generate code
    let project = match solscript_codegen::generate(&program) {
        Ok(proj) => proj,
        Err(err) => {
            return Err(BuildResponse {
                success: false,
                bytecode: None,
                idl: None,
                errors: vec![BuildError {
                    phase: "codegen".to_string(),
                    message: err.to_string(),
                    start: None,
                    end: None,
                }],
                build_time_secs: Some(start.elapsed().as_secs_f64()),
            });
        }
    };

    // TODO: When running in Docker with cargo-build-sbf, write project to temp dir,
    // run cargo build-sbf, read the .so file, and return base64-encoded bytecode.
    // For now, return the IDL and generated code as confirmation of successful compilation.

    Ok(BuildResponse {
        success: true,
        bytecode: None, // Will be populated when BPF build is available
        idl: Some(project.idl_json),
        errors: vec![],
        build_time_secs: Some(start.elapsed().as_secs_f64()),
    })
}
