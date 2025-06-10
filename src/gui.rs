// Updated version: Switch from egui_tiles to egui_dock

use std::collections::BTreeSet;
use eframe::egui::{self, Color32};
use egui::Ui;
use egui_code_editor::{CodeEditor, ColorTheme, Syntax};
use egui_dock::{DockArea, NodeIndex, Style, Tree, DockState, TabViewer};
use egui_file::State;
use crate::shared;
use crate::shared::AppState;

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
        .with_comment_multiline(["/*", "*/"])
        .with_types(BTreeSet::from([
            "string", "integer", "float", "boolean", "object", "array",
        ]))
}

struct MyTabViewer {
    app_state: shared::AppState
}

impl MyTabViewer {
    pub fn new( state: shared::AppState) -> MyTabViewer {
        MyTabViewer {
            app_state: state
        }
    }
}

impl TabViewer for MyTabViewer {
    type Tab = shared::Tab;

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        let background_color = shared::NordColor::Nord0.to_color32();
        ui.painter().rect_filled(ui.max_rect(), 0.0, background_color);

        match tab {
            shared::Tab::SqlEditor => {
                ui.label("SQL Editor");
                
                CodeEditor::default()
                    .id_source("sql_editor")
                    .with_rows(12)
                    .with_fontsize(14.0)
                    .with_theme(nord_color_theme())
                    .with_syntax(Syntax::sql())
                    .with_numlines(true)
                    .show(ui, &mut self.app_state.sql_query);
            }
            shared::Tab::TableView => {
                ui.label("Table View");
            }
            shared::Tab::ConfigEditor => {
                ui.label("Config Editor");
                let mut config: String = shared::get_config_content(&mut self.app_state).expect("Failed to load config");
                CodeEditor::default()
                    .id_source("config_editor")
                    .with_rows(12)
                    .with_fontsize(14.0)
                    .with_theme(nord_color_theme())
                    .with_syntax(jsonc_lang())
                    .with_numlines(true)
                    .show(ui, &mut config);
                shared::set_config_content(config).expect("Failed to save connections");
            }
        }
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        match tab {
            shared::Tab::SqlEditor => "SQL Editor".into(),
            shared::Tab::TableView => "Table View".into(),
            shared::Tab::ConfigEditor => "Config Editor".into(),
        }
    }
}

pub fn main_gui() -> Result<(), eframe::Error> {

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    let mut dock_state = DockState::new(vec![
        shared::Tab::SqlEditor,
        shared::Tab::TableView,
        shared::Tab::ConfigEditor,
    ]);

    let mut tab_viewer = MyTabViewer::new(shared::AppState::default());

    eframe::run_simple_native("simplesql", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            DockArea::new(&mut dock_state)
                .style(Style::from_egui(ui.style().as_ref()))
                .show_inside(ui, &mut tab_viewer);
        });
    })
} 
