// Copyright (c) 2025 mcpeaps_HD
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
#[allow(unused_imports)]
use crate::shared;
#[allow(unused_imports)]
use edtui::{
    syntect::parsing::{Scope, SyntaxReference}, EditorEventHandler, EditorState, EditorStatusLine, EditorTheme, EditorView,
    Lines,
    SyntaxHighlighter,
};
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use ratatui::layout::Flex;
#[allow(unused_imports)]
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::*,
    style::{Color, Style},
    widgets::*,
};
#[allow(unused_imports)]
use std::error::Error;
use std::fmt::format;
use std::ops::Deref;
use std::time::SystemTime;
use tui_logger::{TuiLoggerLevelOutput, TuiLoggerSmartWidget, TuiLoggerWidget};
use tui_textarea::{CursorMove, TextArea};
#[allow(unused_imports)]
use widgetui::{
    crossterm::event::{
        Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, ModifierKeyCode,
        MouseEvent,
    },
    *,
};

#[allow(dead_code)]
#[derive(Clone, State)]
pub struct ExtendedAppState {
    pub shared: shared::AppState,
    pub editor_state: EditorState,
    pub db_imput: bool,
    pub db_textarea: TextArea<'static>,
}
impl Default for ExtendedAppState {
    fn default() -> Self {
        let shared = shared::AppState::default();
        let lines = vec![shared.db.clone()];
        ExtendedAppState {
            shared,
            editor_state: EditorState::default(),
            db_imput: false,
            db_textarea: TextArea::new(lines),
        }
    }
}

fn inactivate(textarea: &mut TextArea<'_>) {
    textarea.set_cursor_line_style(Style::default());
    textarea.set_cursor_style(Style::default());
    textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .style(Style::default().fg(Color::White))
            .title("DB"),
    );
}

fn activate(textarea: &mut TextArea<'_>) {
    textarea.set_cursor_line_style(Style::default().add_modifier(Modifier::UNDERLINED));
    textarea.set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
    textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .style(Style::default())
            .title("DB"),
    );
}

fn widget(
    mut frame: ResMut<WidgetFrame>,
    mut events: ResMut<Events>,
    mut state: ResMut<ExtendedAppState>,
) -> WidgetResult {
    //helplinetext
    let helplinetext = "F1: SQL Editor | F2: Table View | F3: Imput DB | F4: Select User | F5: Run | F8: Export SQL | F10: Logs | F12: Quit";
    let min_width: u16 = helplinetext.chars().count() as u16 + 2; // minimum width for the terminal based on help text length
    let min_height: u16 = 10; // Minimum height for the terminal

    // Prüfe die aktuelle Terminal-Größe
    let terminal_size = frame.size();

    // Wenn das Terminal zu klein ist, zeige eine Fehlermeldung
    if terminal_size.width < min_width || terminal_size.height < min_height {
        let error_msg = format!(
            "Terminal too small!\nAt least {}x{} required\nCurrent: {}x{}",
            min_width, min_height, terminal_size.width, terminal_size.height
        );
        let error_title = "Terminal Size Error";

        frame.render_widget(
            Paragraph::new(error_msg)
                .style(Style::default().fg(Color::Red))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(error_title)
                        .border_style(Style::default().fg(Color::Red))
                        .border_type(BorderType::Rounded)
                        .title_alignment(Alignment::Center),
                ),
            terminal_size,
        );

        match events.event.clone() {
            Some(event) => match event {
                Event::Key(key_event) => match key_event.modifiers {
                    KeyModifiers::CONTROL => match key_event.code {
                        KeyCode::Char('c') => {
                            events.register_exit();
                        }
                        KeyCode::Char('d') => {
                            events.register_exit();
                        }
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }

        return Ok(());
    }

    // Create main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(3),
        ])
        .split(frame.size());

    let tab_string = vec!["SQL Editor", "Table View", "Log Viewer"];

    // Create and render tabs
    let tabs = Tabs::new(tab_string)
        .select(state.shared.current_tab.to_index())
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().bold().fg(Color::Black).bg(Color::White))
        .divider("|")
        .block(
            Block::default()
                .title("Tabs")
                .borders(Borders::ALL)
                .border_type(BorderType::Thick),
        );

    let h0chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(60),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ])
        .split(chunks[0]);

    match state.shared.current_tab {
        shared::Tab::SqlEditor => {
            state.editor_state.lines = Lines::from(state.shared.sql_query.clone());
        }
        _ => {}
    }
    // Create SQL SyntaxHighlighter
    let sql_syntax_highlighter: SyntaxHighlighter = SyntaxHighlighter::new("nord", "sql");
    frame.render_widget(tabs.clone(), h0chunks[0]);
    frame.render_widget(
        Paragraph::new(state.shared.user.name.clone())
            .style(Style::default().fg(Color::White))
            .block(
                Block::default()
                    .title("Selected User")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Thick),
            ),
        h0chunks[2],
    );
    if state.db_imput {
        activate(&mut state.db_textarea);
    } else {
        inactivate(&mut state.db_textarea);
    }
    frame.render_widget(&state.db_textarea, h0chunks[1]);
    // Render main content based on selected tab
    match state.shared.current_tab {
        shared::Tab::SqlEditor => frame.render_widget(
            EditorView::new(&mut state.editor_state)
                .wrap(true)
                .theme(Theme::new().editor)
                .syntax_highlighter(Some(sql_syntax_highlighter)),
            chunks[1],
        ),
        shared::Tab::TableView => {
            if state.shared.table.headers.is_empty() && state.shared.table.rows.is_empty() {
                frame.render_widget(
                    Paragraph::new("No data available")
                        .style(Style::default().fg(Color::White))
                        .block(
                            Block::default()
                                .title("Table View")
                                .borders(Borders::ALL)
                                .border_type(BorderType::Thick),
                        ),
                    chunks[1],
                );
            } else {
                // Header-Zeile
                let header = Row::new(state.shared.table.headers.clone());

                // Datenzeilen
                let rows = state
                    .shared
                    .table
                    .rows
                    .iter()
                    .map(|row| Row::new(row.clone()));

                // Spaltenbreiten dynamisch anpassen
                let col_count = state.shared.table.headers.len();
                let col_widths =
                    vec![Constraint::Percentage(100 / col_count.max(1) as u16); col_count];

                frame.render_widget(
                    Table::default()
                        .rows(rows)
                        .header(header)
                        .block(
                            Block::default()
                                .title("Table View")
                                .borders(Borders::ALL)
                                .border_type(BorderType::Thick),
                        )
                        .cell_highlight_style(Style::default().fg(Color::White))
                        .row_highlight_style(Style::default().fg(Color::Black).bg(Color::White))
                        .widths(&col_widths),
                    chunks[1],
                );
            }
        }

        shared::Tab::LogViewer => frame.render_widget(
            TuiLoggerWidget::default()
                .output_separator('-')
                .output_timestamp(Some("[%Y-%m-%d %H:%M:%S]".to_string()))
                .output_level(Some(TuiLoggerLevelOutput::Long))
                .output_line(true)
                .output_target(true)
                .block(
                    Block::default()
                        .border_type(BorderType::Thick)
                        .title("Log Viewer")
                        .borders(Borders::ALL),
                )
                .style(Style::default().fg(Color::Gray)),
            chunks[1],
        ),
    }

    // Render help bar
    let help_text = Paragraph::new(helplinetext)
        .style(Style::default().fg(Color::Gray))
        .block(
            Block::default()
                .title("Help")
                .borders(Borders::ALL)
                .border_type(BorderType::Thick),
        );
    frame.render_widget(help_text, chunks[2]);

    if state.db_imput {
        match events.event.clone() {
            Some(event) => {
                match event {
                    Event::Key(key_event) => {
                        // Handle modifier keys
                        match key_event.modifiers {
                            KeyModifiers::CONTROL => match key_event.code {
                                KeyCode::Char('c') => {
                                    events.register_exit();
                                }
                                KeyCode::Char('d') => {
                                    events.register_exit();
                                }
                                _ => {}
                            },
                            _ => {}
                        }
                        match key_event.code {
                            KeyCode::F(12) => {
                                events.register_exit();
                            }
                            KeyCode::Esc | KeyCode::Enter => {
                                // Exit DB input mode
                                state.db_imput = false;
                                debug!("Exiting DB input mode");
                            }
                            _ => {}
                        }
                        state
                            .db_textarea
                            .input(tui_textarea::Input::from(key_event));
                    }
                    Event::Mouse(mouse_event) => {
                        // Handle mouse events if needed
                        debug!("Mouse event: {:?}", mouse_event);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        state.shared.db = state.db_textarea.lines().join("\n");
    } else {
        // Handle editor events
        if let Some(event) = events.event.clone() {
            EditorEventHandler::default().on_event(event, &mut state.editor_state);
        }
        match state.shared.current_tab {
            shared::Tab::SqlEditor => {
                state.shared.sql_query = get_editor_lines_as_string(&state);
            }
            _ => {}
        }
        // new code for handling key events
        match events.event.clone() {
            Some(event) => {
                match event {
                    // Handle key events
                    Event::Key(key_event) => {
                        match key_event.code {
                            KeyCode::F(12) => {
                                events.register_exit();
                            }
                            KeyCode::F(1) => {
                                state.shared.current_tab = shared::Tab::SqlEditor;
                                info!("Switched to SQL Editor tab");
                            }
                            KeyCode::F(2) => {
                                state.shared.current_tab = shared::Tab::TableView;
                                info!("Switched to Table View tab");
                            }
                            KeyCode::F(3) => {
                                state.db_imput = !state.db_imput;
                            }
                            KeyCode::F(4) => {
                                state.shared.set_next_user();
                            }
                            KeyCode::F(10) => {
                                state.shared.current_tab = shared::Tab::LogViewer;
                            }
                            KeyCode::F(5) => {
                                if let Err(e) = shared::run_query(&mut state.shared) {
                                    error!("Error running query: {}", e);
                                }
                            }
                            KeyCode::F(8) => {
                                // save the current query to a file
                                let filename = format!(
                                    "query_{}.sql",
                                    humantime::format_rfc3339_seconds(SystemTime::now())
                                );
                                shared::write_file(
                                    format!("./{}", filename).as_str(),
                                    state.shared.sql_query.as_str(),
                                )
                                .unwrap();
                            }
                            KeyCode::F(9) => {
                                info!("test to load a query from a file");
                            }
                            _ => {}
                        }
                        // Handle modifier keys
                        match key_event.modifiers {
                            KeyModifiers::CONTROL => match key_event.code {
                                KeyCode::Char('c') => {
                                    events.register_exit();
                                }
                                KeyCode::Char('d') => {
                                    events.register_exit();
                                }
                                _ => {}
                            },
                            _ => {}
                        }
                    }

                    Event::Mouse(mouse_event) => {
                        // Handle mouse events if needed
                        debug!("Mouse event: {:?}", mouse_event);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    Ok(())
}

pub fn main_tui(file_content: String) -> Result<(), Box<dyn Error>> {
    // Initialize the application state
    let mut app_state = ExtendedAppState::default();
    if !file_content.is_empty() {
        app_state.shared.sql_query = file_content;
    }
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
                        .border_style(Style::default().fg(Color::White))
                        .border_type(BorderType::Thick),
                )
                .base(Style::default().fg(Color::White))
                .cursor_style(Style::default().bg(Color::White).fg(Color::Black))
                .selection_style(Style::default().bg(Color::Gray).fg(Color::Black))
                .status_line(
                    EditorStatusLine::default()
                        .style_text(Style::default().bg(Color::Black).fg(Color::White))
                        .style_line(Style::default().bg(Color::Black).fg(Color::LightGreen))
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
