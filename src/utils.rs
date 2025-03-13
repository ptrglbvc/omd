use base64::{engine::general_purpose, Engine as _};
use std::fs;
use std::process::Command;

/// Reads the contents of the CSS file from assets/style.css.
pub fn read_style_css() -> String {
    fs::read_to_string("assets/style.css").unwrap_or_else(|_| String::from(""))
}

/// Reads asset file at the given path and returns a base64 encoded string.
pub fn read_asset_base64(path: &str) -> String {
    match fs::read(path) {
        Ok(bytes) => general_purpose::STANDARD.encode(bytes),
        Err(_) => String::from(""),
    }
}

/// Reads font files and favicon from assets and returns (font_regular, font_medium, font_light, favicon).
pub fn read_fonts() -> (String, String, String, String) {
    let font_regular = read_asset_base64("assets/fonts/Oswald-Regular.ttf");
    let font_medium = font_regular.clone();
    let font_light = read_asset_base64("assets/fonts/Oswald-Light.ttf");
    let favicon = read_asset_base64("assets/favicon.ico");
    (font_regular, font_medium, font_light, favicon)
}

/// Builds the full HTML document by embedding CSS, fonts, and favicon.
pub fn build_full_html(title: &str, body: &str) -> String {
    let style = read_style_css();
    let (font_regular, font_medium, font_light, favicon) = read_fonts();

    let css = format!(
        r#"
<link rel="icon" href="data:image/x-icon;base64,{}">
<style>
    @font-face {{
        font-family: 'Oswald';
        src: url(data:font/truetype;charset=utf-8;base64,{}) format('truetype');
        font-weight: 400;
        font-style: normal;
    }}
    @font-face {{
        font-family: 'Oswald';
        src: url(data:font/truetype;charset=utf-8;base64,{}) format('truetype');
        font-weight: 700;
        font-style: normal;
    }}
    @font-face {{
        font-family: 'Oswald';
        src: url(data:font/truetype;charset=utf-8;base64,{}) format('truetype');
        font-weight: 300;
        font-style: normal;
    }}
    {}
</style>
"#,
        favicon, font_regular, font_medium, font_light, style
    );

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    {}
</head>
<body>
    {}
</body>
</html>"#,
        title, css, body
    )
}

/// Opens the given URL or HTML content in the default web browser.
/// If the input starts with "http://" or "https://", it is opened directly;
/// otherwise, the content is written to a temporary HTML file which is then opened.
pub fn open_in_browser(content: &str) {
    if content.starts_with("http://") || content.starts_with("https://") {
        #[cfg(target_os = "macos")]
        {
            Command::new("open")
                .arg(content)
                .spawn()
                .expect("Failed to open browser on macOS");
        }
        #[cfg(target_os = "windows")]
        {
            Command::new("cmd")
                .args(&["/C", "start", content])
                .spawn()
                .expect("Failed to open browser on Windows");
        }
        #[cfg(target_os = "linux")]
        {
            Command::new("xdg-open")
                .arg(content)
                .spawn()
                .expect("Failed to open browser on Linux");
        }
    } else {
        use std::fs::File;
        use std::io::Write;
        use tempfile::Builder;

        let temp_file = Builder::new()
            .prefix("markdown_preview_")
            .suffix(".html")
            .tempfile()
            .expect("Failed to create temporary file");
        let temp_path = temp_file.path().to_string_lossy().to_string();
        let mut file = File::create(&temp_path).expect("Failed to open temporary file for writing");
        file.write_all(content.as_bytes())
            .expect("Failed to write HTML content to file");

        #[cfg(target_os = "macos")]
        {
            Command::new("open")
                .arg(&temp_path)
                .spawn()
                .expect("Failed to open browser on macOS");
        }
        #[cfg(target_os = "windows")]
        {
            Command::new("cmd")
                .args(&["/C", "start", &temp_path])
                .spawn()
                .expect("Failed to open browser on Windows");
        }
        #[cfg(target_os = "linux")]
        {
            Command::new("xdg-open")
                .arg(&temp_path)
                .spawn()
                .expect("Failed to open browser on Linux");
        }
    }
}
