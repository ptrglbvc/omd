#![allow(warnings)]
use std::default;
use std::fs::File;
use std::io::{self, Read, Write};
use std::net::IpAddr;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

use base64::encode;
use clap::Parser;
use futures_util::stream::{Stream, StreamExt};
use local_ip_address::local_ip;
use notify::Watcher;
use pulldown_cmark::{html, CowStr, Event, Options, Parser as MdParser};
use tokio::sync::{broadcast, RwLock};
use warp::{sse, Filter};

/// Renders Markdown input to HTML.

pub fn render_markdown_to_html(markdown_input: &str) -> String {
    let mut options = Options::all();

    let parser = MdParser::new_ext(&markdown_input, options);
    let mut html_output = String::new();
    html::push_html(
        &mut html_output,
        parser.map(|event| match event {
            Event::SoftBreak => Event::Html("<br>".into()),
            Event::InlineMath(s) => {
                let mut str = String::from("<span class=\"math math-inline\">$");
                str.push_str(&s.into_string());
                str.push_str("$</span>");
                Event::Html(CowStr::from(str))
            }
            Event::DisplayMath(s) => {
                let mut str = String::from("<span class=\"math math-display\">$$");
                str.push_str(&s.into_string());
                str.push_str("$$</span>");
                Event::Html(CowStr::from(str))
            }
            _ => event,
        }),
    );
    html_output
}
