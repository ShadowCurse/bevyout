use bevy::{app::AppExit, prelude::*};

use crate::game::GameState;
use crate::ui::{UiState, UiStyle};
use crate::utils::remove_all_with;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(UiState::MainMenu).with_system(main_menu_setup));
        app.add_system_set(SystemSet::on_update(UiState::MainMenu).with_system(button_system));
        app.add_system_set(
            SystemSet::on_pause(UiState::MainMenu)
                .with_system(remove_all_with::<UiMainMenuElement>),
        );
        app.add_system_set(
            SystemSet::on_exit(UiState::MainMenu).with_system(remove_all_with::<UiMainMenuElement>),
        );
        app.add_system_set(SystemSet::on_resume(UiState::MainMenu).with_system(main_menu_setup));
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct UiMainMenuElement;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum MainMenuButton {
    Start,
    Settings,
    Exit,
}

fn main_menu_setup(mut commands: Commands, style: Res<UiStyle>) {
    let ui = commands
        .spawn_bundle(NodeBundle {
            style: style.menu_style.clone(),
            color: style.menu_color.into(),
            ..default()
        })
        .insert(UiMainMenuElement)
        .id();

    spawn_button(&mut commands, ui, &style, MainMenuButton::Start);
    spawn_button(&mut commands, ui, &style, MainMenuButton::Settings);
    spawn_button(&mut commands, ui, &style, MainMenuButton::Exit);
}

fn spawn_button(commands: &mut Commands, parent: Entity, style: &UiStyle, button: MainMenuButton) {
    let child = commands
        .spawn_bundle(ButtonBundle {
            style: style.btn_style.clone(),
            color: style.btn_color_normal.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text::with_section(
                        format!("{:?}", button),
                        style.btn_style_text.clone(),
                        Default::default(),
                    ),
                    ..default()
                })
                .insert(UiMainMenuElement);
        })
        .insert(UiMainMenuElement)
        .insert(button)
        .id();

    commands.entity(parent).push_children(&[child]);
}

fn button_system(
    style: Res<UiStyle>,
    mut ui_state: ResMut<State<UiState>>,
    mut game_state: ResMut<State<GameState>>,
    mut interaction_query: Query<
        (&MainMenuButton, &Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut exit: EventWriter<AppExit>,
) {
    for (button, interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = style.btn_color_pressed.into();
                match button {
                    MainMenuButton::Start => {
                        ui_state.push(UiState::InGame).unwrap();
                        game_state.push(GameState::InGame).unwrap();
                    }
                    MainMenuButton::Settings => ui_state.push(UiState::Settings).unwrap(),
                    MainMenuButton::Exit => exit.send(AppExit),
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
