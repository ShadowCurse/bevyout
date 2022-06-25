use bevy::prelude::*;

#[derive(StageLabel, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CleanUpStage;

pub fn remove_all_with<T: Component>(mut commands: Commands, entities: Query<Entity, With<T>>) {
    for e in entities.iter() {
        commands.entity(e).despawn();
    }
}
