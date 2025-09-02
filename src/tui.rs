// Copyright (c) 2025 mcpeaps_HD
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
#[allow(unused_imports)]
use crate::shared;
use crate::shared::Tab;
#[allow(unused_imports)]
use edtui::{
    syntect::parsing::{Scope, SyntaxReference}, EditorEventHandler, EditorState, EditorStatusLine, EditorTheme, EditorView,
    Lines,
    SyntaxHighlighter,
};
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use ratatui::layout::Flex;
use ratatui::symbols::border;
#[allow(unused_imports)]
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::*,
    style::{Color, Style},
    widgets::*,
};
use std::cmp::PartialEq;
#[allow(unused_imports)]
use std::error::Error;
use std::fmt::format;
use std::ops::Deref;
use std::time::SystemTime;
use tui_logger::{TuiLoggerLevelOutput, TuiLoggerSmartWidget, TuiLoggerWidget};
use tui_popup::Popup;
use tui_textarea::{CursorMove, TextArea};
#[allow(unused_imports)]
use widgetui::{
    crossterm::event::{
        Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, ModifierKeyCode,
        MouseEvent,
    },
    *,
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
#[derive(Clone, State)]
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

fn widget(
    mut frame: ResMut<WidgetFrame>,
    mut events: ResMut<Events>,
    mut state: ResMut<ExtendedAppState>,
) -> WidgetResult {
    let min_width: u16 = 115; // minimum width for the terminal based on help text length
    let min_height: u16 = SHORTCUTS.len() as u16 + 5; // Minimum height for the terminal

    // Prüfe die aktuelle Terminal-Größe
    let terminal_size = frame.size();
    let popup_size = Rect {
        x: (terminal_size.width * 10u16) / 100u16,
        y: terminal_size.height / 4,
        width: (terminal_size.width * 80u16) / 100u16,
        height: 3,
    };

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
        .constraints([Constraint::Length(3), Constraint::Fill(1)])
        .split(frame.size());

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
                    .enumerate()
                    .map(|(idx, row)| {
                        let style = if idx % 2 == 0 {
                            Style::default().bg(Color::Black)
                        } else {
                            Style::default().bg(Color::DarkGray)
                        };
                        Row::new(row.clone()).style(style)
                    });

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

    if state.show_file_popup {
        if state.file_save == Some(FileAction::Save) && !state.file_popup_is_active {
            state.file_popup_is_active = true;
            let mut curret_path: String = state.file_textarea.lines().first().unwrap().to_string();
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
        let frame_size = frame.size();
        let help_paragraoh = Text::from(helplinetext);
        let help_popup = Popup::new(help_paragraoh)
            .style(Style::default().fg(Color::Gray).bg(Color::DarkGray))
            .title("Help - Shortcuts")
            .borders(Borders::ALL)
            .border_set(border::THICK);
        frame.render_widget(&help_popup, frame_size);
    }

    if state.show_help {
        match events.event.clone() {
            Some(event) => match event {
                Event::Key(key_event) => {
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
                        KeyCode::Esc | KeyCode::F(1) => {
                            state.show_help = false;
                            info!("close Help popup");
                        }
                        _ => {}
                    }
                }
                _ => {}
            },
            _ => {}
        }
    } else {
        if state.db_input {
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
                            // Handle mouse events if needed
                            debug!("Mouse event: {:?}", mouse_event);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
            state.shared.db = state.db_textarea.lines().join("\n");
        } else if state.show_file_popup {
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
                                KeyCode::Esc => {
                                    // Exit DB input mode
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
                                                        state.editor_state.lines = Lines::from(
                                                            state.shared.sql_query.clone(),
                                                        );
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
                            // Handle mouse events if needed
                            debug!("Mouse event: {:?}", mouse_event);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
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
                                    state.show_help = true;
                                    info!("show Help popup");
                                }
                                KeyCode::F(2) => {
                                    state.shared.current_tab = state.shared.current_tab.next();
                                    info!(
                                        "Switched to tab: {}",
                                        state.shared.current_tab.to_string()
                                    );
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
