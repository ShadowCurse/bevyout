use bevy::prelude::*;

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreStartup,
            (setup_game_settings, setup_game_config, setup_ui_config),
        );
    }
}

#[derive(Debug, Clone, Resource)]
pub struct GameSettings {
    pub sound_volume: f32,
}

pub fn setup_game_settings(mut commands: Commands) {
    let settings = GameSettings { sound_volume: 0.1 };

    commands.insert_resource(settings);
}

#[derive(Debug, Clone, Resource)]
pub struct GameConfig {
    pub ball_radius: f32,
    pub ball_speed: f32,
    pub ball_max_speed_multiplier: f32,
    pub ball_base_color: Color,
    pub ball_max_speed_color: Color,

    pub bricks_pos_x: f32,
    pub bricks_pos_y: f32,
    pub bricks_width: f32,
    pub bricks_height: f32,
    pub bricks_cols: u32,
    pub bricks_rows: u32,
    pub bricks_gap_x: f32,
    pub bricks_gap_y: f32,
    pub bricks_health: u32,
    pub bricks_sound: Handle<AudioSource>,
    pub bricks_color: Color,

    pub platform_width: f32,
    pub platform_height: f32,
    pub platform_speed: f32,
    pub platform_lifes: u32,
    pub platform_color: Color,

    pub scene_width: f32,
    pub scene_height: f32,
    pub scene_border_color: Color,
}

pub fn setup_game_config(mut commands: Commands, asset_server: Res<AssetServer>) {
    let config = GameConfig {
        ball_radius: 5.0,
        ball_speed: 100.0,
        ball_max_speed_multiplier: 2.5,
        ball_base_color: Color::srgb_u8(0x00, 0x75, 0x5f),
        ball_max_speed_color: Color::srgb_u8(0xdd, 0xdf, 0x00),

        bricks_pos_x: 100.0,
        bricks_pos_y: 200.0,
        bricks_width: 15.0,
        bricks_height: 10.0,
        bricks_cols: 9,
        bricks_rows: 5,
        bricks_gap_x: 5.0,
        bricks_gap_y: 5.0,
        bricks_health: 1,
        bricks_sound: asset_server.load("audio/bling.ogg"),
        bricks_color: Color::srgb_u8(0xe8, 0x5d, 0x04),

        platform_width: 50.0,
        platform_height: 10.0,
        platform_speed: 100.0,
        platform_lifes: 5,
        platform_color: Color::srgb_u8(0x6a, 0x04, 0x0f),

        scene_width: 200.0,
        scene_height: 350.0,
        scene_border_color: Color::srgb_u8(0xfa, 0xa3, 0x07),
    };

    // camera
    let cam_pos = Vec3::new(config.scene_width / 2.0, config.scene_height / 2.0, 500.0);
    let cam_look_at = Vec3::new(config.scene_width / 2.0, config.scene_height / 2.0, 0.0);

    commands.insert_resource(config);

    commands.spawn(Camera3dBundle {
        camera: Camera { ..default() },
        transform: Transform::from_translation(cam_pos).looking_at(cam_look_at, Vec3::Y),
        ..default()
    });

    // turn off ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.0,
    });

    // light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 300.0,
            ..default()
        },
        ..default()
    });
}

#[derive(Debug, Clone, Resource)]
pub struct UiConfig {
    pub btn_style: Style,
    pub btn_color_normal: Color,
    pub btn_color_hover: Color,
    pub btn_color_pressed: Color,
    pub menu_style: Style,
    pub menu_color: Color,
    pub text_style: TextStyle,
}

fn setup_ui_config(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(UiConfig {
        btn_style: Style {
            width: Val::Percent(150.0),
            margin: UiRect::all(Val::Percent(10.0)),
            padding: UiRect::all(Val::Percent(10.0)),
            justify_items: JustifyItems::Center,
            justify_self: JustifySelf::Center,
            align_items: AlignItems::Center,
            align_self: AlignSelf::Center,
            ..default()
        },
        btn_color_normal: Color::srgb(0.15, 0.15, 0.15),
        btn_color_hover: Color::srgb(0.25, 0.25, 0.25),
        btn_color_pressed: Color::srgb(0.35, 0.75, 0.35),
        menu_style: Style {
            display: Display::Grid,
            margin: UiRect::all(Val::Auto),
            justify_items: JustifyItems::Center,
            justify_self: JustifySelf::Center,
            align_items: AlignItems::Center,
            align_self: AlignSelf::Center,
            ..default()
        },
        menu_color: Color::NONE,
        text_style: TextStyle {
            font: asset_server.load("fonts/monaco.ttf"),
            font_size: 20.0,
            color: Color::srgb_u8(0xfa, 0xa3, 0x07),
        },
    });
}
