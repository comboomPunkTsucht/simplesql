// Copyright (c) 2025 mcpeaps_HD
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

#[allow(unused_imports)]
use crate::shared;
#[allow(unused_imports)]
use crate::shared::{AppState, NordColor};
#[allow(unused_imports)]
use iced::{
    highlighter, widget::{button, column, container, row, text, text_input}, Alignment, ContentFit, Element, Fill, Length,
    Theme,
};
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
#[allow(unused_imports)]
use std::time::SystemTime;
use widgetui::State;

pub fn main_gui() -> Result<(), Box<dyn std::error::Error>> {
    if let Err(e) = iced::application(
        "simplesql",
        ExtendedAppState::update,
        ExtendedAppState::view,
    )
    .theme(theme)
    .run()
    {
        error!("Application error: {}", e);
    }
    Ok(())
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
enum Message {
    SQLQuery(String),
    SetUser(shared::Credential),
    RunQuery,
}

#[derive(Clone, State)]
struct ExtendedAppState {
    pub shared: shared::AppState,
    // More fields can be added here as needed
}
impl Default for ExtendedAppState {
    fn default() -> Self {
        ExtendedAppState {
            shared: shared::AppState::default(),
        }
    }
}
impl ExtendedAppState {
    fn update(&mut self, message: Message) {
        match message {
            Message::SQLQuery(query) => {
                self.shared.sql_query = query;
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
    fn view(&self) -> Element<Message> {
        let input = text_input("SQL Query", &self.shared.sql_query)
            .on_input(Message::SQLQuery)
            .padding(10)
            .size(16)
            .width(Length::Fill);

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
