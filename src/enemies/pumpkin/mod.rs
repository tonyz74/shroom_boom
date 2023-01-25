pub mod state_machine;
pub mod stats;
mod anim;

use std::time::Duration;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    enemies::{EnemyBundle, Enemy},
    assets::PumpkinEnemyAssets,
    combat::{CombatLayerMask, Health, HurtAbility},
    pathfind::{Pathfinder, PathfinderBundle, util::BoundingBox, walk::WalkPathfinder, RangedPathfinder}
};
use crate::coin::drops::CoinHolder;
use crate::combat::{AttackStrength, ColliderAttackBundle, Immunity, ProjectileAttack, ProjectileAttackBundle};
use crate::enemies::stats::{CustomEnemyStats, EnemyStats};
use crate::util::{deg_to_rad, Facing};
use crate::anim::Animator;
use crate::enemies::DeathAnimation;


#[derive(Component, Copy, Clone, Debug)]
pub struct PumpkinProjectileAttack;

pub struct PumpkinEnemyPlugin;

impl Plugin for PumpkinEnemyPlugin {
    fn build(&self, app: &mut App) {
        let _ = app;
    }
}

#[derive(Component, Default, Debug)]
pub struct PumpkinEnemy;

#[derive(Bundle)]
pub struct PumpkinEnemyBundle {
    #[bundle]
    pub enemy: EnemyBundle,
    pub pumpkin: PumpkinEnemy,
    pub walk: WalkPathfinder,
    pub ranged_pathfinder: RangedPathfinder
}

impl PumpkinEnemyBundle {
    pub fn collider_attack(power: i32) -> ColliderAttackBundle {
        ColliderAttackBundle {
            combat_layer: CombatLayerMask::ENEMY,
            strength: AttackStrength::new(power),
            ..ColliderAttackBundle::from_size(Vec2::new(48.0, 32.0))
        }
    }

    pub fn spawn_with_stats(commands: &mut Commands, mut item: Self, stats: EnemyStats) -> Entity {
        item.enemy.health.hp = stats.health;
        item.enemy.path.pathfinder.speed = stats.speed;
        item.enemy.path.pathfinder.patrol_speed = stats.patrol_speed;
        item.walk.jump_speed = stats.jump_speed;
        item.ranged_pathfinder.projectile.strength.power = stats.attack_damage;

        let extra = match stats.custom {
            CustomEnemyStats::Ranged(ranged) => ranged,
            _ => panic!("Ranged pathfinder not configured with ranged stats!")
        };

        item.ranged_pathfinder.max_shoot_distance = extra.max_shoot_dist;
        item.ranged_pathfinder.shoot_pause.set_duration(Duration::from_secs_f32(extra.atk_freq));
        item.ranged_pathfinder.projectile.attack.speed = extra.proj_speed;

        commands.spawn(item).with_children(|p| {
            p.spawn(Self::collider_attack(stats.collision_damage));
        }).id()
    }

    pub fn from_assets(assets: &PumpkinEnemyAssets) -> Self {
        PumpkinEnemyBundle {
            enemy: EnemyBundle {
                anim: Animator::new(assets.anims["IDLE"].clone()),
                death_anim: DeathAnimation::new(assets.anims["DEATH"].clone()),

                facing: Facing::default(),
                immunity: Immunity::default(),
                coins: CoinHolder::default(),
                collider: Collider::cuboid(32.0, 32.0),
                rigid_body: RigidBody::KinematicPositionBased,

                character_controller: KinematicCharacterController {
                    slide: true,
                    snap_to_ground: Some(CharacterLength::Relative(0.2)),
                    offset: CharacterLength::Relative(0.02),
                    filter_flags: QueryFilterFlags::EXCLUDE_SENSORS,
                    ..default()
                },

                state_machine: state_machine::pumpkin_enemy_state_machine(),

                sprite_sheet: SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        custom_size: Some(Vec2::new(112.0, 112.0)),
                        ..default()
                    },
                    texture_atlas: assets.anims["IDLE"].clone().tex,
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                    ..default()
                },

                enemy: Enemy::default(),
                sensor: Sensor,

                path: PathfinderBundle {
                    pathfinder: Pathfinder {
                        bb: BoundingBox::new(24.0, 24.0),
                        ..default()
                    },
                    ..default()
                },

                combat_layer: CombatLayerMask::ENEMY,
                hurt_ability: HurtAbility::new(0.5, None),
                health: Health::default()
            },

            pumpkin: PumpkinEnemy,

            walk: WalkPathfinder {
                ..default()
            },

            ranged_pathfinder: RangedPathfinder {
                shoot_startup: Timer::from_seconds(0.1, TimerMode::Once),
                shoot_pause: Timer::from_seconds(0.1, TimerMode::Once),
                shoot_cooldown: Timer::from_seconds(0.0, TimerMode::Once),

                max_shoot_angle: deg_to_rad(45.0),
                max_shoot_distance: 0.0,

                extra_spawn: |cmd, e| { cmd.entity(e).insert(PumpkinProjectileAttack); },

                projectile: ProjectileAttackBundle {
                    attack: ProjectileAttack {
                        speed: 0.0,
                        ..default()
                    },

                    anim: Animator::default(),

                    sprite_sheet: SpriteSheetBundle {
                        sprite: TextureAtlasSprite {
                            custom_size: Some(Vec2::new(16.0, 16.0)),
                            ..default()
                        },
                        texture_atlas: assets.anims["IDLE"].clone().tex,
                        ..default()
                    },

                    strength: AttackStrength::new(0),
                    combat_layer: CombatLayerMask::ENEMY,
                    ..ProjectileAttackBundle::from_size(Vec2::new(16.0, 16.0))
                },

                ..default()
            },
        }
    }
}
