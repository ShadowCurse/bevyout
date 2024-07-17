use bevy::{app::AppExit, prelude::*};

use crate::config::UiConfig;
use crate::game::GameState;
use crate::ui::{spawn_button, UiState};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UiState::MainMenu), main_menu_setup);
        app.add_systems(Update, button_system.run_if(in_state(UiState::MainMenu)));
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum MainMenuButton {
    Start,
    Settings,
    Exit,
}

fn main_menu_setup(mut commands: Commands, config: Res<UiConfig>) {
    commands
        .spawn(NodeBundle {
            style: config.menu_style.clone(),
            background_color: config.menu_color.into(),
            ..default()
        })
        .insert(StateScoped(UiState::MainMenu))
        .with_children(|builder| {
            spawn_button(builder, &config, MainMenuButton::Start);
            spawn_button(builder, &config, MainMenuButton::Settings);
            spawn_button(builder, &config, MainMenuButton::Exit);
        });
}

fn button_system(
    style: Res<UiConfig>,
    mut ui_state: ResMut<NextState<UiState>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&MainMenuButton, &Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut exit: EventWriter<AppExit>,
) {
    for (button, interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = style.btn_color_pressed.into();
                match button {
                    MainMenuButton::Start => {
                        ui_state.set(UiState::InGame);
                        game_state.set(GameState::InGame);
                    }
                    MainMenuButton::Settings => ui_state.set(UiState::Settings),
                    MainMenuButton::Exit => _ = exit.send(AppExit::Success),
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
