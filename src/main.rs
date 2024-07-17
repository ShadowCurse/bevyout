use bevy::{
    prelude::*,
    window::{PresentMode, WindowMode},
};

mod config;
mod events;
mod game;
mod ui;
// mod utils;

use config::ConfigPlugin;
use events::EventsPlugin;
use game::GamePlugin;
use ui::UiPlugin;

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::BLACK));

    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::Immediate,
                mode: WindowMode::Windowed,
                ..Default::default()
            }),
            ..Default::default()
        }),
        ConfigPlugin,
        UiPlugin,
        GamePlugin,
        EventsPlugin,
    ));

    app.run();
}
