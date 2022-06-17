use crate::physics::{Ball, CollisionEvent, Dynamic, PhysicsStage};
use bevy::prelude::*;

// TODO move to config file
const BALL_RADIUS: f32 = 5.0;
const BALL_VELOCITY: (f32, f32) = (1.0, 1.0);
const BALL_SPEED: f32 = 100.0;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(ball_spawn);
        app.add_system_to_stage(PhysicsStage::Movement, ball_movement);
        app.add_system_to_stage(PhysicsStage::CollisionResolution, ball_collision);
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
            velocity: Vec2::new(BALL_VELOCITY.0, BALL_VELOCITY.1).normalize(),
            speed: BALL_SPEED,
        });
}

fn ball_movement(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut ball: Query<(&GameBall, &mut Transform)>,
) {
    let (ball, mut transform) = ball.single_mut();

    transform.translation.x += ball.velocity.x * ball.speed * time.delta_seconds();
    transform.translation.y += ball.velocity.y * ball.speed * time.delta_seconds();

    if keys.pressed(KeyCode::Up) {
        transform.translation.y += ball.speed * time.delta_seconds();
    }
    if keys.pressed(KeyCode::Down) {
        transform.translation.y -= ball.speed * time.delta_seconds();
    }
    if keys.pressed(KeyCode::Right) {
        transform.translation.x += ball.speed * time.delta_seconds();
    }
    if keys.pressed(KeyCode::Left) {
        transform.translation.x -= ball.speed * time.delta_seconds();
    }
}

fn ball_collision(
    mut collision_events: EventReader<CollisionEvent>,
    mut balls: Query<(&Ball, &mut GameBall, &mut Transform), With<Dynamic>>,
) {
    for event in collision_events.iter() {
        if let Ok((ball, mut game_ball, mut transform)) = balls.get_mut(event.entity1) {
            let offset = Vec2::new(
                transform.translation.x - event.collision_point.x,
                transform.translation.y - event.collision_point.y,
            );
            let distance = offset.length();
            let normal = Vec2::new(offset.x / distance, offset.y / distance);
            let diff = ball.radius - distance;
            transform.translation.x += diff * normal.x;
            transform.translation.y += diff * normal.y;

            let new_vel = -2.0 * game_ball.velocity.dot(normal) * normal + game_ball.velocity;
            game_ball.velocity = new_vel.normalize();
        }
    }
}
