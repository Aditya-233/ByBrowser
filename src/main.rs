mod engine;
mod profile;

use anyhow::{Context, Result};
use std::env;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn print_help() {
    println!(
        "\x1b[1;95mByBrowser\x1b[0m v{VERSION} - High-Performance DPI-Bypassing Browser Launcher"
    );
    println!("Launches isolated browser sessions through local SOCKS5 DPI circumvention.\n");
    println!("\x1b[1mUSAGE:\x1b[0m");
    println!("    bybrowser [OPTIONS] [URLS...]\n");
    println!("\x1b[1mARGUMENTS:\x1b[0m");
    println!("    <URLS>...    URLs to open (default: https://nyaa.si)\n");
    println!("\x1b[1mOPTIONS:\x1b[0m");
    println!("    -h, --help       Print help information");
    println!("    -v, --version    Print version information\n");
    println!("\x1b[1mEXAMPLES:\x1b[0m");
    println!("    bybrowser");
    println!("    bybrowser https://nyaa.si https://fitgirl-repacks.site");
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut urls = Vec::new();
    for arg in args {
        match arg.as_str() {
            "-h" | "--help" => {
                print_help();
                return Ok(());
            }
            "-v" | "--version" => {
                println!("bybrowser v{VERSION}");
                return Ok(());
            }
            url => {
                urls.push(url.to_string());
            }
        }
    }

    // 1. Ensure DPI Bypass Engine is active
    engine::ensure_engine_running()
        .await
        .context("failed to start or connect to DPI bypass engine")?;

    // 2. Detect installed browser
    let browser = profile::detect_browser().context(
        "no supported browser found (install firefox, chromium, google-chrome, or brave)",
    )?;

    // 3. Launch isolated browser session
    profile::launch_browser(browser, &urls).await?;

    Ok(())
}
