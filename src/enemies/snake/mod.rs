pub mod state_machine;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    common::AnimTimer,
    enemies::{EnemyBundle, Enemy},
    assets::SnakeEnemyAssets,
    state::GameState
};

pub struct SnakeEnemyPlugin;

impl Plugin for SnakeEnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Gameplay)
                .with_system(gravity)
                .with_system(do_movement)
        );
    }
}

#[derive(Component, Default)]
pub struct SnakeEnemy;

#[derive(Bundle)]
pub struct SnakeEnemyBundle {
    #[bundle]
    pub enemy: EnemyBundle,
    pub snake: SnakeEnemy
}

impl SnakeEnemyBundle {
    pub fn from_assets(assets: &Res<SnakeEnemyAssets>) -> Self {
       SnakeEnemyBundle {
           enemy: EnemyBundle {
               anim_timer: AnimTimer::from_seconds(0.2),

               collider: Collider::cuboid(28.0, 16.0),

               rigid_body: RigidBody::Fixed,

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
                        custom_size: Some(Vec2::new(56.0, 32.0)),
                        ..default()
                   },
                    texture_atlas: assets.anims["IDLE"].clone().tex,
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                    ..default()
               },

               enemy: Enemy::default(),

               sensor: Sensor
           },

           snake: SnakeEnemy,
       }
    }
}


fn gravity(mut q: Query<&mut Enemy>) {
    for mut enemy in q.iter_mut() {
        enemy.vel.y = -2.0;
    }
}


fn do_movement(mut q: Query<(&Enemy, &mut KinematicCharacterController)>) {
    for (enemy, mut cc) in q.iter_mut() {
        cc.translation = Some(enemy.vel);
    }
}
