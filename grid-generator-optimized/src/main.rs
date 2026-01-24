mod digger;
mod export;
mod generator;
mod grid;
mod solver;
mod transform;

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "takuzu-gen")]
#[command(about = "Generate Takuzu puzzles with varying difficulty")]
struct Cli {
    #[arg(short, long)]
    size: usize,

    #[arg(short, long)]
    count: usize,

    #[arg(short, long)]
    difficulty: u8,

    #[arg(short, long, default_value = "false")]
    try_harder: bool,

    #[arg(short, long, default_value = "grids")]
    output: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    if cli.size < 4 || cli.size % 2 != 0 {
        eprintln!("Error: size must be even and >= 4");
        std::process::exit(1);
    }

    if !(1..=4).contains(&cli.difficulty) {
        eprintln!("Error: difficulty must be between 1 and 4");
        std::process::exit(1);
    }

    println!(
        "Generating {} puzzles of size {}x{} at difficulty {} (try_harder: {})...",
        cli.count, cli.size, cli.size, cli.difficulty, cli.try_harder
    );

    let puzzles = generator::generate_puzzles(cli.size, cli.count, cli.difficulty, cli.try_harder);

    println!(
        "\nGeneration complete. Routing puzzles to their proper difficulty files for size {}x{}:",
        cli.size, cli.size
    );

    let solver = solver::HumanSolver::new(cli.difficulty.into());
    let mut puzzles_by_difficulty: std::collections::HashMap<u8, Vec<grid::Puzzle>> = std::collections::HashMap::new();

    for p in puzzles {
        let grid = grid::Grid::from_puzzle(&p);
        let actual_difficulty = solver.evaluate_difficulty(&grid)
            .map(|d| d as u8)
            .unwrap_or(cli.difficulty); // Fallback to target if evaluation fails
        
        puzzles_by_difficulty.entry(actual_difficulty).or_default().push(p);
    }

    for (difficulty, grouped_puzzles) in puzzles_by_difficulty {
        let filename = format!("takuzu_{}x{}_d{}.json", cli.size, cli.size, difficulty);
        let path = cli.output.join(&filename);

        println!(
            "  Saved {} puzzles to {}",
            grouped_puzzles.len(),
            filename
        );

        export::save_puzzles(&grouped_puzzles, &path).expect("Failed to write output file");
    }

    // Update summary file in the output directory
    export::update_summary(&cli.output).expect("Failed to update summary");
}