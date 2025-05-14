#[allow(unused_imports)]
use clap::{Arg, Command};
#[allow(unused_imports)]
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
#[allow(unused_imports)]
use crossterm::{ExecutableCommand, terminal};
#[allow(unused_imports)]
use edtui::{EditorEventHandler, EditorState, EditorView, SyntaxHighlighter};
#[allow(unused_imports)]
use ratatui::{
    Frame,
    crossterm::event,
    layout::{Constraint, Layout},
    widgets::Block,
};
#[allow(unused_imports)]
use std::io::{Write, stdout};

enum TuiTab {
    SqlEditor,
    TableView,
    CredentialsEditor,
    ConnectionsEditor,
    RunLog,
}
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
    // Retrieve environment variables set by the build script
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

    // Set up the CLI application using Clap
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
        main_gui();
    } else if matches.get_flag("tui") {
        // CLI mode
        if let Err(e) = main_tui() {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    } else {
        println!("try --help for more information");
    }
}

fn main_gui() {
    // GUI mode
    panic!("GUI mode is not implemented yet.");
}

fn main_tui() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = stdout();
    enable_raw_mode()?;
    stdout.execute(terminal::EnterAlternateScreen)?;
    stdout.execute(crossterm::cursor::Hide)?;

    let mut event_handler = EditorEventHandler::default();
    let mut state = EditorState::default();
    let mut tui_tab = TuiTab::SqlEditor;
    let backend = ratatui::backend::CrosstermBackend::new(stdout);
    let mut terminal = ratatui::Terminal::new(backend)?;

    let result = (|| {
        loop {
            terminal
                .draw(|f| draw_tui(f, &mut state, &mut tui_tab))
                .unwrap();

            if let Ok(event) = event::read() {
                event_handler.on_event(event.clone(), &mut state);

                if let event::Event::Key(key_event) = event {
                    match key_event.code {
                        event::KeyCode::F(12) => break,
                        event::KeyCode::F(1) => {
                            tui_tab = TuiTab::SqlEditor;
                        }
                        event::KeyCode::F(2) => {
                            tui_tab = TuiTab::TableView;
                        }
                        event::KeyCode::F(3) => {
                            tui_tab = TuiTab::CredentialsEditor;
                        }
                        event::KeyCode::F(4) => {
                            tui_tab = TuiTab::ConnectionsEditor;
                        }
                        event::KeyCode::F(5) => {
                            tui_tab = TuiTab::RunLog;
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    })();

    disable_raw_mode()?;
    terminal
        .backend_mut()
        .execute(terminal::LeaveAlternateScreen)?;
    terminal.backend_mut().execute(crossterm::cursor::Show)?;
    terminal.show_cursor()?;

    result
}

#[allow(unused_variables)]
fn draw_tui(frame: &mut Frame, state: &mut EditorState, tui_tab: &mut TuiTab) {
    use Constraint::{Fill, Length, Min};

    let vertical = Layout::vertical([Length(3), Min(0), Length(3)]);
    let [title_area, main_area, status_area] = vertical.areas(frame.area());
    let horizontal = Layout::horizontal([Fill(1); 2]);
    let [left_area, right_area] = horizontal.areas(main_area);

    let theme_name = "nord";
    let extension = "sql";
    let syntax_highlighter = SyntaxHighlighter::new(theme_name, extension);

    frame.render_widget(Block::bordered().title("Title Bar"), title_area);
    frame.render_widget(Block::bordered().title("Status Bar"), status_area);
    frame.render_widget(
        EditorView::new(state).syntax_highlighter(Some(syntax_highlighter)),
        left_area,
    );
    frame.render_widget(Block::bordered().title("Right"), right_area);
}
