use bevy::prelude::*;

// TODO move to config file
const CURSOR_SIZE: f32 = 10.0;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldCursor(Vec3::ZERO));
        app.add_startup_system(cursor_spawn);
        app.add_system(cursor_move);
        app.add_system(world_cursor_system);
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
            mesh: meshes.add(Mesh::from(shape::Box::new(CURSOR_SIZE, CURSOR_SIZE, 0.0))),
            material: materials.add(Color::GREEN.into()),
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

fn world_cursor_system(
    mut crs: ResMut<WorldCursor>,
    windows: Res<Windows>,
    camera: Query<(&Camera, &GlobalTransform), With<CameraUi>>,
) {
    if let Ok((camera, camera_transform)) = camera.get_single() {
        let window = windows.get_primary().unwrap();
        if let Some(screen_pos) = window.cursor_position() {
            let window_size = Vec2::new(window.width() as f32, window.height() as f32);
            let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

            let ndc_to_world =
                camera_transform.compute_matrix() * camera.projection_matrix().inverse();
            let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

            crs.0 = world_pos;
        }
    }
}
