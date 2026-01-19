//! SolScript CLI
//!
//! Command-line interface for the SolScript compiler.

mod config;
mod package;
mod templates;

use clap::{Parser, Subcommand};
use miette::{IntoDiagnostic, Result, WrapErr};
use notify_debouncer_mini::{new_debouncer, notify::RecursiveMode};
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::Duration;
use std::fs;

#[derive(Parser)]
#[command(name = "solscript")]
#[command(author, version, about = "SolScript compiler for Solana smart contracts", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new SolScript project (deprecated: use 'new')
    Init {
        /// Project name (creates a directory with this name)
        #[arg(value_name = "NAME")]
        name: String,

        /// Use a minimal template (just the contract file)
        #[arg(long)]
        minimal: bool,
    },
    /// Create a new SolScript project from a template
    New {
        /// Project name (creates a directory with this name)
        #[arg(value_name = "NAME")]
        name: Option<String>,

        /// Template to use
        #[arg(short, long, default_value = "counter")]
        template: String,

        /// List available templates
        #[arg(long)]
        list: bool,
    },
    /// Parse a SolScript file and check for syntax errors
    Check {
        /// The source file to check
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },
    /// Parse a SolScript file and print the AST
    Parse {
        /// The source file to parse
        #[arg(value_name = "FILE")]
        file: PathBuf,

        /// Output format (json or debug)
        #[arg(short, long, default_value = "debug")]
        format: String,
    },
    /// Compile a SolScript file to an Anchor project
    Build {
        /// The source file to compile
        #[arg(value_name = "FILE")]
        file: PathBuf,

        /// Output directory for the generated Anchor project
        #[arg(short, long, default_value = "output")]
        output: PathBuf,
    },
    /// Generate Rust/Anchor code without writing to disk
    Codegen {
        /// The source file to compile
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },
    /// Format SolScript source files
    Fmt {
        /// The source file(s) to format
        #[arg(value_name = "FILE")]
        files: Vec<PathBuf>,

        /// Check formatting without making changes
        #[arg(long)]
        check: bool,
    },
    /// Watch for changes and rebuild automatically
    Watch {
        /// The source file to watch and compile
        #[arg(value_name = "FILE")]
        file: PathBuf,

        /// Output directory for the generated Anchor project
        #[arg(short, long, default_value = "output")]
        output: PathBuf,

        /// Watch additional directories for changes
        #[arg(long)]
        include: Vec<PathBuf>,

        /// Only type-check without generating code
        #[arg(long)]
        check_only: bool,
    },
    /// Run tests defined in the SolScript source
    Test {
        /// The source file with #[test] functions
        #[arg(value_name = "FILE")]
        file: PathBuf,

        /// Output directory for the generated Anchor project
        #[arg(short, long, default_value = "output")]
        output: PathBuf,

        /// Only run tests matching this filter
        #[arg(long)]
        filter: Option<String>,

        /// Show test output
        #[arg(long)]
        verbose: bool,
    },
    /// Deploy the compiled program to a Solana cluster
    Deploy {
        /// The source file or generated output directory
        #[arg(value_name = "PATH")]
        path: PathBuf,

        /// Solana cluster to deploy to (localnet, devnet, testnet, mainnet-beta)
        #[arg(short, long, default_value = "localnet")]
        cluster: String,

        /// Path to the keypair file for signing
        #[arg(short, long)]
        keypair: Option<PathBuf>,

        /// Skip confirmation prompt
        #[arg(short = 'y', long)]
        yes: bool,
    },
    /// Add a dependency to the project
    Add {
        /// Package name
        #[arg(value_name = "PACKAGE")]
        name: String,

        /// Version requirement (e.g., "1.0.0", "^1.0", ">=1.0,<2.0")
        #[arg(short, long)]
        version: Option<String>,

        /// GitHub repository (owner/repo format)
        #[arg(long)]
        github: Option<String>,

        /// Git repository URL
        #[arg(long)]
        git: Option<String>,

        /// Git tag
        #[arg(long)]
        tag: Option<String>,

        /// Git branch
        #[arg(long)]
        branch: Option<String>,

        /// Local path to the package
        #[arg(long)]
        path: Option<String>,
    },
    /// Remove a dependency from the project
    Remove {
        /// Package name to remove
        #[arg(value_name = "PACKAGE")]
        name: String,
    },
    /// Install all dependencies
    Install,
    /// Update all dependencies to their latest versions
    Update,
    /// List installed packages
    List,
    /// Compile directly to BPF bytecode
    BuildBpf {
        /// The source file to compile
        #[arg(value_name = "FILE")]
        file: PathBuf,

        /// Output directory for the compiled program
        #[arg(short, long, default_value = "target/deploy")]
        output: PathBuf,

        /// Optimization level (0-3)
        #[arg(short = 'O', long, default_value = "2")]
        opt_level: u8,

        /// Keep intermediate files
        #[arg(long)]
        keep_intermediate: bool,

        /// Use direct LLVM compilation instead of cargo-sbf (requires LLVM 18)
        #[arg(long)]
        llvm: bool,
    },
    /// Check available build tools
    Doctor,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { name, minimal } => init_project(&name, minimal),
        Commands::New { name, template, list } => new_project(name, &template, list),
        Commands::Check { file } => check_file(&file),
        Commands::Parse { file, format } => parse_file(&file, &format),
        Commands::Build { file, output } => build_project(&file, &output),
        Commands::Codegen { file } => codegen_file(&file),
        Commands::Fmt { files, check } => format_files(&files, check),
        Commands::Watch { file, output, include, check_only } => {
            watch_project(&file, &output, &include, check_only)
        }
        Commands::Test { file, output, filter, verbose } => {
            run_tests(&file, &output, filter.as_deref(), verbose)
        }
        Commands::Deploy { path, cluster, keypair, yes } => {
            deploy_program(&path, &cluster, keypair.as_deref(), yes)
        }
        Commands::Add { name, version, github, git, tag, branch, path } => {
            add_dependency(&name, version.as_deref(), github.as_deref(), git.as_deref(), tag.as_deref(), branch.as_deref(), path.as_deref())
        }
        Commands::Remove { name } => remove_dependency(&name),
        Commands::Install => install_dependencies(),
        Commands::Update => update_dependencies(),
        Commands::List => list_dependencies(),
        Commands::BuildBpf { file, output, opt_level, keep_intermediate, llvm } => {
            build_bpf(&file, &output, opt_level, keep_intermediate, llvm)
        }
        Commands::Doctor => check_doctor(),
    }
}

fn check_file(path: &PathBuf) -> Result<()> {
    let source = std::fs::read_to_string(path)
        .into_diagnostic()
        .wrap_err_with(|| format!("Failed to read file: {}", path.display()))?;

    match solscript_parser::parse(&source) {
        Ok(program) => {
            let item_count = program.items.len();
            println!(
                "✓ {} parsed successfully ({} top-level items)",
                path.display(),
                item_count
            );
            Ok(())
        }
        Err(err) => {
            eprintln!("Error parsing {}:", path.display());
            Err(err).into_diagnostic()
        }
    }
}

fn parse_file(path: &PathBuf, format: &str) -> Result<()> {
    let source = std::fs::read_to_string(path)
        .into_diagnostic()
        .wrap_err_with(|| format!("Failed to read file: {}", path.display()))?;

    match solscript_parser::parse(&source) {
        Ok(program) => {
            match format {
                "json" => {
                    let json = serde_json::to_string_pretty(&program)
                        .into_diagnostic()
                        .wrap_err("Failed to serialize AST to JSON")?;
                    println!("{}", json);
                }
                "debug" | _ => {
                    println!("{:#?}", program);
                }
            }
            Ok(())
        }
        Err(err) => {
            eprintln!("Error parsing {}:", path.display());
            Err(err).into_diagnostic()
        }
    }
}

fn build_project(file: &PathBuf, output: &PathBuf) -> Result<()> {
    let source = std::fs::read_to_string(file)
        .into_diagnostic()
        .wrap_err_with(|| format!("Failed to read file: {}", file.display()))?;

    // Parse
    let program = solscript_parser::parse(&source)
        .map_err(|e| miette::miette!("Parse error: {:?}", e))?;

    println!("✓ Parsed {} ({} items)", file.display(), program.items.len());

    // Type check
    if let Err(errors) = solscript_typeck::typecheck(&program, &source) {
        for err in errors {
            // Use miette's Report for nice formatting with source code snippets
            let report = miette::Report::new(err);
            eprintln!("{:?}", report);
        }
        return Err(miette::miette!("Type checking failed"));
    }

    println!("✓ Type checked successfully");

    // Generate code
    let generated = solscript_codegen::generate(&program)
        .map_err(|e| miette::miette!("Codegen error: {:?}", e))?;

    // Write to output directory
    generated.write_to_dir(output)
        .into_diagnostic()
        .wrap_err("Failed to write generated project")?;

    println!("✓ Generated Anchor project in {}", output.display());
    println!();
    println!("To build the Solana program:");
    println!("  cd {}", output.display());
    println!("  anchor build");
    println!();
    println!("To deploy:");
    println!("  anchor deploy");

    Ok(())
}

fn codegen_file(file: &PathBuf) -> Result<()> {
    let source = std::fs::read_to_string(file)
        .into_diagnostic()
        .wrap_err_with(|| format!("Failed to read file: {}", file.display()))?;

    // Parse
    let program = solscript_parser::parse(&source)
        .map_err(|e| miette::miette!("Parse error: {:?}", e))?;

    // Type check
    if let Err(errors) = solscript_typeck::typecheck(&program, &source) {
        for err in errors {
            // Use miette's Report for nice formatting with source code snippets
            let report = miette::Report::new(err);
            eprintln!("{:?}", report);
        }
        return Err(miette::miette!("Type checking failed"));
    }

    // Generate code
    let generated = solscript_codegen::generate(&program)
        .map_err(|e| miette::miette!("Codegen error: {:?}", e))?;

    // Print generated lib.rs
    println!("=== lib.rs ===");
    println!("{}", generated.lib_rs);
    println!();
    println!("=== state.rs ===");
    println!("{}", generated.state_rs);
    println!();
    println!("=== instructions.rs ===");
    println!("{}", generated.instructions_rs);
    println!();
    println!("=== error.rs ===");
    println!("{}", generated.error_rs);
    println!();
    println!("=== events.rs ===");
    println!("{}", generated.events_rs);

    Ok(())
}

fn watch_project(
    file: &PathBuf,
    output: &PathBuf,
    include: &[PathBuf],
    check_only: bool,
) -> Result<()> {
    println!("Starting watch mode...");
    println!("Watching: {}", file.display());
    if !include.is_empty() {
        for dir in include {
            println!("Also watching: {}", dir.display());
        }
    }
    println!();
    println!("Press Ctrl+C to stop");
    println!();

    // Perform initial build
    println!("--- Initial build ---");
    let _ = do_build(file, output, check_only);
    println!();

    // Set up file watcher
    let (tx, rx) = channel();

    let mut debouncer = new_debouncer(Duration::from_millis(300), tx)
        .into_diagnostic()
        .wrap_err("Failed to create file watcher")?;

    // Watch the source file's directory
    let watch_dir = file
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."));

    debouncer
        .watcher()
        .watch(&watch_dir, RecursiveMode::Recursive)
        .into_diagnostic()
        .wrap_err_with(|| format!("Failed to watch directory: {}", watch_dir.display()))?;

    // Watch additional directories
    for dir in include {
        debouncer
            .watcher()
            .watch(dir, RecursiveMode::Recursive)
            .into_diagnostic()
            .wrap_err_with(|| format!("Failed to watch directory: {}", dir.display()))?;
    }

    // Event loop
    loop {
        match rx.recv() {
            Ok(Ok(events)) => {
                // Filter for .sol files only
                let sol_changed = events.iter().any(|e| {
                    e.path
                        .extension()
                        .map(|ext| ext == "sol")
                        .unwrap_or(false)
                });

                if sol_changed {
                    // Clear screen for better readability
                    print!("\x1B[2J\x1B[1;1H");
                    println!("--- Change detected, rebuilding... ---");
                    println!();
                    let _ = do_build(file, output, check_only);
                    println!();
                    println!("Watching for changes... (Ctrl+C to stop)");
                }
            }
            Ok(Err(error)) => {
                eprintln!("Watch error: {:?}", error);
            }
            Err(e) => {
                eprintln!("Channel error: {:?}", e);
                break;
            }
        }
    }

    Ok(())
}

fn do_build(file: &PathBuf, output: &PathBuf, check_only: bool) -> Result<()> {
    let source = match std::fs::read_to_string(file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("✗ Failed to read {}: {}", file.display(), e);
            return Err(miette::miette!("Failed to read file"));
        }
    };

    // Parse
    let program = match solscript_parser::parse(&source) {
        Ok(p) => {
            println!("✓ Parsed {} ({} items)", file.display(), p.items.len());
            p
        }
        Err(e) => {
            eprintln!("✗ Parse error: {:?}", e);
            return Err(miette::miette!("Parse error"));
        }
    };

    // Type check
    if let Err(errors) = solscript_typeck::typecheck(&program, &source) {
        eprintln!("✗ Type check failed:");
        for err in errors {
            let report = miette::Report::new(err);
            eprintln!("{:?}", report);
        }
        return Err(miette::miette!("Type checking failed"));
    }
    println!("✓ Type checked successfully");

    if check_only {
        println!("✓ Check complete (no code generated)");
        return Ok(());
    }

    // Generate code
    let generated = match solscript_codegen::generate(&program) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("✗ Codegen error: {:?}", e);
            return Err(miette::miette!("Codegen error"));
        }
    };

    // Write to output directory
    if let Err(e) = generated.write_to_dir(output) {
        eprintln!("✗ Failed to write output: {}", e);
        return Err(miette::miette!("Failed to write output"));
    }

    println!("✓ Generated Anchor project in {}", output.display());

    Ok(())
}

fn run_tests(
    file: &PathBuf,
    output: &PathBuf,
    filter: Option<&str>,
    verbose: bool,
) -> Result<()> {
    use std::process::Command;

    println!("Running SolScript tests...\n");

    // First, build the project
    let source = std::fs::read_to_string(file)
        .into_diagnostic()
        .wrap_err_with(|| format!("Failed to read file: {}", file.display()))?;

    // Parse
    let program = solscript_parser::parse(&source)
        .map_err(|e| miette::miette!("Parse error: {:?}", e))?;

    // Type check
    if let Err(errors) = solscript_typeck::typecheck(&program, &source) {
        for err in errors {
            let report = miette::Report::new(err);
            eprintln!("{:?}", report);
        }
        return Err(miette::miette!("Type checking failed"));
    }

    // Generate code
    let generated = solscript_codegen::generate(&program)
        .map_err(|e| miette::miette!("Codegen error: {:?}", e))?;

    if !generated.has_tests {
        println!("No tests found. Add #[test] functions to your contract.");
        return Ok(());
    }

    // Write to output directory
    generated.write_to_dir(output)
        .into_diagnostic()
        .wrap_err("Failed to write output")?;

    println!("Generated project with tests to {}", output.display());
    println!();

    // Run cargo test in the generated project
    let program_dir = output.join("programs").join("solscript_program");

    let mut cmd = Command::new("cargo");
    cmd.arg("test");

    if let Some(f) = filter {
        cmd.arg(f);
    }

    if verbose {
        cmd.arg("--").arg("--nocapture");
    }

    cmd.current_dir(&program_dir);

    println!("Running: cargo test in {}", program_dir.display());
    println!();

    let status = cmd.status()
        .into_diagnostic()
        .wrap_err("Failed to run cargo test")?;

    if status.success() {
        println!("\n✓ All tests passed!");
    } else {
        return Err(miette::miette!("Some tests failed"));
    }

    Ok(())
}

fn deploy_program(
    path: &PathBuf,
    cluster: &str,
    keypair: Option<&std::path::Path>,
    skip_confirm: bool,
) -> Result<()> {
    use std::process::Command;
    use std::io::{self, Write};

    // Determine if path is a source file or output directory
    let output_dir = if path.extension().map(|e| e == "sol").unwrap_or(false) {
        // It's a source file, need to build first
        println!("Building project from source...\n");

        let source = std::fs::read_to_string(path)
            .into_diagnostic()
            .wrap_err_with(|| format!("Failed to read file: {}", path.display()))?;

        let program = solscript_parser::parse(&source)
            .map_err(|e| miette::miette!("Parse error: {:?}", e))?;

        if let Err(errors) = solscript_typeck::typecheck(&program, &source) {
            for err in errors {
                let report = miette::Report::new(err);
                eprintln!("{:?}", report);
            }
            return Err(miette::miette!("Type checking failed"));
        }

        let generated = solscript_codegen::generate(&program)
            .map_err(|e| miette::miette!("Codegen error: {:?}", e))?;

        let output = PathBuf::from("output");
        generated.write_to_dir(&output)
            .into_diagnostic()
            .wrap_err("Failed to write output")?;

        println!("✓ Generated Anchor project\n");
        output
    } else {
        path.clone()
    };

    // Validate cluster
    let valid_clusters = ["localnet", "devnet", "testnet", "mainnet-beta"];
    if !valid_clusters.contains(&cluster) {
        return Err(miette::miette!(
            "Invalid cluster '{}'. Valid options: localnet, devnet, testnet, mainnet-beta",
            cluster
        ));
    }

    // Confirm deployment (unless --yes flag)
    if !skip_confirm && cluster != "localnet" {
        print!(
            "Deploy to {} cluster? This will use SOL for transaction fees. [y/N] ",
            cluster
        );
        io::stdout().flush().into_diagnostic()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input).into_diagnostic()?;

        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Deployment cancelled.");
            return Ok(());
        }
    }

    println!("Deploying to {} cluster...\n", cluster);

    // Run anchor build first
    println!("Building with Anchor...");
    let build_status = Command::new("anchor")
        .arg("build")
        .current_dir(&output_dir)
        .status()
        .into_diagnostic()
        .wrap_err("Failed to run 'anchor build'. Is Anchor installed?")?;

    if !build_status.success() {
        return Err(miette::miette!("Anchor build failed"));
    }
    println!("✓ Build successful\n");

    // Run anchor deploy
    println!("Deploying...");
    let mut deploy_cmd = Command::new("anchor");
    deploy_cmd
        .arg("deploy")
        .arg("--provider.cluster")
        .arg(cluster);

    if let Some(kp) = keypair {
        deploy_cmd
            .arg("--provider.wallet")
            .arg(kp);
    }

    deploy_cmd.current_dir(&output_dir);

    let deploy_status = deploy_cmd
        .status()
        .into_diagnostic()
        .wrap_err("Failed to run 'anchor deploy'. Is Anchor installed?")?;

    if deploy_status.success() {
        println!("\n✓ Deployment successful!");
        println!("\nProgram deployed to {} cluster.", cluster);
        println!("Check the program ID in Anchor.toml or the deploy output above.");
    } else {
        return Err(miette::miette!("Deployment failed"));
    }

    Ok(())
}

// ============ Project Creation ============

fn new_project(name: Option<String>, template_id: &str, list_only: bool) -> Result<()> {
    // If --list flag is set, show available templates
    if list_only {
        return list_templates();
    }

    // Name is required if not listing
    let name = name.ok_or_else(|| {
        miette::miette!("Project name is required. Usage: solscript new <name> [--template <template>]")
    })?;

    // Look up the template
    let template = templates::get_template(template_id).ok_or_else(|| {
        miette::miette!(
            "Unknown template '{}'. Run 'solscript new --list' to see available templates.",
            template_id
        )
    })?;

    create_project_from_template(&name, template)
}

fn list_templates() -> Result<()> {
    println!("\nAvailable templates:\n");

    for template in templates::TEMPLATES {
        let difficulty = match template.metadata.difficulty {
            templates::Difficulty::Beginner => "Beginner",
            templates::Difficulty::Intermediate => "Intermediate",
            templates::Difficulty::Advanced => "Advanced",
        };

        let default_marker = if template.metadata.id == "counter" {
            " [DEFAULT]"
        } else {
            ""
        };

        println!(
            "  {} ({}) - {}{}",
            template.metadata.id,
            difficulty,
            template.metadata.description,
            default_marker
        );
        println!("    Features: {}", template.metadata.features.join(", "));
        println!();
    }

    println!("Usage: solscript new <project-name> --template <template>");
    println!("       solscript new <project-name>  (uses 'counter' by default)");
    println!();

    Ok(())
}

fn create_project_from_template(name: &str, template: &templates::Template) -> Result<()> {
    let project_dir = PathBuf::from(name);

    // Check if directory already exists
    if project_dir.exists() {
        return Err(miette::miette!(
            "Directory '{}' already exists. Please choose a different name or remove the existing directory.",
            name
        ));
    }

    // Create project structure
    fs::create_dir_all(&project_dir)
        .into_diagnostic()
        .wrap_err("Failed to create project directory")?;

    let src_dir = project_dir.join("src");
    fs::create_dir_all(&src_dir)
        .into_diagnostic()
        .wrap_err("Failed to create src directory")?;

    // Generate contract name from project name
    let contract_name = to_pascal_case(name);

    // Format features list for README
    let features_list = template
        .metadata
        .features
        .iter()
        .map(|f| format!("- {}", f))
        .collect::<Vec<_>>()
        .join("\n");

    // Replace placeholders in templates
    let config_content = template
        .config_template
        .replace("{{PROJECT_NAME}}", name)
        .replace("{{CONTRACT_NAME}}", &contract_name)
        .replace("{{DESCRIPTION}}", template.metadata.description);

    let readme_content = template
        .readme_template
        .replace("{{PROJECT_NAME}}", name)
        .replace("{{CONTRACT_NAME}}", &contract_name)
        .replace("{{DESCRIPTION}}", template.metadata.description)
        .replace("{{FEATURES_LIST}}", &features_list);

    // Write files
    fs::write(src_dir.join("main.sol"), template.main_sol)
        .into_diagnostic()
        .wrap_err("Failed to write contract file")?;

    fs::write(project_dir.join("solscript.toml"), &config_content)
        .into_diagnostic()
        .wrap_err("Failed to write config file")?;

    fs::write(project_dir.join("README.md"), &readme_content)
        .into_diagnostic()
        .wrap_err("Failed to write README.md")?;

    fs::write(project_dir.join(".gitignore"), template.gitignore)
        .into_diagnostic()
        .wrap_err("Failed to write .gitignore")?;

    // Print success message
    println!(
        "\n✓ Created new SolScript project '{}' using '{}' template\n",
        name, template.metadata.name
    );
    println!("Project structure:");
    println!("  {}/", name);
    println!("  ├── src/");
    println!("  │   └── main.sol");
    println!("  ├── solscript.toml");
    println!("  ├── .gitignore");
    println!("  └── README.md");
    println!();
    println!("Next steps:");
    println!("  cd {}", name);
    println!("  solscript check src/main.sol");
    println!("  solscript build src/main.sol");
    println!();

    Ok(())
}

fn init_project(name: &str, minimal: bool) -> Result<()> {
    // Show deprecation notice
    eprintln!("Note: 'init' is deprecated. Use 'solscript new' instead.\n");

    // Map to new command
    let template_id = if minimal { "simple" } else { "counter" };
    new_project(Some(name.to_string()), template_id, false)
}

fn to_pascal_case(s: &str) -> String {
    s.split(|c: char| c == '-' || c == '_' || c.is_whitespace())
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().chain(chars).collect(),
            }
        })
        .collect()
}

fn format_files(files: &[PathBuf], check_only: bool) -> Result<()> {
    if files.is_empty() {
        return Err(miette::miette!("No files specified. Usage: solscript fmt <FILE>..."));
    }

    let mut any_changes = false;
    let mut any_errors = false;

    for file in files {
        match format_single_file(file, check_only) {
            Ok(changed) => {
                if changed {
                    any_changes = true;
                    if check_only {
                        println!("Would reformat: {}", file.display());
                    } else {
                        println!("Formatted: {}", file.display());
                    }
                }
            }
            Err(e) => {
                eprintln!("Error formatting {}: {}", file.display(), e);
                any_errors = true;
            }
        }
    }

    if any_errors {
        return Err(miette::miette!("Some files could not be formatted"));
    }

    if check_only && any_changes {
        return Err(miette::miette!("Some files need formatting"));
    }

    if !any_changes {
        println!("All files are properly formatted");
    }

    Ok(())
}

fn format_single_file(path: &PathBuf, check_only: bool) -> Result<bool> {
    let source = fs::read_to_string(path)
        .into_diagnostic()
        .wrap_err_with(|| format!("Failed to read file: {}", path.display()))?;

    // Parse the file to ensure it's valid
    let program = solscript_parser::parse(&source)
        .map_err(|e| miette::miette!("Parse error: {:?}", e))?;

    // Format the AST back to source code
    let formatted = format_program(&program);

    // Check if the formatted version is different
    let normalized_source = normalize_whitespace(&source);
    let normalized_formatted = normalize_whitespace(&formatted);

    if normalized_source == normalized_formatted {
        return Ok(false); // No changes needed
    }

    if !check_only {
        fs::write(path, &formatted)
            .into_diagnostic()
            .wrap_err_with(|| format!("Failed to write file: {}", path.display()))?;
    }

    Ok(true) // Changes were made (or would be made)
}

fn normalize_whitespace(s: &str) -> String {
    s.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_program(program: &solscript_ast::Program) -> String {
    let mut output = String::new();

    for (i, item) in program.items.iter().enumerate() {
        if i > 0 {
            output.push_str("\n");
        }
        output.push_str(&format_item(item));
    }

    output
}

fn format_item(item: &solscript_ast::Item) -> String {
    use solscript_ast::Item;

    match item {
        Item::Contract(c) => format_contract(c),
        Item::Interface(i) => format_interface(i),
        Item::Struct(s) => format_struct(s),
        Item::Enum(e) => format_enum(e),
        Item::Event(e) => format_event(e),
        Item::Error(e) => format_error(e),
        Item::Import(i) => format_import(i),
        Item::Function(f) => format_function(f, 0),
    }
}

fn format_contract(c: &solscript_ast::ContractDef) -> String {
    let mut output = String::new();

    if c.is_abstract {
        output.push_str("abstract ");
    }
    output.push_str(&format!("contract {}", c.name.name));

    if !c.bases.is_empty() {
        output.push_str(" is ");
        let bases: Vec<String> = c.bases.iter().map(|b| b.name().to_string()).collect();
        output.push_str(&bases.join(", "));
    }

    output.push_str(" {\n");

    for member in &c.members {
        output.push_str(&format_contract_member(member));
    }

    output.push_str("}\n");
    output
}

fn format_contract_member(member: &solscript_ast::ContractMember) -> String {
    use solscript_ast::ContractMember;

    match member {
        ContractMember::StateVar(v) => {
            let mut line = String::from("    ");
            line.push_str(&format_type(&v.ty));
            if let Some(vis) = &v.visibility {
                line.push_str(&format!(" {}", format_visibility(vis)));
            }
            line.push_str(&format!(" {};\n", v.name.name));
            line
        }
        ContractMember::Function(f) => format_function(f, 1),
        ContractMember::Constructor(c) => format_constructor(c),
        ContractMember::Modifier(m) => format_modifier(m),
        ContractMember::Event(e) => format!("    {};\n", format_event_inline(e)),
        ContractMember::Error(e) => format!("    {};\n", format_error_inline(e)),
        ContractMember::Struct(s) => {
            // Indent struct inside contract
            let formatted = format_struct(s);
            formatted.lines().map(|l| format!("    {}\n", l)).collect()
        }
        ContractMember::Enum(e) => {
            // Indent enum inside contract
            let formatted = format_enum(e);
            formatted.lines().map(|l| format!("    {}\n", l)).collect()
        }
    }
}

fn format_function(f: &solscript_ast::FnDef, indent: usize) -> String {
    let indent_str = "    ".repeat(indent);
    let mut output = String::new();

    output.push_str(&indent_str);
    output.push_str(&format!("function {}(", f.name.name));

    // Format parameters
    let params: Vec<String> = f
        .params
        .iter()
        .map(|p| format!("{} {}", format_type(&p.ty), p.name.name))
        .collect();
    output.push_str(&params.join(", "));
    output.push_str(")");

    // Visibility
    if let Some(vis) = &f.visibility {
        output.push_str(&format!(" {}", format_visibility(vis)));
    }

    // State mutability
    for sm in &f.state_mutability {
        output.push_str(&format!(" {}", format_state_mutability(sm)));
    }

    // Modifiers
    for m in &f.modifiers {
        output.push_str(&format!(" {}", m.name.name));
        if !m.args.is_empty() {
            output.push_str("(...)");
        }
    }

    // Return type
    if !f.return_params.is_empty() {
        output.push_str(" returns (");
        let returns: Vec<String> = f
            .return_params
            .iter()
            .map(|p| format_type(&p.ty))
            .collect();
        output.push_str(&returns.join(", "));
        output.push_str(")");
    }

    // Body
    if let Some(_body) = &f.body {
        output.push_str(" {\n");
        output.push_str(&format!("{}    // ... function body\n", indent_str));
        output.push_str(&format!("{}}}\n", indent_str));
    } else {
        output.push_str(";\n");
    }

    output.push_str("\n");
    output
}

fn format_constructor(c: &solscript_ast::ConstructorDef) -> String {
    let mut output = String::from("    constructor(");

    let params: Vec<String> = c
        .params
        .iter()
        .map(|p| format!("{} {}", format_type(&p.ty), p.name.name))
        .collect();
    output.push_str(&params.join(", "));
    output.push_str(") {\n");
    output.push_str("        // ... constructor body\n");
    output.push_str("    }\n\n");

    output
}

fn format_modifier(m: &solscript_ast::ModifierDef) -> String {
    let mut output = String::from("    modifier ");
    output.push_str(&m.name.name.to_string());
    output.push_str("(");

    let params: Vec<String> = m
        .params
        .iter()
        .map(|p| format!("{} {}", format_type(&p.ty), p.name.name))
        .collect();
    output.push_str(&params.join(", "));
    output.push_str(") {\n");
    output.push_str("        // ... modifier body\n");
    output.push_str("        _;\n");
    output.push_str("    }\n\n");

    output
}

fn format_interface(i: &solscript_ast::InterfaceDef) -> String {
    let mut output = format!("interface {}", i.name.name);

    if !i.bases.is_empty() {
        output.push_str(" is ");
        let bases: Vec<String> = i.bases.iter().map(|b| b.name().to_string()).collect();
        output.push_str(&bases.join(", "));
    }

    output.push_str(" {\n");

    for sig in &i.members {
        output.push_str("    function ");
        output.push_str(&sig.name.name.to_string());
        output.push_str("(");

        let params: Vec<String> = sig
            .params
            .iter()
            .map(|p| format!("{} {}", format_type(&p.ty), p.name.name))
            .collect();
        output.push_str(&params.join(", "));
        output.push_str(")");

        if let Some(vis) = &sig.visibility {
            output.push_str(&format!(" {}", format_visibility(vis)));
        }

        for sm in &sig.state_mutability {
            output.push_str(&format!(" {}", format_state_mutability(sm)));
        }

        if !sig.return_params.is_empty() {
            output.push_str(" returns (");
            let returns: Vec<String> = sig
                .return_params
                .iter()
                .map(|p| format_type(&p.ty))
                .collect();
            output.push_str(&returns.join(", "));
            output.push_str(")");
        }

        output.push_str(";\n");
    }

    output.push_str("}\n");
    output
}

fn format_struct(s: &solscript_ast::StructDef) -> String {
    let mut output = format!("struct {} {{\n", s.name.name);

    for field in &s.fields {
        output.push_str(&format!("    {} {};\n", format_type(&field.ty), field.name.name));
    }

    output.push_str("}\n");
    output
}

fn format_enum(e: &solscript_ast::EnumDef) -> String {
    let mut output = format!("enum {} {{\n", e.name.name);

    for (i, variant) in e.variants.iter().enumerate() {
        output.push_str(&format!("    {}", variant.name.name));
        if i < e.variants.len() - 1 {
            output.push_str(",");
        }
        output.push_str("\n");
    }

    output.push_str("}\n");
    output
}

fn format_event(e: &solscript_ast::EventDef) -> String {
    format!("{};\n", format_event_inline(e))
}

fn format_event_inline(e: &solscript_ast::EventDef) -> String {
    let mut output = format!("event {}(", e.name.name);

    let params: Vec<String> = e
        .params
        .iter()
        .map(|p| {
            let mut param = format_type(&p.ty);
            if p.indexed {
                param.push_str(" indexed");
            }
            param.push_str(&format!(" {}", p.name.name));
            param
        })
        .collect();
    output.push_str(&params.join(", "));
    output.push_str(")");

    output
}

fn format_error(e: &solscript_ast::ErrorDef) -> String {
    format!("{};\n", format_error_inline(e))
}

fn format_error_inline(e: &solscript_ast::ErrorDef) -> String {
    let mut output = format!("error {}(", e.name.name);

    let params: Vec<String> = e
        .params
        .iter()
        .map(|p| format!("{} {}", format_type(&p.ty), p.name.name))
        .collect();
    output.push_str(&params.join(", "));
    output.push_str(")");

    output
}

fn format_import(i: &solscript_ast::ImportStmt) -> String {
    let mut output = String::from("import { ");

    let items: Vec<String> = i.items.iter().map(|item| item.name.name.to_string()).collect();
    output.push_str(&items.join(", "));

    output.push_str(&format!(" }} from \"{}\";\n", i.source));

    output
}

fn format_type(ty: &solscript_ast::TypeExpr) -> String {
    use solscript_ast::TypeExpr;

    match ty {
        TypeExpr::Path(p) => p.name().to_string(),
        TypeExpr::Array(arr) => {
            let base = arr.element.name().to_string();
            if arr.sizes.len() == 1 {
                if let Some(size) = arr.sizes[0] {
                    format!("{}[{}]", base, size)
                } else {
                    format!("{}[]", base)
                }
            } else {
                base
            }
        }
        TypeExpr::Mapping(m) => {
            format!("mapping({} => {})", format_type(&m.key), format_type(&m.value))
        }
        TypeExpr::Tuple(t) => {
            let types: Vec<String> = t.elements.iter().map(format_type).collect();
            format!("({})", types.join(", "))
        }
    }
}

fn format_visibility(v: &solscript_ast::Visibility) -> String {
    use solscript_ast::Visibility;
    match v {
        Visibility::Public => "public".to_string(),
        Visibility::Private => "private".to_string(),
        Visibility::Internal => "internal".to_string(),
        Visibility::External => "external".to_string(),
    }
}

fn format_state_mutability(sm: &solscript_ast::StateMutability) -> String {
    use solscript_ast::StateMutability;
    match sm {
        StateMutability::View => "view".to_string(),
        StateMutability::Pure => "pure".to_string(),
        StateMutability::Payable => "payable".to_string(),
    }
}

// =============================================================================
// Package Manager Commands
// =============================================================================

fn find_config() -> Result<PathBuf> {
    let cwd = std::env::current_dir()
        .into_diagnostic()
        .wrap_err("Failed to get current directory")?;

    config::Config::find(&cwd).ok_or_else(|| {
        miette::miette!(
            "No solscript.toml found in current directory or any parent.\n\
             Run 'solscript init <project-name>' to create a new project."
        )
    })
}

fn add_dependency(
    name: &str,
    version: Option<&str>,
    github: Option<&str>,
    git: Option<&str>,
    tag: Option<&str>,
    branch: Option<&str>,
    path: Option<&str>,
) -> Result<()> {
    let config_path = find_config()?;

    println!("Adding {} to dependencies...", name);

    package::add_package(
        &config_path,
        name,
        version,
        github,
        git,
        tag,
        branch,
        path,
    )?;

    println!("✓ Added {} to solscript.toml", name);
    println!("✓ Package installed");

    Ok(())
}

fn remove_dependency(name: &str) -> Result<()> {
    let config_path = find_config()?;

    println!("Removing {}...", name);

    package::remove_package(&config_path, name)?;

    println!("✓ Removed {} from solscript.toml", name);

    Ok(())
}

fn install_dependencies() -> Result<()> {
    let config_path = find_config()?;
    let config = config::Config::load(&config_path)?;

    if config.dependencies.is_empty() {
        println!("No dependencies to install.");
        return Ok(());
    }

    println!("Installing {} dependencies...\n", config.dependencies.len());

    let project_root = config_path.parent().unwrap_or(std::path::Path::new("."));
    let pm = package::PackageManager::new(project_root.to_path_buf());

    pm.install_all(&config)?;

    println!("\n✓ All dependencies installed");

    Ok(())
}

fn update_dependencies() -> Result<()> {
    let config_path = find_config()?;

    println!("Updating dependencies...\n");

    package::update_packages(&config_path)?;

    println!("\n✓ All dependencies updated");

    Ok(())
}

fn list_dependencies() -> Result<()> {
    let config_path = find_config()?;
    let config = config::Config::load(&config_path)?;

    if config.dependencies.is_empty() {
        println!("No dependencies installed.");
        return Ok(());
    }

    println!("Dependencies:\n");

    for (name, dep) in &config.dependencies {
        let source = if dep.is_path() {
            format!("path: {}", dep.local_path().unwrap_or("unknown"))
        } else if dep.is_git() {
            if let Some(url) = dep.git_url() {
                let git_ref = dep.git_ref().unwrap_or_else(|| "HEAD".to_string());
                format!("git: {} ({})", url, git_ref)
            } else {
                "git".to_string()
            }
        } else if let Some(v) = dep.version() {
            format!("version: {}", v)
        } else {
            "unknown".to_string()
        };

        println!("  {} - {}", name, source);
    }

    // Also show installed packages
    let project_root = config_path.parent().unwrap_or(std::path::Path::new("."));
    let pm = package::PackageManager::new(project_root.to_path_buf());

    if let Ok(installed) = pm.list_installed() {
        if !installed.is_empty() {
            println!("\nInstalled packages:");
            for pkg in installed {
                println!("  {}", pkg);
            }
        }
    }

    Ok(())
}

// =============================================================================
// BPF Compilation Commands
// =============================================================================

fn build_bpf(file: &PathBuf, output: &PathBuf, opt_level: u8, keep_intermediate: bool, use_llvm: bool) -> Result<()> {
    if use_llvm {
        println!("Compiling {} to BPF using LLVM...\n", file.display());
    } else {
        println!("Compiling {} to BPF...\n", file.display());
    }

    let source = std::fs::read_to_string(file)
        .into_diagnostic()
        .wrap_err_with(|| format!("Failed to read file: {}", file.display()))?;

    // Parse
    let program = solscript_parser::parse(&source)
        .map_err(|e| miette::miette!("Parse error: {:?}", e))?;

    println!("✓ Parsed {} ({} items)", file.display(), program.items.len());

    // Configure compilation options
    let options = solscript_bpf::CompileOptions {
        opt_level,
        debug_info: false,
        output_dir: output.clone(),
        use_cargo_sbf: !use_llvm, // Use direct LLVM if --llvm flag is passed
        keep_intermediate,
    };

    // Compile to BPF
    let result = solscript_bpf::compile(&program, &source, &options)
        .map_err(|e| miette::miette!("Compilation error: {}", e))?;

    println!("✓ Type checked successfully");
    if use_llvm {
        println!("✓ Generated LLVM IR");
        println!("✓ Compiled to BPF via LLVM");
    } else {
        println!("✓ Generated Anchor code");
        println!("✓ Compiled to BPF");
    }
    println!();
    println!("Output: {}", result.program_path.display());
    println!("Build time: {:.2}s", result.build_time_secs);

    if let Some(id) = result.program_id {
        println!("Program ID: {}", id);
    }

    println!();
    println!("To deploy:");
    println!("  solana program deploy {}", result.program_path.display());

    Ok(())
}

fn check_doctor() -> Result<()> {
    println!("SolScript Build Environment\n");

    let status = solscript_bpf::check_tools()
        .map_err(|e| miette::miette!("Failed to check tools: {}", e))?;

    println!("{}", status.summary());
    println!();

    if status.can_build() {
        println!("✓ Ready to build SolScript programs");
    } else {
        println!("✗ Missing required tools");
        println!();
        println!("To install the Solana CLI:");
        println!("  sh -c \"$(curl -sSfL https://release.solana.com/stable/install)\"");
        println!();
        println!("To install Anchor:");
        println!("  cargo install --git https://github.com/coral-xyz/anchor avm --locked");
        println!("  avm install latest");
        println!("  avm use latest");
    }

    Ok(())
}
