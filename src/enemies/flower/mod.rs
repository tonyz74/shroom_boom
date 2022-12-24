pub mod state_machine;
use state_machine as s;

use bevy::prelude::*;
use seldom_state::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    common::AnimTimer,
    enemies::{EnemyBundle, Enemy},
    assets::SnakeEnemyAssets,
};

use crate::pathfind::{
    Pathfinder, BoundingBox,
    walk::WalkPathfinder,
};

pub const FLOWER_FALL_GRAVITY: f32 = -40.0;
pub const FLOWER_TERMINAL_VELOCITY: f32 = -20.0;

pub struct FlowerEnemyPlugin;

impl Plugin for FlowerEnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(TriggerPlugin::<s::FallTrigger>::default());
        app.add_plugin(TriggerPlugin::<s::GroundedTrigger>::default());
        app.add_plugin(TriggerPlugin::<s::NeedsJumpTrigger>::default());
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
    pub fn from_assets(assets: &Res<SnakeEnemyAssets>) -> Self {
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
               }
           },

           snake: FlowerEnemy,

           crawl: WalkPathfinder {
               jump_speed: 8.0,
               ..default()
           }
       }
    }
}