use anyhow::{Context, Result};
use clap::Parser;
use ignore::WalkBuilder;
use serde::Deserialize;
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

// --- CONFIGURATION AND CLI ---

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input directory (project to scan)
    #[arg(short, long, default_value = "./input")]
    input_dir: PathBuf,

    /// Output directory for reports
    #[arg(short, long, default_value = "./output")]
    output_dir: PathBuf,

    /// Ignore the project's .gitignore file
    #[arg(long)]
    no_gitignore: bool,

    /// Verbose mode
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Debug, Deserialize, Clone)]
struct ProjectConfig {
    #[serde(default)]
    code_extensions: HashSet<String>,
    #[serde(default)]
    ignore_dirs: HashSet<String>,
    #[serde(default)]
    ignore_files: HashSet<String>,
    #[serde(default)]
    ignore_extensions: HashSet<String>,
    #[serde(default)]
    max_file_size: u64,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            code_extensions: HashSet::from([
                // Web
                "js", "jsx", "ts", "tsx", "html", "css", "scss", "vue", "json",
                // Backend
                "py", "java", "kt", "rs", "go", "rb", "php", "cs",
                // Systems
                "c", "cpp", "h", "hpp", "swift", "dart",
                // Config/Docs
                "md", "yaml", "yml", "toml", "xml", "sh", "bash", "sql", "txt", "dockerfile", "makefile"
            ].map(|s| s.to_string())),
            ignore_dirs: HashSet::from([
                "node_modules", "dist", "build", "target", "bin", "obj", 
                ".git", ".idea", ".vscode", ".next", ".nuxt", 
                "__pycache__", "venv", "env", ".venv", "coverage",
                "pods", "deriveddata"
            ].map(|s| s.to_string())),
            ignore_files: HashSet::from([
                ".ds_store", "thumbs.db", "package-lock.json", "yarn.lock", 
                "pnpm-lock.yaml", "cargo.lock", "gemfile.lock", "go.sum"
            ].map(|s| s.to_string())),
            ignore_extensions: HashSet::from([
                "png", "jpg", "jpeg", "gif", "ico", "svg", "woff", "woff2", 
                "ttf", "eot", "mp3", "mp4", "avi", "mov", "zip", "tar", "gz", 
                "7z", "rar", "exe", "dll", "so", "dylib", "class", "jar", 
                "pyc", "pyo", "pyd"
            ].map(|s| s.to_string())),
            max_file_size: 1_048_576, // 1MB
        }
    }
}

// --- PROJECT TYPE DETECTION ---

fn detect_project_type(path: &Path) -> String {
    if path.join("package.json").exists() {
        if path.join("next.config.js").exists() { return "Next.js".to_string(); }
        if path.join("tsconfig.json").exists() { return "TypeScript/Node".to_string(); }
        return "Node.js".to_string();
    }
    if path.join("Cargo.toml").exists() { return "Rust".to_string(); }
    if path.join("requirements.txt").exists() || path.join("pyproject.toml").exists() || path.join("setup.py").exists() {
        if path.join("manage.py").exists() { return "Django".to_string(); }
        return "Python".to_string();
    }
    if path.join("pom.xml").exists() || path.join("build.gradle").exists() { return "Java/Kotlin".to_string(); }
    if path.join("go.mod").exists() { return "Go".to_string(); }
    if path.join("pubspec.yaml").exists() { return "Flutter".to_string(); }
    
    "Genérico".to_string()
}

// --- HELPERS ---

fn is_binary(path: &Path) -> bool {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return true, // Assume binary if the file cannot be opened
    };
    let mut buffer = [0; 1024];
    let n = match file.read(&mut buffer) {
        Ok(n) => n,
        Err(_) => return true,
    };
    // Look for a null byte, a classic binary indicator
    buffer[..n].contains(&0)
}

fn load_config(project_path: &Path) -> ProjectConfig {
    let mut config = ProjectConfig::default();
    let config_path = project_path.join(".scanner-config.json");

    if config_path.exists() {
        if let Ok(content) = fs::read_to_string(&config_path) {
            if let Ok(custom_config) = serde_json::from_str::<ProjectConfig>(&content) {
                // Basic merge: replace lists if provided
                if !custom_config.code_extensions.is_empty() { config.code_extensions = custom_config.code_extensions; }
                if !custom_config.ignore_dirs.is_empty() { config.ignore_dirs = custom_config.ignore_dirs; }
                if custom_config.max_file_size > 0 { config.max_file_size = custom_config.max_file_size; }
            }
        }
    }
    config
}

fn format_size(size: u64) -> String {
    humansize::format_size(size, humansize::DECIMAL)
}

// --- MAIN PROCESSING LOGIC ---

fn process_project(project_path: &Path, output_dir: &Path, args: &Args) -> Result<()> {
    let project_name = project_path.file_name().unwrap().to_string_lossy();
    let output_file_path = output_dir.join(format!("{}_project_code.txt", project_name));
    let project_type = detect_project_type(project_path);
    let config = load_config(project_path);

    println!("📦 Processando: {} ({})", project_name, project_type);

    let mut output_file = File::create(&output_file_path)?;

    // Header
    writeln!(output_file, "╔═══════════════════════════════════════════════════════════════╗")?;
    writeln!(output_file, "║ PROJETO: {:<45}║", project_name)?;
    writeln!(output_file, "║ Tipo: {:<48}║", project_type)?;
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    writeln!(output_file, "║ Data: {:<48}║", now)?;
    writeln!(output_file, "╚═══════════════════════════════════════════════════════════════╝")?;
    writeln!(output_file, "\n📂 ESTRUTURA DE DIRETÓRIOS")?;
    writeln!(output_file, "═══════════════════════════════════════════════════════════════")?;

    // Walker configuration (respects .gitignore by default)
    let walker = WalkBuilder::new(project_path)
        .git_ignore(!args.no_gitignore)
        .hidden(false) // Include hidden files (we filter .git manually later)
        .build();

    let mut valid_files: Vec<PathBuf> = Vec::new();
    let mut stats_total_size = 0u64;
    let mut stats_skipped_count = 0;

    // Step 1: Collect and filter
    for result in walker {
        match result {
            Ok(entry) => {
                let path = entry.path();
                
                // Skip the root directory in the listing
                if path == project_path { continue; }

                let relative_path = pathdiff::diff_paths(path, project_path).unwrap_or(path.to_path_buf());
                let file_name = path.file_name().unwrap_or_default().to_string_lossy().to_lowercase();
                
                // Manual ignore filtering for directories/files (beyond .gitignore)
                let is_dir = path.is_dir();
                
                // Check ignored directories from config
                if config.ignore_dirs.contains(&file_name) {
                    continue; // WalkBuilder already skips directories; this reinforces the rule
                }

                if is_dir {
                    // Only print in the tree; do not add to the read list
                    let depth = relative_path.components().count();
                    let indent = "  ".repeat(depth.saturating_sub(1));
                    writeln!(output_file, "{}├── {}/", indent, relative_path.file_name().unwrap().to_string_lossy())?;
                    continue;
                }

                // File filtering
                if config.ignore_files.contains(&file_name) {
                    stats_skipped_count += 1;
                    continue; 
                }

                // Extension check
                let ext = path.extension()
                    .map(|e| e.to_string_lossy().to_string().to_lowercase())
                    .unwrap_or_default();

                if config.ignore_extensions.contains(&ext) {
                    stats_skipped_count += 1;
                    continue;
                }
                
                // If extension is set and not allowlisted, check for well-known config files
                // If there is no extension (e.g., Dockerfile), generally accept
                if !ext.is_empty() && !config.code_extensions.contains(&ext) {
                    // Special case for files without extension in the list (e.g., Dockerfile)
                    if !config.code_extensions.contains(&file_name) {
                        stats_skipped_count += 1;
                        continue;
                    }
                }

                // Size check
                let metadata = match path.metadata() {
                    Ok(m) => m,
                    Err(_) => continue,
                };

                if metadata.len() > config.max_file_size {
                    if args.verbose { println!("Ignorando {} (tamanho excessivo)", relative_path.display()); }
                    stats_skipped_count += 1;
                    continue;
                }

                // If it passed all filters, add to the list
                valid_files.push(path.to_path_buf());
                
                // Print entry in the tree
                let depth = relative_path.components().count();
                let indent = "  ".repeat(depth.saturating_sub(1));
                writeln!(output_file, "{}└── {}", indent, relative_path.file_name().unwrap().to_string_lossy())?;
            }
            Err(err) => if args.verbose { eprintln!("Erro ao ler entrada: {}", err); }
        }
    }

    valid_files.sort();

    // Step 2: File contents
    writeln!(output_file, "\n📄 CONTEÚDO DOS ARQUIVOS")?;
    writeln!(output_file, "═══════════════════════════════════════════════════════════════")?;

    for path in &valid_files {
        let relative_path = pathdiff::diff_paths(path, project_path).unwrap();
        let relative_str = relative_path.to_string_lossy();
        let size = path.metadata()?.len();
        
        writeln!(output_file, "┌─────────────────────────────────────────────────────────────")?;
        writeln!(output_file, "│ 📄 {}", relative_str)?;
        writeln!(output_file, "│ 📊 Tamanho: {}", format_size(size))?;
        writeln!(output_file, "├─────────────────────────────────────────────────────────────")?;

        if is_binary(path) {
             writeln!(output_file, "│ [Binary file or unsupported encoding - content omitted]")?;
        } else {
            match fs::read_to_string(path) {
                Ok(content) => {
                    for (i, line) in content.lines().enumerate() {
                        writeln!(output_file, "{:>4} │ {}", i + 1, line)?;
                    }
                },
                Err(_) => {
                    writeln!(output_file, "│ [Error reading file as UTF-8 text]")?;
                }
            }
        }
        writeln!(output_file, "└─────────────────────────────────────────────────────────────\n")?;
        stats_total_size += size;
    }

    // Footer
    writeln!(output_file, "\n═══════════════════════════════════════════════════════════════")?;
    writeln!(output_file, "📊 RESUMO")?;
    writeln!(output_file, "  ✅ Arquivos processados: {}", valid_files.len())?;
    writeln!(output_file, "  ⏭️  Arquivos ignorados (estimado): {}", stats_skipped_count)?;
    writeln!(output_file, "  💾 Tamanho total do conteúdo: {}", format_size(stats_total_size))?;
    writeln!(output_file, "═══════════════════════════════════════════════════════════════")?;

    println!("  ✅ Salvo em: {}", output_file_path.display());
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    if !args.input_dir.exists() {
        anyhow::bail!("Diretório de entrada não encontrado: {:?}", args.input_dir);
    }

    fs::create_dir_all(&args.output_dir).context("Falha ao criar diretório de saída")?;

    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║               RUST CODE SCANNER (UNIFIED)                     ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!("📍 Entrada: {:?}", args.input_dir);
    println!("📍 Saída:   {:?}", args.output_dir);
    println!();

    // Detect whether the input is a single project or a folder containing projects
    let is_single_project = args.input_dir.join("package.json").exists() 
        || args.input_dir.join("Cargo.toml").exists() 
        || args.input_dir.join(".git").exists()
        || args.input_dir.join("requirements.txt").exists();

    if is_single_project {
        process_project(&args.input_dir, &args.output_dir, &args)?;
    } else {
        // Iterate over subdirectories
        let entries = fs::read_dir(&args.input_dir)?;
        let mut projects_found = 0;
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                process_project(&path, &args.output_dir, &args)?;
                projects_found += 1;
            }
        }
        if projects_found == 0 {
            println!("ℹ️  No subdirectories found. Processing root as a single project.");
            process_project(&args.input_dir, &args.output_dir, &args)?;
        }
    }

    println!("\n✨ Concluído!");
    Ok(())
}
