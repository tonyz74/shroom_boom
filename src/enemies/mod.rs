use bevy::prelude::*;
use seldom_state::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::combat::{CombatLayerMask, Health, HurtAbility, KnockbackResistance};

use crate::common::{AnimTimer, UpdateStage};
use crate::pathfind::PathfinderBundle;
use crate::state::GameState;

pub mod flower;
pub mod pumpkin;
pub mod dandelion;

#[derive(Default, Component)]
pub struct Enemy {
    pub vel: Vec2,
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

    pub hurt_ability: HurtAbility,

    pub health: Health,
    pub kb_res: KnockbackResistance,
    pub combat_layer: CombatLayerMask,

    #[bundle]
    pub sprite_sheet: SpriteSheetBundle,

    #[bundle]
    pub path: PathfinderBundle
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(flower::FlowerEnemyPlugin)
            .add_system_set(
                SystemSet::on_update(GameState::Gameplay)
                    .label(UpdateStage::Physics)
                    .with_system(move_enemies)
                    .with_system(handle_dead_enemies)
            );
    }
}

fn move_enemies(mut q: Query<(&Enemy, &mut KinematicCharacterController)>) {
    for (enemy, mut cc) in q.iter_mut() {
        cc.translation = Some(enemy.vel);
    }
}

fn handle_dead_enemies(
    mut commands: Commands,
    enemies: Query<(Entity, &Health), With<Enemy>>
) {
    for (entity, health) in enemies.iter() {
        if health.hp <= 0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}