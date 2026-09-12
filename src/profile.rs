use anyhow::{Context, Result};
use std::process::Stdio;
use tempfile::TempDir;
use tokio::process::Command;

pub enum BrowserKind {
    FirefoxLike(String),
    ChromiumLike(String),
}

pub fn detect_browser() -> Option<BrowserKind> {
    for binary in ["firefox", "librewolf"] {
        if which_binary(binary) {
            return Some(BrowserKind::FirefoxLike(binary.to_string()));
        }
    }
    for binary in [
        "chromium",
        "google-chrome-stable",
        "google-chrome",
        "brave",
        "brave-browser",
        "microsoft-edge",
    ] {
        if which_binary(binary) {
            return Some(BrowserKind::ChromiumLike(binary.to_string()));
        }
    }
    None
}

fn which_binary(name: &str) -> bool {
    if let Ok(path_var) = std::env::var("PATH") {
        for dir in std::env::split_paths(&path_var) {
            let candidate = dir.join(name);
            if candidate.is_file() {
                return true;
            }
        }
    }
    false
}

fn cleanup_legacy_profile() {
    if let Ok(home) = std::env::var("HOME") {
        let legacy = std::path::PathBuf::from(home).join(".config/bybrowser");
        if legacy.exists() {
            let _ = std::fs::remove_dir_all(legacy);
        }
    }
}

pub fn create_ephemeral_firefox_profile() -> Result<TempDir> {
    let temp_dir = tempfile::Builder::new()
        .prefix("bybrowser_firefox_")
        .tempdir()
        .context("failed to create temporary profile directory")?;

    let user_js = temp_dir.path().join("user.js");
    let config_content = r#"
user_pref("network.proxy.type", 1);
user_pref("network.proxy.socks", "127.0.0.1");
user_pref("network.proxy.socks_port", 1080);
user_pref("network.proxy.socks_version", 5);
user_pref("network.proxy.socks_remote_dns", true);
user_pref("network.proxy.share_proxy_settings", false);
user_pref("network.proxy.http", "");
user_pref("network.proxy.http_port", 0);
user_pref("network.proxy.ssl", "");
user_pref("network.proxy.ssl_port", 0);
user_pref("network.proxy.no_proxies_on", "localhost, 127.0.0.1, fast.com, .fast.com, nflxvideo.net, .nflxvideo.net, netflix.com, .netflix.com, speedtest.net, .speedtest.net, ooklaserver.net");
user_pref("dom.security.https_only_mode", true);
user_pref("network.http.max-persistent-connections-per-server", 16);
user_pref("network.http.max-persistent-connections-per-proxy", 64);
user_pref("network.ssl_tokens_cache_enabled", true);
user_pref("network.dnsCacheExpiration", 3600);
user_pref("network.dnsCacheEntries", 1000);

// Ephemeral zero-trace settings
user_pref("browser.privatebrowsing.autostart", true);
user_pref("places.history.enabled", false);
user_pref("privacy.history.custom", true);
user_pref("privacy.sanitize.sanitizeOnShutdown", true);
user_pref("privacy.clearOnShutdown.cache", true);
user_pref("privacy.clearOnShutdown.cookies", true);
user_pref("privacy.clearOnShutdown.history", true);
user_pref("privacy.clearOnShutdown.formdata", true);
user_pref("privacy.clearOnShutdown.downloads", true);
user_pref("privacy.clearOnShutdown.sessions", true);
user_pref("browser.formfill.enable", false);
user_pref("signon.rememberSignons", false);
user_pref("browser.cache.disk.enable", false);
user_pref("browser.cache.memory.enable", true);
user_pref("browser.cache.memory.capacity", 524288);
user_pref("browser.shell.checkDefaultBrowser", false);
user_pref("browser.startup.homepage", "https://nyaa.si");
"#;

    std::fs::write(&user_js, config_content)
        .with_context(|| format!("failed to write {:?}", user_js))?;

    Ok(temp_dir)
}

pub async fn launch_browser(kind: BrowserKind, urls: &[String]) -> Result<()> {
    cleanup_legacy_profile();

    let default_url = "https://nyaa.si".to_string();
    let target_urls = if urls.is_empty() {
        vec![default_url]
    } else {
        urls.to_vec()
    };

    match kind {
        BrowserKind::FirefoxLike(bin) => {
            let temp_profile = create_ephemeral_firefox_profile()?;
            let profile_path = temp_profile.path().to_path_buf();

            println!(
                "\x1b[1;92m[✓] Launching ephemeral {bin} session through DPI bypass proxy...\x1b[0m"
            );
            println!("\x1b[96m    Profile: {} (use-and-throw)\x1b[0m", profile_path.display());
            println!("\x1b[96m    Target:  {}\x1b[0m\n", target_urls.join(" "));

            let mut child = Command::new(&bin)
                .arg("--profile")
                .arg(&profile_path)
                .arg("--no-remote")
                .args(&target_urls)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .with_context(|| format!("failed to launch {bin}"))?;

            let _ = child.wait().await;

            drop(temp_profile);
            if profile_path.exists() {
                let _ = std::fs::remove_dir_all(&profile_path);
            }
            println!("\x1b[1;92m[✓] Browser session closed. All temporary data wiped completely.\x1b[0m");
        }
        BrowserKind::ChromiumLike(bin) => {
            let temp_profile = tempfile::Builder::new()
                .prefix("bybrowser_chromium_")
                .tempdir()
                .context("failed to create temporary profile directory")?;
            let profile_path = temp_profile.path().to_path_buf();

            let proxy_flag = "--proxy-server=socks5://127.0.0.1:1080";
            let user_data_flag = format!("--user-data-dir={}", profile_path.display());

            println!(
                "\x1b[1;92m[✓] Launching ephemeral {bin} session with proxy {proxy_flag}...\x1b[0m"
            );
            println!("\x1b[96m    Profile: {} (use-and-throw incognito)\x1b[0m", profile_path.display());
            println!("\x1b[96m    Target:  {}\x1b[0m\n", target_urls.join(" "));

            let mut child = Command::new(&bin)
                .arg(proxy_flag)
                .arg(&user_data_flag)
                .arg("--incognito")
                .arg("--no-first-run")
                .arg("--no-default-browser-check")
                .args(&target_urls)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .with_context(|| format!("failed to launch {bin}"))?;

            let _ = child.wait().await;

            drop(temp_profile);
            if profile_path.exists() {
                let _ = std::fs::remove_dir_all(&profile_path);
            }
            println!("\x1b[1;92m[✓] Browser session closed. All temporary data wiped completely.\x1b[0m");
        }
    }

    Ok(())
}
