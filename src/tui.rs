// Copyright (c) 2025 mcpeaps_HD
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crossterm::event::KeyCode;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::*,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Tabs},
};
use std::error::Error;
use widgetui::*;

use crate::shared;

fn widget(
    mut frame: ResMut<WidgetFrame>,
    mut events: ResMut<Events>,
    mut state: ResMut<shared::AppState>,
) -> WidgetResult {
    // Create main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(3),
        ])
        .split(frame.size());

    // Create and render tabs
    let tabs = Tabs::new(vec![
        "SQL Editor",
        "Table View",
        "Credentials",
        "Connections",
        "Run Log",
    ])
    .select(state.current_tab.to_index())
    .style(Style::default())
    .highlight_style(Style::default().bold().fg(Color::Black).bg(Color::White))
    .divider("|")
    .block(Block::default().title("Tabs").borders(Borders::ALL));
    frame.render_widget(tabs.clone(), chunks[0]);
    // Render main content based on selected tab
    match state.current_tab {
        shared::Tab::SqlEditor => frame.render_widget(
            Block::default().title("SQL Editor").borders(Borders::ALL),
            chunks[1],
        ),
        shared::Tab::TableView => frame.render_widget(
            Block::default().title("Table View").borders(Borders::ALL),
            chunks[1],
        ),
        shared::Tab::CredentialsEditor => frame.render_widget(
            Block::default()
                .title("Credentials Editor")
                .borders(Borders::ALL),
            chunks[1],
        ),
        shared::Tab::ConnectionsEditor => frame.render_widget(
            Block::default()
                .title("Connections Editor")
                .borders(Borders::ALL),
            chunks[1],
        ),
        shared::Tab::RunLog => frame.render_widget(
            Block::default().title("Run Log").borders(Borders::ALL),
            chunks[1],
        ),
    }

    // Render help bar
    let help_text = Paragraph::new(
        "F1: SQL Editor | F2: Table View | F3: Credentials | F4: Connections | F5: Run Log | F12: Quit"
    )
    .block(Block::default().title("Help").borders(Borders::ALL));
    frame.render_widget(help_text, chunks[2]);

    // Handle key events
    if events.key(KeyCode::F(12)) {
        events.register_exit();
    } else if events.key(KeyCode::F(1)) {
        state.current_tab = shared::Tab::SqlEditor;
    } else if events.key(KeyCode::F(2)) {
        state.current_tab = shared::Tab::TableView;
    } else if events.key(KeyCode::F(3)) {
        state.current_tab = shared::Tab::CredentialsEditor;
    } else if events.key(KeyCode::F(4)) {
        state.current_tab = shared::Tab::ConnectionsEditor;
    } else if events.key(KeyCode::F(5)) {
        state.current_tab = shared::Tab::RunLog;
    }

    Ok(())
}

pub fn main_tui() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the application state
    let app_state = shared::AppState::default();
    Ok(App::new(100)?.widgets(widget).states(app_state).run()?)
}
