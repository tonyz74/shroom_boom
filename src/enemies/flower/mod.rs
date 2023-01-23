pub mod stats;
pub mod state_machine;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    enemies::{EnemyBundle, Enemy},
    assets::FlowerEnemyAssets,
    combat::{CombatLayerMask, Health, HurtAbility},
    pathfind::{Pathfinder, PathfinderBundle, util::BoundingBox, walk::WalkPathfinder, MeleePathfinder}
};
use crate::coin::drops::CoinHolder;
use crate::combat::{AttackStrength, ColliderAttackBundle, Immunity};
use crate::enemies::flower::state_machine::register_flower_enemy_state_machine;
use crate::enemies::stats::EnemyStats;
use crate::anim::Animator;
use crate::util::Facing;

pub struct FlowerEnemyPlugin;

impl Plugin for FlowerEnemyPlugin {
    fn build(&self, app: &mut App) {
        register_flower_enemy_state_machine(app);
    }
}

#[derive(Component, Debug)]
pub struct FlowerEnemy {
    pub countdown: Timer,
    pub explosion_power: i32
}

impl Default for FlowerEnemy {
    fn default() -> Self {
        Self {
            explosion_power: 0,
            countdown: Timer::from_seconds(0.5, TimerMode::Once)
        }
    }
}

#[derive(Bundle)]
pub struct FlowerEnemyBundle {
    #[bundle]
    pub enemy: EnemyBundle,
    pub flower: FlowerEnemy,
    pub walk: WalkPathfinder,
    pub melee_pathfinder: MeleePathfinder
}

impl FlowerEnemyBundle {
    pub fn collider_attack(collision_dmg: i32) -> ColliderAttackBundle {
        ColliderAttackBundle {
            combat_layer: CombatLayerMask::ENEMY,
            strength: AttackStrength::new(collision_dmg),
            ..ColliderAttackBundle::from_size(Vec2::new(24.0, 24.0))
        }
    }

    pub fn spawn_with_stats(commands: &mut Commands, mut item: Self, stats: EnemyStats) -> Entity {
        item.enemy.health.hp = stats.health;
        item.flower.explosion_power = stats.attack_damage;
        item.enemy.path.pathfinder.speed = stats.speed;
        item.enemy.path.pathfinder.patrol_speed = stats.patrol_speed;
        item.walk.jump_speed = stats.jump_speed;

        commands.spawn(item).with_children(|p| {
            p.spawn(Self::collider_attack(stats.collision_damage));
        }).id()
    }

    pub fn from_assets(assets: &FlowerEnemyAssets) -> Self {
        FlowerEnemyBundle {
            enemy: EnemyBundle {
                facing: Facing::default(),
                immunity: Immunity::default(),
                coins: CoinHolder::default(),
                anim: Animator::new(assets.anims["IDLE"].clone()),
                collider: Collider::cuboid(24.0, 24.0),
                rigid_body: RigidBody::KinematicPositionBased,

                character_controller: KinematicCharacterController {
                    slide: true,
                    snap_to_ground: Some(CharacterLength::Relative(0.2)),
                    offset: CharacterLength::Relative(0.02),
                    filter_flags: QueryFilterFlags::EXCLUDE_SENSORS,
                    ..default()
                },

                state_machine: state_machine::flower_enemy_state_machine(),

                sprite_sheet: SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        custom_size: Some(Vec2::new(48.0, 48.0)),
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
                health: Health::default(),
            },

            flower: FlowerEnemy::default(),
            walk: WalkPathfinder::default(),
            melee_pathfinder: MeleePathfinder,
        }
    }
}
