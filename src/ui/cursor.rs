use bevy::prelude::*;

use crate::ui::{GameUiCamera, UiState};

// TODO move to config file
const CURSOR_COLOR: Color = Color::GREEN;
const CURSOR_SIZE: f32 = 5.0;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldCursor(Vec2::ZERO));
        app.add_system(world_cursor_system);

        app.add_system_set(SystemSet::on_enter(UiState::InGame).with_system(cursor_spawn));
        app.add_system_set(SystemSet::on_update(UiState::InGame).with_system(cursor_move));
        app.add_system_set(SystemSet::on_exit(UiState::InGame).with_system(cursor_remove));
    }
}

pub struct WorldCursor(pub Vec2);

#[derive(Component)]
pub struct GameCursor;

fn cursor_spawn(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: CURSOR_COLOR,
                custom_size: Some(Vec2::new(CURSOR_SIZE, CURSOR_SIZE)),
                ..default()
            },
            ..default()
        })
        .insert(GameCursor);
}

fn cursor_move(
    world_cursor: ResMut<WorldCursor>,
    mut cursor: Query<&mut Transform, With<GameCursor>>,
) {
    cursor.get_single_mut().unwrap().translation = world_cursor.0.extend(0.0);
}

fn cursor_remove(mut commands: Commands, cursor: Query<Entity, With<GameCursor>>) {
    commands.entity(cursor.get_single().unwrap()).despawn();
}

fn world_cursor_system(
    mut crs: ResMut<WorldCursor>,
    windows: Res<Windows>,
    camera: Query<(&Camera, &GlobalTransform), With<GameUiCamera>>,
) {
    if let Ok((camera, camera_transform)) = camera.get_single() {
        let window = windows.get_primary().unwrap();
        if let Some(screen_pos) = window.cursor_position() {
            let window_size = Vec2::new(window.width() as f32, window.height() as f32);
            let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

            let ndc_to_world =
                camera_transform.compute_matrix() * camera.projection_matrix().inverse();
            let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

            crs.0 = world_pos.truncate();
        }
    }
}
