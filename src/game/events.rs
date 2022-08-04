use bevy::prelude::*;

use crate::ui::UiState;
use super::GameState;

#[derive(Debug, Clone, Default)]
pub struct EndGame;

pub enum GameEvents {
    EndGame,
    Pause,
}

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameEvents>();
        app.add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(keyboard_input)
                .with_system(handle_game_events)
        );
    }
}

fn keyboard_input(keys: Res<Input<KeyCode>>, mut game_events: EventWriter<GameEvents>) {
    if keys.pressed(KeyCode::Escape) {
        game_events.send(GameEvents::Pause);
    }
}

fn handle_game_events(
    mut game_events: EventReader<GameEvents>,
    mut ui_state: ResMut<State<UiState>>,
    mut game_state: ResMut<State<GameState>>,
) {
    for event in game_events.iter() {
        match event {
            GameEvents::EndGame => {
                ui_state.push(UiState::EndGame).unwrap();
                game_state.push(GameState::EndGame).unwrap();
            }
            GameEvents::Pause => {
                ui_state.push(UiState::Paused).unwrap();
                game_state.push(GameState::Paused).unwrap();
            }
        }
    }
}
