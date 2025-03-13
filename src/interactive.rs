use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use tracing::info;
use warp::Filter;

use crate::render::render_markdown_to_html;
use crate::utils::{build_full_html, open_in_browser};

/// Scans the current directory for Markdown files and prompts the user
/// to view each file one at a time. For each accepted file, the file is read,
/// rendered to HTML, written to a temporary file that is persisted, and opened
/// in the browser. Once the user presses Enter, the file is manually removed.
pub async fn interactive_viewer(host: &str, port: &str) -> io::Result<()> {
    info!("Entering interactive viewer mode...");
    let entries = fs::read_dir(".")?;
    let md_files: Vec<PathBuf> = entries
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                let path = e.path();
                if path.extension().map(|ext| ext == "md").unwrap_or(false) {
                    Some(path)
                } else {
                    None
                }
            })
        })
        .collect();

    if md_files.is_empty() {
        eprintln!("No Markdown files found in the current directory.");
        return Ok(());
    }

    for file in md_files {
        println!("View file {}? (y/N): ", file.display());
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if matches!(input.trim().to_lowercase().as_str(), "y" | "yes") {
            let file_name = file
                .file_name()
                .unwrap_or_else(|| "Unknown".as_ref())
                .to_string_lossy()
                .to_string();
            let mut file_handle = fs::File::open(&file)?;
            let mut markdown_input = String::new();
            file_handle.read_to_string(&mut markdown_input)?;

            let html_output = render_markdown_to_html(&markdown_input);
            let full_html = build_full_html(&file_name, &html_output);

            // Persist the file so it is not deleted when temp_file is dropped.
            let route = warp::path::end().map(move || warp::reply::html(full_html.clone()));
            let addr: std::net::SocketAddr = format!("{}:{}", host, port).parse().unwrap();
            info!("Server running at http://{}", addr);
            open_in_browser(format!("http://{}:{}", host, port).as_str());
            warp::serve(route).run(addr).await;
            println!("Press Enter to continue to the next file...");
            let mut dummy = String::new();
            io::stdin().read_line(&mut dummy)?;
        }
    }
    Ok(())
}
