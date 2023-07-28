use clap::{Parser, Subcommand};
use crossterm::style::Stylize;
use dotenv::dotenv;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::task::{JoinError, JoinSet};
use tokio::time::sleep;

mod api;
mod config;
mod loading_animator;

use loading_animator::LoadingAnimator;

static LOADING_CHARS: &[char] = &['|', '/', '-', '\\'];
static LOADING_DELAY: Duration = Duration::from_millis(100);

/// A CLI for URL shortening
#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// List shortened URLs
    List,

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

    // TODO: output format
    /// Displays stats
    #[command(arg_required_else_help = true)]
    Stats {
        /// The ID to display stats for
        id: String,
    },
}

fn check_env_var(var: &str) {
    use std::env::VarError;

    match std::env::var(var) {
        Err(VarError::NotPresent) => {
            panic!("env var `{}` not set", var.red())
        }
        Err(VarError::NotUnicode(_)) => {
            panic!("env var `{}` not unicode", var.red())
        }
        Ok(_) => {}
    }
}

#[tokio::main]
async fn main() -> Result<(), JoinError> {
    dotenv().ok();
    check_env_var("URL_SHORTENER_API_URL");

    let args = Cli::parse();

    let api_request = async {
        match args.command {
            Commands::List => match api::get_all_urls().await {
                Ok(all_urls) => Ok(all_urls
                    .into_iter()
                    .map(|(id, info)| format!("{id} {url}", id = id.green().bold(), url = info.url))
                    .collect::<Vec<_>>()
                    .join("\n")),
                Err(err) => Err(err.to_string()),
            },

            Commands::Shorten { url } => match api::shorten(url).await {
                Ok(shortened) => Ok(shortened.id),
                Err(err) => Err(err.to_string()),
            },

            Commands::Lengthen { id } => match api::lengthen(id).await {
                Ok(lengthened) => Ok(lengthened.url),
                Err(err) => Err(err.to_string()),
            },

            Commands::Stats { id } => {
                // TODO: output format
                match api::stats(id).await {
                    Ok(stats) => Ok(format!("{:#?}", stats)),
                    Err(err) => Err(err.to_string()),
                }
            }
        }
    };

    // joinset: https://stackoverflow.com/a/69424585
    let mut join_set = JoinSet::new();

    join_set.spawn(api_request);

    let animator = {
        let animator = Arc::new(Mutex::new(
            LoadingAnimator::new(LOADING_CHARS).expect("chars should not be empty"),
        ));
        let ani = animator.clone();

        join_set.spawn(async move {
            loop {
                ani.lock().unwrap().display().unwrap();
                sleep(LOADING_DELAY).await;
            }
        });

        animator
    };

    // The API request is the only future that will complete
    if let Some(res) = join_set.join_next().await {
        // manually stop loading animation
        animator.lock().unwrap().stop_and_clear().unwrap();

        let out = res?;
        match out {
            Ok(s) => println!("{s}"),
            Err(e) => eprintln!("{}", e.red().bold()),
        }
    }

    // Abort loading animation task
    join_set.abort_all();

    Ok(())
}
