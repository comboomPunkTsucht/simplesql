// Copyright (c) 2025 mcpeaps_HD
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

#[allow(unused_imports)]
use crate::shared;
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
use std::io::stdout;

pub fn main_tui() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = stdout();
    enable_raw_mode()?;
    stdout.execute(terminal::EnterAlternateScreen)?;
    stdout.execute(crossterm::cursor::Hide)?;

    let mut event_handler = EditorEventHandler::default();
    let mut state = EditorState::default();
    let mut tui_tab = shared::Tab::SqlEditor;
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
                            tui_tab = shared::Tab::SqlEditor;
                        }
                        event::KeyCode::F(2) => {
                            tui_tab = shared::Tab::TableView;
                        }
                        event::KeyCode::F(3) => {
                            tui_tab = shared::Tab::CredentialsEditor;
                        }
                        event::KeyCode::F(4) => {
                            tui_tab = shared::Tab::ConnectionsEditor;
                        }
                        event::KeyCode::F(5) => {
                            tui_tab = shared::Tab::RunLog;
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
fn draw_tui(frame: &mut Frame, state: &mut EditorState, tui_tab: &mut shared::Tab) {
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
