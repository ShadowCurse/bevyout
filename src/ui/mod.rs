use bevy::prelude::*;

pub mod cursor;
pub mod end_game;
pub mod hud;
pub mod main_menu;
pub mod paused;
pub mod settings;

use crate::config::UiConfig;
use cursor::CursorPlugin;
use end_game::EndGamePlugin;
use hud::HudPlugin;
use main_menu::MainMenuPlugin;
use paused::PausedPlugin;
use settings::SettingsPlugin;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(UiState::MainMenu);

        app.add_plugin(CursorPlugin);
        app.add_plugin(EndGamePlugin);
        app.add_plugin(HudPlugin);
        app.add_plugin(MainMenuPlugin);
        app.add_plugin(PausedPlugin);
        app.add_plugin(SettingsPlugin);
    }
}

/// Ui states
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UiState {
    MainMenu,
    InGame,
    Paused,
    Settings,
    EndGame,
}

#[derive(Component, Debug, Clone)]
pub struct GameUiCamera;

fn spawn_button<B, M>(child_builder: &mut ChildBuilder, style: &UiConfig, button: B, marker: M)
where
    B: Component + std::fmt::Debug,
    M: Component + Copy,
{
    child_builder
        .spawn_bundle(ButtonBundle {
            style: style.btn_style.clone(),
            color: style.btn_color_normal.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text::from_section(format!("{:?}", button), style.text_style.clone()),
                    ..default()
                })
                .insert(marker);
        })
        .insert(button)
        .insert(marker);
}
