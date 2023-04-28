
use bevy::prelude::*;

mod app;
use app::AppPlugin;

mod keybind;
use keybind::KeybindPlugin;

fn main() {
    App::new()
        .add_plugin(AppPlugin)
        .add_plugin(KeybindPlugin)
        .add_plugins(DefaultPlugins)
        .run();
}
