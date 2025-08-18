use anyhow::Result;
use clap::{Parser, Subcommand};
use release_test_core::DataModel;
use release_test_utils::{format_data, serialize_data};

#[derive(Parser)]
#[command(name = "release-test")]
#[command(about = "A test CLI for release-plz", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Process {
        #[arg(short, long)]
        id: u64,
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        value: f64,
    },
    
    Format {
        #[arg(short, long)]
        json: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Process { id, name, value } => {
            let model = DataModel::new(id, name, value)?;
            println!("Original: {}", format_data(&model));
            println!("Processed value: {}", model.process());
        }
        Commands::Format { json } => {
            let model = DataModel::new(1, "example".to_string(), 100.0)?;
            if json {
                println!("{}", serialize_data(&model)?);
            } else {
                println!("{}", format_data(&model));
            }
        }
    }

    Ok(())
}