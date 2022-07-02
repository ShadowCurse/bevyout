use bevy::prelude::*;

mod config;
mod game;
mod ui;
mod utils;

use config::ConfigPlugin;
use game::GamePlugin;
use ui::UiPlugin;

fn main() {
    let mut app = App::new();

    app.insert_resource(bevy::log::LogSettings {
        level: bevy::log::Level::DEBUG,
        ..Default::default()
    });
    app.insert_resource(ClearColor(Color::BLACK));

    app.add_plugins(DefaultPlugins);
    app.add_plugin(ConfigPlugin);
    app.add_plugin(UiPlugin);
    app.add_plugin(GamePlugin);

    app.run();
}
