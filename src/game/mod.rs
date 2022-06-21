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
        app.add_plugin(PhysicsPlugin { debug: true });
        app.add_plugin(ScenePlugin);
        app.add_plugin(PlatformPlugin);
        app.add_plugin(BallPlugin);
        app.add_plugin(BricksPlugin);
    }
}
