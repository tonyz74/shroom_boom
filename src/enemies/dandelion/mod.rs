pub mod state_machine;
use rand::prelude::*;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    enemies::EnemyBundle,
    pathfind::FlyPathfinder
};
use crate::assets::DandelionEnemyAssets;
use crate::coin::drops::CoinHolder;
use crate::combat::{AttackStrength, ColliderAttackBundle, CombatLayerMask, Health, HurtAbility, KnockbackResistance};
use crate::common::AnimTimer;
use crate::enemies::Enemy;
use crate::pathfind::{util::BoundingBox, Pathfinder, PathfinderBundle};


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
    pub fn collider_attack() -> ColliderAttackBundle {
        ColliderAttackBundle {
            combat_layer: CombatLayerMask::ENEMY,
            strength: AttackStrength::new(2),
            ..ColliderAttackBundle::from_size(Vec2::new(36.0, 36.0))
        }
    }

    pub fn spawn(commands: &mut Commands, enemy: Self) {
        commands.spawn(enemy).with_children(|p| {
            p.spawn(Self::collider_attack());
        });
    }


    pub fn from_assets(assets: &Res<DandelionEnemyAssets>) -> DandelionEnemyBundle {
        DandelionEnemyBundle {
            enemy: EnemyBundle {
                coins: CoinHolder::default(),

                anim_timer: AnimTimer::from_seconds(assets.anims["IDLE"].speed),

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
                        speed: thread_rng().gen_range(1.5..2.5),
                        patrol_speed: thread_rng().gen_range(0.8..1.2),
                        bb: BoundingBox::new(24.0, 24.0),
                        ..default()
                    },
                    ..default()
                },

                kb_res: KnockbackResistance::new(1.0),
                combat_layer: CombatLayerMask::ENEMY,

                hurt_ability: HurtAbility::new(0.5, Some(0.5)),

                health: Health::new(1000),
            },

            dandelion: DandelionEnemy,

            fly: FlyPathfinder::default()
        }
    }
}