use bevy::prelude::*;

use crate::game::physics::{CollisionEvent, PhysicsStage, PhysicsState, Rectangle};
use crate::game::GameElement;
use crate::AppState;

// TODO move to config file
const BRICKS_POS_X: f32 = 100.0;
const BRICKS_POS_Y: f32 = 200.0;
const BRICKS_WIDTH: f32 = 15.0;
const BRICKS_HEIGHT: f32 = 10.0;
const BRICKS_COLS: u32 = 9;
const BRICKS_ROWS: u32 = 5;
const BRICKS_GAP_X: f32 = 5.0;
const BRICKS_GAP_Y: f32 = 5.0;
const BRICKS_HEALTH: u32 = 1;

pub struct BricksPlugin;

impl Plugin for BricksPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(bricks_spawn));
        app.add_system_set_to_stage(
            PhysicsStage::CollisionResolution,
            SystemSet::on_update(PhysicsState::Running).with_system(bricks_collision),
        );
    }
}

#[derive(Component)]
pub struct GameBrick {
    health: u32,
}

fn bricks_spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let brick_mesh = meshes.add(Mesh::from(shape::Box::new(
        BRICKS_WIDTH,
        BRICKS_HEIGHT,
        1.0,
    )));

    let brick_material = materials.add(Color::INDIGO.into());

    for pos in spawn_grid(
        Vec2::new(BRICKS_POS_X, BRICKS_POS_Y),
        BRICKS_ROWS,
        BRICKS_COLS,
        BRICKS_WIDTH,
        BRICKS_HEIGHT,
        BRICKS_GAP_X,
        BRICKS_GAP_Y,
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
                width: BRICKS_WIDTH,
                height: BRICKS_HEIGHT,
            })
            .insert(GameBrick {
                health: BRICKS_HEALTH,
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

    (0..cols)
        .map(move |x| {
            (0..rows).map(move |y| {
                Vec3::new(
                    pos.x + x as f32 * (width + gap_x),
                    pos.y + y as f32 * (height + gap_y),
                    0.0,
                )
            })
        })
        .flatten()
        .into_iter()
}

fn bricks_collision(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut bricks: Query<(Entity, &mut GameBrick)>,
) {
    for event in collision_events.iter() {
        if let Ok((brick, mut game_brick)) = bricks.get_mut(event.entity2) {
            game_brick.health -= 1;
            if game_brick.health == 0 {
                commands.entity(brick).despawn();
            }
        }
    }
}
