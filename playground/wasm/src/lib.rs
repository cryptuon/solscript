use miette::Diagnostic;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct WasmDiagnostic {
    pub severity: String,
    pub message: String,
    pub code: Option<String>,
    pub start: usize,
    pub end: usize,
    pub help: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ParseResult {
    pub success: bool,
    pub diagnostics: Vec<WasmDiagnostic>,
}

#[derive(Serialize, Deserialize)]
pub struct CheckResult {
    pub success: bool,
    pub diagnostics: Vec<WasmDiagnostic>,
}

#[derive(Serialize, Deserialize)]
pub struct CompileResult {
    pub success: bool,
    pub lib_rs: Option<String>,
    pub state_rs: Option<String>,
    pub instructions_rs: Option<String>,
    pub error_rs: Option<String>,
    pub events_rs: Option<String>,
    pub client_ts: Option<String>,
    pub tests_ts: Option<String>,
    pub idl_json: Option<String>,
    pub anchor_toml: Option<String>,
    pub cargo_toml: Option<String>,
    pub package_json: Option<String>,
    pub diagnostics: Vec<WasmDiagnostic>,
}

fn parse_error_to_diagnostic(err: &solscript_parser::ParseError) -> WasmDiagnostic {
    let message = err.to_string();
    let code = err.code().map(|c| c.to_string());
    let help = err.help().map(|h| h.to_string());

    let (start, end) = if let Some(labels) = err.labels() {
        let labels: Vec<_> = labels.collect();
        if let Some(label) = labels.first() {
            (label.offset(), label.offset() + label.len())
        } else {
            (0, 0)
        }
    } else {
        (0, 0)
    };

    WasmDiagnostic {
        severity: "error".to_string(),
        message,
        code,
        start,
        end,
        help,
    }
}

fn type_error_to_diagnostic(err: &solscript_typeck::TypeError) -> WasmDiagnostic {
    let message = err.to_string();
    let code = err.code().map(|c| c.to_string());
    let help = err.help().map(|h| h.to_string());

    let (start, end) = if let Some(labels) = err.labels() {
        let labels: Vec<_> = labels.collect();
        if let Some(label) = labels.first() {
            (label.offset(), label.offset() + label.len())
        } else {
            (0, 0)
        }
    } else {
        (0, 0)
    };

    WasmDiagnostic {
        severity: "error".to_string(),
        message,
        code,
        start,
        end,
        help,
    }
}

/// Parse SolScript source code and return diagnostics as JSON
#[wasm_bindgen]
pub fn parse(source: &str) -> String {
    let result = match solscript_parser::parse(source) {
        Ok(_) => ParseResult {
            success: true,
            diagnostics: vec![],
        },
        Err(err) => ParseResult {
            success: false,
            diagnostics: vec![parse_error_to_diagnostic(&err)],
        },
    };
    serde_json::to_string(&result).unwrap_or_else(|e| {
        format!(r#"{{"success":false,"diagnostics":[{{"severity":"error","message":"Serialization error: {}","code":null,"start":0,"end":0,"help":null}}]}}"#, e)
    })
}

/// Parse and type-check SolScript source code, return diagnostics as JSON
#[wasm_bindgen]
pub fn check(source: &str) -> String {
    let program = match solscript_parser::parse(source) {
        Ok(prog) => prog,
        Err(err) => {
            let result = CheckResult {
                success: false,
                diagnostics: vec![parse_error_to_diagnostic(&err)],
            };
            return serde_json::to_string(&result).unwrap_or_default();
        }
    };

    let result = match solscript_typeck::typecheck(&program, source) {
        Ok(()) => CheckResult {
            success: true,
            diagnostics: vec![],
        },
        Err(errors) => CheckResult {
            success: false,
            diagnostics: errors.iter().map(type_error_to_diagnostic).collect(),
        },
    };
    serde_json::to_string(&result).unwrap_or_default()
}

/// Full compilation pipeline: parse + typecheck + codegen, return generated code as JSON
#[wasm_bindgen]
pub fn compile(source: &str) -> String {
    // Parse
    let program = match solscript_parser::parse(source) {
        Ok(prog) => prog,
        Err(err) => {
            let result = CompileResult {
                success: false,
                lib_rs: None,
                state_rs: None,
                instructions_rs: None,
                error_rs: None,
                events_rs: None,
                client_ts: None,
                tests_ts: None,
                idl_json: None,
                anchor_toml: None,
                cargo_toml: None,
                package_json: None,
                diagnostics: vec![parse_error_to_diagnostic(&err)],
            };
            return serde_json::to_string(&result).unwrap_or_default();
        }
    };

    // Type check
    if let Err(errors) = solscript_typeck::typecheck(&program, source) {
        let result = CompileResult {
            success: false,
            lib_rs: None,
            state_rs: None,
            instructions_rs: None,
            error_rs: None,
            events_rs: None,
            client_ts: None,
            tests_ts: None,
            idl_json: None,
            anchor_toml: None,
            cargo_toml: None,
            package_json: None,
            diagnostics: errors.iter().map(type_error_to_diagnostic).collect(),
        };
        return serde_json::to_string(&result).unwrap_or_default();
    }

    // Generate code
    match solscript_codegen::generate(&program) {
        Ok(project) => {
            let result = CompileResult {
                success: true,
                lib_rs: Some(project.lib_rs),
                state_rs: Some(project.state_rs),
                instructions_rs: Some(project.instructions_rs),
                error_rs: Some(project.error_rs),
                events_rs: Some(project.events_rs),
                client_ts: Some(project.client_ts),
                tests_ts: Some(project.tests_ts),
                idl_json: Some(project.idl_json),
                anchor_toml: Some(project.anchor_toml),
                cargo_toml: Some(project.cargo_toml),
                package_json: Some(project.package_json),
                diagnostics: vec![],
            };
            serde_json::to_string(&result).unwrap_or_default()
        }
        Err(err) => {
            let result = CompileResult {
                success: false,
                lib_rs: None,
                state_rs: None,
                instructions_rs: None,
                error_rs: None,
                events_rs: None,
                client_ts: None,
                tests_ts: None,
                idl_json: None,
                anchor_toml: None,
                cargo_toml: None,
                package_json: None,
                diagnostics: vec![WasmDiagnostic {
                    severity: "error".to_string(),
                    message: err.to_string(),
                    code: Some("solscript::codegen".to_string()),
                    start: 0,
                    end: 0,
                    help: None,
                }],
            };
            serde_json::to_string(&result).unwrap_or_default()
        }
    }
}
