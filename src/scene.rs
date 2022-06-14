use bevy::prelude::*;

use crate::physics::Rectangle;

// TODO move to config file
const WIDTH: u32 = 200;
const HEIGHT: u32 = 350;

pub struct SceneSize {
    pub width: u32,
    pub height: u32,
}

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SceneSize {
            width: WIDTH,
            height: HEIGHT,
        });
        app.add_startup_system(scene_spawn);
    }
}

fn scene_spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let top_bot_mesh = meshes.add(Mesh::from(shape::Box::new(WIDTH as f32, 1.0, 1.0)));
    let left_right_mesh = meshes.add(Mesh::from(shape::Box::new(1.0, HEIGHT as f32, 1.0)));

    let border_material = materials.add(Color::WHITE.into());

    // top
    commands
        .spawn_bundle(PbrBundle {
            mesh: top_bot_mesh.clone(),
            material: border_material.clone(),
            transform: Transform::from_xyz(WIDTH as f32 / 2.0, HEIGHT as f32, 0.0),
            ..default()
        })
        .insert(Rectangle {
            width: WIDTH as f32,
            height: 1.0,
        });
    // bot
    commands
        .spawn_bundle(PbrBundle {
            mesh: top_bot_mesh,
            material: border_material.clone(),
            transform: Transform::from_xyz(WIDTH as f32 / 2.0, 0.0, 0.0),
            ..default()
        })
        .insert(Rectangle {
            width: WIDTH as f32,
            height: 1.0,
        });
    // left
    commands
        .spawn_bundle(PbrBundle {
            mesh: left_right_mesh.clone(),
            material: border_material.clone(),
            transform: Transform::from_xyz(0.0, HEIGHT as f32 / 2.0, 0.0),
            ..default()
        })
        .insert(Rectangle {
            width: 1.0,
            height: HEIGHT as f32,
        });
    // right
    commands
        .spawn_bundle(PbrBundle {
            mesh: left_right_mesh,
            material: border_material,
            transform: Transform::from_xyz(WIDTH as f32, HEIGHT as f32 / 2.0, 0.0),
            ..default()
        })
        .insert(Rectangle {
            width: 1.0,
            height: HEIGHT as f32,
        });
}
