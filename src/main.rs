use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{ExecutableCommand, terminal};
use edtui::{EditorEventHandler, EditorState, EditorView, SyntaxHighlighter};
use ratatui::{
    Frame,
    crossterm::event,
    layout::{Constraint, Layout},
    widgets::Block,
};
use std::io::{Write, stdout};

enum TuiTab {
    SqlEditor,
    TableView,
    CredentialsEditor,
    ConnectionsEditor,
    RunLog,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
                .draw(|f| draw(f, &mut state, &mut tui_tab))
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
#[!warn(unused_variables)]
fn draw(frame: &mut Frame, state: &mut EditorState, tui_tab: &mut TuiTab) {
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
