use bevy::prelude::*;

use crate::game::physics::Rectangle;
use crate::game::GameElement;
use crate::AppState;

// TODO move to config file
const WIDTH: f32 = 200.0;
const HEIGHT: f32 = 350.0;
const BORDER_COLOR: Color = Color::WHITE;

pub struct SceneParams {
    pub width: f32,
    pub height: f32,
    pub border_color: Color,
}

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SceneParams {
            width: WIDTH,
            height: HEIGHT,
            border_color: BORDER_COLOR,
        });
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(scene_spawn));
    }
}

fn scene_spawn(
    scene_params: Res<SceneParams>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let top_bot_mesh = meshes.add(Mesh::from(shape::Box::new(scene_params.width, 1.0, 1.0)));
    let left_right_mesh = meshes.add(Mesh::from(shape::Box::new(1.0, scene_params.height, 1.0)));

    let border_material = materials.add(scene_params.border_color.into());

    // top
    commands
        .spawn_bundle(PbrBundle {
            mesh: top_bot_mesh.clone(),
            material: border_material.clone(),
            transform: Transform::from_xyz(scene_params.width / 2.0, scene_params.height, 0.0),
            ..default()
        })
        .insert(GameElement)
        .insert(Rectangle {
            width: scene_params.width,
            height: 1.0,
        });
    // bot
    commands
        .spawn_bundle(PbrBundle {
            mesh: top_bot_mesh,
            material: border_material.clone(),
            transform: Transform::from_xyz(scene_params.width / 2.0, 0.0, 0.0),
            ..default()
        })
        .insert(GameElement)
        .insert(Rectangle {
            width: scene_params.width,
            height: 1.0,
        });
    // left
    commands
        .spawn_bundle(PbrBundle {
            mesh: left_right_mesh.clone(),
            material: border_material.clone(),
            transform: Transform::from_xyz(0.0, scene_params.height / 2.0, 0.0),
            ..default()
        })
        .insert(GameElement)
        .insert(Rectangle {
            width: 1.0,
            height: scene_params.height,
        });
    // right
    commands
        .spawn_bundle(PbrBundle {
            mesh: left_right_mesh,
            material: border_material,
            transform: Transform::from_xyz(scene_params.width, scene_params.height / 2.0, 0.0),
            ..default()
        })
        .insert(GameElement)
        .insert(Rectangle {
            width: 1.0,
            height: scene_params.height,
        });
}
