use bevy::prelude::*;
use seldom_state::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::coin::drops::CoinHolder;
use crate::combat::{ColliderAttack, CombatLayerMask, Health, HurtAbility};

use crate::common::AnimTimer;
use crate::entity_states::Die;
use crate::pathfind::PathfinderBundle;
use crate::state::GameState;

pub mod flower;
pub mod pumpkin;
pub mod dandelion;

#[derive(Default, Component, Clone, Copy)]
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
    pub combat_layer: CombatLayerMask,

    pub coins: CoinHolder,

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
                    .with_system(move_enemies)
                    .with_system(enemies_died)
            );
    }
}

fn move_enemies(mut q: Query<(&Enemy, &mut KinematicCharacterController)>) {
    for (enemy, mut cc) in q.iter_mut() {
        cc.translation = Some(enemy.vel);
    }
}

fn enemies_died(
    mut collider_attacks: Query<&mut ColliderAttack>,
    mut enemies: Query<(&Children, &mut Die), (With<Enemy>, Added<Die>)>
) {
    for (children, mut death) in enemies.iter_mut() {
        for child in children.iter() {
            if let Ok(mut collider_attacks) = collider_attacks.get_mut(*child) {
                collider_attacks.enabled = false;
            }
        }

        death.should_despawn = true;
    }
}