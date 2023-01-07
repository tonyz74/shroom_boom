use bevy::prelude::*;
use seldom_state::prelude::*;

use crate::{
    state::GameState,
    assets::PlayerAssets,
    combat::{MeleeAttack, MeleeAttackBundle},
    common::AnimTimer,
    player::{
        Player,
        consts::PLAYER_ATTACK_COOLDOWN,
        state_machine as s
    }
};
use crate::combat::{AttackStrength, CombatLayerMask};
use crate::player::state_machine::Slash;

// HELPER FUNCTIONS
fn deg_to_rad(deg: f32) -> f32 {
    deg * (std::f64::consts::PI as f32 / 180.0)
}

fn quat_rot2d(deg: f32) -> Quat {
    Quat::from_rotation_z(deg_to_rad(deg))
}

// MAIN

#[derive(Component)]
pub struct SlashAbility {
    pub damage: u32,
    pub cd: Timer,
    pub dur: Timer,
}

impl Default for SlashAbility {
    fn default() -> Self {
        Self {
            damage: 1,
            cd: Timer::from_seconds(PLAYER_ATTACK_COOLDOWN, TimerMode::Once),
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

enum SlashDirection {
    Up,
    Down,
    Left,
    Right
}

fn transform_for_direction(dir: SlashDirection) -> (Transform, BVec2) {
    use SlashDirection as Dir;

    let mut flip = BVec2::new(false, false);
    let mut tf = Transform::from_xyz(0.0, 0.0, 1.0);

    match dir {
        Dir::Up => {
            tf.rotate(quat_rot2d(90.0));
            tf = tf.with_translation(Vec3::new(0.0, 32.0, 0.0));
        },

        Dir::Down => {
            tf.rotate(quat_rot2d(-90.0));
            tf = tf.with_translation(Vec3::new(0.0, -32.0, 0.0));
        },

        Dir::Left => {
            tf = tf.with_translation(Vec3::new(-24.0, 0.0, 0.0));
            flip.x = true;
        },

        Dir::Right => {
            tf = tf.with_translation(Vec3::new(24.0, 0.0, 0.0));
        }
    };

    (tf, flip)
}


fn slash_ability_trigger(
    mut commands: Commands,
    assets: Res<PlayerAssets>,
    mut q: Query<(
        Entity,
        &mut Player,
        &TextureAtlasSprite,
        &mut SlashAbility
    ), Added<s::Slash>>
) {
    for (entity, player, spr, mut slash) in q.iter_mut() {
        slash.cd.reset();
        slash.dur.reset();

        let direction;

        if player.vel.y < -8.0 {
            direction = SlashDirection::Down;
        } else if player.vel.y > 10.0 {
            direction = SlashDirection::Up;
        } else {
            if spr.flip_x {
                direction = SlashDirection::Left;
            } else {
                direction = SlashDirection::Right;
            }
        }

        let (tf, flip) = transform_for_direction(direction);

        commands.entity(entity).with_children(|parent| {
            parent.spawn(MeleeAttackBundle {
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
                    power: 2
                },

                combat_layer: CombatLayerMask::PLAYER,

                anim_timer: AnimTimer::from_seconds(assets.slash_anim.speed),

                ..MeleeAttackBundle::from_size(Vec2::new(72.0, 48.0))
            });
        });
    }
}


pub fn slash_ability_update(
    mut commands: Commands,
    time: Res<Time>,
    slashing: Query<&Slash>,
    mut player_query: Query<(Entity, &mut SlashAbility)>,
    melees: Query<Entity, With<MeleeAttack>>
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
    mut q: Query<&mut SlashAbility>
) {
    for mut slash in q.iter_mut() {
        slash.cd.tick(time.delta());
    }
}