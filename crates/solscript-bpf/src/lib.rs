//! BPF Compilation for SolScript
//!
//! This module provides BPF bytecode compilation for SolScript programs.
//!
//! Two compilation modes are supported:
//! 1. **Standard mode** (default): Uses Rust/Anchor codegen + cargo build-sbf
//! 2. **Direct LLVM mode** (feature: `llvm`): Compiles directly to BPF via LLVM
//!
//! The standard mode is recommended for most use cases as it leverages the
//! well-tested Anchor framework. Direct LLVM mode provides faster compilation
//! but requires LLVM 18 with Polly support.

#[cfg(feature = "llvm")]
mod codegen;
#[cfg(feature = "llvm")]
mod types;
#[cfg(feature = "llvm")]
mod intrinsics;

use solscript_ast::Program;
use std::path::{Path, PathBuf};
use std::process::Command;
use thiserror::Error;

#[cfg(feature = "llvm")]
pub use codegen::Compiler;

/// Errors that can occur during BPF compilation
#[derive(Debug, Error)]
pub enum BpfError {
    #[error("Codegen error: {0}")]
    CodegenError(String),

    #[error("Build error: {0}")]
    BuildError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Tool not found: {0}")]
    ToolNotFound(String),

    #[cfg(feature = "llvm")]
    #[error("LLVM error: {0}")]
    LlvmError(String),

    #[cfg(feature = "llvm")]
    #[error("Target error: {0}")]
    TargetError(String),

    #[cfg(feature = "llvm")]
    #[error("Unsupported feature: {0}")]
    Unsupported(String),
}

pub type Result<T> = std::result::Result<T, BpfError>;

/// BPF compilation options
#[derive(Debug, Clone)]
pub struct CompileOptions {
    /// Optimization level (0-3)
    pub opt_level: u8,
    /// Generate debug information
    pub debug_info: bool,
    /// Output directory
    pub output_dir: PathBuf,
    /// Use cargo build-sbf (standard mode)
    pub use_cargo_sbf: bool,
    /// Keep intermediate files
    pub keep_intermediate: bool,
}

impl Default for CompileOptions {
    fn default() -> Self {
        Self {
            opt_level: 2,
            debug_info: false,
            output_dir: PathBuf::from("target/deploy"),
            use_cargo_sbf: true,
            keep_intermediate: false,
        }
    }
}

/// BPF compilation result
#[derive(Debug)]
pub struct CompileResult {
    /// Path to the compiled .so file
    pub program_path: PathBuf,
    /// Program ID (if available)
    pub program_id: Option<String>,
    /// Build duration in seconds
    pub build_time_secs: f64,
}

/// Compile a SolScript program to BPF
pub fn compile(
    program: &Program,
    source: &str,
    options: &CompileOptions,
) -> Result<CompileResult> {
    let start = std::time::Instant::now();

    if options.use_cargo_sbf {
        compile_via_anchor(program, source, options, start)
    } else {
        #[cfg(feature = "llvm")]
        {
            compile_direct_llvm(program, options, start)
        }
        #[cfg(not(feature = "llvm"))]
        {
            Err(BpfError::BuildError(
                "Direct LLVM compilation requires the 'llvm' feature".to_string(),
            ))
        }
    }
}

/// Compile via Anchor/cargo build-sbf (standard mode)
fn compile_via_anchor(
    program: &Program,
    source: &str,
    options: &CompileOptions,
    start: std::time::Instant,
) -> Result<CompileResult> {
    // First, type check
    if let Err(errors) = solscript_typeck::typecheck(program, source) {
        let msgs: Vec<_> = errors.iter().map(|e| e.to_string()).collect();
        return Err(BpfError::CodegenError(msgs.join("\n")));
    }

    // Generate Anchor code
    let generated = solscript_codegen::generate(program)
        .map_err(|e| BpfError::CodegenError(e.to_string()))?;

    // Write to output directory
    let anchor_dir = options.output_dir.join("anchor_project");
    generated
        .write_to_dir(&anchor_dir)
        .map_err(|e| BpfError::IoError(e))?;

    // Check if cargo build-sbf is available
    let build_sbf_available = Command::new("cargo")
        .args(["build-sbf", "--version"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if !build_sbf_available {
        // Try cargo build-bpf (older command)
        let build_bpf_available = Command::new("cargo")
            .args(["build-bpf", "--version"])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);

        if !build_bpf_available {
            return Err(BpfError::ToolNotFound(
                "cargo build-sbf (or cargo build-bpf) not found. \
                 Install with: cargo install solana-cli"
                    .to_string(),
            ));
        }
    }

    // Run cargo build-sbf
    let build_cmd = if build_sbf_available {
        "build-sbf"
    } else {
        "build-bpf"
    };

    let program_dir = anchor_dir.join("programs").join("solscript_program");

    let mut cmd = Command::new("cargo");
    cmd.arg(build_cmd);

    // Add optimization flags
    match options.opt_level {
        0 => {}
        1 => {
            cmd.env("CARGO_PROFILE_RELEASE_OPT_LEVEL", "1");
        }
        2 => {
            cmd.env("CARGO_PROFILE_RELEASE_OPT_LEVEL", "2");
        }
        _ => {
            cmd.env("CARGO_PROFILE_RELEASE_OPT_LEVEL", "3");
        }
    }

    cmd.current_dir(&program_dir);

    let output = cmd
        .output()
        .map_err(|e| BpfError::BuildError(format!("Failed to run {}: {}", build_cmd, e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(BpfError::BuildError(format!(
            "Build failed:\n{}",
            stderr
        )));
    }

    // Find the compiled .so file
    let deploy_dir = anchor_dir.join("target/deploy");
    let so_path = deploy_dir.join("solscript_program.so");

    if !so_path.exists() {
        // Try alternative path
        let alt_path = program_dir.join("target/deploy/solscript_program.so");
        if alt_path.exists() {
            let final_path = options.output_dir.join("solscript_program.so");
            std::fs::copy(&alt_path, &final_path)?;

            return Ok(CompileResult {
                program_path: final_path,
                program_id: read_program_id(&program_dir),
                build_time_secs: start.elapsed().as_secs_f64(),
            });
        }

        return Err(BpfError::BuildError(
            "Compiled program not found".to_string(),
        ));
    }

    // Copy to output directory
    let final_path = options.output_dir.join("solscript_program.so");
    std::fs::create_dir_all(&options.output_dir)?;
    std::fs::copy(&so_path, &final_path)?;

    // Clean up if not keeping intermediate files
    if !options.keep_intermediate {
        let _ = std::fs::remove_dir_all(&anchor_dir);
    }

    Ok(CompileResult {
        program_path: final_path,
        program_id: read_program_id(&program_dir),
        build_time_secs: start.elapsed().as_secs_f64(),
    })
}

/// Read program ID from the keypair file
fn read_program_id(program_dir: &Path) -> Option<String> {
    let keypair_path = program_dir.join("target/deploy/solscript_program-keypair.json");
    if keypair_path.exists() {
        // The keypair file contains the program ID
        // For now, just return None - we'd need to parse the keypair
        None
    } else {
        None
    }
}

#[cfg(feature = "llvm")]
fn compile_direct_llvm(
    program: &Program,
    options: &CompileOptions,
    start: std::time::Instant,
) -> Result<CompileResult> {
    use inkwell::context::Context;
    use inkwell::targets::{
        CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine, TargetTriple,
    };
    use inkwell::OptimizationLevel;

    // Initialize BPF target
    Target::initialize_bpf(&InitializationConfig::default());

    let context = Context::create();
    let module = context.create_module("solscript_program");

    // Compile to LLVM IR
    let mut compiler = Compiler::new(&context, &module);
    compiler.compile_program(program)?;

    // Verify module
    if let Err(msg) = module.verify() {
        return Err(BpfError::LlvmError(msg.to_string()));
    }

    // Set up BPF target
    let triple = TargetTriple::create("bpfel-unknown-none");
    let target = Target::from_triple(&triple)
        .map_err(|e| BpfError::TargetError(e.to_string()))?;

    let opt = match options.opt_level {
        0 => OptimizationLevel::None,
        1 => OptimizationLevel::Less,
        2 => OptimizationLevel::Default,
        _ => OptimizationLevel::Aggressive,
    };

    let target_machine = target
        .create_target_machine(
            &triple,
            "generic",
            "",
            opt,
            RelocMode::PIC,
            CodeModel::Default,
        )
        .ok_or_else(|| BpfError::TargetError("Failed to create target machine".to_string()))?;

    // Emit object file
    std::fs::create_dir_all(&options.output_dir)?;
    let obj_path = options.output_dir.join("solscript_program.o");

    target_machine
        .write_to_file(&module, FileType::Object, &obj_path)
        .map_err(|e| BpfError::LlvmError(e.to_string()))?;

    // Link to create .so (would need lld-bpf)
    let so_path = options.output_dir.join("solscript_program.so");

    // For now, just return the object file
    // Full linking requires BPF linker
    Ok(CompileResult {
        program_path: obj_path,
        program_id: None,
        build_time_secs: start.elapsed().as_secs_f64(),
    })
}

/// Check if BPF build tools are available
pub fn check_tools() -> Result<ToolStatus> {
    let cargo_sbf = Command::new("cargo")
        .args(["build-sbf", "--version"])
        .output()
        .map(|o| {
            if o.status.success() {
                Some(String::from_utf8_lossy(&o.stdout).trim().to_string())
            } else {
                None
            }
        })
        .unwrap_or(None);

    let cargo_bpf = Command::new("cargo")
        .args(["build-bpf", "--version"])
        .output()
        .map(|o| {
            if o.status.success() {
                Some(String::from_utf8_lossy(&o.stdout).trim().to_string())
            } else {
                None
            }
        })
        .unwrap_or(None);

    let solana_cli = Command::new("solana")
        .args(["--version"])
        .output()
        .map(|o| {
            if o.status.success() {
                Some(String::from_utf8_lossy(&o.stdout).trim().to_string())
            } else {
                None
            }
        })
        .unwrap_or(None);

    let anchor = Command::new("anchor")
        .args(["--version"])
        .output()
        .map(|o| {
            if o.status.success() {
                Some(String::from_utf8_lossy(&o.stdout).trim().to_string())
            } else {
                None
            }
        })
        .unwrap_or(None);

    Ok(ToolStatus {
        cargo_build_sbf: cargo_sbf,
        cargo_build_bpf: cargo_bpf,
        solana_cli,
        anchor,
        #[cfg(feature = "llvm")]
        llvm_available: check_llvm(),
        #[cfg(not(feature = "llvm"))]
        llvm_available: false,
    })
}

#[cfg(feature = "llvm")]
fn check_llvm() -> bool {
    use inkwell::targets::{InitializationConfig, Target};
    Target::initialize_bpf(&InitializationConfig::default());
    Target::from_name("bpf").is_some()
}

/// Status of available build tools
#[derive(Debug)]
pub struct ToolStatus {
    pub cargo_build_sbf: Option<String>,
    pub cargo_build_bpf: Option<String>,
    pub solana_cli: Option<String>,
    pub anchor: Option<String>,
    pub llvm_available: bool,
}

impl ToolStatus {
    /// Check if any BPF build method is available
    pub fn can_build(&self) -> bool {
        self.cargo_build_sbf.is_some() || self.cargo_build_bpf.is_some() || self.llvm_available
    }

    /// Get a summary of available tools
    pub fn summary(&self) -> String {
        let mut lines = Vec::new();

        if let Some(v) = &self.cargo_build_sbf {
            lines.push(format!("✓ cargo build-sbf: {}", v));
        } else if let Some(v) = &self.cargo_build_bpf {
            lines.push(format!("✓ cargo build-bpf: {}", v));
        } else {
            lines.push("✗ cargo build-sbf: not found".to_string());
        }

        if let Some(v) = &self.solana_cli {
            lines.push(format!("✓ solana: {}", v));
        } else {
            lines.push("✗ solana: not found".to_string());
        }

        if let Some(v) = &self.anchor {
            lines.push(format!("✓ anchor: {}", v));
        } else {
            lines.push("✗ anchor: not found".to_string());
        }

        if self.llvm_available {
            lines.push("✓ LLVM BPF target: available".to_string());
        }

        lines.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_options() {
        let opts = CompileOptions::default();
        assert_eq!(opts.opt_level, 2);
        assert!(!opts.debug_info);
        assert!(opts.use_cargo_sbf);
    }

    #[test]
    fn test_tool_status_can_build() {
        let status = ToolStatus {
            cargo_build_sbf: Some("1.0".to_string()),
            cargo_build_bpf: None,
            solana_cli: None,
            anchor: None,
            llvm_available: false,
        };
        assert!(status.can_build());
    }
}
