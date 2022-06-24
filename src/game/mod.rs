use bevy::prelude::*;

pub mod ball;
pub mod bricks;
pub mod physics;
pub mod platform;
pub mod scene;

use ball::BallPlugin;
use bricks::BricksPlugin;
use physics::PhysicsPlugin;
use platform::PlatformPlugin;
use scene::ScenePlugin;
use crate::AppState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PhysicsPlugin { debug: true });
        app.add_plugin(ScenePlugin);
        app.add_plugin(PlatformPlugin);
        app.add_plugin(BallPlugin);
        app.add_plugin(BricksPlugin);

        app.add_system_set(SystemSet::on_update(AppState::InGame).with_system(game_exit));
    }
}

fn game_exit(keys: Res<Input<KeyCode>>, mut game_state: ResMut<State<AppState>>) {
    if keys.pressed(KeyCode::Escape) {
        game_state.set(AppState::MainMenu).unwrap()
    }
}
