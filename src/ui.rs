use bevy::prelude::*;

// TODO move to config file
const UI_WIDTH: f32 = 500.0;
const UI_HEIGHT: f32 = 300.0;
const BUTTON_WIDTH: f32 = 130.0;
const BUTTON_HEIGHT: f32 = 80.0;
const UI_BACKGROUND: Color = Color::GRAY;
const UI_FOREGROUND: Color = Color::ORANGE_RED;
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(ui_setup);
        app.add_system(button_system);
    }
}

fn ui_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Ui
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(UI_WIDTH), Val::Px(UI_HEIGHT)),
                // center button
                margin: UiRect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Stretch,
                ..default()
            },
            color: UI_BACKGROUND.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(BUTTON_WIDTH), Val::Px(BUTTON_HEIGHT)),
                        // center button
                        margin: UiRect::all(Val::Auto),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Start",
                            TextStyle {
                                font: asset_server.load("fonts/monaco.ttf"),
                                font_size: 20.0,
                                color: UI_FOREGROUND,
                            },
                            Default::default(),
                        ),
                        ..default()
                    });
                });

            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(BUTTON_WIDTH), Val::Px(BUTTON_HEIGHT)),
                        // center button
                        margin: UiRect::all(Val::Auto),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Settings",
                            TextStyle {
                                font: asset_server.load("fonts/monaco.ttf"),
                                font_size: 20.0,
                                color: UI_FOREGROUND,
                            },
                            Default::default(),
                        ),
                        ..default()
                    });
                });

            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(BUTTON_WIDTH), Val::Px(BUTTON_HEIGHT)),
                        // center button
                        margin: UiRect::all(Val::Auto),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Exit",
                            TextStyle {
                                font: asset_server.load("fonts/monaco.ttf"),
                                font_size: 20.0,
                                color: UI_FOREGROUND,
                            },
                            Default::default(),
                        ),
                        ..default()
                    });
                });
        });
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
