use bevy::prelude::*;

use crate::config::UiConfig;
use crate::game::GameState;
use crate::ui::{spawn_button, UiState};

pub struct EndGamePlugin;

impl Plugin for EndGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UiState::EndGame), end_game_setup);
        app.add_systems(Update, button_system.run_if(in_state(UiState::EndGame)));
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum EndGameButton {
    Retry,
    BackToMainMenu,
}

fn end_game_setup(mut commands: Commands, config: Res<UiConfig>) {
    commands
        .spawn(NodeBundle {
            style: config.menu_style.clone(),
            background_color: config.menu_color.into(),
            ..default()
        })
        .insert(StateScoped(UiState::MainMenu))
        .with_children(|builder| {
            spawn_button(builder, &config, EndGameButton::Retry);
            spawn_button(builder, &config, EndGameButton::BackToMainMenu);
        });
}

fn button_system(
    style: Res<UiConfig>,
    mut ui_state: ResMut<NextState<UiState>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&EndGameButton, &Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (button, interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = style.btn_color_pressed.into();
                match button {
                    EndGameButton::Retry => {
                        ui_state.set(UiState::InGame);
                        game_state.set(GameState::InGame);
                    }
                    EndGameButton::BackToMainMenu => {
                        ui_state.set(UiState::MainMenu);
                        game_state.set(GameState::NotInGame);
                    }
                }
            }
            Interaction::Hovered => {
                *color = style.btn_color_hover.into();
            }
            Interaction::None => {
                *color = style.btn_color_normal.into();
            }
        }
    }
}
