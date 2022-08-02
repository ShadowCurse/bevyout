use bevy::prelude::*;

use crate::config::UiConfig;
use crate::game::GameState;
use crate::ui::{spawn_button, UiState};
use crate::utils::remove_all_with;

pub struct EndGamePlugin;

impl Plugin for EndGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(UiState::EndGame).with_system(end_game_setup));
        app.add_system_set(SystemSet::on_update(UiState::EndGame).with_system(button_system));
        app.add_system_set(
            SystemSet::on_exit(UiState::EndGame).with_system(remove_all_with::<UiEndGameElement>),
        );
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct UiEndGameElement;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum EndGameButton {
    Retry,
    BackToMainMenu,
}

fn end_game_setup(mut commands: Commands, config: Res<UiConfig>) {
    let ui = commands
        .spawn_bundle(NodeBundle {
            style: config.menu_style.clone(),
            color: config.menu_color.into(),
            ..default()
        })
        .insert(UiEndGameElement)
        .id();

    spawn_button(
        &mut commands,
        ui,
        &config,
        EndGameButton::Retry,
        UiEndGameElement,
    );
    spawn_button(
        &mut commands,
        ui,
        &config,
        EndGameButton::BackToMainMenu,
        UiEndGameElement,
    );
}

fn button_system(
    style: Res<UiConfig>,
    mut ui_state: ResMut<State<UiState>>,
    mut game_state: ResMut<State<GameState>>,
    mut interaction_query: Query<
        (&EndGameButton, &Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (button, interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = style.btn_color_pressed.into();
                match button {
                    EndGameButton::Retry => {
                        ui_state.replace(UiState::InGame).unwrap();
                        game_state.replace(GameState::InGame).unwrap();
                    }
                    EndGameButton::BackToMainMenu => {
                        ui_state.replace(UiState::MainMenu).unwrap();
                        game_state.replace(GameState::NotInGame).unwrap();
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
