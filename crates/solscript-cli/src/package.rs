//! Package manager for SolScript
//!
//! Handles fetching, installing, and managing SolScript packages.

use crate::config::{Config, Dependency, DependencySpec};
use miette::{IntoDiagnostic, Result, WrapErr};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;

/// The packages directory name
const PACKAGES_DIR: &str = ".solscript/packages";

/// Package manager for handling dependencies
pub struct PackageManager {
    /// Project root directory
    project_root: PathBuf,
    /// Packages cache directory
    packages_dir: PathBuf,
}

impl PackageManager {
    /// Create a new package manager for the given project root
    pub fn new(project_root: PathBuf) -> Self {
        let packages_dir = project_root.join(PACKAGES_DIR);
        Self {
            project_root,
            packages_dir,
        }
    }

    /// Initialize the packages directory
    pub fn init(&self) -> Result<()> {
        if !self.packages_dir.exists() {
            std::fs::create_dir_all(&self.packages_dir)
                .into_diagnostic()
                .wrap_err("Failed to create packages directory")?;
        }
        Ok(())
    }

    /// Install all dependencies from the config
    pub fn install_all(&self, config: &Config) -> Result<InstalledPackages> {
        self.init()?;

        let mut installed = InstalledPackages::new();

        for (name, dep) in &config.dependencies {
            println!("Installing {}...", name);
            let pkg_path = self.install_package(name, dep)?;
            installed.packages.insert(name.clone(), pkg_path);
            println!("  ✓ Installed {}", name);
        }

        Ok(installed)
    }

    /// Install a single package
    pub fn install_package(&self, name: &str, dep: &Dependency) -> Result<PathBuf> {
        self.init()?;

        let pkg_dir = self.packages_dir.join(name);

        // If already installed, check if we need to update
        if pkg_dir.exists() {
            // For git dependencies, we might need to update
            if dep.is_git() {
                return self.update_git_package(&pkg_dir, dep);
            }
            // For path dependencies, just return the path
            if dep.is_path() {
                if let Some(path) = dep.local_path() {
                    return Ok(self.project_root.join(path));
                }
            }
            // Already installed
            return Ok(pkg_dir);
        }

        // Install based on dependency type
        if dep.is_path() {
            if let Some(path) = dep.local_path() {
                return Ok(self.project_root.join(path));
            }
        }

        if dep.is_git() {
            return self.install_git_package(name, dep);
        }

        // Registry-based dependency (GitHub releases)
        self.install_registry_package(name, dep)
    }

    /// Install a package from git
    fn install_git_package(&self, name: &str, dep: &Dependency) -> Result<PathBuf> {
        let pkg_dir = self.packages_dir.join(name);

        let git_url = dep
            .git_url()
            .ok_or_else(|| miette::miette!("No git URL for package {}", name))?;

        // Clone the repository
        let mut cmd = Command::new("git");
        cmd.arg("clone")
            .arg("--depth")
            .arg("1"); // Shallow clone

        // Add branch/tag if specified
        if let Some(git_ref) = dep.git_ref() {
            cmd.arg("--branch").arg(&git_ref);
        }

        cmd.arg(&git_url).arg(&pkg_dir);

        let output = cmd
            .output()
            .into_diagnostic()
            .wrap_err("Failed to run git clone")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(miette::miette!("Git clone failed: {}", stderr));
        }

        Ok(pkg_dir)
    }

    /// Update a git package
    fn update_git_package(&self, pkg_dir: &Path, dep: &Dependency) -> Result<PathBuf> {
        let git_ref = dep.git_ref();

        // Fetch latest
        let mut fetch = Command::new("git");
        fetch.arg("fetch").arg("--depth").arg("1");

        if let Some(ref_name) = &git_ref {
            fetch.arg("origin").arg(ref_name);
        }

        fetch.current_dir(pkg_dir);

        let output = fetch
            .output()
            .into_diagnostic()
            .wrap_err("Failed to run git fetch")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(miette::miette!("Git fetch failed: {}", stderr));
        }

        // Reset to the fetched ref
        let mut reset = Command::new("git");
        reset.arg("reset").arg("--hard");

        if let Some(ref_name) = git_ref {
            reset.arg(format!("origin/{}", ref_name));
        } else {
            reset.arg("origin/HEAD");
        }

        reset.current_dir(pkg_dir);

        let output = reset
            .output()
            .into_diagnostic()
            .wrap_err("Failed to run git reset")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(miette::miette!("Git reset failed: {}", stderr));
        }

        Ok(pkg_dir.to_path_buf())
    }

    /// Install a package from the registry (GitHub releases)
    fn install_registry_package(&self, name: &str, dep: &Dependency) -> Result<PathBuf> {
        let version = dep
            .version()
            .ok_or_else(|| miette::miette!("No version specified for package {}", name))?;

        // For now, we use a simple GitHub-based registry
        // Package format: https://github.com/solscript-packages/{name}/releases/download/v{version}/{name}.tar.gz
        let pkg_dir = self.packages_dir.join(name);

        // Try to find the package on GitHub
        // Default organization for SolScript packages
        let github_url = format!(
            "https://github.com/solscript-packages/{}/archive/refs/tags/v{}.tar.gz",
            name, version
        );

        println!("  Downloading from {}...", github_url);

        // Download using curl
        let archive_path = self.packages_dir.join(format!("{}-{}.tar.gz", name, version));

        let output = Command::new("curl")
            .arg("-fsSL")
            .arg("-o")
            .arg(&archive_path)
            .arg(&github_url)
            .output()
            .into_diagnostic()
            .wrap_err("Failed to run curl")?;

        if !output.status.success() {
            // Try alternative URL format
            let alt_url = format!(
                "https://github.com/solscript/{}/archive/refs/tags/v{}.tar.gz",
                name, version
            );

            let output = Command::new("curl")
                .arg("-fsSL")
                .arg("-o")
                .arg(&archive_path)
                .arg(&alt_url)
                .output()
                .into_diagnostic()
                .wrap_err("Failed to download package")?;

            if !output.status.success() {
                return Err(miette::miette!(
                    "Package {} version {} not found. Try using a git dependency instead:\n\n  [dependencies]\n  {} = {{ github = \"owner/{}\", tag = \"v{}\" }}",
                    name, version, name, name, version
                ));
            }
        }

        // Extract the archive
        std::fs::create_dir_all(&pkg_dir)
            .into_diagnostic()
            .wrap_err("Failed to create package directory")?;

        let output = Command::new("tar")
            .arg("-xzf")
            .arg(&archive_path)
            .arg("-C")
            .arg(&pkg_dir)
            .arg("--strip-components=1")
            .output()
            .into_diagnostic()
            .wrap_err("Failed to extract package")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(miette::miette!("Failed to extract package: {}", stderr));
        }

        // Clean up archive
        let _ = std::fs::remove_file(&archive_path);

        Ok(pkg_dir)
    }

    /// Remove a package
    pub fn remove_package(&self, name: &str) -> Result<()> {
        let pkg_dir = self.packages_dir.join(name);
        if pkg_dir.exists() {
            std::fs::remove_dir_all(&pkg_dir)
                .into_diagnostic()
                .wrap_err_with(|| format!("Failed to remove package {}", name))?;
        }
        Ok(())
    }

    /// List installed packages
    pub fn list_installed(&self) -> Result<Vec<String>> {
        if !self.packages_dir.exists() {
            return Ok(Vec::new());
        }

        let mut packages = Vec::new();
        for entry in std::fs::read_dir(&self.packages_dir)
            .into_diagnostic()
            .wrap_err("Failed to read packages directory")?
        {
            let entry = entry.into_diagnostic()?;
            if entry.path().is_dir() {
                if let Some(name) = entry.file_name().to_str() {
                    packages.push(name.to_string());
                }
            }
        }

        Ok(packages)
    }

    /// Get the path to a package's source files
    #[allow(dead_code)]
    pub fn package_source_dir(&self, name: &str) -> PathBuf {
        self.packages_dir.join(name).join("src")
    }
}

/// Represents installed packages and their locations
#[derive(Debug, Default)]
pub struct InstalledPackages {
    pub packages: HashMap<String, PathBuf>,
}

impl InstalledPackages {
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the path to a package
    #[allow(dead_code)]
    pub fn get(&self, name: &str) -> Option<&PathBuf> {
        self.packages.get(name)
    }
}

/// Add a package to the project
pub fn add_package(
    config_path: &Path,
    name: &str,
    version: Option<&str>,
    github: Option<&str>,
    git: Option<&str>,
    tag: Option<&str>,
    branch: Option<&str>,
    path: Option<&str>,
) -> Result<()> {
    let mut config = Config::load(config_path)?;

    let dep = if let Some(path) = path {
        // Local path dependency
        Dependency::Detailed(DependencySpec {
            path: Some(path.to_string()),
            ..Default::default()
        })
    } else if let Some(github) = github {
        // GitHub shorthand
        Dependency::Detailed(DependencySpec {
            github: Some(github.to_string()),
            tag: tag.map(|t| t.to_string()),
            branch: branch.map(|b| b.to_string()),
            ..Default::default()
        })
    } else if let Some(git) = git {
        // Git URL
        Dependency::Detailed(DependencySpec {
            git: Some(git.to_string()),
            tag: tag.map(|t| t.to_string()),
            branch: branch.map(|b| b.to_string()),
            ..Default::default()
        })
    } else if let Some(version) = version {
        // Simple version
        Dependency::Version(version.to_string())
    } else {
        // Default to latest
        Dependency::Version("*".to_string())
    };

    config.add_dependency(name.to_string(), dep);
    config.save(config_path)?;

    // Install the package
    let project_root = config_path.parent().unwrap_or(Path::new("."));
    let pm = PackageManager::new(project_root.to_path_buf());
    pm.install_package(name, &config.dependencies[name])?;

    Ok(())
}

/// Remove a package from the project
pub fn remove_package(config_path: &Path, name: &str) -> Result<()> {
    let mut config = Config::load(config_path)?;

    if config.remove_dependency(name).is_none() {
        return Err(miette::miette!("Package '{}' not found in dependencies", name));
    }

    config.save(config_path)?;

    // Remove the package files
    let project_root = config_path.parent().unwrap_or(Path::new("."));
    let pm = PackageManager::new(project_root.to_path_buf());
    pm.remove_package(name)?;

    Ok(())
}

/// Update all packages
pub fn update_packages(config_path: &Path) -> Result<()> {
    let config = Config::load(config_path)?;
    let project_root = config_path.parent().unwrap_or(Path::new("."));
    let pm = PackageManager::new(project_root.to_path_buf());

    for (name, dep) in &config.dependencies {
        println!("Updating {}...", name);

        // Remove and reinstall
        let _ = pm.remove_package(name);
        pm.install_package(name, dep)?;

        println!("  ✓ Updated {}", name);
    }

    Ok(())
}

impl Default for DependencySpec {
    fn default() -> Self {
        Self {
            version: None,
            git: None,
            branch: None,
            tag: None,
            rev: None,
            path: None,
            github: None,
        }
    }
}
