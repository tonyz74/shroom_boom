pub mod state_machine;
pub mod stats;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    enemies::EnemyBundle,
    pathfind::FlyPathfinder
};
use crate::assets::DandelionEnemyAssets;
use crate::coin::drops::CoinHolder;
use crate::combat::{AttackStrength, ColliderAttackBundle, CombatLayerMask, Health, HurtAbility, Immunity};
use crate::enemies::Enemy;
use crate::enemies::stats::EnemyStats;
use crate::pathfind::{util::BoundingBox, Pathfinder, PathfinderBundle};

use crate::anim::Animator;
use crate::util::Facing;

#[derive(Component, Copy, Clone)]
pub struct DandelionEnemy;

#[derive(Bundle)]
pub struct DandelionEnemyBundle {
    #[bundle]
    pub enemy: EnemyBundle,
    pub dandelion: DandelionEnemy,
    pub fly: FlyPathfinder
}

impl DandelionEnemyBundle {
    pub fn collider_attack(power: i32) -> ColliderAttackBundle {
        ColliderAttackBundle {
            combat_layer: CombatLayerMask::ENEMY,
            strength: AttackStrength::new(power),
            ..ColliderAttackBundle::from_size(Vec2::new(24.0, 24.0))
        }
    }

    pub fn spawn_with_stats(commands: &mut Commands, mut item: Self, stats: EnemyStats) -> Entity {
        item.enemy.health.hp = stats.health;
        item.enemy.path.pathfinder.speed = stats.speed;
        item.enemy.path.pathfinder.patrol_speed = stats.patrol_speed;

        commands.spawn(item).with_children(|p| {
            p.spawn(Self::collider_attack(stats.collision_damage));
        }).id()
    }


    pub fn from_assets(assets: &DandelionEnemyAssets) -> DandelionEnemyBundle {
        DandelionEnemyBundle {
            enemy: EnemyBundle {
                facing: Facing::default(),
                immunity: Immunity::default(),
                coins: CoinHolder::default(),
                anim: Animator::new(assets.anims["IDLE"].clone()),
                collider: Collider::cuboid(24.0, 24.0),
                rigid_body: RigidBody::KinematicPositionBased,

                character_controller: KinematicCharacterController {
                    slide: true,
                    offset: CharacterLength::Relative(0.02),
                    filter_flags: QueryFilterFlags::EXCLUDE_SENSORS,
                    ..default()
                },

                state_machine: state_machine::dandelion_enemy_state_machine(),

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

                hurt_ability: HurtAbility::new(0.5, Some(0.5)),

                health: Health::new(20),
            },

            dandelion: DandelionEnemy,

            fly: FlyPathfinder::default()
        }
    }
}