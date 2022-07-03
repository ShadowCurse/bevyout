use bevy::prelude::*;

use crate::config::GameConfig;
use crate::game::physics::{Ball, CollisionEvent, Dynamic, PhysicsStage};
use crate::game::GameElement;
use crate::game::GameState;

use crate::game::platform::GamePlatform;
use crate::ui::cursor::WorldCursor;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::InGame).with_system(ball_spawn));
        app.add_system_set_to_stage(
            PhysicsStage::Movement,
            SystemSet::on_update(GameState::InGame).with_system(ball_movement),
        );
        app.add_system_set_to_stage(
            PhysicsStage::CollisionResolution,
            SystemSet::on_update(GameState::InGame).with_system(ball_collision),
        );
    }
}

pub enum GameBallState {
    // Attached to the platform
    Attached,
    // Moves freely
    Detached,
}

#[derive(Component)]
pub struct GameBall {
    pub velocity: Vec2,
    pub speed: f32,
    pub state: GameBallState,
}

fn ball_spawn(
    config: Res<GameConfig>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: config.ball_radius,
                subdivisions: 10,
            })),
            material: materials.add(Color::TEAL.into()),
            transform: Transform::from_xyz(100.0, 50.0, 0.0),
            ..default()
        })
        .insert(GameElement)
        .insert(Ball {
            radius: config.ball_radius,
        })
        .insert(Dynamic)
        .insert(GameBall {
            velocity: Vec2::default(),
            speed: config.ball_speed,
            state: GameBallState::Attached,
        });
}

fn ball_movement(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    cursor: Res<WorldCursor>,
    platform: Query<&Transform, (With<GamePlatform>, Without<GameBall>)>,
    mut ball: Query<(&mut GameBall, &mut Transform), Without<GamePlatform>>,
) {
    if let Ok((mut ball, mut transform)) = ball.get_single_mut() {
        match ball.state {
            GameBallState::Attached => {
                if keys.just_pressed(KeyCode::Space) {
                    ball.state = GameBallState::Detached;
                    ball.velocity.x = cursor.0.x - transform.translation.x;
                    ball.velocity.y = cursor.0.y - transform.translation.y;
                    ball.velocity = ball.velocity.normalize();
                } else {
                    if let Ok(platform_transform) = platform.get_single() {
                        transform.translation.x = platform_transform.translation.x;
                        transform.translation.y = platform_transform.translation.y + 50.0;
                    }
                }
            }
            GameBallState::Detached => {
                if keys.just_pressed(KeyCode::Space) {
                    ball.state = GameBallState::Attached;
                    ball.velocity = Vec2::ZERO;
                } else {
                    transform.translation.x += ball.velocity.x * ball.speed * time.delta_seconds();
                    transform.translation.y += ball.velocity.y * ball.speed * time.delta_seconds();
                }
            }
        }
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
