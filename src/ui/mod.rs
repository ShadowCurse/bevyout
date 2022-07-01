use bevy::prelude::*;

pub mod cursor;
pub mod hud;
pub mod main_menu;
pub mod paused;
pub mod settings;

use cursor::CursorPlugin;
use hud::HudPlugin;
use main_menu::MainMenuPlugin;
use paused::PausedPlugin;
use settings::SettingsPlugin;

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

        app.add_state(UiState::MainMenu);

        app.add_plugin(CursorPlugin);
        app.add_plugin(HudPlugin);
        app.add_plugin(MainMenuPlugin);
        app.add_plugin(PausedPlugin);
        app.add_plugin(SettingsPlugin);
    }
}

/// Ui states
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UiState {
    MainMenu,
    InGame,
    Paused,
    Settings,
}

#[derive(Component, Debug, Clone)]
pub struct UiStyle {
    btn_style: Style,
    btn_color_normal: Color,
    btn_color_hover: Color,
    btn_color_pressed: Color,
    menu_style: Style,
    menu_color: Color,
    text_style: TextStyle,
}

#[derive(Component, Debug, Clone)]
pub struct GameUiCamera;

fn ui_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
        btn_color_normal: NORMAL_BUTTON,
        btn_color_hover: HOVERED_BUTTON,
        btn_color_pressed: PRESSED_BUTTON,
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
        menu_color: UI_BACKGROUND,
        text_style: TextStyle {
            font: asset_server.load("fonts/monaco.ttf"),
            font_size: 20.0,
            color: UI_FOREGROUND,
        },
    });
}

fn spawn_button<B, M>(
    commands: &mut Commands,
    parent: Entity,
    style: &UiStyle,
    button: B,
    marker: M,
) where
    B: Component + std::fmt::Debug,
    M: Component + Copy,
{
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
                        style.text_style.clone(),
                        Default::default(),
                    ),
                    ..default()
                })
                .insert(marker);
        })
        .insert(button)
        .insert(marker)
        .id();

    commands.entity(parent).push_children(&[child]);
}
