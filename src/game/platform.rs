use bevy::prelude::*;

use crate::config::GameConfig;
use crate::events::GameEvents;
use crate::game::physics::{CollisionEvent, Dynamic, PhysicsSet, Rectangle};
use crate::game::GameState;

use super::ball::{GameBall, GameBallEvent};

pub struct PlatformPlugin;

impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), platform_spawn);
        app.add_systems(
            Update,
            platform_movement
                .in_set(PhysicsSet::Movement)
                .run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            Update,
            platform_lifes
                .in_set(PhysicsSet::CollisionDetection)
                .run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            Update,
            platform_collision
                .in_set(PhysicsSet::CollisionResolution)
                .run_if(in_state(GameState::InGame)),
        );
    }
}

#[derive(Component)]
pub struct GamePlatform {
    pub width: f32,
    pub height: f32,
    pub speed: f32,
}

#[derive(Debug, Clone, Resource)]
pub struct PlatformLifes {
    pub max: u32,
    pub current: u32,
}

fn platform_spawn(
    config: Res<GameConfig>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(PlatformLifes {
        max: config.platform_lifes,
        current: config.platform_lifes,
    });

    let material = materials.add(StandardMaterial {
        emissive: config.platform_color.into(),
        ..default()
    });

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(
                config.platform_width,
                config.platform_height,
                1.0,
            ))),
            material,
            transform: Transform::from_xyz(config.scene_width as f32 / 2.0, 10.0, 0.0),
            ..default()
        })
        .insert(StateScoped(GameState::InGame))
        .insert(Rectangle {
            width: config.platform_width,
            height: config.platform_height,
        })
        .insert(Dynamic)
        .insert(GamePlatform {
            width: config.platform_width,
            height: config.platform_height,
            speed: config.platform_speed,
        });
}

fn platform_movement(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut platform: Query<(&GamePlatform, &mut Transform)>,
) {
    if let Ok((platform, mut transform)) = platform.get_single_mut() {
        if keys.pressed(KeyCode::KeyA) {
            transform.translation.x -= platform.speed * time.delta_seconds();
        }

        if keys.pressed(KeyCode::KeyD) {
            transform.translation.x += platform.speed * time.delta_seconds();
        }
    }
}

fn platform_lifes(
    platform: Query<&Transform, With<GamePlatform>>,
    ball: Query<&Transform, (With<GameBall>, Without<GamePlatform>)>,
    mut lifes: ResMut<PlatformLifes>,
    mut game_events: EventWriter<GameEvents>,
    mut ball_events: EventWriter<GameBallEvent>,
) {
    if let (Ok(ball), Ok(platform)) = (ball.get_single(), platform.get_single()) {
        if ball.translation.y < platform.translation.y {
            lifes.current -= 1;
            ball_events.send(GameBallEvent::ChangeState);
        }
        if lifes.current == 0 {
            game_events.send(GameEvents::EndGame);
        }
    }
}

fn platform_collision(
    mut collision_events: EventReader<CollisionEvent>,
    mut platform: Query<(Entity, &Rectangle, &mut Transform), With<GamePlatform>>,
) {
    if let Ok((platform_entity, platform_rect, mut platform_transform)) = platform.get_single_mut()
    {
        for event in collision_events.read() {
            if event.entity1 == platform_entity {
                if event.collision_point.x < platform_transform.translation.x {
                    let diff = event.collision_point.x
                        - (platform_transform.translation.x - platform_rect.width / 2.0);
                    platform_transform.translation.x =
                        event.collision_point.x + diff + platform_rect.width / 2.0;
                } else {
                    let diff = (platform_transform.translation.x + platform_rect.width / 2.0)
                        - event.collision_point.x;
                    platform_transform.translation.x =
                        event.collision_point.x - diff - platform_rect.width / 2.0;
                }
            }
        }
    }
}
