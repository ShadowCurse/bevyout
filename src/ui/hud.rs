use bevy::prelude::*;

use crate::config::UiConfig;
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
struct LifesCount;

fn hud_setup(mut cmd: Commands, config: Res<UiConfig>) {
    cmd.spawn_bundle(NodeBundle {
        color: UiColor(Color::rgba(0.5, 0.5, 0.5, 0.0)),
        style: Style {
            size: Size::new(Val::Auto, Val::Auto),
            margin: UiRect::all(Val::Auto),
            align_self: AlignSelf::Center,
            flex_direction: FlexDirection::ColumnReverse,
            align_items: AlignItems::Stretch,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    })
    .insert(UiHudElement);

    // lifes count
    cmd.spawn_bundle(TextBundle {
        text: Text::from_section("Lifes: ---", config.text_style.clone()).with_alignment(
            TextAlignment {
                vertical: VerticalAlign::Bottom,
                horizontal: HorizontalAlign::Right,
            },
        ),
        ..default()
    })
    .insert(LifesCount)
    .insert(UiHudElement);
}

fn hud_update(
    platform_lifes: Res<PlatformLifes>,
    mut lifes_count: Query<&mut Text, With<LifesCount>>,
) {
    let mut text = lifes_count.single_mut();
    let str = format!("Lifes: {} / {}", platform_lifes.current, platform_lifes.max);
    text.sections[0].value = str;
}
