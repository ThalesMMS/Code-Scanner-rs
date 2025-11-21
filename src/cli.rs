use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Input directory (project to scan)
    #[arg(short, long, default_value = "./input")]
    pub input_dir: PathBuf,

    /// Output directory for reports
    #[arg(short, long, default_value = "./output")]
    pub output_dir: PathBuf,

    /// Ignore the project's .gitignore file
    #[arg(long)]
    pub no_gitignore: bool,

    /// Verbose mode
    #[arg(short, long)]
    pub verbose: bool,
}
