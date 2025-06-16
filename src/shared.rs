// Copyright (c) 2025 mcpeaps_HD
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
#[allow(unused_imports)]
use json;
#[allow(unused_imports)]
use std::time::SystemTime;
#[allow(unused_imports)]
use std::{
    fs::{create_dir_all, File},
    io::{Read, Write},
};
#[allow(unused_imports)]
use widgetui::State;

#[derive(Clone, Copy)]
pub enum Tab {
    SqlEditor,
    TableView,
    ConfigEditor,
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
            _ => panic!("Invalid tab index"),
        }
    }
    #[allow(dead_code)]
    pub fn to_index(self) -> usize {
        match self {
            Tab::SqlEditor => 0,
            Tab::TableView => 1,
            Tab::ConfigEditor => 2,
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, State)]
pub struct AppState {
    pub current_tab: Tab,
    pub config: json::JsonValue,
    pub sql_query: String,
    pub user: String,
}

impl Default for AppState {
    fn default() -> Self {
        let config = json::parse(&get_config_defaults()).unwrap();
        let users = config["credentials"].members().collect::<Vec<_>>();
        let mut user = String::new();
        if !users.is_empty() {
            user = users[0]["name"].to_string();
        }
        AppState {
            current_tab: Tab::default(),
            config,
            sql_query: String::from("select * from test;"),
            user,
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

fn get_log_path() -> String {
    format!("{}/output.log", get_config_base_path())
}

pub fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}],[{}]-{} - {}",
                record.level(),
                record.target(),
                humantime::format_rfc3339_seconds(SystemTime::now()),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(fern::log_file(get_log_path())?)
        .apply()?;
    Ok(())
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
    gen_log_file()?;
    // 2. Check and create credential file if it doesn't exist
    let config_path = get_config_path();
    if !std::path::Path::new(&config_path).exists() {
        let mut file = File::create(&config_path)?;
        file.write_all(get_config_defaults().as_bytes())?;
    }

    Ok(())
}

pub fn get_config_content(state: &mut AppState) -> std::io::Result<String> {
    let mut f = File::open(get_config_path())?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;

    state.config = json::parse(&buffer).unwrap();
    //println!("{}", json::stringify_pretty(state.config.clone(), 2));
    Ok(buffer)
}

pub fn set_config_content(buffer: String) -> std::io::Result<()> {
    let mut f = File::create(get_config_path())?;
    f.write_all(buffer.as_bytes())?;
    Ok(())
}
pub fn gen_log_file() -> std::io::Result<()> {
    let mut f = File::create(get_log_path())?;
    f.write_all(String::new().as_bytes())?;
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
#[allow(dead_code)]
pub const NORDCOLOR_NORD0: NordColor = NordColor::Nord0;
#[allow(dead_code)]
pub const NORDCOLOR_NORD1: NordColor = NordColor::Nord1;
#[allow(dead_code)]
pub const NORDCOLOR_NORD2: NordColor = NordColor::Nord2;
#[allow(dead_code)]
pub const NORDCOLOR_NORD3: NordColor = NordColor::Nord3;
#[allow(dead_code)]
pub const NORDCOLOR_NORD4: NordColor = NordColor::Nord4;
#[allow(dead_code)]
pub const NORDCOLOR_NORD5: NordColor = NordColor::Nord5;
#[allow(dead_code)]
pub const NORDCOLOR_NORD6: NordColor = NordColor::Nord6;
#[allow(dead_code)]
pub const NORDCOLOR_NORD7: NordColor = NordColor::Nord7;
#[allow(dead_code)]
pub const NORDCOLOR_NORD8: NordColor = NordColor::Nord8;
#[allow(dead_code)]
pub const NORDCOLOR_NORD9: NordColor = NordColor::Nord9;
#[allow(dead_code)]
pub const NORDCOLOR_NORD10: NordColor = NordColor::Nord10;
#[allow(dead_code)]
pub const NORDCOLOR_NORD11: NordColor = NordColor::Nord11;
#[allow(dead_code)]
pub const NORDCOLOR_NORD12: NordColor = NordColor::Nord12;
#[allow(dead_code)]
pub const NORDCOLOR_NORD13: NordColor = NordColor::Nord13;
#[allow(dead_code)]
pub const NORDCOLOR_NORD14: NordColor = NordColor::Nord14;
#[allow(dead_code)]
pub const NORDCOLOR_NORD15: NordColor = NordColor::Nord15;
