mod grid;
mod generator;
mod solver;
mod digger;
mod export;

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "takuzu-gen")]
#[command(about = "Generate Takuzu puzzles with varying difficulty")]
struct Cli {
    /// Grid size (must be even, 4-14+)
    #[arg(short, long)]
    size: usize,

    /// Number of puzzles to generate
    #[arg(short, long)]
    count: usize,

    /// Difficulty level (1-5)
    #[arg(short, long)]
    difficulty: u8,

    /// Output directory
    #[arg(short, long, default_value = ".")]
    output: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    if cli.size < 4 || cli.size % 2 != 0 {
        eprintln!("Error: size must be even and >= 4");
        std::process::exit(1);
    }

    if !(1..=5).contains(&cli.difficulty) {
        eprintln!("Error: difficulty must be between 1 and 5");
        std::process::exit(1);
    }

    println!(
        "Generating {} puzzles of size {}x{} at difficulty {}...",
        cli.count, cli.size, cli.size, cli.difficulty
    );
    // Ensure output directory exists
    std::fs::create_dir_all(&cli.output).expect("Failed to create output directory");
    let puzzles = generator::generate_puzzles(cli.size, cli.count, cli.difficulty);

    let filename = format!(
        "takuzu_{}x{}_d{}.json",
        cli.size, cli.size, cli.difficulty
    );
    let path = cli.output.join(&filename);

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("Failed to create output directory");
    }

    export::save_puzzles(&puzzles, &path).expect("Failed to write output file");

    // Update summary file in the output directory
    export::update_summary(&cli.output).expect("Failed to update summary");

    println!("Saved {} puzzles to {}", puzzles.len(), path.display());
}