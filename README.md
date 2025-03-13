# omd

**omd** is a simple, fast, and lightweight Markdown renderer and previewer written in Rust. It allows you to convert Markdown files to HTML and preview them in your browser, either statically or with live-reload support.

## Features

- **Server Mode**: Run a local server to preview your Markdown files with live-reload functionality as you edit them.
- **Static Mode**: Convert Markdown files to HTML and open them directly in your default browser without running a server.
- **Clipboard Support**: Render Markdown content directly from your clipboard.
- **CommonMark Extensions**: Supports strikethrough, tables, footnotes, task lists, etc.
- **Full LaTeX Support**: Render math equations using `$...$` for inline math and `$$...$$` for block math.
- **Customizable Host and Port**: Specify the host and port for the server to suit your needs.

## Installation

### Prerequisites

- [Rust and Cargo](https://www.rust-lang.org/tools/install) (for building from source)

### Build from Source

1. **Clone the Repository**

   ```bash
   git clone https://github.com/ptrglbvc/omd.git
   cd omd
   ```

2. **Build the Project**

   ```bash
   cargo build --release
   ```

3. **Install**

   Optionally, you can install `omd` to your local Cargo bin directory:

   ```bash
   cargo install --path .
   ```

   This allows you to run `omd` from anywhere on your system.

### Get it from crates.io

   ```bash
   cargo install omd
   ```

   That is it.

## Usage

```
omd [OPTIONS] [FILE]
```

### Options

- `-s`, `--static-mode`: Run in static mode. Converts the Markdown file to HTML and opens it in your default browser without starting a server.
- `-C`, `--clipboard`: Render Markdown content from your clipboard instead of a file. Cannot be used with a file argument.
- `-H`, `--host <HOST>`: Specify the host address for the server (default: `127.0.0.1`). Use `0.0.0.0` to make the server accessible on your local network.
- `-P`, `--port <PORT>`: Specify the port for the server (default: `3030`). Useful if the default port is already in use.
- `-h`, `--help`: Print help information.
- `-V`, `--version`: Print version information.

### Examples

#### Render Clipboard Contents

Render Markdown content from your clipboard:

```bash
omd --clipboard
```

#### Server Mode (Live Preview)

Start a local server to preview your Markdown file with live-reload functionality:

```bash
omd README.md
```

Open [http://localhost:3030](http://localhost:3030) in your browser. Whenever you save changes to `README.md`, the browser will automatically reload to reflect the updates.

If the default port is already in use, specify a different port:

```bash
omd --port 6969 README.md
```

Make the server accessible on your local network:

```bash
omd --host 0.0.0.0 README.md
```

#### Static Mode

Convert a Markdown file to HTML and open it in your browser:

```bash
omd --static-mode README.md
```

```bash
cat README.md | omd --static-mode
```

## How It Works

- **Static Mode**: Renders the Markdown to HTML, writes it to a temporary file, and opens it in your default browser.
- **Server Mode**: Starts a local web server using [Warp](https://github.com/seanmonstar/warp) and watches the Markdown file for changes using [Notify](https://github.com/notify-rs/notify). The browser automatically reloads when changes are detected.
- **Clipboard Support**: Reads Markdown content directly from your clipboard and renders it to HTML.

## Dependencies

- [Pulldown-Cmark](https://github.com/raphlinus/pulldown-cmark) for parsing and rendering Markdown.
- [Warp](https://github.com/seanmonstar/warp) for running the web server in server mode.
- [Notify](https://github.com/notify-rs/notify) for watching file changes.
- [Clipboard](https://github.com/aweinstock314/rust-clipboard) for reading Markdown content from the clipboard.

## License

This project is licensed under the [MIT License](LICENSE).

## Contributing

Contributions are welcome! Please follow these steps:

1. **Fork the repository**.
2. **Create a new branch** for your feature or bugfix.
3. **Commit your changes** with clear messages.
4. **Push to your fork** and submit a **Pull Request**.

Please make sure to update tests as appropriate.

## Contributors

[davehorner](https://github.com/davehorner/) - v0.2.3 - lib interface, tracing, example, removal of clipboard and replacement with arboard, other things.

## Acknowledgments

- Thanks to the Rust community for their amazing crates that make projects like this possible.
- Inspired by the need for a simple Markdown previewer without unnecessary overhead.
- Thank you contributors, in particular [dylanfair](https://www.github.com/dylanfair) for the Linux support and [phaker97](https://github.com/phaker97) for the Latex support.

## Contact

For questions or suggestions, feel free to open an issue or reach out via email at [me@ptrglbvc.dev](mailto:me@ptrglbvc.dev).
