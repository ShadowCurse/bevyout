use bevy::prelude::*;

use crate::config::{GameSettings, UiConfig};
use crate::events::SettingsEvents;
use crate::ui::{spawn_button, UiState};
use crate::utils::remove_all_with;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(UiState::Settings).with_system(settings_setup));
        app.add_system_set(
            SystemSet::on_update(UiState::Settings)
                .with_system(button_system)
                .with_system(volume_update),
        );
        app.add_system_set(
            SystemSet::on_exit(UiState::Settings).with_system(remove_all_with::<UiSettingsElement>),
        );
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct UiSettingsElement;

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
        .spawn_bundle(NodeBundle {
            style: Style {
                margin: UiRect::all(Val::Auto),
                align_self: AlignSelf::Center,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                ..default()
            },
            color: config.menu_color.into(),
            ..default()
        })
        .insert(UiSettingsElement)
        .with_children(|builder| {
            // Back button
            builder
                .spawn_bundle(NodeBundle {
                    style: Style {
                        margin: UiRect::all(Val::Auto),
                        ..default()
                    },
                    color: config.menu_color.into(),
                    ..default()
                })
                .insert(UiSettingsElement)
                .with_children(|builder| {
                    spawn_button(builder, &config, SettingsButton::Back, UiSettingsElement);
                });
            // Display and Sound settings
            // Horizontal layout
            builder
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    color: config.menu_color.into(),
                    ..default()
                })
                .insert(UiSettingsElement)
                .with_children(|builder| {
                    // Display
                    // Horizontal layout
                    builder
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            color: config.menu_color.into(),
                            ..default()
                        })
                        .insert(UiSettingsElement)
                        .with_children(|builder| {
                            spawn_button(
                                builder,
                                &config,
                                SettingsButton::DisplayFullScreen,
                                UiSettingsElement,
                            );
                            spawn_button(
                                builder,
                                &config,
                                SettingsButton::DisplayWindowed,
                                UiSettingsElement,
                            );
                        });
                    // Sound
                    // Vertical layout
                    builder
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            color: config.menu_color.into(),
                            ..default()
                        })
                        .insert(UiSettingsElement)
                        .with_children(|builder| {
                            // Volume up and down
                            // Horizontal layout
                            builder
                                .spawn_bundle(NodeBundle {
                                    style: Style {
                                        flex_direction: FlexDirection::Row,
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    color: config.menu_color.into(),
                                    ..default()
                                })
                                .insert(UiSettingsElement)
                                .with_children(|builder| {
                                    spawn_button(
                                        builder,
                                        &config,
                                        SettingsButton::VolumeDown,
                                        UiSettingsElement,
                                    );
                                    spawn_button(
                                        builder,
                                        &config,
                                        SettingsButton::VolumeUp,
                                        UiSettingsElement,
                                    );
                                });
                            // Volume value
                            builder
                                .spawn_bundle(TextBundle {
                                    text: Text::from_section(
                                        format!("Volume: {}%", 100),
                                        config.text_style.clone(),
                                    ),
                                    ..default()
                                })
                                .insert(UiSettingsElement)
                                .insert(UiSettingsVolume);
                        });
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
    mut ui_state: ResMut<State<UiState>>,
    mut interaction_query: Query<
        (&SettingsButton, &Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut settings_events: EventWriter<SettingsEvents>,
) {
    for (button, interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = style.btn_color_pressed.into();
                match button {
                    SettingsButton::Back => {
                        ui_state.pop().unwrap();
                    }
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
