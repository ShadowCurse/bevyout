use bevy::prelude::*;

use crate::config::UiConfig;
use crate::game::bricks::Score;
use crate::game::platform::PlatformLifes;
use crate::ui::UiState;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UiState::InGame), hud_setup);
        app.add_systems(Update, hud_update.run_if(in_state(UiState::InGame)));
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct UiLifesCount;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct UiScore;

fn hud_setup(mut command: Commands, config: Res<UiConfig>) {
    command
        .spawn(NodeBundle {
            style: Style {
                align_self: AlignSelf::Auto,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexEnd,
                ..default()
            },
            background_color: Color::NONE.into(),
            ..default()
        })
        .insert(StateScoped(UiState::MainMenu))
        .with_children(|builder| {
            // lifes count
            builder
                .spawn(TextBundle {
                    text: Text::from_section("Lifes: ---", config.text_style.clone()),
                    // .with_alignment(TextAlignment {
                    //     vertical: VerticalAlign::Center,
                    //     horizontal: HorizontalAlign::Left,
                    // }),
                    ..default()
                })
                .insert(UiLifesCount);

            // score
            builder
                .spawn(TextBundle {
                    text: Text::from_section("Score: ---", config.text_style.clone()),
                    // .with_alignment(TextAlignment {
                    //     vertical: VerticalAlign::Center,
                    //     horizontal: HorizontalAlign::Left,
                    // }),
                    ..default()
                })
                .insert(UiScore);
        });
}

fn hud_update(
    platform_lifes: Res<PlatformLifes>,
    score: Res<Score>,
    mut ui_lifes_count: Query<&mut Text, (With<UiLifesCount>, Without<UiScore>)>,
    mut ui_score: Query<&mut Text, (With<UiScore>, Without<UiLifesCount>)>,
) {
    let mut text = ui_lifes_count.single_mut();
    let str = format!("Lifes: {} / {}", platform_lifes.current, platform_lifes.max);
    text.sections[0].value = str;

    let mut text = ui_score.single_mut();
    let str = format!("Score: {}", score.score);
    text.sections[0].value = str;
}
