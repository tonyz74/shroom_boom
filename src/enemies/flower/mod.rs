pub mod state_machine;
use state_machine as s;

use bevy::prelude::*;
use seldom_state::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    common::AnimTimer,
    enemies::{EnemyBundle, Enemy},
    assets::SnakeEnemyAssets,
    state::GameState
};
use crate::pathfind::crawl::CrawlPathfinder;
use crate::pathfind::Pathfinder;

pub const FLOWER_FALL_GRAVITY: f32 = -40.0;
pub const FLOWER_TERMINAL_VELOCITY: f32 = -20.0;

pub struct FlowerEnemyPlugin;

impl Plugin for FlowerEnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Gameplay)
                .with_system(gravity)
                .with_system(do_movement)
                .with_system(enter_idle)
                .with_system(enter_jump)
                .with_system(update_sprite_if_flipped)
        );

        app.add_plugin(TriggerPlugin::<s::FallTrigger>::default());
        app.add_plugin(TriggerPlugin::<s::GroundedTrigger>::default());
        app.add_plugin(TriggerPlugin::<s::NeedsJumpTrigger>::default());
    }
}

#[derive(Component, Default, Debug)]
pub struct FlowerEnemy;

#[derive(Bundle)]
pub struct SnakeEnemyBundle {
    #[bundle]
    pub enemy: EnemyBundle,
    pub snake: FlowerEnemy,
    pub crawl: CrawlPathfinder
}

impl SnakeEnemyBundle {
    pub fn from_assets(assets: &Res<SnakeEnemyAssets>) -> Self {
       SnakeEnemyBundle {
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

               state_machine: state_machine::snake_enemy_state_machine(),

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

               path: Pathfinder::default()
           },

           snake: FlowerEnemy,

           crawl: CrawlPathfinder::default()
       }
    }
}

fn gravity(mut q: Query<&mut Enemy, Without<s::Run>>) {
    for mut enemy in q.iter_mut() {
        enemy.vel.y += 0.016667 * FLOWER_FALL_GRAVITY;

        if enemy.vel.y <= FLOWER_TERMINAL_VELOCITY {
            enemy.vel.y = FLOWER_TERMINAL_VELOCITY;
        }
    }
}

fn enter_jump(mut q: Query<&mut Enemy, Added<s::Jump>>) {
    for mut enemy in q.iter_mut() {
        enemy.vel.y = 8.0;
        enemy.vel.x = 0.0;
    }
}

fn enter_idle(mut q: Query<&mut Enemy, Added<s::Run>>) {
    for mut enemy in q.iter_mut() {
        enemy.vel.x = 0.0;
        enemy.vel.y = 0.0;
    }
}

fn do_movement(mut q: Query<(&Enemy, &mut KinematicCharacterController)>) {
    for (enemy, mut cc) in q.iter_mut() {
        cc.translation = Some(enemy.vel);
    }
}

fn update_sprite_if_flipped(mut q: Query<(&Enemy, &mut TextureAtlasSprite)>) {
    for (enemy, mut spr) in q.iter_mut() {
        if enemy.vel.x >= 0.1 {
            spr.flip_x = false;
        } else if enemy.vel.x <= -0.1 {
            spr.flip_x = true;
        }
    }
}