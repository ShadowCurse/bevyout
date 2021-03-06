use bevy::prelude::*;

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_game);
        app.add_startup_system(setup_ui);
    }
}

#[derive(Debug)]
pub struct GameConfig {
    pub ball_radius: f32,
    pub ball_speed: f32,

    pub bricks_pos_x: f32,
    pub bricks_pos_y: f32,
    pub bricks_width: f32,
    pub bricks_height: f32,
    pub bricks_cols: u32,
    pub bricks_rows: u32,
    pub bricks_gap_x: f32,
    pub bricks_gap_y: f32,
    pub bricks_health: u32,

    pub platform_width: f32,
    pub platform_height: f32,
    pub platform_speed: f32,
    pub platform_lifes: u32,

    pub scene_width: f32,
    pub scene_height: f32,
    pub scene_border_color: Color,
}

pub fn setup_game(mut commands: Commands) {
    let config = GameConfig {
        ball_radius: 5.0,
        ball_speed: 100.0,

        bricks_pos_x: 100.0,
        bricks_pos_y: 200.0,
        bricks_width: 15.0,
        bricks_height: 10.0,
        bricks_cols: 9,
        bricks_rows: 5,
        bricks_gap_x: 5.0,
        bricks_gap_y: 5.0,
        bricks_health: 1,

        platform_width: 50.0,
        platform_height: 10.0,
        platform_speed: 100.0,
        platform_lifes: 5,

        scene_width: 200.0,
        scene_height: 350.0,
        scene_border_color: Color::WHITE,
    };

    // camera
    let cam_pos = Vec3::new(
        config.scene_width as f32 / 2.0,
        config.scene_height as f32 / 2.0,
        500.0,
    );
    let cam_look_at = Vec3::new(
        config.scene_width as f32 / 2.0,
        config.scene_height as f32 / 2.0,
        0.0,
    );

    commands.insert_resource(config);

    commands.spawn_bundle(Camera3dBundle {
        camera: Camera {
            priority: 1,
            ..default()
        },
        transform: Transform::from_translation(cam_pos).looking_at(cam_look_at, Vec3::Y),
        ..default()
    });

    // light
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            ..Default::default()
        },
        ..Default::default()
    });
}

#[derive(Debug)]
pub struct UiConfig {
    pub btn_style: Style,
    pub btn_color_normal: Color,
    pub btn_color_hover: Color,
    pub btn_color_pressed: Color,
    pub menu_style: Style,
    pub menu_color: Color,
    pub text_style: TextStyle,

    pub cursor_color: Color,
    pub cursor_radius: f32,
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(UiConfig {
        btn_style: Style {
            size: Size::new(Val::Px(200.0), Val::Px(100.0)),
            // center button
            margin: UiRect::all(Val::Auto),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            ..default()
        },
        btn_color_normal: Color::rgb(0.15, 0.15, 0.15),
        btn_color_hover: Color::rgb(0.25, 0.25, 0.25),
        btn_color_pressed: Color::rgb(0.35, 0.75, 0.35),
        menu_style: Style {
            size: Size::new(Val::Px(500.0), Val::Px(300.0)),
            // center button
            margin: UiRect::all(Val::Auto),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Stretch,
            ..default()
        },
        menu_color: Color::GRAY,
        text_style: TextStyle {
            font: asset_server.load("fonts/monaco.ttf"),
            font_size: 20.0,
            color: Color::ORANGE,
        },

        cursor_color: Color::GREEN,
        cursor_radius: 2.0,
    });
}
