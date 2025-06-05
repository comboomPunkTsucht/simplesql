// Copyright (c) 2025 mcpeaps_HD
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{collections::BTreeSet, fmt::format};

// hide console window on Windows in release
#[allow(unused_imports)]
use crate::shared;
use eframe::{Frame, egui};
use egui::{Color32, frame};
use egui_code_editor::{CodeEditor, ColorTheme, Syntax};
use egui_logger;
use egui_tiles::{Tile, TileId, Tiles};

#[allow(dead_code)]
impl shared::NordColor {
    pub fn to_color32(&self) -> Color32 {
        Color32::from_hex(self.as_str()).unwrap()
    }
}


#[allow(dead_code)]
fn nord_color_theme() -> ColorTheme {
    ColorTheme {
        name: "Nord",
        dark: true,
        bg: shared::NordColor::Nord0.as_str(),
        cursor: shared::NordColor::Nord6.as_str(),
        selection: shared::NordColor::Nord13.as_str(),
        comments: shared::NordColor::Nord3.as_str(),
        functions: shared::NordColor::Nord8.as_str(),
        keywords: shared::NordColor::Nord15.as_str(),
        literals: shared::NordColor::Nord14.as_str(),
        numerics: shared::NordColor::Nord13.as_str(),
        punctuation: shared::NordColor::Nord5.as_str(),
        strs: shared::NordColor::Nord14.as_str(),
        types: shared::NordColor::Nord15.as_str(),
        special: shared::NordColor::Nord0.as_str(),
    }
}

#[allow(dead_code)]
fn jsonc_lang() -> Syntax {
    Syntax::new("jsonc")
        .with_comment("//")
        .with_comment_multiline(["/*","*/"])
        .with_types(BTreeSet::from(["string", "integer", "float", "boolean", "object","array"]))
}
struct TreeBehavior {}

impl egui_tiles::Behavior<shared::AppState> for TreeBehavior {
    fn tab_title_for_pane(&mut self, pane: &shared::AppState) -> egui::WidgetText {
        let label: &'static str;
        match pane.current_tab {
            shared::Tab::SqlEditor => label = "SQL Editor",
            shared::Tab::TableView => label = "Table View",
            shared::Tab::ConfigEditor => label = "Config Editor",
            shared::Tab::RunLog => label = "Run and Log",
        }
        format!("{}", label).into()
    }

    fn pane_ui(
        &mut self,
        ui: &mut egui::Ui,
        _tile_id: egui_tiles::TileId,
        pane: &mut shared::AppState,
    ) -> egui_tiles::UiResponse {
        // Give each pane a unique color:
        let background_color = shared::NordColor::Nord0.to_color32();
        ui.painter()
            .rect_filled(ui.max_rect(), 0.0, background_color);
        ui.label(
            self.tab_title_for_pane(pane)
                .color(shared::NordColor::Nord6.to_color32()),
        );
        match pane.current_tab {
            shared::Tab::SqlEditor => {
                CodeEditor::default()
                    .id_source("code editor")
                    .with_rows(12)
                    .with_fontsize(14.0)
                    .with_theme(nord_color_theme()) //(nord_color_thme())
                    .with_syntax(Syntax::sql())
                    .with_numlines(true)
                    .show(ui, &mut pane.sql_query);
            }
            shared::Tab::TableView => {}
            shared::Tab::ConfigEditor => {
                let mut config: String =
                    shared::get_config_content().expect("Failed to load config");
                CodeEditor::default()
                    .id_source("code editor")
                    .with_rows(12)
                    .with_fontsize(14.0)
                    .with_theme(nord_color_theme()) //(nord_color_thme())
                    .with_syntax(jsonc_lang())
                    .with_numlines(true)
                    .show(ui, &mut config);
                shared::set_config_content(config).expect("Failed to save connections");
            }
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

fn create_tree() -> egui_tiles::Tree<shared::AppState> {
    let mut next_view_nr: shared::Tab = shared::Tab::SqlEditor;
    let mut gen_pane = || {
        let pane = shared::AppState {
            current_tab: next_view_nr,
            ..Default::default()
        };
        match next_view_nr {
            shared::Tab::SqlEditor => {
                next_view_nr = shared::Tab::TableView;
            }
            shared::Tab::TableView => {
                next_view_nr = shared::Tab::ConfigEditor;
            }
            shared::Tab::ConfigEditor => {
                next_view_nr = shared::Tab::RunLog;
            }
            _ => {
                next_view_nr = shared::Tab::SqlEditor;
            }
        }
        pane
    };

    let mut tiles = egui_tiles::Tiles::default();

    let mut tabs = vec![];
    tabs.push(tiles.insert_pane(gen_pane()));
    tabs.push(tiles.insert_pane(gen_pane()));
    tabs.push(tiles.insert_pane(gen_pane()));
    tabs.push(tiles.insert_pane(gen_pane()));

    let root = tiles.insert_tab_tile(tabs);

    egui_tiles::Tree::new("my_tree", root, tiles)
}
