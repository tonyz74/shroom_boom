use bevy::prelude::*;
use bevy_debug_text_overlay::screen_print;
use crate::assets::BossAssets;
use crate::bossfight::Boss;
use crate::bossfight::consts::{BOSS_EASY_HEALTH_THRESHOLD, BOSS_HARD_HEALTH_THRESHOLD, BOSS_HEALTH, BOSS_MEDIUM_HEALTH_THRESHOLD};
use crate::bossfight::stage::BossStage;
use crate::combat::Health;
use crate::entity_states::Die;
use crate::ui::hud::index_for_value;

#[derive(Resource, Debug, Clone)]
pub struct BossHud {
    entity: Entity
}

#[derive(Component, Debug, Clone, PartialEq)]
pub struct BossBar;

pub fn register_boss_bar(app: &mut App) {
    app.insert_resource(BossHud { entity: Entity::from_raw(0) }).add_system_set(
        SystemSet::new()
            .with_system(spawn_bossbar)
            .with_system(sync_bossbar)
            .with_system(remove_bossbar)
    );
}

fn spawn_bossbar(
    mut hud: ResMut<BossHud>,
    mut commands: Commands,
    assets: Res<BossAssets>,
    boss_q: Query<Entity, Added<Boss>>
) {
    if boss_q.is_empty() {
        return;
    }

    let id = commands.spawn(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::FlexEnd,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        ..default()
    }).with_children(|parent| {
        parent.spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(128.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceEvenly,
                align_self: AlignSelf::FlexEnd,
                ..default()
            },
            ..default()
        }).with_children(|parent| {
            parent.spawn((
                ImageBundle {
                    style: Style {
                        size: Size::new(Val::Px(128.0), Val::Px(256.0)),
                        align_self: AlignSelf::Center,
                        ..default()
                    },
                    image: assets.health_bar_easy[0].clone().into(),
                    ..default()
                },
                BossBar
            ));
        });
    }).id();

    hud.entity = id;
}

fn sync_bossbar(
    boss_health: Query<(&BossStage, &Health)>,
    mut q: Query<&mut UiImage, With<BossBar>>,
    assets: Res<BossAssets>
) {
    if q.is_empty() || boss_health.is_empty() {
        return;
    }

    let (stage, health) = boss_health.single();
    let mut img = q.single_mut();

    let (max_hp, next_max, parts, imgs) = match stage {
        BossStage::Waiting | BossStage::SummonEasy | BossStage::VulnerableEasy => {
            (BOSS_HEALTH, BOSS_EASY_HEALTH_THRESHOLD, 7, &assets.health_bar_easy)
        },
        BossStage::SummonMedium | BossStage::VulnerableMedium => {
            (BOSS_EASY_HEALTH_THRESHOLD, BOSS_MEDIUM_HEALTH_THRESHOLD, 7, &assets.health_bar_medium)
        },
        BossStage::SummonHard | BossStage::VulnerableHard => {
            (BOSS_MEDIUM_HEALTH_THRESHOLD, BOSS_HARD_HEALTH_THRESHOLD, 7, &assets.health_bar_hard)
        },
        BossStage::Enraged => {
            (BOSS_HARD_HEALTH_THRESHOLD, 0, 14, &assets.health_bar_enraged)
        }
    };

    let idx = percent_idx(health.hp - next_max, max_hp - next_max, parts);
    img.0 = imgs[idx].clone()
}

fn percent_idx(hp: i32, max_hp: i32, parts: usize) -> usize {
    if hp < 0 || hp > max_hp {
        return parts - 1;
    }

    let p = ((hp as f32 / max_hp as f32) * parts as f32).ceil();
    (parts - p as usize).clamp(0, parts - 1)
}

fn remove_bossbar(
    mut commands: Commands,
    mut hud: ResMut<BossHud>,
    boss_q: Query<&Boss, Added<Die>>
) {
    if boss_q.is_empty() {
        return;
    }

    commands.entity(hud.entity).despawn_recursive();
    hud.entity = Entity::from_raw(0);
}