# ByBrowser 🌐🛡️

> High-performance, isolated DPI-bypassing browser launcher for Linux. Built entirely in Rust.

[![Language](https://img.shields.io/badge/language-Rust-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-lightgrey.svg?style=flat-square)](#)
[![Binary Size](https://img.shields.io/badge/binary--size-1.2_MB-blueviolet.svg?style=flat-square)](#)

**ByBrowser** is a dedicated, zero-configuration utility designed exclusively for launching browser sessions through low-level Out-Of-Band (OOB) TCP desync DPI circumvention. It bypasses institutional and ISP middlebox filters (such as Fortinet FortiGate, enterprise UTMs, and college firewalls) to restore instant access to blocked resources (`nyaa.si`, `fitgirl-repacks.site`, `1337x.to`, `rutracker.org`, etc.) with zero setup.

---

## ✨ Features

- ⚡ **Zero Setup & Exclusive Focus:** No subcommands, no service management menus, no extraneous features. Runs directly: `bybrowser [URL...]`.
- 🛡️ **Embedded DPI Bypass Engine:** Automatically boots a lightweight, non-blocking asynchronous SOCKS5 proxy on `127.0.0.1:1080` (or reuses an existing instance).
- 🦊 **Isolated Firefox / Librewolf Profiles:** Generates an isolated browser profile in `~/.config/bybrowser/firefox_profile` with remote DNS resolution (`network.proxy.socks_remote_dns`), HTTPS-only mode, and optimized connection pooling without polluting your daily browser cache.
- 🚀 **Chromium / Chrome / Brave Support:** Automatically detects installed Chromium-based browsers and passes `--proxy-server="socks5://127.0.0.1:1080"` out-of-the-box.
- 📦 **Single Standalone Native Binary:** 100% Rust with no Python or shell script dependencies.

---

## 🎮 Usage

```bash
# Launch browser directly to default homepage (https://nyaa.si)
bybrowser

# Open specific blocked websites
bybrowser https://nyaa.si https://fitgirl-repacks.site

# View command help
bybrowser --help
```

---

## 🛠️ Installation & Building

```bash
# Clone and build release binary
git clone https://github.com/Aditya-233/ByBrowser.git
cd ByBrowser
cargo build --release

# Install to user PATH
cp target/release/bybrowser ~/.local/bin/
```
