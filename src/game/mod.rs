use bevy::prelude::*;

pub mod ball;
pub mod bricks;
pub mod physics;
pub mod platform;
pub mod scene;

use crate::{utils::remove_all_with, AppState};
use ball::BallPlugin;
use bricks::BricksPlugin;
use physics::{PhysicsPlugin, PhysicsState};
use platform::PlatformPlugin;
use scene::ScenePlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PhysicsPlugin { debug: true });
        app.add_plugin(ScenePlugin);
        app.add_plugin(PlatformPlugin);
        app.add_plugin(BallPlugin);
        app.add_plugin(BricksPlugin);

        app.add_system_set(SystemSet::on_update(AppState::InGame).with_system(game_exit));
        app.add_system_set(
            SystemSet::on_exit(AppState::InGame).with_system(remove_all_with::<GameElement>),
        );
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GameElement;

fn game_exit(
    mut game_state: ResMut<State<AppState>>,
    mut physics_state: ResMut<State<PhysicsState>>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.pressed(KeyCode::Escape) {
        game_state.set(AppState::MainMenu).unwrap();
        physics_state.set(PhysicsState::NotRunning).unwrap();
    }
}
