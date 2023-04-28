
use bevy::prelude::*;
use bevy::winit::WinitWindows;

pub struct AppPlugin;
pub struct AppToggleEvent;

impl Plugin for AppPlugin {

  fn build(&self, app: &mut App) {
    
    app

      .add_event::<AppToggleEvent>()
      .add_system(await_toggle)

      .add_startup_system(setup);

  }

}

fn setup(
  mut windows: Query<&mut Window>,
  winit_buffer: NonSend<WinitWindows>
) {

  for mut window in windows.iter_mut() {
    window.title = format!("{} ({})", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
  }

  for window in winit_buffer.windows.iter() {
    window.1.set_visible(false);
  }

}

fn await_toggle(
    event_reader: EventReader<AppToggleEvent>,
    winit_buffer: NonSend<WinitWindows>
) {

    if event_reader.is_empty() {
        return;
    }

    for window in winit_buffer.windows.iter() {
      window.1.set_visible(!window.1.is_visible().unwrap());
    }


}