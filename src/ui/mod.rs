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
        app.init_state::<UiState>();
        app.enable_state_scoped_entities::<UiState>();

        app.add_plugins((
            CursorPlugin,
            EndGamePlugin,
            HudPlugin,
            MainMenuPlugin,
            PausedPlugin,
            SettingsPlugin,
        ));
    }
}

/// Ui states
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum UiState {
    #[default]
    MainMenu,
    InGame,
    Paused,
    Settings,
    EndGame,
}

fn spawn_button<B>(child_builder: &mut ChildBuilder, style: &UiConfig, button: B)
where
    B: Component + std::fmt::Debug,
{
    child_builder
        .spawn(ButtonBundle {
            style: style.btn_style.clone(),
            background_color: style.btn_color_normal.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(format!("{:?}", button), style.text_style.clone()),
                ..default()
            });
        })
        .insert(button);
}
