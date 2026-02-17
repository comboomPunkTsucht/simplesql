// Copyright (c) 2025 mcpeaps_HD
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
#[allow(unused_imports)]
use crate::shared;
use crate::shared::Tab;
#[allow(unused_imports)]
use edtui::{
    EditorEventHandler, EditorState, EditorStatusLine, EditorTheme, EditorView, Lines,
    SyntaxHighlighter,
    syntect::parsing::{Scope, SyntaxReference},
};
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use ratatui::layout::Flex;
use ratatui::symbols::border;
use std::cmp::PartialEq;
#[allow(unused_imports)]
use std::error::Error;
use std::fmt::format;
use std::ops::Deref;
use std::time::SystemTime;
use tui_logger::{TuiLoggerLevelOutput, TuiLoggerSmartWidget, TuiLoggerWidget};
use tui_popup::Popup;
use tui_textarea::{CursorMove, TextArea};

use crossterm::event::{
    Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, ModifierKeyCode,
    MouseEvent,
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::*,
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{
        Block, BorderType, Borders, Cell, Paragraph, Row, Scrollbar, ScrollbarOrientation,
        ScrollbarState, Table, TableState, Tabs, Wrap,
    },
};

struct AlternativeShortcut {
    key: KeyCode,
    modifiers: Option<KeyModifiers>,
}

struct Shortcut<'a> {
    key: KeyCode,
    modifiers: Option<KeyModifiers>,
    description: &'static str,
    alternative_shortcut: Option<&'a [AlternativeShortcut]>,
}

const SHORTCUTS: &[Shortcut<'_>] = &[
    Shortcut {
        key: KeyCode::F(1),
        modifiers: None,
        description: "Toggle Help Popup",
        alternative_shortcut: None,
    },
    Shortcut {
        key: KeyCode::F(2),
        modifiers: None,
        description: "Select Tab",
        alternative_shortcut: None,
    },
    Shortcut {
        key: KeyCode::F(3),
        modifiers: None,
        description: "Input the Databasename for the Connection to the Database",
        alternative_shortcut: None,
    },
    Shortcut {
        key: KeyCode::F(4),
        modifiers: None,
        description: "Select User for the Connection to the Database",
        alternative_shortcut: None,
    },
    Shortcut {
        key: KeyCode::F(5),
        modifiers: None,
        description: "Query the Database",
        alternative_shortcut: None,
    },
    Shortcut {
        key: KeyCode::F(8),
        modifiers: None,
        description: "Export the SQL Statement to a File",
        alternative_shortcut: None,
    },
    Shortcut {
        key: KeyCode::F(9),
        modifiers: None,
        description: "Import a SQL Statement from a File",
        alternative_shortcut: None,
    },
    Shortcut {
        key: KeyCode::F(12),
        modifiers: None,
        description: "Quit",
        alternative_shortcut: Some(&[
            AlternativeShortcut {
                key: KeyCode::Char('c'),
                modifiers: Some(KeyModifiers::CONTROL),
            },
            AlternativeShortcut {
                key: KeyCode::Char('d'),
                modifiers: Some(KeyModifiers::CONTROL),
            },
        ]),
    },
];

impl AlternativeShortcut {
    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        if let Some(modifiers) = self.modifiers {
            let mut parts = Vec::new();
            if modifiers.contains(KeyModifiers::CONTROL) {
                parts.push("Ctrl".to_string());
            }
            if modifiers.contains(KeyModifiers::ALT) {
                parts.push("Alt".to_string());
            }
            if modifiers.contains(KeyModifiers::SHIFT) {
                parts.push("Shift".to_string());
            }
            parts.push(match self.key {
                KeyCode::F(n) => format!("F{}", n),
                KeyCode::Char(c) => c.to_string(),
                KeyCode::Enter => "Enter".to_string(),
                KeyCode::Esc => "Esc".to_string(),
                KeyCode::Tab => "Tab".to_string(),
                KeyCode::Backspace => "Backspace".to_string(),
                KeyCode::Left => "Left".to_string(),
                KeyCode::Right => "Right".to_string(),
                KeyCode::Up => "Up".to_string(),
                KeyCode::Down => "Down".to_string(),
                _ => "Other".to_string(),
            });
            parts.join("+")
        } else {
            match self.key {
                KeyCode::F(n) => format!("F{}", n),
                KeyCode::Char(c) => c.to_string(),
                KeyCode::Enter => "Enter".to_string(),
                KeyCode::Esc => "Esc".to_string(),
                KeyCode::Tab => "Tab".to_string(),
                KeyCode::Backspace => "Backspace".to_string(),
                KeyCode::Left => "Left".to_string(),
                KeyCode::Right => "Right".to_string(),
                KeyCode::Up => "Up".to_string(),
                KeyCode::Down => "Down".to_string(),
                _ => "Other".to_string(),
            }
        }
    }

    #[allow(dead_code)]
    pub fn as_str(&self) -> &'static str {
        Box::leak(self.to_string().into_boxed_str())
    }
}

impl Shortcut<'_> {
    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        let mut parts = Vec::new();
        if let Some(modifiers) = self.modifiers {
            if modifiers.contains(KeyModifiers::CONTROL) {
                parts.push("Ctrl".to_string());
            }
            if modifiers.contains(KeyModifiers::ALT) {
                parts.push("Alt".to_string());
            }
            if modifiers.contains(KeyModifiers::SHIFT) {
                parts.push("Shift".to_string());
            }
        }
        parts.push(match self.key {
            KeyCode::F(n) => format!("F{}", n),
            KeyCode::Char(c) => c.to_string(),
            KeyCode::Enter => "Enter".to_string(),
            KeyCode::Esc => "Esc".to_string(),
            KeyCode::Tab => "Tab".to_string(),
            KeyCode::Backspace => "Backspace".to_string(),
            KeyCode::Left => "Left".to_string(),
            KeyCode::Right => "Right".to_string(),
            KeyCode::Up => "Up".to_string(),
            KeyCode::Down => "Down".to_string(),
            _ => "Other".to_string(),
        });
        #[allow(unused_assignments)]
        let mut shortcut: String = String::new();
        let main_shortcut = parts.join("+");
        if let Some(alternatives) = self.alternative_shortcut {
            let alt_strings: Vec<String> = alternatives.iter().map(|alt| alt.to_string()).collect();
            shortcut = format!("{} ({})", main_shortcut, alt_strings.join(" / "));
        } else {
            shortcut = main_shortcut;
        }

        format!("{} - {}", shortcut, self.description)
    }

    #[allow(dead_code)]
    pub fn as_str(&self) -> &'static str {
        Box::leak(self.to_string().into_boxed_str())
    }
}

#[derive(Clone)]
pub enum FileAction {
    Save,
    Load,
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct ExtendedAppState {
    pub shared: shared::AppState,
    pub editor_state: EditorState,
    pub db_input: bool,
    pub db_textarea: TextArea<'static>,
    pub file_textarea: TextArea<'static>,
    pub show_help: bool,
    pub show_file_popup: bool,
    pub file_save: Option<FileAction>,
    pub file_popup_is_active: bool,
    pub table_selected: usize,
    pub table_offset: usize,
    pub table_col_offset: usize,
}
impl Default for ExtendedAppState {
    fn default() -> Self {
        let shared = shared::AppState::default();
        let lines = vec![shared.db.clone()];
        ExtendedAppState {
            shared,
            editor_state: EditorState::default(),
            db_input: false,
            db_textarea: TextArea::new(lines),
            file_textarea: TextArea::new(vec![
                std::env::current_dir()
                    .unwrap()
                    .as_os_str()
                    .to_str()
                    .unwrap()
                    .to_string(),
            ]),
            show_help: false,
            show_file_popup: false,
            file_save: None,
            file_popup_is_active: false,
            table_selected: 0,
            table_offset: 0,
            table_col_offset: 0,
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

impl PartialEq for FileAction {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (FileAction::Save, FileAction::Save) => true,
            (FileAction::Load, FileAction::Load) => true,
            _ => false,
        }
    }
}

// ── Rendering ─────────────────────────────────────────────────────────────

fn ui(frame: &mut ratatui::Frame, state: &mut ExtendedAppState) {
    let min_width: u16 = 115;
    let min_height: u16 = SHORTCUTS.len() as u16 + 5;

    let terminal_size = frame.area();
    let popup_size = Rect {
        x: (terminal_size.width * 10u16) / 100u16,
        y: terminal_size.height / 4,
        width: (terminal_size.width * 80u16) / 100u16,
        height: 3,
    };

    // If terminal too small, show error
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
        return;
    }

    // Create main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Fill(1)])
        .split(frame.area());

    let tab_string = vec![
        Tab::SqlEditor.to_string(),
        Tab::TableView.to_string(),
        Tab::LogViewer.to_string(),
    ];

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
    if state.db_input {
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
            let table_arc = state.shared.table.clone();
            let table = table_arc.lock().unwrap();
            if table.headers.is_empty() && table.rows.is_empty() {
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
                // Header row
                let block = Block::default()
                    .title("Table View")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Thick);
                frame.render_widget(block, chunks[1]);
                let inner_area = chunks[1].inner(ratatui::layout::Margin::new(1, 1));

                let max_rows_visible = inner_area.height.saturating_sub(1) as usize;
                let has_vertical_scroll = table.rows.len() > max_rows_visible;
                let v_scroll_width = if has_vertical_scroll { 1 } else { 0 };

                let available_width = inner_area.width.saturating_sub(v_scroll_width);

                let mut max_widths: Vec<u16> =
                    table.headers.iter().map(|h| h.len() as u16).collect();

                for row in table.rows.iter().take(100) {
                    for (i, cell) in row.iter().enumerate() {
                        if i < max_widths.len() {
                            max_widths[i] = max_widths[i].max(cell.len() as u16);
                        }
                    }
                }

                let all_col_widths: Vec<u16> =
                    max_widths.iter().map(|&w| (w + 2).clamp(5, 30)).collect();

                let mut current_width = 0;
                let mut visible_cols_end_idx = state.table_col_offset;

                for (i, width) in all_col_widths
                    .iter()
                    .enumerate()
                    .skip(state.table_col_offset)
                {
                    if current_width + width > available_width {
                        break;
                    }
                    current_width += width;
                    visible_cols_end_idx = i + 1;
                }

                let has_horizontal_scroll =
                    visible_cols_end_idx < table.headers.len() || state.table_col_offset > 0;
                let h_scroll_height = if has_horizontal_scroll { 1 } else { 0 };

                let table_area = Rect {
                    x: inner_area.x,
                    y: inner_area.y,
                    width: inner_area.width.saturating_sub(v_scroll_width),
                    height: inner_area.height.saturating_sub(h_scroll_height),
                };

                let content_height = table_area.height.saturating_sub(1) as usize;

                if state.table_selected < state.table_offset {
                    state.table_offset = state.table_selected;
                } else if state.table_selected >= state.table_offset + content_height {
                    state.table_offset = state
                        .table_selected
                        .saturating_sub(content_height)
                        .saturating_add(1);
                }

                let start_index = state.table_offset;
                let end_index = (start_index + content_height).min(table.rows.len());

                let visible_headers = &table.headers[state.table_col_offset..visible_cols_end_idx];
                let header = Row::new(visible_headers.iter().map(|s| s.as_str()));

                let rows =
                    table.rows[start_index..end_index]
                        .iter()
                        .enumerate()
                        .map(|(idx, row)| {
                            let actual_idx = start_index + idx;
                            let visible_cells = row
                                .iter()
                                .enumerate()
                                .skip(state.table_col_offset)
                                .take(visible_cols_end_idx - state.table_col_offset)
                                .map(|(col_idx, s)| {
                                    let width = all_col_widths[col_idx] as usize;
                                    if s.chars().count() > width {
                                        let mut truncated: String =
                                            s.chars().take(width.saturating_sub(1)).collect();
                                        truncated.push('…');
                                        Cell::from(truncated)
                                    } else {
                                        Cell::from(s.clone())
                                    }
                                });
                            let style = if actual_idx == state.table_selected {
                                style::Style::default().bg(Color::White).fg(Color::Black)
                            } else if actual_idx % 2 == 0 {
                                style::Style::default().bg(Color::Black)
                            } else {
                                style::Style::default().bg(Color::DarkGray)
                            };
                            Row::new(visible_cells).style(style)
                        });

                let col_widths: Vec<Constraint> = all_col_widths
                    [state.table_col_offset..visible_cols_end_idx]
                    .iter()
                    .map(|&w| Constraint::Length(w))
                    .collect();

                frame.render_widget(
                    Table::default()
                        .rows(rows)
                        .header(header)
                        .cell_highlight_style(Style::default().fg(Color::White))
                        .row_highlight_style(Style::default().fg(Color::Black).bg(Color::White))
                        .widths(&col_widths),
                    table_area,
                );

                if has_vertical_scroll {
                    draw_custom_scrollbar(
                        frame,
                        Rect {
                            x: table_area.right(),
                            y: table_area.y,
                            width: 1,
                            height: table_area.height,
                        },
                        table.rows.len(),
                        content_height,
                        state.table_offset,
                        ScrollbarOrientation::VerticalRight,
                    );
                }

                if has_horizontal_scroll {
                    draw_custom_scrollbar(
                        frame,
                        Rect {
                            x: table_area.x,
                            y: table_area.bottom(),
                            width: table_area.width,
                            height: 1,
                        },
                        table.headers.len(),
                        visible_cols_end_idx.saturating_sub(state.table_col_offset),
                        state.table_col_offset,
                        ScrollbarOrientation::HorizontalBottom,
                    );
                }
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

    if state.show_file_popup {
        if state.file_save == Some(FileAction::Save) && !state.file_popup_is_active {
            state.file_popup_is_active = true;
            #[allow(unused_assignments, unused_mut)]
            let mut curret_path: String = state.file_textarea.lines().first().unwrap().to_string();
            #[allow(unused_assignments, unused_mut)]
            let mut save_path: String = String::new();
            if !curret_path.contains(".sql") {
                let suggested_filename = format!(
                    "{}_{}.sql",
                    "query",
                    humantime::format_rfc3339_seconds(SystemTime::now())
                );
                if std::env::consts::OS == "windows" {
                    save_path = format!("\\{}", suggested_filename);
                } else {
                    save_path = format!("/{}", suggested_filename);
                }
                state.file_textarea.move_cursor(CursorMove::End);
                state.file_textarea.insert_str(&save_path.as_str());
            }
        }
        state
            .file_textarea
            .set_cursor_line_style(Style::default().add_modifier(Modifier::UNDERLINED));
        state
            .file_textarea
            .set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
        let title = match state.file_save {
            Some(FileAction::Save) => "Save SQL to File",
            Some(FileAction::Load) => "Load SQL from File",
            None => "File Action",
        };
        state.file_textarea.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Thick)
                .title(title),
        );
        frame.render_widget(&state.file_textarea, popup_size);
    }

    // Render help popup
    if state.show_help {
        let mut helplinetext = String::new();
        for shortcut in SHORTCUTS {
            helplinetext.push_str(&format!("{}\n", shortcut.to_string()));
        }
        let frame_size = frame.area();
        let help_paragraoh = Text::from(helplinetext);
        let help_popup = Popup::new(help_paragraoh)
            .style(Style::default().fg(Color::Gray).bg(Color::DarkGray))
            .title("Help - Shortcuts")
            .borders(Borders::ALL)
            .border_set(border::THICK);
        frame.render_widget(&help_popup, frame_size);
    }
}

// ── Event handling ────────────────────────────────────────────────────────

/// Returns `true` if the app should quit.
fn handle_event(event: Event, state: &mut ExtendedAppState) -> bool {
    let terminal_size = crossterm::terminal::size().unwrap_or((0, 0));
    let min_width: u16 = 115;
    let min_height: u16 = SHORTCUTS.len() as u16 + 5;

    // If terminal too small, only allow quit
    if terminal_size.0 < min_width || terminal_size.1 < min_height {
        if let Event::Key(key_event) = event {
            if key_event.modifiers == KeyModifiers::CONTROL {
                match key_event.code {
                    KeyCode::Char('c') | KeyCode::Char('d') => return true,
                    _ => {}
                }
            }
        }
        return false;
    }

    if state.show_help {
        if let Event::Key(key_event) = event {
            if key_event.modifiers == KeyModifiers::CONTROL {
                match key_event.code {
                    KeyCode::Char('c') | KeyCode::Char('d') => return true,
                    _ => {}
                }
            }
            match key_event.code {
                KeyCode::F(12) => return true,
                KeyCode::Esc | KeyCode::F(1) => {
                    state.show_help = false;
                    info!("close Help popup");
                }
                _ => {}
            }
        }
    } else if state.db_input {
        match event {
            Event::Key(key_event) => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    match key_event.code {
                        KeyCode::Char('c') | KeyCode::Char('d') => return true,
                        _ => {}
                    }
                }
                match key_event.code {
                    KeyCode::F(12) => return true,
                    KeyCode::Esc | KeyCode::Enter => {
                        state.db_input = false;
                        debug!("Exiting DB input mode");
                    }
                    _ => {
                        state
                            .db_textarea
                            .input(tui_textarea::Input::from(key_event));
                    }
                }
            }
            Event::Mouse(mouse_event) => {
                debug!("Mouse event: {:?}", mouse_event);
            }
            _ => {}
        }
        state.shared.db = state.db_textarea.lines().join("\n");
    } else if state.show_file_popup {
        match event {
            Event::Key(key_event) => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    match key_event.code {
                        KeyCode::Char('c') | KeyCode::Char('d') => return true,
                        _ => {}
                    }
                }
                match key_event.code {
                    KeyCode::F(12) => return true,
                    KeyCode::Esc => {
                        state.show_file_popup = false;
                        state.file_popup_is_active = false;
                        debug!("Exiting DB input mode");
                    }
                    KeyCode::Enter => {
                        if let Some(action) = &state.file_save {
                            match action {
                                FileAction::Save => {
                                    if let Err(e) = shared::write_file(
                                        state.file_textarea.lines().join("\n").as_str(),
                                        state.shared.sql_query.as_str(),
                                    ) {
                                        error!("Error saving file: {}", e);
                                    } else {
                                        info!("File saved successfully");
                                    }
                                }
                                FileAction::Load => {
                                    match shared::read_file(
                                        state.file_textarea.lines().join("\n").as_str(),
                                    ) {
                                        Ok(content) => {
                                            state.shared.sql_query = content;
                                            state.editor_state.lines =
                                                Lines::from(state.shared.sql_query.clone());
                                            info!("File loaded successfully");
                                        }
                                        Err(e) => {
                                            error!("Error loading file: {}", e);
                                        }
                                    }
                                }
                            }
                        }
                        state.show_file_popup = false;
                        state.file_popup_is_active = false;
                    }
                    _ => {
                        state
                            .file_textarea
                            .input(tui_textarea::Input::from(key_event));
                    }
                }
            }
            Event::Mouse(mouse_event) => {
                debug!("Mouse event: {:?}", mouse_event);
            }
            _ => {}
        }
    } else {
        // Handle editor events
        EditorEventHandler::default().on_event(event.clone(), &mut state.editor_state);
        match state.shared.current_tab {
            shared::Tab::SqlEditor => {
                state.shared.sql_query = get_editor_lines_as_string(&state);
            }
            _ => {}
        }
        // Handle key events
        match event {
            Event::Key(key_event) => {
                match key_event.code {
                    KeyCode::F(12) => return true,
                    KeyCode::F(1) => {
                        state.show_help = true;
                        info!("show Help popup");
                    }
                    KeyCode::F(2) => {
                        state.shared.current_tab = state.shared.current_tab.next();
                        info!("Switched to tab: {}", state.shared.current_tab.to_string());
                    }
                    KeyCode::F(3) => {
                        state.db_input = !state.db_input;
                    }
                    KeyCode::F(4) => {
                        state.shared.set_next_user();
                    }
                    KeyCode::F(5) => {
                        if let Err(e) = shared::run_query(&mut state.shared) {
                            error!("Error running query: {}", e);
                        }
                    }
                    KeyCode::F(8) => {
                        state.file_save = Some(FileAction::Save);
                        state.show_file_popup = !state.show_file_popup;
                    }
                    KeyCode::F(9) => {
                        state.file_save = Some(FileAction::Load);
                        state.show_file_popup = !state.show_file_popup;
                    }
                    KeyCode::Down => {
                        if state.shared.current_tab == shared::Tab::TableView {
                            let table_len = state.shared.table.lock().unwrap().rows.len();
                            if table_len > 0 {
                                if state.table_selected < table_len - 1 {
                                    state.table_selected += 1;
                                } else {
                                    state.table_selected = 0;
                                }
                            }
                        }
                    }
                    KeyCode::Up => {
                        if state.shared.current_tab == shared::Tab::TableView {
                            let table_len = state.shared.table.lock().unwrap().rows.len();
                            if table_len > 0 {
                                if state.table_selected > 0 {
                                    state.table_selected -= 1;
                                } else {
                                    state.table_selected = table_len - 1;
                                }
                            }
                        }
                    }
                    KeyCode::Right => {
                        if state.shared.current_tab == shared::Tab::TableView {
                            let table_headers_len =
                                state.shared.table.lock().unwrap().headers.len();
                            if state.table_col_offset < table_headers_len.saturating_sub(1) {
                                state.table_col_offset += 1;
                            }
                        }
                    }
                    KeyCode::Left => {
                        if state.shared.current_tab == shared::Tab::TableView {
                            if state.table_col_offset > 0 {
                                state.table_col_offset -= 1;
                            }
                        }
                    }
                    _ => {}
                }
                // Handle modifier keys
                if key_event.modifiers == KeyModifiers::CONTROL {
                    match key_event.code {
                        KeyCode::Char('c') | KeyCode::Char('d') => return true,
                        _ => {}
                    }
                }
            }

            Event::Mouse(mouse_event) => {
                debug!("Mouse event: {:?}", mouse_event);
            }
            _ => {}
        }
    }

    false
}

// ── Entry point ───────────────────────────────────────────────────────────

pub fn main_tui(file_content: String) -> Result<(), Box<dyn Error>> {
    // Terminal init
    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    crossterm::execute!(
        stdout,
        crossterm::terminal::EnterAlternateScreen,
        crossterm::event::EnableMouseCapture
    )?;
    let backend = ratatui::backend::CrosstermBackend::new(stdout);
    let mut terminal = ratatui::Terminal::new(backend)?;

    // App state
    let mut state = ExtendedAppState::default();
    if !file_content.is_empty() {
        state.shared.sql_query = file_content;
    }

    let tick_rate = std::time::Duration::from_millis(100);

    loop {
        terminal.draw(|frame| {
            ui(frame, &mut state);
        })?;

        if crossterm::event::poll(tick_rate)? {
            let event = crossterm::event::read()?;
            if handle_event(event, &mut state) {
                break;
            }
        }
    }

    // Restore terminal
    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        crossterm::terminal::LeaveAlternateScreen,
        crossterm::event::DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

// ── Theme ─────────────────────────────────────────────────────────────────

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

// ── Custom scrollbar ──────────────────────────────────────────────────────

fn draw_custom_scrollbar(
    frame: &mut ratatui::Frame,
    area: Rect,
    content_len: usize,
    viewport_len: usize,
    scroll_pos: usize,
    orientation: ScrollbarOrientation,
) {
    let track_len = match orientation {
        ScrollbarOrientation::VerticalRight | ScrollbarOrientation::VerticalLeft => area.height,
        ScrollbarOrientation::HorizontalBottom | ScrollbarOrientation::HorizontalTop => area.width,
    };

    if track_len == 0 || content_len <= viewport_len {
        return;
    }

    let thumb_size =
        ((viewport_len as f64 / content_len as f64) * track_len as f64).max(1.0) as u16;
    let max_thumb_pos = track_len.saturating_sub(thumb_size);
    let thumb_pos = ((scroll_pos as f64 / (content_len.saturating_sub(viewport_len)) as f64)
        * max_thumb_pos as f64)
        .min(max_thumb_pos as f64) as u16;

    let (track_symbol, thumb_symbol) = match orientation {
        ScrollbarOrientation::VerticalRight | ScrollbarOrientation::VerticalLeft => ("│", "┃"),
        ScrollbarOrientation::HorizontalBottom | ScrollbarOrientation::HorizontalTop => ("─", "━"),
    };

    let mut text_lines = Vec::new();

    if matches!(
        orientation,
        ScrollbarOrientation::VerticalRight | ScrollbarOrientation::VerticalLeft
    ) {
        for i in 0..track_len {
            let is_thumb = i >= thumb_pos && i < thumb_pos + thumb_size;
            let (symbol, style) = if is_thumb {
                (thumb_symbol, Style::default().fg(Color::Gray))
            } else {
                (track_symbol, Style::default().fg(Color::DarkGray))
            };
            text_lines.push(Line::from(Span::styled(symbol, style)));
        }
        frame.render_widget(Paragraph::new(text_lines), area);
    } else {
        let mut spans = Vec::new();
        for i in 0..track_len {
            let is_thumb = i >= thumb_pos && i < thumb_pos + thumb_size;
            let (symbol, style) = if is_thumb {
                (thumb_symbol, Style::default().fg(Color::Gray))
            } else {
                (track_symbol, Style::default().fg(Color::DarkGray))
            };
            spans.push(Span::styled(symbol, style));
        }
        frame.render_widget(Paragraph::new(Line::from(spans)), area);
    }
}
