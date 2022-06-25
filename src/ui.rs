use bevy::{app::AppExit, prelude::*};

use crate::game::physics::PhysicsState;
use crate::utils::remove_all_with;
use crate::AppState;

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
        app.add_startup_system(ui_style_setup);
        app.add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(main_menu_setup));
        app.add_system_set(SystemSet::on_update(AppState::MainMenu).with_system(button_system));
        app.add_system_set(
            SystemSet::on_exit(AppState::MainMenu).with_system(remove_all_with::<UiElement>),
        );
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct UiElement;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum UiButtons {
    Start,
    Settings,
    Exit,
}

#[derive(Component, Debug, Clone)]
pub struct UiStyle {
    btn_style: Style,
    btn_style_text: TextStyle,
    menu_style: Style,
}

fn ui_style_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(UiStyle {
        btn_style: Style {
            size: Size::new(Val::Px(BUTTON_WIDTH), Val::Px(BUTTON_HEIGHT)),
            // center button
            margin: UiRect::all(Val::Auto),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            ..default()
        },
        btn_style_text: TextStyle {
            font: asset_server.load("fonts/monaco.ttf"),
            font_size: 20.0,
            color: UI_FOREGROUND,
        },
        menu_style: Style {
            size: Size::new(Val::Px(UI_WIDTH), Val::Px(UI_HEIGHT)),
            // center button
            margin: UiRect::all(Val::Auto),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Stretch,
            ..default()
        },
    });
}

fn main_menu_setup(mut commands: Commands, style: Res<UiStyle>) {
    let ui = commands
        .spawn_bundle(NodeBundle {
            style: style.menu_style.clone(),
            color: UI_BACKGROUND.into(),
            ..default()
        })
        .insert(UiElement)
        .id();

    spawn_button(&mut commands, ui, &style, UiButtons::Start);
    spawn_button(&mut commands, ui, &style, UiButtons::Settings);
    spawn_button(&mut commands, ui, &style, UiButtons::Exit);
}

fn spawn_button(commands: &mut Commands, parent: Entity, style: &UiStyle, button: UiButtons) {
    let child = commands
        .spawn_bundle(ButtonBundle {
            style: style.btn_style.clone(),
            color: NORMAL_BUTTON.into(),
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
                .insert(UiElement);
        })
        .insert(UiElement)
        .insert(button)
        .id();

    commands.entity(parent).push_children(&[child]);
}

fn button_system(
    mut game_state: ResMut<State<AppState>>,
    mut physics_state: ResMut<State<PhysicsState>>,
    mut interaction_query: Query<
        (&UiButtons, &Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut exit: EventWriter<AppExit>,
) {
    for (button, interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                match button {
                    UiButtons::Start => {
                        game_state.set(AppState::InGame).unwrap();
                        physics_state.set(PhysicsState::Running).unwrap();
                    }
                    UiButtons::Settings => game_state.set(AppState::Settings).unwrap(),
                    UiButtons::Exit => exit.send(AppExit),
                }
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
