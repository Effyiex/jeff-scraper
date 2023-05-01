
use bevy::prelude::*;

mod scraper;
mod jeff;
mod app;
mod keybind;

fn main() {
    App::new()
        .add_startup_system(scraper::register_resource)
        .add_system(jeff::handle_jeff_components)
        .add_plugin(app::AppPlugin)
        .add_plugin(keybind::KeybindPlugin)
        .add_plugins(DefaultPlugins)
        .run();
}


