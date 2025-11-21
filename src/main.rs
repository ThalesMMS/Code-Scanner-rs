mod cli;
mod config;
mod project;
mod scanner;
mod utils;

use crate::cli::Args;
use crate::project::is_single_project_root;
use crate::scanner::process_project;
use anyhow::{bail, Context, Result};
use clap::Parser;
use std::fs;

fn main() -> Result<()> {
    let args = Args::parse();

    if !args.input_dir.exists() {
        bail!("Diretório de entrada não encontrado: {:?}", args.input_dir);
    }

    fs::create_dir_all(&args.output_dir).context("Falha ao criar diretório de saída")?;

    print_banner(&args);

    if is_single_project_root(&args.input_dir) {
        process_project(&args.input_dir, &args.output_dir, &args)?;
    } else {
        process_subdirectories(&args)?;
    }

    println!("\n✨ Concluído!");
    Ok(())
}

fn process_subdirectories(args: &Args) -> Result<()> {
    let mut projects_found = 0;

    for entry in fs::read_dir(&args.input_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            process_project(&path, &args.output_dir, args)?;
            projects_found += 1;
        }
    }

    if projects_found == 0 {
        println!("ℹ️  No subdirectories found. Processing root as a single project.");
        process_project(&args.input_dir, &args.output_dir, args)?;
    }

    Ok(())
}

fn print_banner(args: &Args) {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║               RUST CODE SCANNER (UNIFIED)                     ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!("📍 Entrada: {:?}", args.input_dir);
    println!("📍 Saída:   {:?}", args.output_dir);
    println!();
}
