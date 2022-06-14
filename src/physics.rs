use bevy::prelude::*;

pub struct PhysicsPlugin {
    pub debug: bool,
}

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>();
        app.add_system(ball_rect_collision_system);
        app.add_system(rect_rect_collision_system);
        if self.debug {
            app.add_system(debug_physics_event);
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
}

fn ball_rect_collision_system(
    mut collision_events: EventWriter<CollisionEvent>,
    balls: Query<(Entity, &Ball, &Transform), With<Dynamic>>,
    rectangles: Query<(Entity, &Rectangle, &Transform)>,
) {
    for (ball_entity, ball, ball_transform) in balls.iter() {
        for (rect_entity, rect, rect_transform) in rectangles.iter() {
            if ball_rect_collision(ball, ball_transform, rect, rect_transform) {
                collision_events.send(CollisionEvent {
                    entity1: ball_entity,
                    entity2: rect_entity,
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
) -> bool {
    let cd_x = (ball_transform.translation.x - rect_transform.translation.x).abs();
    let cd_y = (ball_transform.translation.y - rect_transform.translation.y).abs();

    if rect.width / 2.0 + ball.radius < cd_x {
        return false;
    }
    if rect.height / 2.0 + ball.radius < cd_y {
        return false;
    }

    if cd_x <= rect.width / 2.0 {
        return true;
    }
    if cd_y <= rect.height / 2.0 {
        return true;
    }

    let corner_distance_sq = (cd_x - rect.width / 2.0).powi(2) + (cd_y - rect.height / 2.0).powi(2);

    corner_distance_sq <= ball.radius.powi(2)
}

pub fn rect_rect_collision_system(
    mut collision_events: EventWriter<CollisionEvent>,
    dynamic_rectangles: Query<(Entity, &Rectangle, &Transform), With<Dynamic>>,
    rectangles: Query<(Entity, &Rectangle, &Transform), Without<Dynamic>>,
) {
    for (dyn_entity, dyn_rect, dyn_transform) in dynamic_rectangles.iter() {
        for (rect_entity, rect, rect_transform) in rectangles.iter() {
            if rect_rect_collision(dyn_rect, dyn_transform, rect, rect_transform) {
                collision_events.send(CollisionEvent {
                    entity1: dyn_entity,
                    entity2: rect_entity,
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
) -> bool {
    let dx = dyn_transform.translation.x - rect_transform.translation.x;
    let px = (dyn_rect.width + rect.width) / 2.0 - dx.abs();
    if px <= 0.0 {
        return false;
    }

    let dy = dyn_transform.translation.y - rect_transform.translation.y;
    let py = (dyn_rect.height + rect.height) / 2.0 - dy.abs();
    if py <= 0.0 {
        return false;
    }

    true
}

fn debug_physics_event(mut collision_events: EventReader<CollisionEvent>) {
    for event in collision_events.iter() {
        debug!("collision event: {:?}", event);
    }
}
