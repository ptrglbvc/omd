use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Markdown file to read
    #[arg(value_name = "FILE")]
    pub file: Option<PathBuf>,
    /// The host name
    #[arg(short = 'H', long = "host", default_value = "127.0.0.1")]
    pub host: String,
    /// The port number
    #[arg(short = 'P', long = "port", default_value = "3030")]
    pub port: u16,
    /// Temporary HTML file instead of a server.
    #[arg(short, long)]
    pub static_mode: bool,
    /// Renders the markdown in clipboard
    #[arg(short = 'C', long = "clipboard")]
    pub clipboard: bool,
}
