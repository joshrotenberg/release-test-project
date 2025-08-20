use anyhow::Result;
use clap::{Parser, Subcommand};
use release_test_core::DataModel;
use release_test_utils::{format_data, serialize_data};

#[derive(Parser)]
#[command(name = "release-test")]
#[command(version, about = "A demo CLI for automated release testing", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Process data with the given parameters
    Process {
        /// Unique identifier for the data
        #[arg(short, long)]
        id: u64,
        /// Name or label for the data
        #[arg(short, long)]
        name: String,
        /// Numeric value to process
        #[arg(short, long)]
        value: f64,
    },

    /// Format and display sample data
    Format {
        /// Output in JSON format instead of plain text
        #[arg(short, long)]
        json: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Process { id, name, value } => {
            if name.is_empty() {
                anyhow::bail!("Name cannot be empty");
            }
            let model = DataModel::new(id, name, value)?;
            println!("Original: {}", format_data(&model));
            println!("Processed value: {}", model.process());
            Ok(())
        }
        Commands::Format { json } => {
            let model = DataModel::new(1, "example".to_string(), 100.0)?;
            if json {
                println!("{}", serialize_data(&model)?);
            } else {
                println!("{}", format_data(&model));
            }
            Ok(())
        }
    };

    // Ensure proper exit code on error
    if let Err(e) = &result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    result
}
