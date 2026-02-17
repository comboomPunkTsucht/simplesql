// Copyright (c) 2025 mcpeaps_HD
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

#[allow(unused_imports)]
use crate::shared::{AppState, NordColor};
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use std::iter::{FilterMap, Map};
#[allow(unused_imports)]
use std::time::SystemTime;
use toml::Value::Table;

use crate::shared;
use crate::shared::Tab;
use colorful::Colorful;
use egui::{Id, TextBuffer, Ui, ViewportBuilder, WidgetText};
use egui_code_editor::{CodeEditor, ColorTheme, Syntax};
use egui_dock::TabViewer;
use egui_extras;
use egui_extras::Column;
use egui_file_dialog::{DialogState, FileDialog};

struct ExtendedAppState {
    pub shared: AppState,
    pub ctx: egui::Context,
    pub dock_state: egui_dock::DockState<Tab>,
    pub file_dialog: FileDialog,
}

impl Clone for ExtendedAppState {
    fn clone(&self) -> Self {
        ExtendedAppState {
            shared: self.shared.clone(),
            ctx: self.ctx.clone(),
            dock_state: self.dock_state.clone(),
            ..Default::default()
        }
    }
    fn clone_from(&mut self, source: &Self) {
        self.shared = source.shared.clone();
        self.ctx = source.ctx.clone();
        self.dock_state = source.dock_state.clone();
        self.file_dialog = Self::default().file_dialog;
    }
}
impl Default for ExtendedAppState {
    fn default() -> Self {
        let shared = AppState::default();
        let ctx = egui::Context::default();
        let dock_state = egui_dock::DockState::new(vec![Tab::SqlEditor, Tab::TableView]);
        let file_dialog = FileDialog::new()
            .resizable(true)
            .title("load sql-file")
            .default_file_filter("sql")
            .add_file_filter_extensions("sql", vec!["sql"])
            .add_save_extension("sql", "sql")
            .default_save_extension("sql")
            .initial_directory(std::env::home_dir().unwrap())
            .as_modal(true)
            .show_working_directory_button(true)
            .show_back_button(true)
            .show_devices(true)
            .show_current_path(true)
            .show_forward_button(true)
            .show_hidden_option(true)
            .show_left_panel(true)
            .show_menu_button(true)
            .show_parent_button(true)
            .show_pinned_folders(true)
            .show_places(true)
            .show_top_panel(true)
            .show_search(true)
            .show_system_files_option(true);
        Self {
            shared,
            ctx,
            dock_state,
            file_dialog,
        }
    }
}

impl TabViewer for ExtendedAppState {
    type Tab = Tab;

    fn title(&mut self, tab: &mut Self::Tab) -> WidgetText {
        match tab {
            Tab::SqlEditor => WidgetText::from("SQL Editor".to_string()),
            Tab::TableView => WidgetText::from("Table View".to_string()),
            _ => WidgetText::from("Unknown Tab".to_string()),
        }
    }

    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        match tab {
            Tab::SqlEditor => {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(Tab::SqlEditor.to_string());
                        if ui.button("load sql file").clicked() {
                            self.file_dialog.pick_file();
                            match self.file_dialog.state() {
                                DialogState::Picked(path) => {
                                    self.shared.sql_query =
                                        shared::read_file(path.to_str().unwrap()).unwrap();
                                }
                                _ => {}
                            }
                        } else if ui.button("save sql file").clicked() {
                            self.file_dialog.save_file();
                            match self.file_dialog.state() {
                                DialogState::Picked(path) => {
                                    shared::write_file(
                                        path.to_str().unwrap(),
                                        self.shared.sql_query.as_str(),
                                    )
                                    .unwrap();
                                }
                                _ => {}
                            }
                        }
                    });
                    CodeEditor::default()
                        .with_syntax(Syntax::sql())
                        .with_theme(theme())
                        .with_fontsize(14.0)
                        .with_numlines(true)
                        .vscroll(true)
                        .show(ui, &mut self.shared.sql_query);
                });
            }
            Tab::TableView => {
                ui.label("Table View");
                ui.label("Table View");
                let table = self.shared.table.lock().unwrap();
                if table.headers.is_empty() && table.rows.is_empty() {
                    ui.label("No data to display.");
                } else {
                    egui::ScrollArea::both()
                        .animated(true)
                        .hscroll(true)
                        .vscroll(true)
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            egui_extras::TableBuilder::new(ui)
                                .resizable(true)
                                .vscroll(true)
                                .striped(true)
                                .auto_shrink([false, false])
                                .animate_scrolling(true)
                                .drag_to_scroll(false)
                                .columns(
                                    Column::initial(150.0)
                                        .at_most(400.0)
                                        .resizable(true)
                                        .clip(true),
                                    table.headers.len(),
                                )
                                .header(20.0, |mut header| {
                                    for header_cell in &table.headers {
                                        header.col(|ui| {
                                            ui.heading(header_cell);
                                        });
                                    }
                                })
                                .body(|body| {
                                    body.rows(30.0, table.rows.len(), |mut row| {
                                        let row_index = row.index();
                                        if let Some(r) = table.rows.get(row_index) {
                                            for cell in r {
                                                row.col(|ui| {
                                                    ui.add(egui::Label::new(cell).truncate());
                                                });
                                            }
                                        }
                                    });
                                });
                        });
                }
            }
            _ => {}
        }
    }
}

impl eframe::App for ExtendedAppState {
    /*fn persist_egui_memory(&self) -> bool {
        todo!()
    }
    fn auto_save_interval(&self) -> Duration {
        todo!()
    }
    fn save(&mut self, _storage: &mut dyn Storage) {
        todo!()
    }
    fn on_exit(&mut self, _gl: Option<&Context>) {
        todo!()
    }
    fn clear_color(&self, _visuals: &Visuals) -> [f32; 4] {
        todo!()
    }
    fn raw_input_hook(&mut self, _ctx: &egui::Context, _raw_input: &mut RawInput) {
        todo!()
    }*/
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.ctx = ctx.clone();
        self.file_dialog.update(ctx);

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Database:");
                ui.text_edit_singleline(&mut self.shared.db);
                ui.label("User:");
                {
                    let config = self.shared.config.lock().unwrap();
                    egui::ComboBox::from_id_salt("user_select")
                        .selected_text(self.shared.user.name.clone())
                        .show_ui(ui, |ui| {
                            for cred in &config.credentials {
                                ui.selectable_value(
                                    &mut self.shared.user,
                                    cred.clone(),
                                    &cred.name,
                                );
                            }
                        });
                }
                if ui.button("Run").clicked() {
                    if let Err(e) = shared::run_query(&mut self.shared) {
                        error!("Error running query: {}", e);
                    }
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.set_min_height(720.0);
            ui.set_min_width(1280.0);
            // ui.set_height(1080.0); // Remove fixed size to allow resizing
            // ui.set_width(1920.0);  // Remove fixed size to allow resizing
            let mut state = self.dock_state.clone();
            egui_dock::DockArea::new(&mut state)
                .tab_context_menus(true)
                .show_tab_name_on_hover(true)
                .secondary_button_context_menu(true)
                .draggable_tabs(true)
                .allowed_splits(egui_dock::AllowedSplits::All)
                .show_secondary_button_hint(true)
                .show_leaf_collapse_buttons(false)
                .show_leaf_close_all_buttons(false)
                .show_close_buttons(false)
                .show_add_buttons(false)
                .show_inside(ui, self);
            self.dock_state = state.clone();
        });
    }
}

pub fn main_gui(file_content: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut state: ExtendedAppState = Default::default();
    if file_content.is_empty() {
        state.shared.sql_query = file_content;
    }
    let persistence_path = format!("{}/persistence.json", shared::get_config_base_path());
    let mut native_options = eframe::NativeOptions::default();
    native_options.centered = true;
    native_options.persist_window = true;
    native_options.persistence_path = Some(std::path::PathBuf::from(persistence_path));
    native_options.hardware_acceleration = eframe::HardwareAcceleration::Preferred;
    native_options.vsync = true;
    let mut viewport = ViewportBuilder::default();
    viewport.resizable = Some(true);
    viewport.decorations = Some(true);
    viewport.transparent = Some(true);
    viewport.min_inner_size = Some(egui::Vec2::new(1280.0, 720.0));
    viewport.inner_size = Some(egui::Vec2::new(1920.0, 1080.0));
    viewport.window_level = Some(egui::WindowLevel::Normal);
    viewport.window_type = Some(egui::X11WindowType::Utility);
    viewport.app_id = Some(format!("app.comboompunktsucht.{}", env!("CARGO_PKG_NAME")));
    viewport.title = Some(env!("CARGO_PKG_NAME").to_string());
    viewport.movable_by_window_background = Some(false);
    viewport.clamp_size_to_monitor_size = Some(false);
    viewport.close_button = Some(true);
    viewport.maximize_button = Some(true);
    viewport.minimize_button = Some(true);
    viewport.drag_and_drop = Some(false);
    viewport.taskbar = Some(true);
    viewport.title_shown = Some(true);
    viewport.titlebar_shown = Some(true);
    viewport.titlebar_buttons_shown = Some(true);
    native_options.viewport = viewport;
    eframe::run_native(
        env!("CARGO_PKG_NAME"),
        native_options,
        Box::new(|cc| {
            Ok(Box::new(ExtendedAppState {
                shared: state.shared,
                ctx: cc.egui_ctx.clone(),
                ..Default::default()
            }))
        }),
    )?;
    Ok(())
}

fn theme() -> ColorTheme {
    let mut theme = ColorTheme::default();
    theme.bg = NordColor::Nord0.as_str();
    theme.name = "Nord";
    theme.cursor = NordColor::Nord6.as_str();
    theme.comments = NordColor::Nord1.as_str();
    theme.functions = NordColor::Nord13.as_str();
    theme.keywords = NordColor::Nord11.as_str();
    theme.literals = NordColor::Nord5.as_str();
    theme.numerics = NordColor::Nord15.as_str();
    theme.punctuation = NordColor::Nord12.as_str();
    theme.special = NordColor::Nord7.as_str();
    theme.strs = NordColor::Nord14.as_str();
    theme.types = NordColor::Nord13.as_str();
    theme.dark = true;
    theme.selection = NordColor::Nord3.as_str();

    theme
}
