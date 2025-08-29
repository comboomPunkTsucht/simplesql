// Copyright (c) 2025 mcpeaps_HD
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use json5::*;
#[cfg(test)]
use serde::*;
use serde_json::*;
#[allow(unused_imports)]
use sqlx::{
    any::{AnyPoolOptions, AnyQueryResult, AnyRow}, mysql::{MySqlPoolOptions, MySqlQueryResult, MySqlRow}, postgres::{PgPoolOptions, PgQueryResult, PgRow}, Any, Column,
    MySql,
    Postgres,
    Row,
};
use std::ops::Deref;
use std::path::Path;
#[allow(unused_imports)]
use std::time::SystemTime;
#[allow(unused_imports)]
use std::{
    fs,
    fs::{create_dir_all, remove_dir_all, remove_file, File},
    io::{Read, Write},
    result::Result,
};
use toml::*;
use tui_logger::TuiLoggerFile;
#[allow(unused_imports)]
use widgetui::State;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Tab {
    SqlEditor,
    TableView,
    LogViewer,
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
            2 => Tab::LogViewer,
            _ => panic!("Invalid tab index"),
        }
    }
    #[allow(dead_code)]
    pub fn to_index(self) -> usize {
        match self {
            Tab::SqlEditor => 0,
            Tab::TableView => 1,
            Tab::LogViewer => 2,
        }
    }
}
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Connection {
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub host: String,
    pub port: u16,
}
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Credential {
    pub name: String,
    pub connection: String,
    pub username: String,
    pub password: String,
}
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub schema: Option<String>,
    pub connections: Vec<Connection>,
    pub credentials: Vec<Credential>,
}
#[allow(dead_code)]
pub enum RawRow {
    MySql(Vec<MySqlRow>),
    Postgres(Vec<PgRow>),
    Any(Vec<AnyRow>),
}

#[allow(dead_code)]
pub enum RawData {
    MySql(MySqlQueryResult),
    Postgres(PgQueryResult),
    Any(AnyQueryResult),
}

#[allow(dead_code)]
pub struct Table {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub raw_data: Option<RawData>,
}
impl Clone for Table {
    fn clone(&self) -> Self {
        Table {
            headers: self.headers.clone(),
            rows: self.rows.clone(),
            raw_data: None,
        }
    }
    fn clone_from(&mut self, source: &Self) {
        self.headers.clone_from(&source.headers);
        self.rows.clone_from(&source.rows);
        self.raw_data = None;
    }
}
impl Table {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn from_raw_row(raw_row: RawRow, raw_data: RawData) -> Self {
        let mut headers = Vec::new();
        let mut rows = Vec::new();

        match raw_row {
            RawRow::MySql(row_vec) => {
                if let Some(first_row) = row_vec.first() {
                    headers = first_row
                        .columns()
                        .iter()
                        .map(|col| col.name().to_string())
                        .collect();

                    for row in &row_vec {
                        let values: Vec<String> = headers
                            .iter()
                            .map(|h| {
                                row.try_get::<Option<String>, _>(h.as_str())
                                    .unwrap_or(None)
                                    .unwrap_or_else(|| "NULL".to_string())
                            })
                            .collect();
                        rows.push(values);
                    }
                }
            }
            RawRow::Postgres(row_vec) => {
                if let Some(first_row) = row_vec.first() {
                    headers = first_row
                        .columns()
                        .iter()
                        .map(|col| col.name().to_string())
                        .collect();

                    for row in &row_vec {
                        let values: Vec<String> = headers
                            .iter()
                            .map(|h| {
                                row.try_get::<Option<String>, _>(h.as_str())
                                    .unwrap_or(None)
                                    .unwrap_or_else(|| "NULL".to_string())
                            })
                            .collect();
                        rows.push(values);
                    }
                }
            }
            RawRow::Any(row_vec) => {
                if let Some(first_row) = row_vec.first() {
                    headers = first_row
                        .columns()
                        .iter()
                        .map(|col| col.name().to_string())
                        .collect();

                    for row in &row_vec {
                        let values: Vec<String> = headers
                            .iter()
                            .map(|h| {
                                row.try_get::<Option<String>, _>(h.as_str())
                                    .unwrap_or(None)
                                    .unwrap_or_else(|| "NULL".to_string())
                            })
                            .collect();
                        rows.push(values);
                    }
                }
            }
        }
        Table {
            headers,
            rows,
            raw_data: Some(raw_data),
        }
    }
}
impl Default for Table {
    fn default() -> Self {
        Table {
            headers: Vec::new(),
            rows: Vec::new(),
            raw_data: None,
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, State)]
pub struct AppState {
    pub current_tab: Tab,
    pub config: Config,
    pub sql_query: String,
    pub user: Credential,
    pub table: Table, // Optional table name for TableView tab
    pub db: String,
}

impl Default for AppState {
    fn default() -> Self {
        let config: Config = get_config();
        let user: Credential = config.credentials[0].clone();
        AppState {
            current_tab: Tab::default(),
            config,
            sql_query: String::from("select * from data;"),
            user,
            table: Table::default(),
            db: String::from("bewerbungen"),
        }
    }
}
#[allow(dead_code)]
impl AppState {
    pub fn set_next_user(&mut self) {
        if self.config.credentials.is_empty() {
            panic!("No credentials available in config");
        }
        let current_index = self
            .config
            .credentials
            .iter()
            .position(|c| c.name == self.user.name)
            .unwrap_or(0);
        let next_index = (current_index + 1) % self.config.credentials.len();
        self.user = self.config.credentials[next_index].clone();
    }
}

fn get_config() -> Config {
    let toml_path = format!("{}/config.toml", get_config_base_path());
    let config: Config;

    // TOML bevorzugt
    if Path::new(&toml_path).exists() {
        let content = fs::read_to_string(&toml_path).expect("Failed to read config.toml");
        config = toml::from_str(&content).expect("Invalid TOML in config file");
    } else {
        // Fallback: Defaults
        config = toml::from_str(&get_config_defaults()).expect("Default config is invalid");
    }
    config
}

fn get_config_base_path() -> String {
    match std::env::consts::OS {
        "linux" | "macos" | "freebsd" => {
            format!("{}/.simplesql", std::env::var("HOME").unwrap())
        }
        "windows" => format!("{}/.simplesql", std::env::var("APPDATA").unwrap()),
        _ => panic!("Unsupported platform"),
    }
}
fn get_config_path() -> String {
    format!("{}/config.toml", get_config_base_path())
}

fn get_log_path() -> String {
    format!("{}/output.log", get_config_base_path())
}

pub fn setup_logger(is_tui: bool) -> Result<(), fern::InitError> {
    let log_level = if cfg!(debug_assertions) {
        log::LevelFilter::Trace
    } else {
        log::LevelFilter::Info
    };
    if is_tui {
        tui_logger::init_logger(log_level).unwrap();
        tui_logger::set_default_level(log_level);
        tui_logger::set_log_file(TuiLoggerFile::new(get_log_path().as_str()));
    } else {
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
            .level(log_level)
            .chain(std::io::stdout())
            .chain(fern::log_file(get_log_path())?)
            .apply()?;
    }
    Ok(())
}
fn get_config_defaults() -> String {
    r#"
[[connections]]
  name = "Local mariaDB"
  type = "mariadb"
  host = "localhost"
  port = 3306

[[connections]]
  name = "Local MySQL"
  type = "mysql"
  host = "localhost"
  port = 3306

[[connections]]
  name = "Local PostgreSQL"
  type = "postgresql"
  host = "localhost"
  port = 5432

[[credentials]]
  name = "mysql_default"
  connection = "Local mariaDB"
  username = "root"
  password = ""

[[credentials]]
  name = "postgresql_default"
  connection = "Local PostgreSQL"
  username = "postgres"
  password = ""
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
    if !Path::new(&config_path).exists() {
        let mut file = File::create(&config_path)?;
        file.write_all(get_config_defaults().as_bytes())?;
    }

    Ok(())
}

pub fn get_config_content(state: &mut AppState) -> std::io::Result<String> {
    let mut f = File::open(get_config_path()).unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    if buffer.is_empty() {
        buffer = get_config_defaults();
    }

    state.config = get_config();
    Ok(buffer)
}
pub fn set_config_content(buffer: String) -> std::io::Result<()> {
    // immer TOML schreiben
    let config: Config = toml::from_str(&buffer).or_else(|_| serde_json::from_str(&buffer))?;
    let toml_str = toml::to_string_pretty(&config).expect("Failed to serialize config to TOML");
    let mut f = File::create(get_config_path())?;
    f.write_all(toml_str.as_bytes())?;
    Ok(())
}
pub fn gen_log_file() -> std::io::Result<()> {
    let mut f = File::create(get_log_path())?;
    f.write_all(String::new().as_bytes())?;
    Ok(())
}
#[allow(dead_code)]
pub fn write_file(path: &str, content: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
#[allow(dead_code)]
pub fn read_file(path: &str) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

#[tokio::main]
pub async fn run_query(state: &mut AppState) -> Result<(), sqlx::Error> {
    sqlx::any::install_default_drivers();
    let connection: &Connection = state
        .config
        .connections
        .iter()
        .find(|c| c.name == state.user.connection)
        .expect("Connection not found");
    match connection.r#type.as_str() {
        "mariadb" | "mysql" => {
            let pool = MySqlPoolOptions::new()
                .max_connections(10)
                .connect(&format!(
                    "mysql://{}:{}@{}:{}/{}",
                    state.user.username,
                    state.user.password,
                    (*connection).host,
                    (*connection).port,
                    state.db
                ))
                .await?;
            let rows = sqlx::query(&state.sql_query).fetch_all(&pool).await?;
            state.table = Table::from_raw_row(
                RawRow::MySql(rows),
                RawData::MySql(sqlx::query(&state.sql_query).execute(&pool).await?),
            );
        }
        "postgres" => {
            let pool = PgPoolOptions::new()
                .max_connections(10)
                .connect(&format!(
                    "postgres://{}:{}@{}:{}/{}",
                    state.user.username,
                    state.user.password,
                    (*connection).host,
                    (*connection).port,
                    state.db
                ))
                .await?;
            let rows = sqlx::query(&state.sql_query).fetch_all(&pool).await?;
            state.table = Table::from_raw_row(
                RawRow::Postgres(rows),
                RawData::Postgres(sqlx::query(&state.sql_query).execute(&pool).await?),
            );
        }
        _ => {
            let pool = AnyPoolOptions::new()
                .max_connections(10)
                .connect(&format!(
                    "{}://{}:{}@{}:{}/{}",
                    (*connection).r#type,
                    state.user.username,
                    state.user.password,
                    (*connection).host,
                    (*connection).port,
                    state.db
                ))
                .await?;
            let rows = sqlx::query(&state.sql_query).fetch_all(&pool).await?;
            state.table = Table::from_raw_row(
                RawRow::Any(rows),
                RawData::Any(sqlx::query(&state.sql_query).execute(&pool).await?),
            );
        }
    }

    Ok(())
}

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
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

// tests

#[allow(dead_code)]
fn load_user_config() -> String {
    let mut state = AppState::default();
    get_config_content(&mut state).unwrap_or(get_config_defaults())
}

#[allow(dead_code)]
fn save_user_config(config: String) {
    set_config_content(config).unwrap();
}

#[allow(dead_code)]
fn cleanup() {
    let base = get_config_base_path();
    let _ = remove_dir_all(&base);
}

#[test]
fn test_check_and_gen_config_creates_files() {
    let user_config = load_user_config();
    cleanup();
    assert!(check_and_gen_config().is_ok());
    assert!(Path::new(&get_config_path()).exists());
    assert!(Path::new(&get_log_path()).exists());
    save_user_config(user_config);
}

#[test]
fn test_get_and_set_config_content() {
    let user_config = load_user_config();
    cleanup();
    check_and_gen_config().unwrap();
    let mut state = AppState::default();
    let original = get_config_content(&mut state).unwrap();
    let new_content = original.replace("Local mariaDB", "Test mariaDB");
    assert!(set_config_content(new_content.clone()).is_ok());
    let read_back = get_config_content(&mut state).unwrap();
    assert!(read_back.contains("Test mariaDB"));
    // Rücksetzen nur, wenn das Original gültiges JSON ist
    if serde_json::from_str::<Config>(original.as_str()).is_ok() {
        set_config_content(original).unwrap();
    } else {
        panic!("Original-Konfiguration ist ungültig und kann nicht zurückgesetzt werden!");
    }
    save_user_config(user_config);
}

#[test]
fn test_gen_log_file_overwrites() {
    let user_config = load_user_config();
    cleanup();
    check_and_gen_config().unwrap();
    let log_path = get_log_path();
    fs::write(&log_path, "testlog").unwrap();
    assert!(gen_log_file().is_ok());
    let content = fs::read_to_string(&log_path).unwrap();
    assert!(content.is_empty());
    save_user_config(user_config);
}

#[test]
fn test_app_state_default_user() {
    let user_config = load_user_config();
    cleanup();
    check_and_gen_config().unwrap();
    let state = AppState::default();
    assert!(!state.user.name.is_empty());
    assert_eq!(state.current_tab.to_index(), 0);
    assert!(state.sql_query.contains("select"));
    save_user_config(user_config);
}

#[test]
fn test_tab_index_conversion() {
    assert_eq!(Tab::from_index(0), Tab::SqlEditor);
    assert_eq!(Tab::from_index(1), Tab::TableView);
    assert_eq!(Tab::SqlEditor.to_index(), 0);
    assert_eq!(Tab::TableView.to_index(), 1);
}

#[test]
fn test_nordcolor_value_and_string() {
    let c = NordColor::Nord0;
    assert_eq!(c.value(), 0x2e3440ff);
    assert!(c.to_string().starts_with("#"));
    assert!(c.as_str().starts_with("#"));
}
