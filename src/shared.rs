// Copyright (c) 2025 mcpeaps_HD
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::{fmt::format, fs::{File, create_dir_all}, io::{Read, Write}};

use egui::TextBuffer;

#[derive(Clone, Copy)]
pub enum Tab {
  SqlEditor,
  TableView,
  CredentialsEditor,
  ConnectionsEditor,
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
      2 => Tab::CredentialsEditor,
      3 => Tab::ConnectionsEditor,
      4 => Tab::RunLog,
      _ => panic!("Invalid tab index"),
    }
  }
  #[allow(dead_code)]
  pub fn to_index(self) -> usize {
    match self {
      Tab::SqlEditor => 0,
      Tab::TableView => 1,
      Tab::CredentialsEditor => 2,
      Tab::ConnectionsEditor => 3,
      Tab::RunLog => 4,
    }
  }
}


fn get_config_base_path() -> String {
  match std::env::consts::OS {
    "linux" | "macos" | "freebsd" => format!("{}/.simplesql", std::env::var("HOME").unwrap()),
    "windows" => format!("{}/.simplesql", std::env::var("APPDATA").unwrap()),
    _ => panic!("Unsupported platform"),
  }
}
fn get_credential_path() -> String {
  format!("{}/credential.toml", get_config_base_path())
}

fn get_connections_path() -> String {
  format!("{}/connections.toml", get_config_base_path())
}

fn get_credential_defaults() -> String {
  r#"#test
"#.to_string()
}

fn get_connections_defaults() -> String {
  r#"#test
"#.to_string()
}


pub fn check_and_gen_config() -> std::io::Result<()> {
  // 1. Check and create config directory if it doesn't exist
  let config_path = get_config_base_path();
  create_dir_all(&config_path)?;

  // 2. Check and create credential file if it doesn't exist
  let credential_path = get_credential_path();
  if !std::path::Path::new(&credential_path).exists() {
      let mut file = File::create(&credential_path)?;
      file.write_all(get_credential_defaults().as_bytes())?;
  }

  // 3. Check and create connections file if it doesn't exist
  let connections_path = get_connections_path();
  if !std::path::Path::new(&connections_path).exists() {
      let mut file = File::create(&connections_path)?;
      file.write_all(get_connections_defaults().as_bytes())?;
  }

  Ok(())
}

pub fn get_credential_content() -> std::io::Result<String> {
    let mut f = File::open(get_credential_path())?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    Ok(buffer)
}

pub fn get_connections_content() -> std::io::Result<String> {
    let mut f = File::open(get_connections_path())?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    Ok(buffer)
}

pub fn set_credential_content(buffer: String) -> std::io::Result<()> {
    let mut f = File::create(get_credential_path())?;
    f.write_all(buffer.as_bytes())?;
    Ok(())
}

pub fn set_connections_content(buffer: String) -> std::io::Result<()> {
    let mut f = File::create(get_connections_path())?;
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
  #[allow(dead_code)]
  pub fn to_string(&self) -> String {
    match self {
      NordColor::Nord0 => "#2e3440".to_string(),
      NordColor::Nord1 => "#3b4252".to_string(),
      NordColor::Nord2 => "#434c5e".to_string(),
      NordColor::Nord3 => "#4c566a".to_string(),
      NordColor::Nord4 => "#d8dee9".to_string(),
      NordColor::Nord5 => "#e5e9f0".to_string(),
      NordColor::Nord6 => "#eceff4".to_string(),
      NordColor::Nord7 => "#8fbcbb".to_string(),
      NordColor::Nord8 => "#88c0d0".to_string(),
      NordColor::Nord9 => "#81a1c1".to_string(),
      NordColor::Nord10 => "#5e81ac".to_string(),
      NordColor::Nord11 => "#bf616a".to_string(),
      NordColor::Nord12 => "#d08770".to_string(),
      NordColor::Nord13 => "#ebcb8b".to_string(),
      NordColor::Nord14 => "#a3be8c".to_string(),
      NordColor::Nord15 => "#b48ead".to_string(),
    }
  }
  #[allow(dead_code)]
  pub fn as_str(&self) -> &'static str {
    match self {
      NordColor::Nord0 => "#2e3440ff",
      NordColor::Nord1 => "#3b4252ff",
      NordColor::Nord2 => "#434c5eff",
      NordColor::Nord3 => "#4c566aff",
      NordColor::Nord4 => "#d8dee9ff",
      NordColor::Nord5 => "#e5e9f0ff",
      NordColor::Nord6 => "#eceff4ff",
      NordColor::Nord7 => "#8fbcbbff",
      NordColor::Nord8 => "#88c0d0ff",
      NordColor::Nord9 => "#81a1c1ff",
      NordColor::Nord10 => "#5e81acff",
      NordColor::Nord11 => "#bf616aff",
      NordColor::Nord12 => "#d08770ff",
      NordColor::Nord13 => "#ebcb8bff",
      NordColor::Nord14 => "#a3be8cff",
      NordColor::Nord15 => "#b48eadff",
    }
  }

}