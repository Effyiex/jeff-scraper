
use bevy::prelude::*;

mod scraper;

mod app;
use app::AppPlugin;

mod keybind;
use keybind::KeybindPlugin;

fn main() {
    App::new()
        .add_startup_system(scraper::register_resource)
        .add_plugin(AppPlugin)
        .add_plugin(KeybindPlugin)
        .add_plugins(DefaultPlugins)
        .run();
}


