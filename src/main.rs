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

fn get_git_hash() -> String {
    use std::process::Command;

    let output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .unwrap();
    let git_hash = String::from_utf8(output.stdout).unwrap();
    git_hash
}

fn main() {
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
    #[allow(unused_variables)]
    let config_path: String = match std::env::consts::OS {
        "linux" | "macos" | "freebsd" => format!("{}/.simplesql", std::env::var("HOME").unwrap()),
        "windows" => format!("{}/.simplesql", std::env::var("APPDATA").unwrap()),
        _ => panic!("Unsupported platform"),
    };

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
                .long_help("When Flag is set the programm runns in the default Terminal User Interface Mode")
                .help("If set the programm runs in tui mode [default]"),
        )
        .get_matches();

    if matches.get_flag("gui") {
        // GUI mode
        gui::main_gui();
    } else if matches.get_flag("tui") {
        // CLI mode
        if let Err(e) = tui::main_tui() {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    } else {
        println!("try --help for more information");
    }
}
