use crate::cli::Args;
use crate::render::render_markdown_to_html;
use crate::utils::open_in_browser;
use std::fs;
use std::io::{self, Read, Write};
use std::net::SocketAddr;
use std::path::PathBuf;
use tempfile;
use tracing::info;
use warp::Filter;

/// Runs omd in static mode:
/// Reads Markdown (from file, clipboard, or stdin), renders it to HTML,
/// writes it to a temporary HTML file, opens it in the browser, and waits for user input.
pub fn run_static_mode(args: &crate::cli::Args) -> io::Result<()> {
    let (file_name, markdown_input) = if args.clipboard {
        let mut clipboard = arboard::Clipboard::new().unwrap();
        let content = clipboard.get_text().unwrap_or_else(|err| {
            eprintln!("Error reading from clipboard: {}", err);
            std::process::exit(1);
        });
        (String::from("Clipboard"), content)
    } else {
        match &args.file {
            Some(file_path) => {
                let mut file = fs::File::open(&file_path).unwrap_or_else(|err| {
                    eprintln!("Error opening file {}: {}", file_path.display(), err);
                    std::process::exit(1);
                });
                let mut content = String::new();
                file.read_to_string(&mut content)?;
                (
                    file_path.file_name().unwrap().to_string_lossy().to_string(),
                    content,
                )
            }
            None => {
                let mut content = String::new();
                io::stdin().read_to_string(&mut content)?;
                (String::from("New file"), content)
            }
        }
    };

    let html_output = render_markdown_to_html(&markdown_input);
    let full_html = crate::utils::build_full_html(&file_name, &html_output);

    let temp_file = tempfile::Builder::new()
        .prefix("markdown_preview_")
        .suffix(".html")
        .tempfile()?;
    let temp_path = temp_file.path().to_string_lossy().to_string();
    open_in_browser(&temp_path);

    let mut file = temp_file.as_file();
    file.write_all(full_html.as_bytes())?;
    file.flush()?;

    println!("Press Enter to exit...");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(())
}

/// Runs omd in server mode:
/// Reads Markdown from the specified file (or clipboard), renders it to HTML,
/// embeds CSS, fonts, and favicon, automatically opens the browser to the server URL,
/// and serves the HTML content.
pub async fn run_server_mode(args: &Args) -> io::Result<()> {
    let (_file_path, _file_name, markdown_input) = if args.clipboard {
        let mut clipboard = arboard::Clipboard::new().unwrap();
        let content = clipboard.get_text().unwrap_or_else(|err| {
            eprintln!("Error reading from clipboard: {}", err);
            std::process::exit(1);
        });
        (
            PathBuf::from("Clipboard"),
            String::from("Clipboard"),
            content,
        )
    } else {
        let file_path = match &args.file {
            Some(path) => path.clone(),
            None => {
                eprintln!("Error: No input file specified in server mode.");
                std::process::exit(1);
            }
        };
        let file_name = file_path.file_name().unwrap().to_string_lossy().to_string();
        let mut file = fs::File::open(&file_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        (file_path, file_name, content)
    };

    let html_output = render_markdown_to_html(&markdown_input);
    let full_html = crate::utils::build_full_html(&_file_name, &html_output);

    let route = warp::path::end().map(move || warp::reply::html(full_html.clone()));
    let addr: SocketAddr = format!("{}:{}", args.host, args.port).parse().unwrap();
    info!("Server running at http://{}", addr);
    open_in_browser(format!("http://{}:{}", args.host, args.port).as_str());
    warp::serve(route).run(addr).await;
    Ok(())
}
