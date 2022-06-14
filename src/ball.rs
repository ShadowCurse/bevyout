use crate::physics::{Ball, Dynamic};
use bevy::prelude::*;

// TODO move to config file
const BALL_RADIUS: f32 = 5.0;
const BALL_VELOCITY: (f32, f32) = (1.0, 0.0);
const BALL_SPEED: f32 = 500.0;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(ball_spawn);
        app.add_system(ball_movement);
    }
}

#[derive(Component)]
pub struct GameBall {
    velocity: Vec2,
    speed: f32,
}

fn ball_spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: BALL_RADIUS,
                subdivisions: 10,
            })),
            material: materials.add(Color::TEAL.into()),
            transform: Transform::from_xyz(100.0, 50.0, 0.0),
            ..default()
        })
        .insert(Ball {
            radius: BALL_RADIUS,
        })
        .insert(Dynamic)
        .insert(GameBall {
            velocity: Vec2::new(BALL_VELOCITY.0, BALL_VELOCITY.1),
            speed: BALL_SPEED,
        });
}

fn ball_movement(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut ball: Query<&mut Transform, With<GameBall>>,
) {
    let mut transform = ball.single_mut();
    if keys.pressed(KeyCode::Up) {
        transform.translation.y += 10.0 * time.delta_seconds();
    }
    if keys.pressed(KeyCode::Down) {
        transform.translation.y -= 10.0 * time.delta_seconds();
    }
    if keys.pressed(KeyCode::Right) {
        transform.translation.x += 10.0 * time.delta_seconds();
    }
    if keys.pressed(KeyCode::Left) {
        transform.translation.x -= 10.0 * time.delta_seconds();
    }
}
