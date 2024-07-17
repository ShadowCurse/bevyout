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

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
        app.enable_state_scoped_entities::<GameState>();

        app.add_plugins((
            PhysicsPlugin { debug: false },
            BallPlugin,
            BricksPlugin,
            PlatformPlugin,
            ScenePlugin,
        ));
    }
}

/// Game states
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum GameState {
    #[default]
    NotInGame,
    InGame,
    Paused,
    EndGame,
}
