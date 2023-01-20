use bevy::prelude::*;
use bevy_rapier2d::prelude::RapierContext;
use seldom_state::prelude::*;

use crate::{
    state::GameState,
    assets::PlayerAssets,
    combat::{MeleeAttack, MeleeAttackBundle},
    common::AnimTimer,
    player::Player,
};
use crate::combat::{AttackStrength, CombatLayerMask};
use crate::entity_states::Die;
use crate::player::abilities::autotarget::{AttackDirection, change_facing_for_direction, direction_for_facing, get_closest_target};
use crate::player::consts::SLASH_LEVELS;
use crate::player::state_machine::Slash;
use crate::util::quat_rot2d_deg;

// MAIN

#[derive(Component)]
pub struct PlayerMeleeAttack;

#[derive(Component)]
pub struct SlashAbility {
    pub damage: i32,
    pub cd: Timer,
    pub dur: Timer,
}

impl Default for SlashAbility {
    fn default() -> Self {
        Self {
            damage: SLASH_LEVELS[0].1,
            cd: Timer::from_seconds(SLASH_LEVELS[0].0, TimerMode::Once),
            dur: Timer::from_seconds(0.15, TimerMode::Once)
        }
    }
}

pub fn register_slash_ability(app: &mut App) {
    app.add_system_set(
        SystemSet::on_update(GameState::Gameplay)
            .with_system(slash_ability_trigger)
            .with_system(slash_ability_update)
            .with_system(slash_ability_cooldown_update)
    );
}

// Systems

fn transform_for_direction(dir: AttackDirection) -> (Transform, BVec2) {
    let mut flip = BVec2::new(false, false);
    let mut tf = Transform::from_xyz(0.0, 0.0, 1.0);

    match dir {
        AttackDirection::Up => {
            tf.rotate(quat_rot2d_deg(90.0));
            tf = tf.with_translation(Vec3::new(0.0, 32.0, 0.0));
        },

        AttackDirection::UpRight => {
            tf.rotate(quat_rot2d_deg(45.0));
            tf = tf.with_translation(Vec3::new(24.0, 24.0, 0.0));
        },

        AttackDirection::UpLeft => {
            tf.rotate(quat_rot2d_deg(135.0));
            tf = tf.with_translation(Vec3::new(-24.0, 24.0, 0.0));
        }

        AttackDirection::Down => {
            tf.rotate(quat_rot2d_deg(-90.0));
            tf = tf.with_translation(Vec3::new(0.0, -32.0, 0.0));
        },

        AttackDirection::DownRight => {
            tf.rotate(quat_rot2d_deg(315.0));
            tf = tf.with_translation(Vec3::new(24.0, -24.0, 0.0));
        },

        AttackDirection::DownLeft => {
            tf.rotate(quat_rot2d_deg(225.0));
            tf = tf.with_translation(Vec3::new(-24.0, -24.0, 0.0));
        },

        AttackDirection::Left => {
            tf = tf.with_translation(Vec3::new(-24.0, 0.0, 0.0));
            flip.x = true;
        },

        AttackDirection::Right => {
            tf = tf.with_translation(Vec3::new(24.0, 0.0, 0.0));
        },
    };

    (tf, flip)
}

fn slash_ability_trigger(
    mut commands: Commands,
    assets: Res<PlayerAssets>,
    mut q: Query<(
        Entity,
        &mut Player,
        &mut SlashAbility
    ), (Added<Slash>, Without<Die>)>,
    transforms: Query<&GlobalTransform>,
    combat_layers: Query<&CombatLayerMask>,
    rapier: Res<RapierContext>
) {
    if q.is_empty() {
        return;
    }

    let (entity, mut player, mut slash) = q.single_mut();

    slash.cd.reset();
    slash.dur.reset();

    let direction = if let Some((_, b)) = get_closest_target(
        entity,
        CombatLayerMask::PLAYER,
        240.0,
        &transforms,
        &combat_layers,
        &rapier
    ) {
        b
    } else {
        direction_for_facing(player.facing)
    };

    change_facing_for_direction(&mut player, direction);
    let (tf, flip) = transform_for_direction(direction);

    commands.entity(entity).with_children(|parent| {
        parent.spawn((
            PlayerMeleeAttack,

            MeleeAttackBundle {
                sprite_sheet: SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        flip_x: flip.x,
                        flip_y: flip.y,
                        custom_size: Some(Vec2::new(72.0, 48.0)),
                        ..default()
                    },

                    transform: tf,

                    texture_atlas: assets.slash_anim.tex.clone(),

                    ..default()
                },

                attack: MeleeAttack {
                    source: Some(entity),
                    ..default()
                },

                strength: AttackStrength {
                    power: slash.damage
                },

                combat_layer: CombatLayerMask::PLAYER,

                anim_timer: AnimTimer::from_seconds(assets.slash_anim.speed),

                ..MeleeAttackBundle::from_size(Vec2::new(72.0, 48.0))
            }
        ));
    });
}


pub fn slash_ability_update(
    mut commands: Commands,
    time: Res<Time>,
    slashing: Query<&Slash>,
    mut player_query: Query<(Entity, &mut SlashAbility), Without<Die>>,
    melees: Query<Entity, (With<MeleeAttack>, With<PlayerMeleeAttack>)>
) {
    if player_query.is_empty() || melees.is_empty() {
        return;
    }

    for (entity, mut slash) in player_query.iter_mut() {
        slash.dur.tick(time.delta());

        if slash.dur.just_finished() {
            commands.entity(melees.single()).despawn();

            if slashing.contains(entity) {
                commands.entity(entity).insert(Done::Success);
            }
        }
    }
}

pub fn slash_ability_cooldown_update(
    time: Res<Time>,
    mut q: Query<&mut SlashAbility, Without<Die>>
) {
    for mut slash in q.iter_mut() {
        slash.cd.tick(time.delta());
    }
}