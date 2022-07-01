use bevy::prelude::*;

use crate::game::GameState;
use crate::ui::{spawn_button, UiState, UiStyle};
use crate::utils::remove_all_with;

pub struct PausedPlugin;

impl Plugin for PausedPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(UiState::Paused).with_system(paused_setup));
        app.add_system_set(SystemSet::on_update(UiState::Paused).with_system(button_system));
        app.add_system_set(
            SystemSet::on_pause(UiState::Paused).with_system(remove_all_with::<UiPausedElement>),
        );
        app.add_system_set(
            SystemSet::on_exit(UiState::Paused).with_system(remove_all_with::<UiPausedElement>),
        );
        app.add_system_set(SystemSet::on_resume(UiState::Paused).with_system(paused_setup));
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct UiPausedElement;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum PausedButton {
    Resume,
    Settings,
    BackToMainMenu,
}

fn paused_setup(mut commands: Commands, style: Res<UiStyle>) {
    let ui = commands
        .spawn_bundle(NodeBundle {
            style: style.menu_style.clone(),
            color: style.menu_color.into(),
            ..default()
        })
        .insert(UiPausedElement)
        .id();

    spawn_button(
        &mut commands,
        ui,
        &style,
        PausedButton::Resume,
        UiPausedElement,
    );
    spawn_button(
        &mut commands,
        ui,
        &style,
        PausedButton::Settings,
        UiPausedElement,
    );
    spawn_button(
        &mut commands,
        ui,
        &style,
        PausedButton::BackToMainMenu,
        UiPausedElement,
    );
}

fn button_system(
    style: Res<UiStyle>,
    mut ui_state: ResMut<State<UiState>>,
    mut game_state: ResMut<State<GameState>>,
    mut interaction_query: Query<
        (&PausedButton, &Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (button, interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = style.btn_color_pressed.into();
                match button {
                    PausedButton::Resume => {
                        ui_state.pop().unwrap();
                        game_state.pop().unwrap();
                    }
                    PausedButton::Settings => {
                        ui_state.push(UiState::Settings).unwrap();
                    }
                    PausedButton::BackToMainMenu => {
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
