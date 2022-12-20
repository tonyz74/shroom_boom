use bevy::prelude::*;
use seldom_state::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::common::AnimTimer;

pub mod snake;


#[derive(Default, Component)]
pub struct Enemy {
    pub vel: Vec2
}

#[derive(Bundle)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub sensor: Sensor,
    pub anim_timer: AnimTimer,
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub state_machine: StateMachine,
    pub character_controller: KinematicCharacterController,
    #[bundle]
    pub sprite_sheet: SpriteSheetBundle,
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(snake::SnakeEnemyPlugin);
    }
}