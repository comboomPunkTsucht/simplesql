#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

//! # simplesql
//! **simplesql** is a modern, lightweight SQL client that runs in either Terminal (TUI) or optional Graphical (GUI) mode. Built for developers, DBAs, and power users who need fast, intuitive access to their databases.
//!
//! ## 🔧 Features
//!
//! - ⚡ Fast and responsive Terminal User Interface (TUI)
//! - 🖼️ Optional Graphical User Interface (GUI) for a more visual experience
//! - 🛠️ Simple command-line controls
//! - 🔁 Cross-platform support: Linux, macOS, Windows, FreeBSD
//!
//! ## 🚀 Installation
//!
//! Clone the repository and build it using Cargo:
//!
//! ```bash
//! git clone https://github.com/comboomPunkTsucht/simplesql.git
//! cd simplesql
//! rustup target add aarch64-apple-darwin aarch64-unknown-linux-gnu aarch64-unknown-linux-musl aarch64-pc-windows-msvc x86_64-apple-darwin x86_64-pc-windows-msvc x86_64-unknown-freebsd x86_64-unknown-linux-gnu x86_64-unknown-linux-musl
//! cargo build --release
//! ```
//!
//! ## ▶️ Usage
//!
//! ```bash
//! ./simplesql [OPTIONS]
//! ```
//!
//! ### Options
//!
//! | Short | Long        | Description                                           |
//! |-------|-------------|-------------------------------------------------------|
//! | `-g`  | `--gui`     | Launch **simplesql** in graphical mode                |
//! | `-t`, `-c` | `--tui`, `--cli`| Launch in terminal mode (default)                     |
//! | `-h`  | `--help`    | Show help message                                     |
//! | `-V`  | `--version` | Show version info                                     |
//!
//! ## 🧪 Example
//!
//! ```bash
//! ./simplesql --tui
//! ```
//!
//! ## 📄 Changelog
//!
//! The `Changelog.md` file is generated during the build process and included with each release.
//!
//! ## 📝 License
//!
//! Licensed under the [MIT License](LICENSE).
//!
//! ---
//!
//! Made with ❤️ in Rust – because SQL should be simple.

#[allow(unused_imports)]
use clap::{Arg, Command};
#[allow(unused_imports)]
use std::io::Write;

#[allow(unused_imports)]
mod gui;
#[allow(unused_imports)]
mod shared;
#[allow(unused_imports)]
mod tui;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
fn get_git_hash() -> String {
    use std::process::Command;

    let output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .unwrap();
    let git_hash = String::from_utf8(output.stdout).unwrap();
    info!("found GIT HASH: {git_hash}");
    git_hash
}

fn main() {
    shared::setup_logger().unwrap();
    // Set up the CLI application using Clap and congigs
    let version_string = format!(
        "v{}, Git-HEAD: {}",
        env!("CARGO_PKG_VERSION"),
        get_git_hash()
    );
    let version: &'static str = Box::leak(version_string.into_boxed_str());
    let name = env!("CARGO_PKG_NAME");
    let description = env!("CARGO_PKG_DESCRIPTION");
    let authors = env!("CARGO_PKG_AUTHORS");

    let matches = Command::new(name)
    .version(version)
    .author(authors)
    .about(description)
    .arg(
      Arg::new("gui")
        .long("gui")
        .short('g')
        .global(true)
        .default_value("false")
        .conflicts_with("tui")
        .action(clap::ArgAction::SetTrue)
        .long_help("When Flag is set the programm runs in the non default Graphical User Interface Mode")
        .help("If set the program runs in gui mode"),
    )
    .arg(
      Arg::new("tui")
        .long("tui")
        .short('t')
        .alias("cli")
        .short_alias('c')
        .visible_alias("cli")
        .visible_short_alias('c')
        .global(true)
        .default_value("true")
        .conflicts_with("gui")
        .action(clap::ArgAction::SetTrue)
        .long_help("When Flag is set the programm runs in the default Terminal User Interface Mode")
        .help("If set the programm runs in tui mode [default]"),
    )
    .get_matches();

    if let Err(e) = shared::check_and_gen_config() {
        eprintln!("Error generating config: {}", e);
        std::process::exit(1);
    }
    if matches.get_flag("gui") {
        // GUI mode
        unsafe {
            std::env::set_var("SLINT_BACKEND", "winit-skia");
        }
        info!("GUI Mode activated");
        if let Err(e) = gui::main_gui() {
            error!("{e}");
            std::process::exit(1);
        }
    } else if matches.get_flag("tui") {
        // CLI mode
        info!("TUI Mode activated");
        if let Err(e) = tui::main_tui() {
            error!("{e}");
            std::process::exit(1);
        }
    } else {
        error!("try --help for more information");
    }
}
