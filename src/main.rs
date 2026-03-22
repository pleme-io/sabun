use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "sabun", about = "Semantic binary diff")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compute semantic diff between two APKs
    Diff {
        /// Path to the base APK
        base: String,
        /// Path to the target APK
        target: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Diff { base, target } => {
            println!("sabun: diffing {base} -> {target}");
        }
    }
}
