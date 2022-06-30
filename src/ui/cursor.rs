use bevy::prelude::*;

use crate::ui::UiState;

// TODO move to config file
const CURSOR_COLOR: Color = Color::GREEN;
const CURSOR_RADIUS: f32 = 2.0;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldCursor(Vec3::ZERO));
        app.add_system(world_cursor_system);

        app.add_system_set(SystemSet::on_enter(UiState::InGame).with_system(cursor_spawn));
        app.add_system_set(SystemSet::on_update(UiState::InGame).with_system(cursor_move));
        app.add_system_set(SystemSet::on_exit(UiState::InGame).with_system(cursor_remove));
    }
}

pub struct WorldCursor(pub Vec3);

#[derive(Component)]
pub struct GameCursor;

fn cursor_spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: CURSOR_RADIUS,
                subdivisions: 10,
            })),
            material: materials.add(CURSOR_COLOR.into()),
            ..default()
        })
        .insert(GameCursor);
}

fn cursor_move(
    world_cursor: ResMut<WorldCursor>,
    mut cursor: Query<&mut Transform, With<GameCursor>>,
) {
    cursor.get_single_mut().unwrap().translation = world_cursor.0;
}

fn cursor_remove(mut commands: Commands, cursor: Query<Entity, With<GameCursor>>) {
    commands.entity(cursor.get_single().unwrap()).despawn();
}

fn world_cursor_system(
    mut crs: ResMut<WorldCursor>,
    windows: Res<Windows>,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    if let Ok((camera, camera_transform)) = camera.get_single() {
        let window = windows.get_primary().unwrap();
        if let Some(screen_pos) = window.cursor_position() {
            let window_size = Vec2::new(window.width() as f32, window.height() as f32);
            // into ndc space
            let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

            // into view space
            let ray_clip = Vec4::new(ndc.x, ndc.y, -1.0, 1.0);
            let ray_eye = camera.projection_matrix().inverse() * ray_clip;
            let ray_eye = Vec4::new(ray_eye.x, ray_eye.y, -1.0, 0.0);

            // into world space
            let ray_world = camera_transform.compute_matrix().inverse() * ray_eye;
            let ray_world = ray_world.truncate().normalize();

            // calculation of point on the xy plane
            // same as -camera_transform.translation.dot(-Vec3::Z) / ray_world.dot(-Vec3::Z);
            let t = camera_transform.translation.z / -ray_world.z;
            let xy_pos = camera_transform.translation + ray_world * t;

            crs.0 = xy_pos;
        }
    }
}
