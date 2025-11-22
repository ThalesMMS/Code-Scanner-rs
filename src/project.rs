//
// project.rs
// Code-Scanner-rs
//
// Detects project types from manifest files and determines whether a path should be treated as a standalone project root before scanning.
//
// Thales Matheus Mendonça Santos - November 2025
//

use std::path::Path;

// Infer a friendly project type name based on well-known manifest files.
pub fn detect_project_type(path: &Path) -> String {
    if path.join("package.json").exists() {
        if path.join("next.config.js").exists() {
            return "Next.js".to_string();
        }
        if path.join("tsconfig.json").exists() {
            return "TypeScript/Node".to_string();
        }
        return "Node.js".to_string();
    }
    if path.join("Cargo.toml").exists() {
        return "Rust".to_string();
    }
    if path.join("requirements.txt").exists()
        || path.join("pyproject.toml").exists()
        || path.join("setup.py").exists()
    {
        if path.join("manage.py").exists() {
            return "Django".to_string();
        }
        return "Python".to_string();
    }
    if path.join("pom.xml").exists() || path.join("build.gradle").exists() {
        return "Java/Kotlin".to_string();
    }
    if path.join("go.mod").exists() {
        return "Go".to_string();
    }
    if path.join("pubspec.yaml").exists() {
        return "Flutter".to_string();
    }

    "Genérico".to_string()
}

// Decide whether the provided path is already a single project root, so we avoid
// recursing into child directories unnecessarily.
pub fn is_single_project_root(path: &Path) -> bool {
    path.join("package.json").exists()
        || path.join("Cargo.toml").exists()
        || path.join(".git").exists()
        || path.join("requirements.txt").exists()
}
