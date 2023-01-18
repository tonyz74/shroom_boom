pub mod state_machine;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use rand::prelude::*;

use crate::{
    common::AnimTimer,
    enemies::{EnemyBundle, Enemy},
    assets::FlowerEnemyAssets,
    combat::{CombatLayerMask, Health, HurtAbility},
    pathfind::{Pathfinder, PathfinderBundle, util::BoundingBox, walk::WalkPathfinder, MeleePathfinder}
};
use crate::coin::drops::CoinHolder;
use crate::combat::{AttackStrength, ColliderAttackBundle, Immunity};
use crate::enemies::flower::state_machine::register_flower_enemy_state_machine;

pub struct FlowerEnemyPlugin;

impl Plugin for FlowerEnemyPlugin {
    fn build(&self, app: &mut App) {
        register_flower_enemy_state_machine(app);
    }
}

#[derive(Component, Debug)]
pub struct FlowerEnemy {
    pub countdown: Timer
}

impl Default for FlowerEnemy {
    fn default() -> Self {
        Self {
            countdown: Timer::from_seconds(1.0, TimerMode::Once)
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
    pub fn collider_attack() -> ColliderAttackBundle {
        ColliderAttackBundle {
            combat_layer: CombatLayerMask::ENEMY,
            strength: AttackStrength::new(2),
            ..ColliderAttackBundle::from_size(Vec2::new(36.0, 36.0))
        }
    }

    pub fn spawn(commands: &mut Commands, enemy: Self) -> Entity {
        commands.spawn(enemy).with_children(|p| {
            p.spawn(Self::collider_attack());
        }).id()
    }

    pub fn from_assets(assets: &FlowerEnemyAssets) -> Self {
        FlowerEnemyBundle {
            enemy: EnemyBundle {
                immunity: Immunity::default(),

                coins: CoinHolder::default(),

                anim_timer: AnimTimer::from_seconds(assets.anims["IDLE"].speed),

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
                        speed: thread_rng().gen_range(1.5..2.5),
                        patrol_speed: thread_rng().gen_range(0.8..1.2),
                        bb: BoundingBox::new(24.0, 24.0),
                        ..default()
                    },
                    ..default()
                },

                combat_layer: CombatLayerMask::ENEMY,

                hurt_ability: HurtAbility::new(0.5, None),

                health: Health::new(10),
            },

            flower: FlowerEnemy::default(),

            walk: WalkPathfinder {
                jump_speed: thread_rng().gen_range(7.0..9.0),
                ..default()
            },

            melee_pathfinder: MeleePathfinder,
        }
    }
}
