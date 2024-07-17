use bevy::prelude::*;

use crate::config::UiConfig;
use crate::game::GameState;
use crate::ui::{spawn_button, UiState};

pub struct PausedPlugin;

impl Plugin for PausedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UiState::Paused), paused_setup);
        app.add_systems(Update, button_system.run_if(in_state(UiState::Paused)));
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum PausedButton {
    Resume,
    Settings,
    BackToMainMenu,
}

fn paused_setup(mut commands: Commands, config: Res<UiConfig>) {
    commands
        .spawn(NodeBundle {
            style: config.menu_style.clone(),
            background_color: config.menu_color.into(),
            ..default()
        })
        .insert(StateScoped(UiState::Paused))
        .with_children(|builder| {
            spawn_button(builder, &config, PausedButton::Resume);
            spawn_button(builder, &config, PausedButton::Settings);
            spawn_button(builder, &config, PausedButton::BackToMainMenu);
        });
}

fn button_system(
    config: Res<UiConfig>,
    mut ui_state: ResMut<NextState<UiState>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&PausedButton, &Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (button, interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = config.btn_color_pressed.into();
                match button {
                    PausedButton::Resume => {
                        // ui_state.pop().unwrap();
                        // game_state.pop().unwrap();
                    }
                    PausedButton::Settings => {
                        ui_state.set(UiState::Settings);
                    }
                    PausedButton::BackToMainMenu => {
                        ui_state.set(UiState::MainMenu);
                        game_state.set(GameState::NotInGame);
                    }
                }
            }
            Interaction::Hovered => {
                *color = config.btn_color_hover.into();
            }
            Interaction::None => {
                *color = config.btn_color_normal.into();
            }
        }
    }
}
