// Copyright (c) 2025 mcpeaps_HD
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use edtui::{
    EditorEventHandler, EditorState, EditorStatusLine, EditorTheme, EditorView, Lines,
    SyntaxHighlighter,
    syntect::parsing::{Scope, SyntaxReference},
};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::*,
    style::{Color, Style},
    widgets::*,
};
use std::error::Error;
use widgetui::{*,crossterm::event::{KeyCode, KeyEvent, KeyModifiers, ModifierKeyCode}};

use crate::shared;

#[allow(dead_code)]
#[derive(Clone, State)]
pub struct ExtendedAppState {
    pub shared: shared::AppState,
    pub editor_state: EditorState,
}
impl Default for ExtendedAppState {
    fn default() -> Self {
        ExtendedAppState {
            shared: shared::AppState::default(),
            editor_state: EditorState::default(),
        }
    }
}

fn widget(
    mut frame: ResMut<WidgetFrame>,
    mut events: ResMut<Events>,
    mut state: ResMut<ExtendedAppState>,
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

    match state.shared.current_tab {
        shared::Tab::SqlEditor => {
            state.editor_state.lines = Lines::from(state.shared.sql_query.clone());
        }
        shared::Tab::ConfigEditor => {
            state.editor_state.lines =
                Lines::from(shared::get_config_content().unwrap_or_default());
        }
        _ => {}
    }
    // Create SQL SyntaxHighlighter
    let sql_syntax_highlighter: SyntaxHighlighter = SyntaxHighlighter::new("nord", "sql");

    //jsonc SyntaxHighlighter
    let jsonc_syntax_highlighter: SyntaxHighlighter = SyntaxHighlighter::new("nord", "json");

    // Create and render tabs
    let tabs = Tabs::new(vec![
        "SQL Editor",
        "Table View",
        "Config Editor",
        "Run Log",
    ])
    .select(state.shared.current_tab.to_index())
    .style(Style::default().bg(Color::Black).fg(Color::White))
    .highlight_style(Style::default().bold().fg(Color::Black).bg(Color::White))
    .divider("|")
    .block(
        Block::default()
            .title("Tabs")
            .borders(Borders::ALL)
            .border_type(BorderType::Thick),
    );
    frame.render_widget(tabs.clone(), chunks[0]);
    // Render main content based on selected tab
    match state.shared.current_tab {
        shared::Tab::SqlEditor => frame.render_widget(
            EditorView::new(&mut state.editor_state)
                .wrap(true)
                .theme(Theme::new().editor)
                .syntax_highlighter(Some(sql_syntax_highlighter)),
            chunks[1],
        ),
        shared::Tab::TableView => frame.render_widget(
            Block::default().title("Table View").borders(Borders::ALL),
            chunks[1],
        ),
        shared::Tab::ConfigEditor => frame.render_widget(
            EditorView::new(&mut state.editor_state)
                .wrap(true)
                .theme(Theme::new().editor)
                .syntax_highlighter(Some(jsonc_syntax_highlighter)),
            chunks[1],
        ),
        shared::Tab::RunLog => frame.render_widget(
            Block::default().title("Run Log").borders(Borders::ALL),
            chunks[1],
        ),
    }

    // Render help bar
    let help_text = Paragraph::new(
        "F1: SQL Editor | F2: Table View | F3: Config Editor | F4: Run Log | F12: Quit"
    )
        .style(Style::default().fg(Color::Gray).bg(Color::Black))
        .block(Block::default().title("Help").borders(Borders::ALL).border_type(BorderType::Thick));
    frame.render_widget(help_text, chunks[2]);

    // Handle editor events
    if let Some(event) = events.event.clone() {
        EditorEventHandler::default().on_event(event, &mut state.editor_state);
    }
    match state.shared.current_tab {
        shared::Tab::SqlEditor => {
            state.shared.sql_query = get_editor_lines_as_string(&state);
        }
        shared::Tab::ConfigEditor => {
            let _ = shared::set_config_content(get_editor_lines_as_string(&state));
        }
        _ => {}
    }
    // Handle key events
    if (events.key(KeyCode::F(12)))
        || events.key_event(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL))
        || events.key_event(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::CONTROL))
    {
        events.register_exit();
    } else if events.key(KeyCode::F(1)) {
        state.shared.current_tab = shared::Tab::SqlEditor;
    } else if events.key(KeyCode::F(2)) {
        state.shared.current_tab = shared::Tab::TableView;
    } else if events.key(KeyCode::F(3)) {
        state.shared.current_tab = shared::Tab::ConfigEditor;
    }  else if events.key(KeyCode::F(4)) {
        state.shared.current_tab = shared::Tab::RunLog;
    }

    Ok(())
}

pub fn main_tui() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the application state
    let app_state = ExtendedAppState::default();
    Ok(App::new(100)?.widgets(widget).states(app_state).run()?)
}

#[derive(Default)]
pub struct Theme<'a> {
    pub editor: EditorTheme<'a>,
}

impl<'a> Theme<'a> {
    pub fn new() -> Self {
        Self {
            editor: EditorTheme::default()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::White).bg(Color::Black))
                        .border_type(BorderType::Thick),
                )
                .base(Style::default().bg(Color::Black).fg(Color::White))
                .cursor_style(Style::default().bg(Color::White).fg(Color::Black))
                .selection_style(Style::default().bg(Color::Gray).fg(Color::Black))
                .status_line(
                    EditorStatusLine::default()
                        .style_text(Style::default().fg(Color::White).bg(Color::Black))
                        .style_line(Style::default().fg(Color::LightGreen).bg(Color::Black))
                        .align_left(true),
                ),
        }
    }
}

pub fn get_editor_lines_as_string(state: &ExtendedAppState) -> String {
    state
        .editor_state
        .lines
        .flatten(&Some('\n'))
        .into_iter()
        .collect()
}
