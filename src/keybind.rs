
use crate::app::AppToggleEvent;

use bevy::prelude::*;

use inputbot::{
  BlockInput,
  KeybdKey::*
};

pub struct KeybindPlugin;

impl Plugin for KeybindPlugin {

  fn build(&self, app: &mut App) {

    app.add_system(await_input);

    LControlKey.blockable_bind(move || {
      if LShiftKey.is_pressed() && GKey.is_pressed() {
        BlockInput::Block
      } else {
        BlockInput::DontBlock
      }
    });

  }

}

fn await_input(
  mut app_toggler: EventWriter<AppToggleEvent>
) {
  if LControlKey.is_pressed() && LShiftKey.is_pressed() && GKey.is_pressed() {
    app_toggler.send(AppToggleEvent);
  }
}
