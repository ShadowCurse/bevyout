use bevy::{prelude::*, window::PrimaryWindow};

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldCursor(Vec3::ZERO));
        app.add_systems(Update, world_cursor_system);
    }
}

#[derive(Resource, Debug, Clone)]
pub struct WorldCursor(pub Vec3);

fn world_cursor_system(
    mut crs: ResMut<WorldCursor>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    let Ok(window) = windows.get_single() else {
        return;
    };
    if let Ok((camera, camera_transform)) = camera.get_single() {
        if let Some(screen_pos) = window.cursor_position() {
            let window_size = Vec2::new(window.width(), window.height());
            // into ndc space
            let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

            // into view space
            let ray_clip = Vec4::new(ndc.x, ndc.y, -1.0, 1.0);
            // let ray_eye = camera.projection_matrix().inverse() * ray_clip;
            let ray_eye = camera.clip_from_view().inverse() * ray_clip;
            let ray_eye = Vec4::new(ray_eye.x, ray_eye.y, -1.0, 0.0);

            // into world space
            let ray_world = camera_transform.compute_matrix().inverse() * ray_eye;
            let ray_world = ray_world.truncate().normalize();

            // calculation of point on the xy plane
            // same as -camera_transform.translation.dot(-Vec3::Z) / ray_world.dot(-Vec3::Z);
            let t = camera_transform.translation().z / -ray_world.z;
            let xy_pos = camera_transform.translation() + ray_world * t;

            crs.0 = xy_pos;
        }
    }
}
