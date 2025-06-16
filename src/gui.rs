use crate::shared;
use crate::shared::{AppState, NordColor, NORDCOLOR_NORD0, NORDCOLOR_NORD14};
use log::{debug, error, info, trace, warn};
use slint;
use std::time::SystemTime;

slint::slint! {
  import { TabWidget } from "std-widgets.slint";
  global Colors {
    // Polar Night
    out property <color> nord0: #2e3440;
    out property <color> nord1: #3b4252;
    out property <color> nord2: #434c5e;
    out property <color> nord3: #4c566a;
    // Snow Storm
    out property <color> nord4: #d8dee9;
    out property <color> nord5: #e5e9f0;
    out property <color> nord6: #eceff4;
    // Frost
    out property <color> nord7: #8fbcbb;
    out property <color> nord8: #88c0d0;
    out property <color> nord9: #81a1c1;
    out property <color> nord10: #5e81ac;
    // Aurora
    out property <color> nord11: #bf616a;
    out property <color> nord12: #d08770;
    out property <color> nord13: #ebcb8b;
    out property <color> nord14: #a3be8c;
    out property <color> nord15: #b48ead;
  }
  export component SimplesqlGUI inherits Window {
    title: "simplesql";
    min-width: 1280px;
    min-height: 720px;
    background: Colors.nord0;
    MenuBar {
      Menu {
        title: @tr("File");
        MenuItem {
          title: @tr("New");
          activated => { file_new(); }
        }
        MenuItem {
          title: @tr("Open");
          activated => { file_open(); }
        }
      }
      Menu {
        title: @tr("Edit");
        MenuItem {
          title: @tr("Copy");
        }
        MenuItem {
          title: @tr("Paste");
        }
        MenuSeparator {}
        Menu {
          title: @tr("Find");
          MenuItem {
            title: @tr("Find in document...");
          }
          MenuItem {
            title: @tr("Find Next");
          }
          MenuItem {
            title: @tr("Find Previous");
          }
        }
      }
    }
    TabWidget {
      Tab {
        title: "test";
        Text {
          text: "test";
          color: Colors.nord14;
        }
      }
      Tab {
        title: "test";
        Text {
          text: "test";
          color: Colors.nord14;
        }
      }
      Tab {
        title: "test";
        Text {
          text: "test";
          color: Colors.nord14;
        }
      }
      Tab {
        title: "test";
        Text {
          text: "test";
          color: Colors.nord14;
        }
      }
    }
    callback file_new();
    callback file_open();
  }
}

impl NordColor {
    #[allow(dead_code)]
    pub fn to_color(&self) -> slint::Color {
        let value = self.value();
        let r = ((value >> 24) & 0xFF) as u8;
        let g = ((value >> 16) & 0xFF) as u8;
        let b = ((value >> 8) & 0xFF) as u8;
        let a = (value & 0xFF) as u8;
        slint::Color::from_argb_u8(a, r, g, b)
    }
}

pub fn main_gui() -> Result<(), Box<dyn std::error::Error>> {
    let gui = SimplesqlGUI::new().unwrap();
    gui.on_file_new(file_new);
    gui.on_file_open(file_open);
    gui.run().unwrap();
    Ok(())
}

fn file_new() {
    log::info!("new");
}

fn file_open() {
    log::info!("open");
}
