use bevy::prelude::*;

use crate::assets::UiAssets;
use crate::coin::drops::CoinHolder;
use crate::combat::Health;
use crate::player::ammo::Ammo;
use crate::player::Player;
use crate::state::GameState;


pub const PLAYER_HUD_DISPLAY_CHUNKS: usize = 24;


#[derive(Component, Debug, Copy, Clone)]
pub struct HealthBar;

#[derive(Component, Debug, Copy, Clone)]
pub struct AmmoBar;

#[derive(Component, Debug, Copy, Clone)]
pub struct WalletText;



#[derive(Clone, Resource, Debug)]
pub struct Hud {
    pub entity: Entity,
}

pub fn register_hud_ui_systems(app: &mut App) {
    app
        .insert_resource(Hud {
            entity: Entity::from_raw(0),
        })
        .add_system_set(
            SystemSet::on_enter(GameState::LevelTransition)
                .with_system(setup_hud)
        )
        .add_system_set(
            SystemSet::new()
                .with_system(sync_hud)
        );
}

fn setup_hud(
    mut commands: Commands,
    mut hud: ResMut<Hud>,
    ui_assets: Res<UiAssets>,
) {
    if hud.entity != Entity::from_raw(0) {
        return;
    }

    let text_style = ui_assets.text_style.clone();

    let entity = commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::FlexStart,
                        size: Size::new(Val::Px(200.0), Val::Px(280.0)),
                        border: UiRect::all(Val::Px(20.0)),
                        ..default()
                    },
                    ..default()
                }).with_children(|parent| {

                parent.spawn((
                    ImageBundle {
                        style: Style {
                            size: Size::new(Val::Px(192.0), Val::Px(48.0)),
                            ..default()
                        },
                        image: ui_assets.health[PLAYER_HUD_DISPLAY_CHUNKS].clone().into(),
                        ..default()
                    },
                    HealthBar
                ));

                parent.spawn(
                    NodeBundle {
                        style: Style {
                            size: Size::new(Val::Px(192.0), Val::Px(4.0)),
                            ..default()
                        },
                        ..default()
                    },
                );

                parent.spawn((
                    ImageBundle {
                        style: Style {
                            size: Size::new(Val::Px(192.0), Val::Px(48.0)),
                            ..default()
                        },
                        image: ui_assets.ammo[PLAYER_HUD_DISPLAY_CHUNKS].clone().into(),
                        ..default()
                    },
                    AmmoBar
                ));

                parent.spawn(NodeBundle {
                    style: Style {
                        align_items: AlignItems::FlexStart,
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::FlexStart,
                        size: Size::new(Val::Percent(100.0), Val::Px(72.0)),
                        ..default()
                    },

                    ..default()
                }).with_children(|parent| {
                    parent.spawn((
                        ImageBundle {
                            style: Style {
                                size: Size::new(Val::Px(48.0), Val::Px(48.0)),
                                ..default()
                            },
                            image: ui_assets.coins.clone().into(),
                            ..default()
                        },
                    ));

                    parent.spawn((
                        TextBundle::from_section("0".to_string(), text_style)
                            .with_style(Style {
                                margin: UiRect::all(Val::Px(8.0)),
                                align_self: AlignSelf::FlexStart,
                                position: UiRect {
                                    top: Val::Percent(55.0),
                                    bottom: Val::Percent(45.0),
                                    ..default()
                                },
                                ..default()
                            }),
                        WalletText
                    ));
                });
            });
        }).id();

    hud.entity = entity;
}




fn sync_hud(
    player_stats: Query<(
        &CoinHolder,
        &Health,
        &Ammo,
    ), (
        With<Player>,
        Or<(Changed<CoinHolder>, Changed<Health>, Changed<Ammo>)>
    )>,

    assets: Res<UiAssets>,
    mut ammo_bar: Query<&mut UiImage, (With<AmmoBar>, Without<HealthBar>)>,
    mut health_bar: Query<&mut UiImage, (With<HealthBar>, Without<AmmoBar>)>,
    mut wallet_text: Query<&mut Text, With<WalletText>>
) {
    if ammo_bar.is_empty() || health_bar.is_empty() || wallet_text.is_empty() {
        return;
    }

    for (coin, health, ammo) in player_stats.iter() {
        let mut health_bar = health_bar.single_mut();
        let index = index_for_value(health.hp, health.max_hp, PLAYER_HUD_DISPLAY_CHUNKS);
        *health_bar = assets.health[index].clone().into();

        let mut ammo_bar = ammo_bar.single_mut();
        let index = index_for_value(ammo.rounds_left as i32, ammo.max_rounds as i32, PLAYER_HUD_DISPLAY_CHUNKS);
        *ammo_bar = assets.ammo[index].clone().into();

        let mut wallet_text = wallet_text.single_mut();
        wallet_text.sections[0].value = coin.total_value.to_string();
    }
}

pub fn index_for_value(val: i32, max: i32, chunks: usize) -> usize {
    if val > 0 {
        let percent = val as f32 / max as f32;
        (percent * chunks as f32).ceil() as usize
    } else {
        0
    }
}