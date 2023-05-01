
use crate::app::AppToggleEvent;

use std::{
  collections::HashMap,
  path::Path,
  fs::{
    File,
    self
  }
};

use bevy::prelude::*;

use inputbot::KeybdKey;

pub struct KeybindPlugin;

impl Plugin for KeybindPlugin {

  fn build(&self, app: &mut App) {
    app
      .add_system(await_input)
      .add_startup_system(setup);
  }

}

#[derive(Component)]
struct KeybindComponent {
  binds: Vec<&'static KeybdKey>,
  status: usize
}

fn setup(mut commands: Commands) {
  
  let mut component: KeybindComponent = KeybindComponent { 
    binds: vec![],
    status: 0
  };

  let config_path: &Path = Path::new("./keybind.cfg");
  if config_path.exists() && config_path.is_file() {
    match fs::read_to_string(config_path) {
      Ok(config_content) => {
        for line in config_content.lines() {
          for bind in line.replace(" ", "").trim().split(',') {

            if bind.is_empty() || !bind.is_ascii() {
              continue;
            }

            if let Some(key) = get_key_from_string(bind.to_string()) {
              component.binds.push(key);
            }

          }
        }
        println!("[{}::keybind::setup] // Interpreted from Keybind-Config.", env!("CARGO_PKG_NAME"));
      },
      _ => {
        println!("[{}::keybind::setup] // Couldn't read from Keybind-Config.", env!("CARGO_PKG_NAME"));
      }
    }
  } else {
    File::create(config_path).unwrap();
    println!("[{}::keybind::setup] // Created Keybind-Config-File.", env!("CARGO_PKG_NAME"));
  }

  commands.spawn_empty().insert(component);

}

fn await_input(
  mut app_toggler: EventWriter<AppToggleEvent>,
  mut component_query: Query<&mut KeybindComponent>
) {

  let mut component = component_query.single_mut();

  let mut latest_status: usize = 0;
  for bind in &component.binds {
    if bind.is_pressed() {
      latest_status += 1;
    }
  }

  if component.binds.len() <= latest_status && latest_status > component.status  {
    app_toggler.send(AppToggleEvent);
    println!("[{}::keybind::await_input] // AppToggleEvent sent.", env!("CARGO_PKG_NAME"));
  }

  component.status = latest_status;

}

fn get_key_map() -> HashMap<String, &'static KeybdKey> {

  let mut keys: HashMap<String, &KeybdKey> = HashMap::new();
  keys.insert("A".to_string(), &KeybdKey::AKey);
  keys.insert("B".to_string(), &KeybdKey::BKey);
  keys.insert("C".to_string(), &KeybdKey::CKey);
  keys.insert("D".to_string(), &KeybdKey::DKey);
  keys.insert("E".to_string(), &KeybdKey::EKey);
  keys.insert("F".to_string(), &KeybdKey::FKey);
  keys.insert("G".to_string(), &KeybdKey::GKey);
  keys.insert("H".to_string(), &KeybdKey::HKey);
  keys.insert("I".to_string(), &KeybdKey::IKey);
  keys.insert("J".to_string(), &KeybdKey::JKey);
  keys.insert("K".to_string(), &KeybdKey::KKey);
  keys.insert("L".to_string(), &KeybdKey::LKey);
  keys.insert("M".to_string(), &KeybdKey::MKey);
  keys.insert("N".to_string(), &KeybdKey::NKey);
  keys.insert("O".to_string(), &KeybdKey::OKey);
  keys.insert("P".to_string(), &KeybdKey::PKey);
  keys.insert("Q".to_string(), &KeybdKey::QKey);
  keys.insert("R".to_string(), &KeybdKey::RKey);
  keys.insert("S".to_string(), &KeybdKey::SKey);
  keys.insert("T".to_string(), &KeybdKey::TKey);
  keys.insert("U".to_string(), &KeybdKey::UKey);
  keys.insert("V".to_string(), &KeybdKey::VKey);
  keys.insert("W".to_string(), &KeybdKey::WKey);
  keys.insert("X".to_string(), &KeybdKey::XKey);
  keys.insert("Y".to_string(), &KeybdKey::YKey);
  keys.insert("Z".to_string(), &KeybdKey::ZKey);
  keys.insert("LCONTROL".to_string(), &KeybdKey::LControlKey);
  keys.insert("RCONTROL".to_string(), &KeybdKey::RControlKey);
  keys.insert("LSHIFT".to_string(), &KeybdKey::LShiftKey);
  keys.insert("RSHIFT".to_string(), &KeybdKey::RShiftKey);
  keys.insert("ENTER".to_string(), &KeybdKey::EnterKey);
  keys.insert("ESCAPE".to_string(), &KeybdKey::EscapeKey);
  keys.insert("BACKSPACE".to_string(), &KeybdKey::BackspaceKey);
  keys.insert("DELETE".to_string(), &KeybdKey::DeleteKey);
  keys.insert("SPACE".to_string(), &KeybdKey::SpaceKey);
  keys.insert("DOWN".to_string(), &KeybdKey::DownKey);
  keys.insert("UP".to_string(), &KeybdKey::UpKey);
  keys.insert("LEFT".to_string(), &KeybdKey::LeftKey);
  keys.insert("RIGHT".to_string(), &KeybdKey::RightKey);
  keys.insert("NUMPAD0".to_string(), &KeybdKey::Numpad0Key);
  keys.insert("NUMPAD1".to_string(), &KeybdKey::Numpad1Key);
  keys.insert("NUMPAD2".to_string(), &KeybdKey::Numpad2Key);
  keys.insert("NUMPAD3".to_string(), &KeybdKey::Numpad3Key);
  keys.insert("NUMPAD4".to_string(), &KeybdKey::Numpad4Key);
  keys.insert("NUMPAD5".to_string(), &KeybdKey::Numpad5Key);
  keys.insert("NUMPAD6".to_string(), &KeybdKey::Numpad6Key);
  keys.insert("NUMPAD7".to_string(), &KeybdKey::Numpad7Key);
  keys.insert("NUMPAD8".to_string(), &KeybdKey::Numpad8Key);
  keys.insert("NUMPAD9".to_string(), &KeybdKey::Numpad9Key);
  keys.insert("NUMROW0".to_string(), &KeybdKey::Numrow0Key);
  keys.insert("NUMROW1".to_string(), &KeybdKey::Numrow1Key);
  keys.insert("NUMROW2".to_string(), &KeybdKey::Numrow2Key);
  keys.insert("NUMROW3".to_string(), &KeybdKey::Numrow3Key);
  keys.insert("NUMROW4".to_string(), &KeybdKey::Numrow4Key);
  keys.insert("NUMROW5".to_string(), &KeybdKey::Numrow5Key);
  keys.insert("NUMROW6".to_string(), &KeybdKey::Numrow6Key);
  keys.insert("NUMROW7".to_string(), &KeybdKey::Numrow7Key);
  keys.insert("NUMROW8".to_string(), &KeybdKey::Numrow8Key);
  keys.insert("NUMROW9".to_string(), &KeybdKey::Numrow9Key);
  keys.insert("TAB".to_string(), &KeybdKey::TabKey);

  keys

}

fn get_key_from_string(id: String) -> Option<&'static KeybdKey> {
  println!("[{}::keybind::get_key] // Searching KeyObject of: \"{}\".", env!("CARGO_PKG_NAME"), id);
  get_key_map().get(&id.to_uppercase()).copied()
}
