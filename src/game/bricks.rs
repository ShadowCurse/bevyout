use bevy::prelude::*;

use crate::config::{GameConfig, GameSettings};
use crate::events::GameEvents;
use crate::game::physics::{CollisionEvent, PhysicsStage, Rectangle};
use crate::game::GameElement;
use crate::game::GameState;

pub struct BricksPlugin;

impl Plugin for BricksPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::InGame).with_system(bricks_spawn));
        app.add_system_set_to_stage(
            PhysicsStage::CollisionResolution,
            SystemSet::on_update(GameState::InGame).with_system(bricks_collision),
        );
    }
}

#[derive(Component)]
pub struct GameBrick {
    health: u32,
}

pub struct BricksCount {
    pub total: u32,
    pub current: u32,
}

pub struct Score {
    pub score: u32,
}

fn bricks_spawn(
    config: Res<GameConfig>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(Score {
        score: 0,
    });
    let total_bricks = config.bricks_cols * config.bricks_rows;
    commands.insert_resource(BricksCount {
        total: total_bricks,
        current: total_bricks,
    });

    let brick_mesh = meshes.add(Mesh::from(shape::Box::new(
        config.bricks_width,
        config.bricks_height,
        1.0,
    )));

    let brick_material = materials.add(Color::INDIGO.into());

    for pos in spawn_grid(
        Vec2::new(config.bricks_pos_x, config.bricks_pos_y),
        config.bricks_rows,
        config.bricks_cols,
        config.bricks_width,
        config.bricks_height,
        config.bricks_gap_x,
        config.bricks_gap_y,
    ) {
        commands
            .spawn_bundle(PbrBundle {
                mesh: brick_mesh.clone(),
                material: brick_material.clone(),
                transform: Transform::from_translation(pos),
                ..default()
            })
            .insert(GameElement)
            .insert(Rectangle {
                width: config.bricks_width,
                height: config.bricks_height,
            })
            .insert(GameBrick {
                health: config.bricks_health,
            });
    }
}

fn spawn_grid(
    mut pos: Vec2,
    rows: u32,
    cols: u32,
    width: f32,
    height: f32,
    gap_x: f32,
    gap_y: f32,
) -> impl IntoIterator<Item = Vec3> {
    if rows % 2 == 0 {
        pos.y += (gap_y / 2.0 + height / 2.0) - (gap_y + height) * ((rows - 1) / 2) as f32;
    } else {
        pos.y += (gap_y + height) * ((rows - 1) / 2) as f32;
    }
    if cols % 2 == 0 {
        pos.x -= (gap_x / 2.0 + width / 2.0) + (gap_x + width) * ((cols - 1) / 2) as f32;
    } else {
        pos.x -= (gap_x + width) * ((cols - 1) / 2) as f32;
    }

    (0..cols).flat_map(move |x| {
        (0..rows).map(move |y| {
            Vec3::new(
                pos.x + x as f32 * (width + gap_x),
                pos.y + y as f32 * (height + gap_y),
                0.0,
            )
        })
    })
}

fn bricks_collision(
    audio: Res<Audio>,
    config: Res<GameConfig>,
    settings: Res<GameSettings>,
    mut commands: Commands,
    mut bricks_count: ResMut<BricksCount>,
    mut score: ResMut<Score>,
    mut collision_events: EventReader<CollisionEvent>,
    mut bricks: Query<(Entity, &mut GameBrick)>,
    mut game_events: EventWriter<GameEvents>,
) {
    for event in collision_events.iter() {
        if let Ok((brick, mut game_brick)) = bricks.get_mut(event.entity2) {
            audio.play_with_settings(
                config.bricks_sound.clone(),
                PlaybackSettings {
                    repeat: false,
                    volume: settings.sound_volume,
                    speed: 1.0,
                },
            );
            game_brick.health -= 1;
            score.score += 1;
            if game_brick.health == 0 {
                bricks_count.current -= 1;
                commands.entity(brick).despawn();
            }
        }
    }
    if bricks_count.current == 0 {
        game_events.send(GameEvents::EndGame);
    }
}
