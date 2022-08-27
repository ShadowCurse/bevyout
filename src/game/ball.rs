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
        app.add_event::<GameBallEvent>();
        app.add_system_set(SystemSet::on_enter(GameState::InGame).with_system(ball_spawn));
        app.add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(ball_controlls)
                .with_system(ball_event_handler),
        );
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

pub enum GameBallEvent {
    ChangeState,
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
    pub radius: f32,
    pub speed: f32,
    pub speed_mul: f32,
    pub state: GameBallState,
    pub material: Handle<StandardMaterial>,
}

fn ball_spawn(
    config: Res<GameConfig>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material = materials.add(StandardMaterial {
        emissive: config.ball_base_color,
        ..default()
    });
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: config.ball_radius,
                subdivisions: 10,
            })),
            material: material.clone(),
            transform: Transform::from_xyz(100.0, 50.0, 0.0),
            ..default()
        })
        .insert_bundle(PointLightBundle {
            point_light: PointLight {
                color: config.ball_base_color,
                intensity: 1000.0,
                range: 1000.0,
                radius: 1000.0,
                ..default()
            },
            ..default()
        })
        .insert(GameElement)
        .insert(Ball {
            radius: config.ball_radius,
        })
        .insert(Dynamic)
        .insert(GameBall {
            velocity: Vec2::default(),
            radius: config.ball_radius,
            speed: config.ball_speed,
            speed_mul: 1.0,
            state: GameBallState::Attached,
            material,
        });
}

fn ball_controlls(keys: Res<Input<KeyCode>>, mut ball_events: EventWriter<GameBallEvent>) {
    if keys.just_pressed(KeyCode::Space) {
        ball_events.send(GameBallEvent::ChangeState);
    }
}

fn ball_movement(
    time: Res<Time>,
    platform: Query<(&Transform, &GamePlatform), Without<GameBall>>,
    mut ball: Query<(&GameBall, &mut Transform), Without<GamePlatform>>,
) {
    if let Ok((ball, mut transform)) = ball.get_single_mut() {
        match ball.state {
            GameBallState::Attached => {
                if let Ok((platform_transform, platform)) = platform.get_single() {
                    transform.translation.x = platform_transform.translation.x;
                    transform.translation.y =
                        platform_transform.translation.y + platform.height * 0.5 + ball.radius;
                }
            }
            GameBallState::Detached => {
                transform.translation.x +=
                    ball.velocity.x * ball.speed * ball.speed_mul * time.delta_seconds();
                transform.translation.y +=
                    ball.velocity.y * ball.speed * ball.speed_mul * time.delta_seconds();
            }
        }
    }
}

fn ball_collision(
    config: Res<GameConfig>,
    mut collision_events: EventReader<CollisionEvent>,
    mut ball: Query<
        (
            Entity,
            &Ball,
            &mut GameBall,
            &mut PointLight,
            &mut Transform,
        ),
        With<Dynamic>,
    >,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if let Ok((ball_entity, ball, mut game_ball, mut point_light, mut transform)) =
        ball.get_single_mut()
    {
        for event in collision_events.iter() {
            if ball_entity == event.entity1 {
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

                game_ball.speed_mul =
                    (game_ball.speed_mul + 0.1).min(config.ball_max_speed_multiplier);

                let mix = (game_ball.speed_mul - 1.0) / (config.ball_max_speed_multiplier - 1.0);
                let new_color =
                    ball_color_mix(config.ball_base_color, config.ball_max_speed_color, mix);
                let mut material = materials.get_mut(&game_ball.material).unwrap();
                material.emissive = new_color;
                point_light.color = new_color;
            }
        }
    }
}

fn ball_color_mix(base: Color, target: Color, mix: f32) -> Color {
    assert!((0.0..=1.0).contains(&mix), "mix: {mix}");
    Color::Rgba {
        red: base.r() + (target.r() - base.r()) * mix,
        green: base.g() + (target.g() - base.g()) * mix,
        blue: base.b() + (target.b() - base.b()) * mix,
        alpha: base.a() + (target.a() - base.a()) * mix,
    }
}

fn ball_event_handler(
    config: Res<GameConfig>,
    cursor: Res<WorldCursor>,
    mut ball_events: EventReader<GameBallEvent>,
    mut ball: Query<(&Transform, &mut GameBall, &mut PointLight)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if let Ok((transform, mut ball, mut point_light)) = ball.get_single_mut() {
        for _event in ball_events.iter() {
            match ball.state {
                GameBallState::Attached => {
                    ball.state = GameBallState::Detached;
                    ball.velocity.x = cursor.0.x - transform.translation.x;
                    ball.velocity.y = cursor.0.y - transform.translation.y;
                    ball.velocity = ball.velocity.normalize();
                    ball.speed = config.ball_speed;
                }
                GameBallState::Detached => {
                    ball.state = GameBallState::Attached;
                    ball.velocity = Vec2::ZERO;
                    ball.speed_mul = 1.0;

                    let mut material = materials.get_mut(&ball.material).unwrap();
                    material.emissive = config.ball_base_color;
                    point_light.color = config.ball_base_color;
                }
            }
        }
    }
}
