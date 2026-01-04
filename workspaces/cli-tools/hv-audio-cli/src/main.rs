use clap::{Parser, Subcommand};
use processing_audio::NormalizeAudio;
use processing_core::{Engine, ExecutionOptions};
use processing_core::model::{Artifact};
use std::collections::HashMap;

#[derive(Parser, Debug)]
#[command(author, version, about = "Hybrid-VID Audio CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// List available audio blocks
    ListBlocks,
    /// Run normalize on an input file
    Normalize { input: String, output: String },
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();
    match cli.command {
        Commands::ListBlocks => {
            println!("normalize_audio - Normalize Audio");
        }
        Commands::Normalize { input, output } => {
            let mut engine = Engine::new();
            engine.register("normalize".into(), NormalizeAudio);
            // For the demo, directly invoke the block
            let mut inputs = HashMap::new();
            inputs.insert(
                "in".to_string(),
                Artifact { port: "in".into(), path: input.clone(), meta: HashMap::new() }
            );
            let result = engine.blocks.get("normalize").unwrap().run(&inputs).unwrap();
            let mut out_artifact = result.get("out").unwrap().clone();
            // Stub: just map to provided output path
            out_artifact.path = output;
            println!("Wrote: {}", out_artifact.path);
        }
    }
}

