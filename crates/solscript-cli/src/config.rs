//! Configuration file parsing for SolScript projects

use miette::{IntoDiagnostic, Result, WrapErr};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;

/// The main configuration file structure (solscript.toml)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub project: ProjectConfig,
    #[serde(default)]
    pub contract: ContractConfig,
    #[serde(default)]
    pub build: BuildConfig,
    #[serde(default)]
    pub solana: SolanaConfig,
    #[serde(default)]
    pub dependencies: BTreeMap<String, Dependency>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    #[serde(default = "default_version")]
    pub version: String,
    #[serde(default)]
    pub authors: Vec<String>,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub license: Option<String>,
    #[serde(default)]
    pub repository: Option<String>,
}

fn default_version() -> String {
    "0.1.0".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContractConfig {
    #[serde(default = "default_main")]
    pub main: String,
    #[serde(default)]
    pub name: Option<String>,
}

fn default_main() -> String {
    "src/main.sol".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BuildConfig {
    #[serde(default = "default_output")]
    pub output: String,
}

fn default_output() -> String {
    "output".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SolanaConfig {
    #[serde(default = "default_cluster")]
    pub cluster: String,
}

fn default_cluster() -> String {
    "devnet".to_string()
}

/// A dependency specification
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Dependency {
    /// Simple version string: `package = "1.0.0"`
    Version(String),
    /// Detailed specification
    Detailed(DependencySpec),
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DependencySpec {
    /// Version requirement (semver)
    #[serde(default)]
    pub version: Option<String>,
    /// Git repository URL
    #[serde(default)]
    pub git: Option<String>,
    /// Git branch
    #[serde(default)]
    pub branch: Option<String>,
    /// Git tag
    #[serde(default)]
    pub tag: Option<String>,
    /// Git revision (commit hash)
    #[serde(default)]
    pub rev: Option<String>,
    /// Local path
    #[serde(default)]
    pub path: Option<String>,
    /// GitHub owner/repo shorthand
    #[serde(default)]
    pub github: Option<String>,
}

impl Dependency {
    /// Get the version string if available
    pub fn version(&self) -> Option<&str> {
        match self {
            Dependency::Version(v) => Some(v),
            Dependency::Detailed(spec) => spec.version.as_deref(),
        }
    }

    /// Check if this is a git dependency
    pub fn is_git(&self) -> bool {
        match self {
            Dependency::Version(_) => false,
            Dependency::Detailed(spec) => spec.git.is_some() || spec.github.is_some(),
        }
    }

    /// Get the git URL for this dependency
    pub fn git_url(&self) -> Option<String> {
        match self {
            Dependency::Version(_) => None,
            Dependency::Detailed(spec) => {
                if let Some(url) = &spec.git {
                    Some(url.clone())
                } else {
                    spec.github
                        .as_ref()
                        .map(|repo| format!("https://github.com/{}.git", repo))
                }
            }
        }
    }

    /// Get the git ref (branch, tag, or rev)
    pub fn git_ref(&self) -> Option<String> {
        match self {
            Dependency::Version(_) => None,
            Dependency::Detailed(spec) => spec
                .tag
                .clone()
                .or_else(|| spec.branch.clone())
                .or_else(|| spec.rev.clone()),
        }
    }

    /// Check if this is a path dependency
    pub fn is_path(&self) -> bool {
        match self {
            Dependency::Version(_) => false,
            Dependency::Detailed(spec) => spec.path.is_some(),
        }
    }

    /// Get the local path for this dependency
    pub fn local_path(&self) -> Option<&str> {
        match self {
            Dependency::Version(_) => None,
            Dependency::Detailed(spec) => spec.path.as_deref(),
        }
    }
}

impl Config {
    /// Load configuration from a file
    pub fn load(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .into_diagnostic()
            .wrap_err_with(|| format!("Failed to read config file: {}", path.display()))?;

        toml::from_str(&content)
            .into_diagnostic()
            .wrap_err("Failed to parse solscript.toml")
    }

    /// Save configuration to a file
    pub fn save(&self, path: &Path) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .into_diagnostic()
            .wrap_err("Failed to serialize configuration")?;

        std::fs::write(path, content)
            .into_diagnostic()
            .wrap_err_with(|| format!("Failed to write config file: {}", path.display()))
    }

    /// Find the config file by walking up the directory tree
    pub fn find(start: &Path) -> Option<std::path::PathBuf> {
        let mut current = start.to_path_buf();
        loop {
            let config_path = current.join("solscript.toml");
            if config_path.exists() {
                return Some(config_path);
            }
            if !current.pop() {
                return None;
            }
        }
    }

    /// Add a dependency
    pub fn add_dependency(&mut self, name: String, dep: Dependency) {
        self.dependencies.insert(name, dep);
    }

    /// Remove a dependency
    pub fn remove_dependency(&mut self, name: &str) -> Option<Dependency> {
        self.dependencies.remove(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_dependency() {
        let toml_str = r#"
[project]
name = "test"

[dependencies]
token = "1.0.0"
"#;

        let config: Config = toml::from_str(toml_str).unwrap();
        assert!(config.dependencies.contains_key("token"));
        assert_eq!(config.dependencies["token"].version(), Some("1.0.0"));
    }

    #[test]
    fn test_parse_git_dependency() {
        let toml_str = r#"
[project]
name = "test"

[dependencies]
token = { github = "cryptuon/token-lib", tag = "v1.0.0" }
"#;

        let config: Config = toml::from_str(toml_str).unwrap();
        let dep = &config.dependencies["token"];
        assert!(dep.is_git());
        assert_eq!(
            dep.git_url(),
            Some("https://github.com/cryptuon/token-lib.git".to_string())
        );
    }

    #[test]
    fn test_parse_path_dependency() {
        let toml_str = r#"
[project]
name = "test"

[dependencies]
mylib = { path = "../mylib" }
"#;

        let config: Config = toml::from_str(toml_str).unwrap();
        let dep = &config.dependencies["mylib"];
        assert!(dep.is_path());
        assert_eq!(dep.local_path(), Some("../mylib"));
    }
}
