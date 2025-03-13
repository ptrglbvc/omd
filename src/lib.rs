//! Public API for omd

pub mod cli;
pub mod interactive;
pub mod render;
pub mod server;
pub mod utils;

// Re-export key functions for easier external access.
pub use render::render_markdown_to_html;
pub use server::{run_server_mode, run_static_mode};
pub use utils::{build_full_html, open_in_browser, read_fonts, read_style_css};
