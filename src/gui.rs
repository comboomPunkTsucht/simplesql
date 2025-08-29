// Copyright (c) 2025 mcpeaps_HD
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

#[allow(unused_imports)]
use crate::shared;
#[allow(unused_imports)]
use crate::shared::{AppState, NordColor};
use iced::futures::{AsyncBufReadExt, AsyncReadExt};
use iced::widget::text_editor::{Action, Content, Edit};
use iced::window::settings::PlatformSpecific;
#[allow(unused_imports)]
use iced::{
  highlighter, widget::{button, column, container, row, text, text_editor}, window, Alignment, ContentFit, Element, Fill, Font, Length, Pixels,
  Size,
  Theme,
};
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use std::sync::Arc;
#[allow(unused_imports)]
use std::time::SystemTime;
use widgetui::State;

pub fn main_gui(_file_content: String) -> Result<(), Box<dyn std::error::Error>> {
    /*let mut state = ExtendedAppState::default();
    if file_content.is_empty() {
        state.shared.sql_query = file_content;
        state.content.perform(Action::Edit(Edit::Paste(Arc::from(
            state.shared.sql_query.clone(),
        ))));
    }*/
    if let Err(e) = iced::application(
        "simplesql",
        ExtendedAppState::update,
        ExtendedAppState::view,
    )
    .theme(theme)
    .window(window::Settings::from(window::Settings {
        size: Size::new(1920.0, 1080.0),
        position: window::Position::Default,
        min_size: Some(Size::new(1280.0, 720.0)),
        max_size: None,
        visible: true,
        decorations: true,
        transparent: true,
        level: window::Level::Normal,
        icon: None,
        platform_specific: window::settings::PlatformSpecific::default(),
        resizable: true,
        exit_on_close_request: true,
    }))
    .settings(iced::Settings {
        id: Some("simplesql".to_string()),
        antialiasing: true,
        default_text_size: Pixels::from(16),
        default_font: Font::default(),
        fonts: vec![],
    })
    .run()
    {
        error!("Application error: {}", e);
    }
    Ok(())
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
enum Message {
    SQLQuery(text_editor::Action),
    SetUser(shared::Credential),
    RunQuery,
}

struct ExtendedAppState {
    pub shared: shared::AppState,
    pub content: text_editor::Content, // More fields can be added here as needed
}
impl Default for ExtendedAppState {
    fn default() -> Self {
        let shared = shared::AppState::default();
        let mut content = text_editor::Content::new();
        content.perform(Action::Edit(Edit::Paste(Arc::from(
            shared.sql_query.clone(),
        ))));
        ExtendedAppState { shared, content }
    }
}
impl ExtendedAppState {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    fn update(&mut self, message: Message) {
        match message {
            Message::SQLQuery(action) => {
                self.content.perform(action);
                self.shared.sql_query = self.content.text();
            }
            Message::SetUser(credential) => {
                self.shared.user = credential;
            }
            Message::RunQuery => {
                // Logic to run the SQL query
                info!("Running SQL query: {}", self.shared.sql_query);
            }
        }
    }
    #[allow(dead_code)]
    fn view(&self) -> Element<Message> {
        let input = text_editor(&self.content)
            .placeholder("Enter your SQL query here...")
            .highlight("sql", highlighter::Theme::SolarizedDark)
            .on_action(Message::SQLQuery)
            .padding(10)
            .size(16)
            .height(Length::Fill);

        let run_button = button("Run Query").on_press(Message::RunQuery).padding(10);

        let user_info = text(format!("User: {}", self.shared.user.name)).size(16);

        container(column![input, run_button, user_info,].spacing(20))
            .padding(20)
            .center_x(Fill)
            .center_y(Fill)
            .into()
    }
}

#[allow(unused_variables)]
fn theme(state: &ExtendedAppState) -> Theme {
    Theme::Nord
}
