use crate::cli::Args;
use crate::config::{load_config, ProjectConfig};
use crate::project::detect_project_type;
use crate::utils::{format_size, is_binary};
use anyhow::{Context, Result};
use chrono::Local;
use ignore::{Walk, WalkBuilder};
use pathdiff::diff_paths;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

pub fn process_project(project_path: &Path, output_dir: &Path, args: &Args) -> Result<()> {
    let project_name = project_path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned();

    let output_file_path = output_dir.join(format!("{}_project_code.txt", project_name));
    let project_type = detect_project_type(project_path);
    let config = load_config(project_path);

    println!("📦 Processando: {} ({})", project_name, project_type);

    let mut output_file = File::create(&output_file_path).with_context(|| {
        format!(
            "Falha ao criar arquivo de saída: {}",
            output_file_path.display()
        )
    })?;

    write_header(&mut output_file, &project_name, &project_type)?;
    writeln!(output_file, "\n📂 ESTRUTURA DE DIRETÓRIOS")?;
    writeln!(
        output_file,
        "═══════════════════════════════════════════════════════════════"
    )?;

    let walker = build_walker(project_path, args);
    let (mut valid_files, mut stats) =
        collect_files(project_path, &config, args, walker, &mut output_file)?;
    valid_files.sort();

    writeln!(output_file, "\n📄 CONTEÚDO DOS ARQUIVOS")?;
    writeln!(
        output_file,
        "═══════════════════════════════════════════════════════════════"
    )?;

    write_file_contents(project_path, &valid_files, &mut output_file, &mut stats)?;
    write_summary(&mut output_file, &stats, valid_files.len())?;

    println!("  ✅ Salvo em: {}", output_file_path.display());
    Ok(())
}

fn build_walker(project_path: &Path, args: &Args) -> Walk {
    WalkBuilder::new(project_path)
        .git_ignore(!args.no_gitignore)
        .hidden(false)
        .build()
}

fn write_header(output_file: &mut File, project_name: &str, project_type: &str) -> Result<()> {
    writeln!(
        output_file,
        "╔═══════════════════════════════════════════════════════════════╗"
    )?;
    writeln!(output_file, "║ PROJETO: {:<45}║", project_name)?;
    writeln!(output_file, "║ Tipo: {:<48}║", project_type)?;
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    writeln!(output_file, "║ Data: {:<48}║", now)?;
    writeln!(
        output_file,
        "╚═══════════════════════════════════════════════════════════════╝"
    )?;
    Ok(())
}

fn collect_files(
    project_path: &Path,
    config: &ProjectConfig,
    args: &Args,
    walker: Walk,
    output_file: &mut File,
) -> Result<(Vec<PathBuf>, ScanStats)> {
    let mut valid_files: Vec<PathBuf> = Vec::new();
    let mut stats = ScanStats::default();

    for result in walker {
        match result {
            Ok(entry) => {
                let path = entry.path();

                if path == project_path {
                    continue;
                }

                let relative_path =
                    diff_paths(path, project_path).unwrap_or_else(|| path.to_path_buf());
                let file_name = path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_lowercase();

                if config.ignore_dirs.contains(&file_name) {
                    continue;
                }

                if path.is_dir() {
                    let depth = relative_path.components().count();
                    let indent = "  ".repeat(depth.saturating_sub(1));
                    writeln!(
                        output_file,
                        "{}├── {}/",
                        indent,
                        relative_path.file_name().unwrap().to_string_lossy()
                    )?;
                    continue;
                }

                if config.ignore_files.contains(&file_name) {
                    stats.skipped += 1;
                    continue;
                }

                let ext = path
                    .extension()
                    .map(|e| e.to_string_lossy().to_string().to_lowercase())
                    .unwrap_or_default();

                if config.ignore_extensions.contains(&ext) {
                    stats.skipped += 1;
                    continue;
                }

                if !ext.is_empty() && !config.code_extensions.contains(&ext) {
                    if !config.code_extensions.contains(&file_name) {
                        stats.skipped += 1;
                        continue;
                    }
                }

                let metadata = match path.metadata() {
                    Ok(m) => m,
                    Err(_) => continue,
                };

                if metadata.len() > config.max_file_size {
                    if args.verbose {
                        println!("Ignorando {} (tamanho excessivo)", relative_path.display());
                    }
                    stats.skipped += 1;
                    continue;
                }

                valid_files.push(path.to_path_buf());

                let depth = relative_path.components().count();
                let indent = "  ".repeat(depth.saturating_sub(1));
                writeln!(
                    output_file,
                    "{}└── {}",
                    indent,
                    relative_path.file_name().unwrap().to_string_lossy()
                )?;
            }
            Err(err) => {
                if args.verbose {
                    eprintln!("Erro ao ler entrada: {}", err);
                }
            }
        }
    }

    Ok((valid_files, stats))
}

fn write_file_contents(
    project_path: &Path,
    files: &[PathBuf],
    output_file: &mut File,
    stats: &mut ScanStats,
) -> Result<()> {
    for path in files {
        let relative_path = diff_paths(path, project_path).unwrap_or_else(|| path.to_path_buf());
        let relative_str = relative_path.to_string_lossy();
        let size = path
            .metadata()
            .with_context(|| format!("Falha ao ler metadata de {}", relative_path.display()))?
            .len();

        writeln!(
            output_file,
            "┌─────────────────────────────────────────────────────────────"
        )?;
        writeln!(output_file, "│ 📄 {}", relative_str)?;
        writeln!(output_file, "│ 📊 Tamanho: {}", format_size(size))?;
        writeln!(
            output_file,
            "├─────────────────────────────────────────────────────────────"
        )?;

        if is_binary(path) {
            writeln!(
                output_file,
                "│ [Binary file or unsupported encoding - content omitted]"
            )?;
        } else {
            match fs::read_to_string(path) {
                Ok(content) => {
                    for (i, line) in content.lines().enumerate() {
                        writeln!(output_file, "{:>4} │ {}", i + 1, line)?;
                    }
                }
                Err(_) => {
                    writeln!(output_file, "│ [Error reading file as UTF-8 text]")?;
                }
            }
        }

        writeln!(
            output_file,
            "└─────────────────────────────────────────────────────────────\n"
        )?;
        stats.total_size += size;
    }

    Ok(())
}

fn write_summary(output_file: &mut File, stats: &ScanStats, processed_count: usize) -> Result<()> {
    writeln!(
        output_file,
        "\n═══════════════════════════════════════════════════════════════"
    )?;
    writeln!(output_file, "📊 RESUMO")?;
    writeln!(
        output_file,
        "  ✅ Arquivos processados: {}",
        processed_count
    )?;
    writeln!(
        output_file,
        "  ⏭️  Arquivos ignorados (estimado): {}",
        stats.skipped
    )?;
    writeln!(
        output_file,
        "  💾 Tamanho total do conteúdo: {}",
        format_size(stats.total_size)
    )?;
    writeln!(
        output_file,
        "═══════════════════════════════════════════════════════════════"
    )?;
    Ok(())
}

#[derive(Default)]
struct ScanStats {
    total_size: u64,
    skipped: u64,
}
