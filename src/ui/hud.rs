use bevy::prelude::*;

use crate::config::UiConfig;
use crate::game::bricks::Score;
use crate::game::platform::PlatformLifes;
use crate::ui::UiState;
use crate::utils::remove_all_with;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(UiState::InGame).with_system(hud_setup));
        app.add_system_set(SystemSet::on_update(UiState::InGame).with_system(hud_update));
        app.add_system_set(
            SystemSet::on_pause(UiState::InGame).with_system(remove_all_with::<UiHudElement>),
        );
        app.add_system_set(SystemSet::on_resume(UiState::InGame).with_system(hud_setup));
        app.add_system_set(
            SystemSet::on_exit(UiState::InGame).with_system(remove_all_with::<UiHudElement>),
        );
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct UiHudElement;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct UiLifesCount;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct UiScore;

fn hud_setup(mut command: Commands, config: Res<UiConfig>) {
    command
        .spawn_bundle(NodeBundle {
            style: Style {
                align_self: AlignSelf::Auto,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexEnd,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(UiHudElement)
        .with_children(|builder| {
            // lifes count
            builder
                .spawn_bundle(TextBundle {
                    text: Text::from_section("Lifes: ---", config.text_style.clone())
                        .with_alignment(TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Left,
                        }),
                    ..default()
                })
                .insert(UiLifesCount)
                .insert(UiHudElement);

            // score
            builder
                .spawn_bundle(TextBundle {
                    text: Text::from_section("Score: ---", config.text_style.clone())
                        .with_alignment(TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Left,
                        }),
                    ..default()
                })
                .insert(UiScore)
                .insert(UiHudElement);
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
