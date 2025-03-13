//! CLI entry point for omd

use clap::Parser;
use tracing::info;

use omd::cli::Args;
use omd::server::{run_server_mode, run_static_mode};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let args = Args::parse();
    info!("Parsed arguments: {:?}", args);

    // if let Err(e) = interactive_viewer("127.0.0.1","3030").await {
    //      eprintln!("Error in interactive viewer: {}", e);
    //  }
    if args.static_mode {
        run_static_mode(&args).expect("Failed to run in static mode");
    } else {
        run_server_mode(&args)
            .await
            .expect("Failed to run server mode");
    }
}
