// Copyright (c) 2025 mcpeaps_HD
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::{
    fmt::format,
    fs::{File, create_dir_all},
    io::{Read, Write},
};
use widgetui::State;

use egui::TextBuffer;

#[derive(Clone, Copy)]
pub enum Tab {
    SqlEditor,
    TableView,
    ConfigEditor,
    RunLog,
}
impl Default for Tab {
    fn default() -> Self {
        Tab::SqlEditor
    }
}

impl Tab {
    #[allow(dead_code)]
    pub fn from_index(index: usize) -> Self {
        match index {
            0 => Tab::SqlEditor,
            1 => Tab::TableView,
            2 => Tab::ConfigEditor,
            3 => Tab::RunLog,
            _ => panic!("Invalid tab index"),
        }
    }
    #[allow(dead_code)]
    pub fn to_index(self) -> usize {
        match self {
            Tab::SqlEditor => 0,
            Tab::TableView => 1,
            Tab::ConfigEditor => 2,
            Tab::RunLog => 3,
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, State)]
pub struct AppState {
    pub current_tab: Tab,
    pub sql_query: String,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            current_tab: Tab::default(),
            sql_query: String::from("select * from test;"),
        }
    }
}
#[allow(dead_code)]
impl AppState {}

fn get_config_base_path() -> String {
    match std::env::consts::OS {
        "linux" | "macos" | "freebsd" => format!("{}/.simplesql", std::env::var("HOME").unwrap()),
        "windows" => format!("{}/.simplesql", std::env::var("APPDATA").unwrap()),
        _ => panic!("Unsupported platform"),
    }
}
fn get_config_path() -> String {
    format!("{}/config.jsonc", get_config_base_path())
}

fn get_config_defaults() -> String {
    r#"{
  "$schema": "https://raw.githubusercontent.com/comboomPunkTsucht/simplesql/main/src/simplesql_config.json",
  "connections": [
    {
      "name": "Local mariaDB",
      "type": "mariadb",
      "host": "localhost",
      "port": 3306
    },
    {
      "name": "Local MySQL",
      "type": "mysql",
      "host": "localhost",
      "port": 3306
    },
    {
      "name": "Local PostgreSQL",
      "type": "postgresql",
      "host": "localhost",
      "port": 5432
    }
  ],
  //test
  "credentials": [
    {
      "name": "mysql_default",
      "connection": "Local mariaDB",
      "username": "root",
      "password": ""
    },
    {
      "name": "postgresql_default",
      "connection": "Local PostgreSQL",
      "username": "postgres",
      "password": ""
    }
  ]
}
"#
    .to_string()
}

pub fn check_and_gen_config() -> std::io::Result<()> {
    // 1. Check and create config directory if it doesn't exist
    let config_base_path = get_config_base_path();
    create_dir_all(&config_base_path)?;

    // 2. Check and create credential file if it doesn't exist
    let config_path = get_config_path();
    if !std::path::Path::new(&config_path).exists() {
        let mut file = File::create(&config_path)?;
        file.write_all(get_config_defaults().as_bytes())?;
    }

    Ok(())
}

pub fn get_config_content() -> std::io::Result<String> {
    let mut f = File::open(get_config_path())?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    Ok(buffer)
}

pub fn set_config_content(buffer: String) -> std::io::Result<()> {
    let mut f = File::create(get_config_path())?;
    f.write_all(buffer.as_bytes())?;
    Ok(())
}

#[allow(dead_code)]
pub enum NordColor {
    // Polar Night
    Nord0 = 0x2e3440ff,
    Nord1 = 0x3b4252ff,
    Nord2 = 0x434c5eff,
    Nord3 = 0x4c566aff,
    // Snow Storm
    Nord4 = 0xd8dee9ff,
    Nord5 = 0xe5e9f0ff,
    Nord6 = 0xeceff4ff,
    // Frost
    Nord7 = 0x8fbcbbff,
    Nord8 = 0x88c0d0ff,
    Nord9 = 0x81a1c1ff,
    Nord10 = 0x5e81acff,
    // Aurora
    Nord11 = 0xbf616aff,
    Nord12 = 0xd08770ff,
    Nord13 = 0xebcb8bff,
    Nord14 = 0xa3be8cff,
    Nord15 = 0xb48eadff,
}

impl NordColor {
    pub fn value(&self) -> u32 {
        match self {
            NordColor::Nord0 => NordColor::Nord0 as u32,
            NordColor::Nord1 => NordColor::Nord1 as u32,
            NordColor::Nord2 => NordColor::Nord2 as u32,
            NordColor::Nord3 => NordColor::Nord3 as u32,
            NordColor::Nord4 => NordColor::Nord4 as u32,
            NordColor::Nord5 => NordColor::Nord5 as u32,
            NordColor::Nord6 => NordColor::Nord6 as u32,
            NordColor::Nord7 => NordColor::Nord7 as u32,
            NordColor::Nord8 => NordColor::Nord8 as u32,
            NordColor::Nord9 => NordColor::Nord9 as u32,
            NordColor::Nord10 => NordColor::Nord10 as u32,
            NordColor::Nord11 => NordColor::Nord11 as u32,
            NordColor::Nord12 => NordColor::Nord12 as u32,
            NordColor::Nord13 => NordColor::Nord13 as u32,
            NordColor::Nord14 => NordColor::Nord14 as u32,
            NordColor::Nord15 => NordColor::Nord15 as u32,
        }
    }

    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        format!("#{:08x}", self.value())
    }

    #[allow(dead_code)]
    pub fn as_str(&self) -> &'static str {
        Box::leak(self.to_string().into_boxed_str())
    }
}

