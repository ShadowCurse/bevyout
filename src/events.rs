use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowMode};

use crate::config::GameSettings;
use crate::game::GameState;
use crate::ui::UiState;

#[derive(Event)]
pub enum GameEvents {
    EndGame,
    Pause,
}

#[derive(Event)]
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
        app.add_systems(
            Update,
            (keyboard_input, handle_game_events).run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            Update,
            handle_settings_events.run_if(in_state(UiState::Settings)),
        );
    }
}

fn keyboard_input(keys: Res<ButtonInput<KeyCode>>, mut game_events: EventWriter<GameEvents>) {
    if keys.pressed(KeyCode::Escape) {
        game_events.send(GameEvents::Pause);
    }
}

fn handle_game_events(
    mut ui_state: ResMut<NextState<UiState>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut game_events: EventReader<GameEvents>,
) {
    for event in game_events.read() {
        match event {
            GameEvents::EndGame => {
                ui_state.set(UiState::EndGame);
                game_state.set(GameState::EndGame);
            }
            GameEvents::Pause => {
                ui_state.set(UiState::Paused);
                game_state.set(GameState::Paused);
            }
        }
    }
}

fn handle_settings_events(
    mut settings: ResMut<GameSettings>,
    mut settings_events: EventReader<SettingsEvents>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let Ok(mut main_window) = windows.get_single_mut() else {
        return;
    };
    for event in settings_events.read() {
        match event {
            SettingsEvents::DisplayWindowed => {
                main_window.mode = WindowMode::Windowed;
            }
            SettingsEvents::DisplayFullScreen => {
                main_window.mode = WindowMode::Fullscreen;
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
