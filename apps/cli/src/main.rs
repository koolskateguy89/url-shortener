use clap::{Args, Parser, Subcommand, ValueEnum};
use common::types::ShortenResponse;

mod api;

/// A CLI for URL shortening
#[derive(Debug, Parser)]
#[command(name = "git")]
#[command(about = "A fictional versioning CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Shortens URL
    #[command(arg_required_else_help = true)]
    Shorten {
        /// The url to shorten
        url: String,
    },

    /// Lengthens ID
    #[command(arg_required_else_help = true)]
    Lengthen {
        /// The ID to lengthen
        id: String,
    },

    // TODO: display format
    /// Displays stats
    #[command(arg_required_else_help = true)]
    Stats {
        /// The ID to display stats for
        id: String,
    },
}

// TODO (probably)
// some sort of console coloring library

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    // TODO: display loading

    let res: Result<String, String> = match args.command {
        Commands::Shorten { url } => {
            match api::shorten(url).await {
                Ok(shortened) => Ok(shortened.id),
                // TODO: proper(?) error handle
                Err(err) => Err(format!("{err:?}")),
            }
        }
        Commands::Lengthen { id } => {
            // TODO
            println!("Lengthening {}", id);
            Err("not impl.".to_string())
        }
        Commands::Stats { id } => {
            // TODO
            println!("Stats for {}", id);
            Err("not impl.".to_string())
        }
    };

    // TODO: colour
    match res {
        Ok(s) => println!("{}", s),
        Err(e) => eprintln!("Error: {}", e),
    }
}
