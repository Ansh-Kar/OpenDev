//! Automated test runner detector for software projects.
//!
//! Inspects repository files to determine the default verification
//! command (e.g. `npm test`, `cargo test`, `pytest`, `go test`).

use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectLanguage {
    Node,
    Rust,
    Python,
    Go,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct TestConfig {
    pub language: ProjectLanguage,
    pub command: String,
}

/// Inspects a repository directory and returns the inferred test command.
pub fn detect_test_command(repo_dir: &Path) -> Option<TestConfig> {
    // 1. Rust (Cargo.toml)
    if repo_dir.join("Cargo.toml").exists() {
        return Some(TestConfig {
            language: ProjectLanguage::Rust,
            command: "cargo test".to_string(),
        });
    }

    // 2. Node.js (package.json)
    let pkg_json = repo_dir.join("package.json");
    if pkg_json.exists() {
        if let Ok(content) = std::fs::read_to_string(&pkg_json) {
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(&content) {
                if val["scripts"]["test"].is_string() {
                    let cmd = if repo_dir.join("pnpm-lock.yaml").exists() {
                        "pnpm test"
                    } else if repo_dir.join("yarn.lock").exists() {
                        "yarn test"
                    } else {
                        "npm test"
                    };
                    return Some(TestConfig {
                        language: ProjectLanguage::Node,
                        command: cmd.to_string(),
                    });
                }
            }
        }
    }

    // 3. Python (pyproject.toml, pytest.ini, requirements.txt)
    if repo_dir.join("pyproject.toml").exists()
        || repo_dir.join("pytest.ini").exists()
        || repo_dir.join("setup.py").exists()
    {
        let cmd = if repo_dir.join("uv.lock").exists() {
            "uv run pytest"
        } else {
            "pytest"
        };
        return Some(TestConfig {
            language: ProjectLanguage::Python,
            command: cmd.to_string(),
        });
    }

    // 4. Go (go.mod)
    if repo_dir.join("go.mod").exists() {
        return Some(TestConfig {
            language: ProjectLanguage::Go,
            command: "go test ./...".to_string(),
        });
    }

    None
}
