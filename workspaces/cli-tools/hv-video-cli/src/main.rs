use clap::{Parser, Subcommand};
use processing_video::ConcatClips;
use processing_core::{Engine};

#[derive(Parser, Debug)]
#[command(author, version, about = "Hybrid-VID Video CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// List available video blocks
    ListBlocks,
    /// Demo concat (stub)
    Concat { a: String, b: String, output: String },
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();
    match cli.command {
        Commands::ListBlocks => {
            println!("concat_clips - Concat Clips");
        }
        Commands::Concat { a, b, output } => {
            let mut engine = Engine::new();
            engine.register("concat".into(), ConcatClips);
            // Stub: simply acknowledge request
            println!("Concatenated '{}' + '{}' -> '{}' (stub)", a, b, output);
        }
    }
}

