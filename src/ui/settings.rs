use bevy::prelude::*;

use crate::ui::{spawn_button, UiState, UiStyle};
use crate::utils::remove_all_with;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(UiState::Settings).with_system(settings_setup));
        app.add_system_set(SystemSet::on_update(UiState::Settings).with_system(button_system));
        app.add_system_set(
            SystemSet::on_exit(UiState::Settings).with_system(remove_all_with::<UiSettingsElement>),
        );
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct UiSettingsElement;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum SettingsButton {
    Gameplay,
    Display,
    Sound,
    Back,
}

fn settings_setup(mut commands: Commands, style: Res<UiStyle>) {
    let ui = commands
        .spawn_bundle(NodeBundle {
            style: style.menu_style.clone(),
            color: style.menu_color.into(),
            ..default()
        })
        .insert(UiSettingsElement)
        .id();

    spawn_button(
        &mut commands,
        ui,
        &style,
        SettingsButton::Gameplay,
        UiSettingsElement,
    );
    spawn_button(
        &mut commands,
        ui,
        &style,
        SettingsButton::Display,
        UiSettingsElement,
    );
    spawn_button(
        &mut commands,
        ui,
        &style,
        SettingsButton::Sound,
        UiSettingsElement,
    );
    spawn_button(
        &mut commands,
        ui,
        &style,
        SettingsButton::Back,
        UiSettingsElement,
    );
}

fn button_system(
    style: Res<UiStyle>,
    mut ui_state: ResMut<State<UiState>>,
    mut interaction_query: Query<
        (&SettingsButton, &Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (button, interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = style.btn_color_pressed.into();
                match button {
                    SettingsButton::Back => {
                        ui_state.pop().unwrap();
                    }
                    _ => {}
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
