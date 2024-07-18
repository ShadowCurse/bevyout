use bevy::prelude::*;

use crate::config::{GameSettings, UiConfig};
use crate::events::SettingsEvents;
use crate::game::GameState;
use crate::ui::{spawn_button, UiState};

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UiState::Settings), settings_setup);

        app.add_systems(
            Update,
            (button_system, volume_update).run_if(in_state(UiState::Settings)),
        );
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum SettingsButton {
    DisplayFullScreen,
    DisplayWindowed,
    VolumeUp,
    VolumeDown,
    Back,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct UiSettingsVolume;

fn settings_setup(mut commands: Commands, config: Res<UiConfig>) {
    commands
        // Vertical layout
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                grid_auto_flow: GridAutoFlow::Row,
                margin: UiRect::all(Val::Auto),
                justify_items: JustifyItems::Center,
                justify_self: JustifySelf::Center,
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                ..default()
            },
            background_color: config.menu_color.into(),
            ..default()
        })
        .insert(StateScoped(UiState::Settings))
        .with_children(|builder| {
            // Display and Sound settings
            // Horizontal layout
            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        grid_auto_flow: GridAutoFlow::Row,
                        ..default()
                    },
                    background_color: config.menu_color.into(),
                    ..default()
                })
                .with_children(|builder| {
                    spawn_button(builder, &config, SettingsButton::DisplayFullScreen);
                    spawn_button(builder, &config, SettingsButton::DisplayWindowed);
                });
            // Sound
            // Horizontal layout
            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        grid_auto_flow: GridAutoFlow::Row,
                        margin: UiRect::all(Val::Auto),
                        align_items: AlignItems::Center,
                        align_self: AlignSelf::Center,
                        ..default()
                    },
                    background_color: config.menu_color.into(),
                    ..default()
                })
                .with_children(|builder| {
                    // Volume up and down
                    spawn_button(builder, &config, SettingsButton::VolumeDown);
                    spawn_button(builder, &config, SettingsButton::VolumeUp);
                    // Volume value
                    builder
                        .spawn(TextBundle {
                            text: Text::from_section(
                                format!("Volume: {}%", 100),
                                config.text_style.clone(),
                            ),
                            ..default()
                        })
                        .insert(UiSettingsVolume);
                });
            // Back button
            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        grid_auto_flow: GridAutoFlow::Row,
                        margin: UiRect::all(Val::Auto),
                        align_items: AlignItems::Center,
                        align_self: AlignSelf::Center,
                        ..default()
                    },
                    background_color: config.menu_color.into(),
                    ..default()
                })
                .with_children(|builder| {
                    spawn_button(builder, &config, SettingsButton::Back);
                });
        });
}

fn volume_update(
    settings: Res<GameSettings>,
    mut volume: Query<&mut Text, With<UiSettingsVolume>>,
) {
    let mut text = volume.single_mut();
    let str = format!("Volume: {}%", (settings.sound_volume * 100.0) as u32);
    text.sections[0].value = str;
}

fn button_system(
    style: Res<UiConfig>,
    game_state: Res<State<GameState>>,
    mut ui_state: ResMut<NextState<UiState>>,
    mut interaction_query: Query<
        (&SettingsButton, &Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut settings_events: EventWriter<SettingsEvents>,
) {
    for (button, interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = style.btn_color_pressed.into();
                match button {
                    SettingsButton::Back => match game_state.get() {
                        GameState::Paused => {
                            ui_state.set(UiState::Paused);
                        }
                        GameState::NotInGame => {
                            ui_state.set(UiState::MainMenu);
                        }
                        _ => unreachable!(
                            "Setting should only be accessed from main menu or pause menu"
                        ),
                    },
                    SettingsButton::DisplayWindowed => {
                        settings_events.send(SettingsEvents::DisplayWindowed);
                    }
                    SettingsButton::DisplayFullScreen => {
                        settings_events.send(SettingsEvents::DisplayFullScreen);
                    }
                    SettingsButton::VolumeUp => {
                        settings_events.send(SettingsEvents::VolumeUp);
                    }
                    SettingsButton::VolumeDown => {
                        settings_events.send(SettingsEvents::VolumeDown);
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
