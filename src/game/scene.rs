use bevy::prelude::*;

use crate::config::GameConfig;
use crate::game::physics::Rectangle;
use crate::game::GameElement;
use crate::game::GameState;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::InGame).with_system(scene_spawn));
    }
}

fn scene_spawn(
    config: Res<GameConfig>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let top_bot_mesh = meshes.add(Mesh::from(shape::Box::new(config.scene_width, 1.0, 1.0)));
    let left_right_mesh = meshes.add(Mesh::from(shape::Box::new(1.0, config.scene_height, 1.0)));

    let border_material = materials.add(StandardMaterial {
        emissive: config.scene_border_color,
        ..default()
    });

    // top
    commands
        .spawn_bundle(PbrBundle {
            mesh: top_bot_mesh.clone(),
            material: border_material.clone(),
            transform: Transform::from_xyz(config.scene_width / 2.0, config.scene_height, 0.0),
            ..default()
        })
        .insert(GameElement)
        .insert(Rectangle {
            width: config.scene_width,
            height: 1.0,
        });
    // bot
    commands
        .spawn_bundle(PbrBundle {
            mesh: top_bot_mesh,
            material: border_material.clone(),
            transform: Transform::from_xyz(config.scene_width / 2.0, 0.0, 0.0),
            ..default()
        })
        .insert(GameElement)
        .insert(Rectangle {
            width: config.scene_width,
            height: 1.0,
        });
    // left
    commands
        .spawn_bundle(PbrBundle {
            mesh: left_right_mesh.clone(),
            material: border_material.clone(),
            transform: Transform::from_xyz(0.0, config.scene_height / 2.0, 0.0),
            ..default()
        })
        .insert(GameElement)
        .insert(Rectangle {
            width: 1.0,
            height: config.scene_height,
        });
    // right
    commands
        .spawn_bundle(PbrBundle {
            mesh: left_right_mesh,
            material: border_material,
            transform: Transform::from_xyz(config.scene_width, config.scene_height / 2.0, 0.0),
            ..default()
        })
        .insert(GameElement)
        .insert(Rectangle {
            width: 1.0,
            height: config.scene_height,
        });
}
