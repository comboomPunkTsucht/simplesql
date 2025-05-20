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
use ratatui::widgets::Widget;
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

    let mut tui_tab = shared::Tab::SqlEditor;
    let backend = ratatui::backend::CrosstermBackend::new(stdout);
    let mut terminal = ratatui::Terminal::new(backend)?;

    let result = (|| {
        loop {
            terminal
                .draw(|f| draw_tui(f, &mut tui_tab))
                .unwrap();
            if let Ok(event) = event::read() {

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
fn draw_tui(frame: &mut Frame, tui_tab: &mut shared::Tab) {
    use Constraint::{Fill, Length, Min};
    use ratatui::prelude::Stylize;

    // Create main layout with tab bar, content area, and help bar
    let main_chunks = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([Length(3), Fill(1), Length(3)].as_ref())
        .split(frame.area());

    // Create tab bar
    let tabs = ratatui::widgets::Tabs::new(vec!["SQL Editor", "Table View", "Credentials", "Connections", "Run Log"])
        .select(match tui_tab {
            shared::Tab::SqlEditor => 0,
            shared::Tab::TableView => 1,
            shared::Tab::CredentialsEditor => 2,
            shared::Tab::ConnectionsEditor => 3,
            shared::Tab::RunLog => 4,
        })
        .style(ratatui::style::Style::default())
        .highlight_style(ratatui::style::Style::default().bold())
        .divider("|")
        .block(Block::default().title("Tabs").borders(ratatui::widgets::Borders::ALL));
    frame.render_widget(tabs, main_chunks[0]);

    // Render main content area based on selected tab
    match tui_tab {
        shared::Tab::SqlEditor => {
            let block = Block::default().title("SQL Editor").borders(ratatui::widgets::Borders::ALL);
            frame.render_widget(block, main_chunks[1]);
        }
        shared::Tab::TableView => {
            let block = Block::default().title("Table View").borders(ratatui::widgets::Borders::ALL);
            frame.render_widget(block, main_chunks[1]);
        }
        shared::Tab::CredentialsEditor => {
            let block = Block::default().title("Credentials Editor").borders(ratatui::widgets::Borders::ALL);
            frame.render_widget(block, main_chunks[1]);
        }
        shared::Tab::ConnectionsEditor => {
            let block = Block::default().title("Connections Editor").borders(ratatui::widgets::Borders::ALL);
            frame.render_widget(block, main_chunks[1]);
        }
        shared::Tab::RunLog => {
            let block = Block::default().title("Run Log").borders(ratatui::widgets::Borders::ALL);
            frame.render_widget(block, main_chunks[1]);
        }
    }

    // Create help bar at bottom
    let help_text = ratatui::widgets::Paragraph::new("F1: SQL Editor | F2: Table View | F3: Credentials | F4: Connections | F5: Run Log | F12: Quit")
        .style(ratatui::style::Style::default())
        .block(Block::default().title("Help").borders(ratatui::widgets::Borders::ALL));
    frame.render_widget(help_text, main_chunks[2]);
}
