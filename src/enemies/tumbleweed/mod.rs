pub mod stats;
pub mod state_machine;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    enemies::{EnemyBundle, Enemy},
    combat::{CombatLayerMask, Health, HurtAbility},
    pathfind::{Pathfinder, PathfinderBundle, util::BoundingBox, walk::WalkPathfinder, MeleePathfinder}
};
use crate::coin::drops::CoinHolder;
use crate::combat::{AttackStrength, ColliderAttackBundle, Immunity};
use crate::enemies::flower::state_machine::register_flower_enemy_state_machine;
use crate::enemies::stats::EnemyStats;
use crate::anim::Animator;
use crate::anim::map::AnimationMap;
use crate::assets::TumbleweedEnemyAssets;
use crate::util::Facing;



pub fn register_tumbleweed_enemy(app: &mut App) {
    let _ = app;
}


#[derive(Component, Debug, Default)]
pub struct TumbleweedEnemy;

#[derive(Bundle)]
pub struct TumbleweedEnemyBundle {
    #[bundle]
    pub enemy: EnemyBundle,
    pub tumbleweed: TumbleweedEnemy,
    pub walk: WalkPathfinder,
    pub melee_pathfinder: MeleePathfinder
}

impl TumbleweedEnemyBundle {
    pub fn collider_attack(collision_dmg: i32) -> ColliderAttackBundle {
        ColliderAttackBundle {
            combat_layer: CombatLayerMask::ENEMY,
            strength: AttackStrength::new(collision_dmg),
            ..ColliderAttackBundle::from_size(Vec2::new(40.0, 40.0))
        }
    }

    pub fn spawn_with_stats(commands: &mut Commands, mut item: Self, stats: EnemyStats) -> Entity {
        item.enemy.health.hp = stats.health;
        item.enemy.path.pathfinder.speed = stats.speed;
        item.enemy.path.pathfinder.patrol_speed = stats.patrol_speed;
        item.walk.jump_speed = stats.jump_speed;

        commands.spawn(item).with_children(|p| {
            p.spawn(Self::collider_attack(stats.collision_damage));
        }).id()
    }

    pub fn from_assets(assets: &TumbleweedEnemyAssets) -> Self {
        TumbleweedEnemyBundle {
            enemy: EnemyBundle {
                anim_map: assets.map.clone(),
                anim: Animator::new(assets.map["IDLE"].clone()),

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

                state_machine: state_machine::tumbleweed_enemy_state_machine(),

                sprite_sheet: SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        custom_size: Some(Vec2::new(112.0, 112.0)),
                        ..default()
                    },
                    texture_atlas: assets.map["IDLE"].clone().tex,
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

            tumbleweed: TumbleweedEnemy::default(),
            walk: WalkPathfinder::default(),
            melee_pathfinder: MeleePathfinder,
        }
    }
}
