use anyhow::{Context, Result};
use std::path::PathBuf;
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

pub fn ensure_firefox_profile() -> Result<PathBuf> {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());
    let profile_dir = PathBuf::from(home).join(".config/bybrowser/firefox_profile");
    std::fs::create_dir_all(&profile_dir)
        .with_context(|| format!("failed to create profile dir {:?}", profile_dir))?;

    let user_js = profile_dir.join("user.js");
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
user_pref("browser.cache.memory.enable", true);
user_pref("browser.cache.memory.capacity", 524288);
user_pref("browser.shell.checkDefaultBrowser", false);
user_pref("browser.startup.homepage", "https://nyaa.si");
"#;

    std::fs::write(&user_js, config_content)
        .with_context(|| format!("failed to write {:?}", user_js))?;

    Ok(profile_dir)
}

pub async fn launch_browser(kind: BrowserKind, urls: &[String]) -> Result<()> {
    let default_url = "https://nyaa.si".to_string();
    let target_urls = if urls.is_empty() {
        vec![default_url]
    } else {
        urls.to_vec()
    };

    match kind {
        BrowserKind::FirefoxLike(bin) => {
            let profile = ensure_firefox_profile()?;
            println!(
                "\x1b[1;92m[✓] Launching isolated {bin} session through DPI bypass proxy...\x1b[0m"
            );
            println!("\x1b[96m    Profile: {}\x1b[0m", profile.display());
            println!("\x1b[96m    Target:  {}\x1b[0m\n", target_urls.join(" "));

            let mut child = Command::new(&bin)
                .arg("--profile")
                .arg(&profile)
                .arg("--no-remote")
                .args(&target_urls)
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .spawn()
                .with_context(|| format!("failed to launch {bin}"))?;

            let _ = child.wait().await;
        }
        BrowserKind::ChromiumLike(bin) => {
            let proxy_flag = "--proxy-server=socks5://127.0.0.1:1080";
            println!(
                "\x1b[1;92m[✓] Launching isolated {bin} session with proxy {proxy_flag}...\x1b[0m"
            );
            println!("\x1b[96m    Target:  {}\x1b[0m\n", target_urls.join(" "));

            let mut child = Command::new(&bin)
                .arg(proxy_flag)
                .args(&target_urls)
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .spawn()
                .with_context(|| format!("failed to launch {bin}"))?;

            let _ = child.wait().await;
        }
    }

    Ok(())
}
