use bevy::prelude::*;

pub mod ball;
pub mod bricks;
pub mod physics;
pub mod platform;
pub mod scene;

use crate::ui::UiState;
use crate::utils::remove_all_with;
use ball::BallPlugin;
use bricks::BricksPlugin;
use physics::PhysicsPlugin;
use platform::PlatformPlugin;
use scene::ScenePlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::NotInGame);

        app.add_plugin(PhysicsPlugin { debug: true });
        app.add_plugin(ScenePlugin);
        app.add_plugin(PlatformPlugin);
        app.add_plugin(BallPlugin);
        app.add_plugin(BricksPlugin);

        app.add_system_set(SystemSet::on_update(GameState::InGame).with_system(game_pause));

        app.add_system_set(
            SystemSet::on_exit(GameState::InGame).with_system(remove_all_with::<GameElement>),
        );
    }
}

/// Game states
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    NotInGame,
    InGame,
    Paused,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GameElement;

fn game_pause(
    mut ui_state: ResMut<State<UiState>>,
    mut game_state: ResMut<State<GameState>>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.pressed(KeyCode::Escape) {
        ui_state.push(UiState::Paused).unwrap();
        game_state.push(GameState::Paused).unwrap();
    }
}
