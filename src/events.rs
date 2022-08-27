use bevy::prelude::*;
use bevy::window::WindowMode;

use crate::config::GameSettings;
use crate::game::GameState;
use crate::ui::UiState;

pub enum GameEvents {
    EndGame,
    Pause,
}

pub enum SettingsEvents {
    DisplayFullScreen,
    DisplayWindowed,
    VolumeUp,
    VolumeDown,
}

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameEvents>();
        app.add_event::<SettingsEvents>();
        app.add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(keyboard_input)
                .with_system(handle_game_events),
        );
        app.add_system_set(
            SystemSet::on_update(UiState::Settings).with_system(handle_settings_events),
        );
    }
}

fn keyboard_input(keys: Res<Input<KeyCode>>, mut game_events: EventWriter<GameEvents>) {
    if keys.pressed(KeyCode::Escape) {
        game_events.send(GameEvents::Pause);
    }
}

fn handle_game_events(
    mut ui_state: ResMut<State<UiState>>,
    mut game_state: ResMut<State<GameState>>,
    mut game_events: EventReader<GameEvents>,
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

fn handle_settings_events(
    mut settings: ResMut<GameSettings>,
    mut windows: ResMut<Windows>,
    mut settings_events: EventReader<SettingsEvents>,
) {
    for event in settings_events.iter() {
        match event {
            SettingsEvents::DisplayWindowed => {
                windows
                    .get_primary_mut()
                    .unwrap()
                    .set_mode(WindowMode::Windowed);
            }
            SettingsEvents::DisplayFullScreen => {
                windows
                    .get_primary_mut()
                    .unwrap()
                    .set_mode(WindowMode::Fullscreen);
            }
            SettingsEvents::VolumeUp => {
                settings.sound_volume += 0.01;
                settings.sound_volume = settings.sound_volume.clamp(0.0, 1.0);
            }
            SettingsEvents::VolumeDown => {
                settings.sound_volume -= 0.01;
                settings.sound_volume = settings.sound_volume.clamp(0.0, 1.0);
            }
        }
    }
}
