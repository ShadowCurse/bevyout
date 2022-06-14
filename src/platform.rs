use bevy::prelude::*;

use crate::physics::{Dynamic, Rectangle};
use crate::scene::SceneSize;

// TODO move to config file
const PLATFORM_WIDTH: f32 = 50.0;
const PLATFORM_HEIGHT: f32 = 10.0;
const PLATFORM_SPEED: f32 = 300.0;

pub struct PlatformPlugin;

impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(platform_spawn);
        app.add_system(platform_movement);
    }
}

#[derive(Component)]
pub struct GamePlatform {
    speed: f32,
}

fn platform_spawn(
    mut commands: Commands,
    scene_size: Res<SceneSize>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(
                PLATFORM_WIDTH,
                PLATFORM_HEIGHT,
                1.0,
            ))),
            material: materials.add(Color::FUCHSIA.into()),
            transform: Transform::from_xyz(scene_size.width as f32 / 2.0, 10.0, 0.0),
            ..default()
        })
        .insert(Rectangle {
            width: PLATFORM_WIDTH,
            height: PLATFORM_HEIGHT,
        })
        .insert(Dynamic)
        .insert(GamePlatform {
            speed: PLATFORM_SPEED,
        });
}

fn platform_movement(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut platform: Query<(&GamePlatform, &mut Transform)>,
) {
    let (platform, mut transform) = platform.single_mut();
    if keys.pressed(KeyCode::A) {
        transform.translation.x -= platform.speed * time.delta_seconds();
    }

    if keys.pressed(KeyCode::D) {
        transform.translation.x += platform.speed * time.delta_seconds();
    }
}
