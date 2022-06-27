use bevy::prelude::*;

mod main_menu;
mod paused;
mod settings;

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
        app.add_startup_system(ui_style_setup);

        app.add_state(UiState::MainMenu);

        app.add_plugin(MainMenuPlugin);
        app.add_plugin(PausedPlugin);
        app.add_plugin(SettingsPlugin);
    }
}

/// Game states
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
    btn_style_text: TextStyle,
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
        btn_style_text: TextStyle {
            font: asset_server.load("fonts/monaco.ttf"),
            font_size: 20.0,
            color: UI_FOREGROUND,
        },
    });
}
