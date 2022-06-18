use bevy::prelude::*;

mod ball;
mod bricks;
mod physics;
mod platform;
mod scene;
mod ui;

use ball::BallPlugin;
use bricks::BricksPlugin;
use physics::PhysicsPlugin;
use platform::PlatformPlugin;
use scene::{ScenePlugin, SceneSize};
use ui::UiPlugin;

fn main() {
    let mut app = App::new();

    app.insert_resource(bevy::log::LogSettings {
        level: bevy::log::Level::DEBUG,
        ..Default::default()
    });

    app.insert_resource(ClearColor(Color::BLACK));

    app.add_plugins(DefaultPlugins);
    app.add_plugin(PhysicsPlugin { debug: true });
    app.add_plugin(ScenePlugin);
    app.add_plugin(PlatformPlugin);
    app.add_plugin(BallPlugin);
    app.add_plugin(BricksPlugin);
    app.add_plugin(UiPlugin);
    app.add_startup_system(setup);
    app.run();
}

fn setup(mut commands: Commands, scene_size: Res<SceneSize>) {
    // camera
    let cam_pos = Vec3::new(
        scene_size.width as f32 / 2.0,
        scene_size.height as f32 / 2.0,
        500.0,
    );
    let cam_look_at = Vec3::new(
        scene_size.width as f32 / 2.0,
        scene_size.height as f32 / 2.0,
        0.0,
    );
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_translation(cam_pos).looking_at(cam_look_at, Vec3::Y),
        ..default()
    });
    // light
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            ..Default::default()
        },
        ..Default::default()
    });
}
