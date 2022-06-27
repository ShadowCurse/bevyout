use crate::game::GameState;
use bevy::prelude::*;

#[derive(StageLabel, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PhysicsStage {
    Movement,
    CollisionDetection,
    CollisionResolution,
}

pub struct PhysicsPlugin {
    pub debug: bool,
}

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>();

        app.add_stage(PhysicsStage::Movement, SystemStage::parallel());
        app.add_stage_after(
            PhysicsStage::Movement,
            PhysicsStage::CollisionDetection,
            SystemStage::parallel(),
        );
        app.add_stage_after(
            PhysicsStage::CollisionDetection,
            PhysicsStage::CollisionResolution,
            SystemStage::parallel(),
        );

        app.add_state_to_stage(PhysicsStage::Movement, GameState::NotInGame);
        app.add_state_to_stage(PhysicsStage::CollisionDetection, GameState::NotInGame);
        app.add_state_to_stage(PhysicsStage::CollisionResolution, GameState::NotInGame);
        app.add_system_set_to_stage(
            PhysicsStage::CollisionDetection,
            SystemSet::on_update(GameState::InGame)
                .with_system(ball_rect_collision_system)
                .with_system(rect_rect_collision_system),
        );
        if self.debug {
            app.add_system_to_stage(PhysicsStage::CollisionResolution, debug_physics_event);
        }
    }
}

#[derive(Component, Debug)]
pub struct Ball {
    pub radius: f32,
}

#[derive(Component, Debug)]
pub struct Rectangle {
    pub width: f32,
    pub height: f32,
}

#[derive(Component, Debug)]
pub struct Dynamic;

#[derive(Debug)]
pub struct CollisionEvent {
    pub entity1: Entity,
    pub entity2: Entity,
    pub collision_point: Vec2,
}

fn ball_rect_collision_system(
    mut collision_events: EventWriter<CollisionEvent>,
    balls: Query<(Entity, &Ball, &Transform), With<Dynamic>>,
    rectangles: Query<(Entity, &Rectangle, &Transform)>,
) {
    for (ball_entity, ball, ball_transform) in balls.iter() {
        for (rect_entity, rect, rect_transform) in rectangles.iter() {
            if let Some(collision_point) =
                ball_rect_collision(ball, ball_transform, rect, rect_transform)
            {
                collision_events.send(CollisionEvent {
                    entity1: ball_entity,
                    entity2: rect_entity,
                    collision_point,
                });
            }
        }
    }
}

fn ball_rect_collision(
    ball: &Ball,
    ball_transform: &Transform,
    rect: &Rectangle,
    rect_transform: &Transform,
) -> Option<Vec2> {
    let mut px = ball_transform.translation.x;
    let mut py = ball_transform.translation.y;
    px = px.max(rect_transform.translation.x - rect.width / 2.0);
    px = px.min(rect_transform.translation.x + rect.width / 2.0);
    py = py.max(rect_transform.translation.y - rect.height / 2.0);
    py = py.min(rect_transform.translation.y + rect.height / 2.0);

    if (ball_transform.translation.x - px).powi(2) + (ball_transform.translation.y - py).powi(2)
        < ball.radius.powi(2)
    {
        Some(Vec2::new(px, py))
    } else {
        None
    }
}

pub fn rect_rect_collision_system(
    mut collision_events: EventWriter<CollisionEvent>,
    dynamic_rectangles: Query<(Entity, &Rectangle, &Transform), With<Dynamic>>,
    rectangles: Query<(Entity, &Rectangle, &Transform), Without<Dynamic>>,
) {
    for (dyn_entity, dyn_rect, dyn_transform) in dynamic_rectangles.iter() {
        for (rect_entity, rect, rect_transform) in rectangles.iter() {
            if let Some(collision_point) =
                rect_rect_collision(dyn_rect, dyn_transform, rect, rect_transform)
            {
                collision_events.send(CollisionEvent {
                    entity1: dyn_entity,
                    entity2: rect_entity,
                    collision_point,
                });
            }
        }
    }
}

fn rect_rect_collision(
    dyn_rect: &Rectangle,
    dyn_transform: &Transform,
    rect: &Rectangle,
    rect_transform: &Transform,
) -> Option<Vec2> {
    let collision_x = dyn_transform.translation.x + dyn_rect.width / 2.0
        >= rect_transform.translation.x - rect.width / 2.0
        && rect_transform.translation.x + rect.width / 2.0
            >= dyn_transform.translation.x - dyn_rect.width / 2.0;

    let collision_y = dyn_transform.translation.y + dyn_rect.height / 2.0
        >= rect_transform.translation.y - rect.height / 2.0
        && rect_transform.translation.y + rect.height / 2.0
            >= dyn_transform.translation.y - dyn_rect.height / 2.0;

    if collision_x && collision_y {
        let top = (dyn_transform.translation.y + dyn_rect.height / 2.0)
            .min(rect_transform.translation.y + rect.height / 2.0);
        let bot = (dyn_transform.translation.y - dyn_rect.height / 2.0)
            .max(rect_transform.translation.y - rect.height / 2.0);
        let right = (dyn_transform.translation.x + dyn_rect.width / 2.0)
            .min(rect_transform.translation.x + rect.width / 2.0);
        let left = (dyn_transform.translation.x - dyn_rect.width / 2.0)
            .max(rect_transform.translation.x - rect.width / 2.0);

        Some(Vec2::new((left + right) / 2.0, (top + bot) / 2.0))
    } else {
        None
    }
}

fn debug_physics_event(
    mut collision_events: EventReader<CollisionEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for event in collision_events.iter() {
        debug!("collision event: {:?}", event);
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::RED.into()),
            transform: Transform::from_xyz(event.collision_point.x, event.collision_point.y, 2.0),
            ..default()
        });
    }
}
