// Copyright (c) 2025 mcpeaps_HD
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] use std::fmt::format;

// hide console window on Windows in release
#[allow(unused_imports)]
use crate::shared;
use eframe::{egui, Frame};
use egui::{frame, Color32};
use egui_tiles::{Tile, TileId, Tiles};
use egui_logger;
use egui_code_editor::{CodeEditor, ColorTheme, Syntax};

#[allow(dead_code)]
fn nord_color_theme() -> ColorTheme {
  let bg = shared::NordColor::Nord0.as_str();
  let cursor = shared::NordColor::Nord6.as_str();
  let selection = shared::NordColor::Nord13.as_str();
  let comments = shared::NordColor::Nord14.as_str();
  let functions = shared::NordColor::Nord8.as_str();
  let keywords = shared::NordColor::Nord15.as_str();
  let literals = shared::NordColor::Nord14.as_str();
  let numerics = shared::NordColor::Nord13.as_str();
  let punctuation = shared::NordColor::Nord5.as_str();
  let strs = shared::NordColor::Nord14.as_str();
  let types = shared::NordColor::Nord15.as_str();
  let special = shared::NordColor::Nord12.as_str();

            ColorTheme {
                name: "Nord",
                dark: true,
                bg: &bg,
                cursor: &cursor,
                selection: &selection,
                comments: &comments,
                functions: &functions,
                keywords: &keywords,
                literals: &literals,
                numerics: &numerics,
                punctuation: &punctuation,
                strs: &strs,
                types: &types,
                special: &special,
            }
}

struct Pane {
    nr: shared::Tab,
}
struct TreeBehavior {}

impl egui_tiles::Behavior<Pane> for TreeBehavior {
    fn tab_title_for_pane(&mut self, pane: &Pane) -> egui::WidgetText {
        let label: &'static str;
        match pane.nr {
            shared::Tab::SqlEditor => { label = "SQL Editor" },
            shared::Tab::TableView => { label = "Table View" },
            shared::Tab::CredentialsEditor => { label = "Credentials" },
            shared::Tab::ConnectionsEditor => { label = "Connections" },
            shared::Tab::RunLog => { label = "Run and Log" },
        }
        format!("{}", label).into()
    }

    fn pane_ui(
        &mut self,
        ui: &mut egui::Ui,
        _tile_id: egui_tiles::TileId,
        pane: &mut Pane,
    ) -> egui_tiles::UiResponse {
        // Give each pane a unique color:
        let background_color = Color32::from_hex(shared::NordColor::Nord0.as_str()).unwrap();
        ui.painter().rect_filled(ui.max_rect(), 0.0, background_color);
        ui.label(self.tab_title_for_pane(pane).color(Color32::from_hex(shared::NordColor::Nord6.as_str()).unwrap()));
        match pane.nr {
            shared::Tab::SqlEditor => {
                let mut sql_query: String = String::from("select * from test;");
                CodeEditor::default()
                    .id_source("code editor")
                    .with_rows(12)
                    .with_fontsize(14.0)
                    .with_theme(ColorTheme::GITHUB_DARK)//(nord_color_thme())
                    .with_syntax(Syntax::sql())
                    .with_numlines(true)
                    .show(ui, &mut sql_query);
            },
            shared::Tab::TableView => {},
            shared::Tab::CredentialsEditor => {
              let mut credential: String = shared::get_credential_content().expect("Failed to load connections");
                CodeEditor::default()
                    .id_source("code editor")
                    .with_rows(12)
                    .with_fontsize(14.0)
                    .with_theme(ColorTheme::GITHUB_DARK)//(nord_color_thme())
                    .with_syntax(Syntax::sql())
                    .with_numlines(true)
                    .show(ui, &mut credential);
                shared::set_credential_content(credential).expect("Failed to save connections");
            },
            shared::Tab::ConnectionsEditor => {
              let mut connections: String = shared::get_connections_content().expect("Failed to load connections");
                CodeEditor::default()
                    .id_source("code editor")
                    .with_rows(12)
                    .with_fontsize(14.0)
                    .with_theme(ColorTheme::GITHUB_DARK)//(nord_color_thme())
                    .with_syntax(Syntax::sql())
                    .with_numlines(true)
                    .show(ui, &mut connections);
                shared::set_connections_content(connections).expect("Failed to save connections");
            },
            shared::Tab::RunLog => {
                egui_logger::logger_ui().show(ui);
            }

        }

        // You can make your pane draggable like so:
        if ui.response().drag_started() {
            egui_tiles::UiResponse::DragStarted
        } else {
            egui_tiles::UiResponse::None
        }

    }
}

pub fn main_gui() -> Result<(), eframe::Error> {
    //env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    let mut tree = create_tree();

    eframe::run_simple_native("simplesql", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut behavior = TreeBehavior {};
            tree.ui(&mut behavior, ui);
        });
    })
}

fn create_tree() -> egui_tiles::Tree<Pane> {
    let mut next_view_nr: shared::Tab = shared::Tab::SqlEditor;
    let mut gen_pane = || {
        let pane = Pane { nr: next_view_nr };
        match next_view_nr {
            shared::Tab::SqlEditor => {next_view_nr = shared::Tab::TableView;},
            shared::Tab::TableView => {next_view_nr = shared::Tab::CredentialsEditor;},
            shared::Tab::CredentialsEditor => {next_view_nr = shared::Tab::ConnectionsEditor;},
            shared::Tab::ConnectionsEditor => {next_view_nr = shared::Tab::RunLog;},
            _ => {next_view_nr = shared::Tab::SqlEditor;}
        }
        pane
    };

    let mut tiles = egui_tiles::Tiles::default();

    let mut tabs = vec![];
    tabs.push(tiles.insert_pane(gen_pane()));
    tabs.push(tiles.insert_pane(gen_pane()));
    tabs.push(tiles.insert_pane(gen_pane()));
    tabs.push(tiles.insert_pane(gen_pane()));
    tabs.push(tiles.insert_pane(gen_pane()));

    let root = tiles.insert_tab_tile(tabs);

    egui_tiles::Tree::new("my_tree", root, tiles)
}