use serde::Deserialize;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Clone)]
pub struct ProjectConfig {
    #[serde(default)]
    pub code_extensions: HashSet<String>,
    #[serde(default)]
    pub ignore_dirs: HashSet<String>,
    #[serde(default)]
    pub ignore_files: HashSet<String>,
    #[serde(default)]
    pub ignore_extensions: HashSet<String>,
    #[serde(default)]
    pub max_file_size: u64,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            code_extensions: HashSet::from(
                [
                    // Web
                    "js",
                    "jsx",
                    "ts",
                    "tsx",
                    "html",
                    "css",
                    "scss",
                    "vue",
                    "json",
                    // Backend
                    "py",
                    "java",
                    "kt",
                    "rs",
                    "go",
                    "rb",
                    "php",
                    "cs",
                    // Systems
                    "c",
                    "cpp",
                    "h",
                    "hpp",
                    "swift",
                    "dart",
                    // Config/Docs
                    "md",
                    "yaml",
                    "yml",
                    "toml",
                    "xml",
                    "sh",
                    "bash",
                    "sql",
                    "txt",
                    "dockerfile",
                    "makefile",
                ]
                .map(|s| s.to_string()),
            ),
            ignore_dirs: HashSet::from(
                [
                    "node_modules",
                    "dist",
                    "build",
                    "target",
                    "bin",
                    "obj",
                    ".git",
                    ".idea",
                    ".vscode",
                    ".next",
                    ".nuxt",
                    "__pycache__",
                    "venv",
                    "env",
                    ".venv",
                    "coverage",
                    "pods",
                    "deriveddata",
                ]
                .map(|s| s.to_string()),
            ),
            ignore_files: HashSet::from(
                [
                    ".ds_store",
                    "thumbs.db",
                    "package-lock.json",
                    "yarn.lock",
                    "pnpm-lock.yaml",
                    "cargo.lock",
                    "gemfile.lock",
                    "go.sum",
                ]
                .map(|s| s.to_string()),
            ),
            ignore_extensions: HashSet::from(
                [
                    "png", "jpg", "jpeg", "gif", "ico", "svg", "woff", "woff2", "ttf", "eot",
                    "mp3", "mp4", "avi", "mov", "zip", "tar", "gz", "7z", "rar", "exe", "dll",
                    "so", "dylib", "class", "jar", "pyc", "pyo", "pyd",
                ]
                .map(|s| s.to_string()),
            ),
            max_file_size: 1_048_576, // 1MB
        }
    }
}

impl ProjectConfig {
    pub fn apply_overrides(&mut self, overrides: ProjectConfig) {
        if !overrides.code_extensions.is_empty() {
            self.code_extensions = overrides.code_extensions;
        }
        if !overrides.ignore_dirs.is_empty() {
            self.ignore_dirs = overrides.ignore_dirs;
        }
        if !overrides.ignore_files.is_empty() {
            self.ignore_files = overrides.ignore_files;
        }
        if !overrides.ignore_extensions.is_empty() {
            self.ignore_extensions = overrides.ignore_extensions;
        }
        if overrides.max_file_size > 0 {
            self.max_file_size = overrides.max_file_size;
        }
    }
}

pub fn load_config(project_path: &Path) -> ProjectConfig {
    let mut config = ProjectConfig::default();
    let config_path = project_path.join(".scanner-config.json");

    if config_path.exists() {
        if let Ok(content) = fs::read_to_string(&config_path) {
            if let Ok(custom_config) = serde_json::from_str::<ProjectConfig>(&content) {
                config.apply_overrides(custom_config);
            }
        }
    }

    config
}
