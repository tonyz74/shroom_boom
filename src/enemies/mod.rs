use bevy::prelude::*;
use seldom_state::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::common::AnimTimer;

pub mod snake;

#[derive(Bundle)]
pub struct EnemyBundle {
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
            .add_plugin(snake::SnakePlugin);
    }
}