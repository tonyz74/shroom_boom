pub mod state_machine;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    common::AnimTimer,
    enemies::{EnemyBundle, Enemy},
    assets::FlowerEnemyAssets,
};
use crate::attack::{CombatLayerMask, Health, HurtAbility, KnockbackResistance};

use crate::pathfind::{
    Pathfinder, BoundingBox,
    walk::WalkPathfinder,
};

pub const FLOWER_FALL_GRAVITY: f32 = -40.0;
pub const FLOWER_TERMINAL_VELOCITY: f32 = -20.0;

pub struct FlowerEnemyPlugin;

impl Plugin for FlowerEnemyPlugin {
    fn build(&self, app: &mut App) {
        let _ = app;
    }
}

#[derive(Component, Default, Debug)]
pub struct FlowerEnemy;

#[derive(Bundle)]
pub struct FlowerEnemyBundle {
    #[bundle]
    pub enemy: EnemyBundle,
    pub snake: FlowerEnemy,
    pub crawl: WalkPathfinder
}

impl FlowerEnemyBundle {
    pub fn from_assets(assets: &Res<FlowerEnemyAssets>) -> Self {
        FlowerEnemyBundle {
            enemy: EnemyBundle {
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

                path: Pathfinder {
                    speed: 2.0,
                    bb: BoundingBox::new(24.0, 24.0),
                    lose_notice_timer: Timer::from_seconds(4.0, TimerMode::Once),
                    ..default()
                },

                kb_res: KnockbackResistance::new(1.0),
                combat_layer: CombatLayerMask::ENEMY,

                hurt_ability: HurtAbility::new(0.2),

                health: Health::new(10),
            },

            snake: FlowerEnemy,

            crawl: WalkPathfinder {
                jump_speed: 8.0,
                ..default()
            },
        }
    }
}
