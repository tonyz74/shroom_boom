use bevy::prelude::*;
use seldom_state::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::attack::{CombatLayerMask, Health, HitEvent, HurtAbility, KnockbackResistance};

use crate::common::{AnimTimer, UpdateStage};
use crate::pathfind::Pathfinder;
use crate::pathfind::state_machine::Hurt;
use crate::state::GameState;

pub mod flower;
pub mod pumpkin;

#[derive(Default, Component)]
pub struct Enemy {
    pub vel: Vec2,
    pub hit_event: Option<HitEvent>,
}

#[derive(Bundle)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub sensor: Sensor,
    pub path: Pathfinder,
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
                    .with_system(handle_hits)
                    .with_system(handle_dead_enemies)
            );
    }
}

fn move_enemies(mut q: Query<(&Enemy, &mut KinematicCharacterController)>) {
    for (enemy, mut cc) in q.iter_mut() {
        cc.translation = Some(enemy.vel);
    }
}

fn handle_hits(
    mut q: Query<(&mut Enemy, &HurtAbility, &mut Health), Without<Hurt>>,
    mut hit_events: EventReader<HitEvent>
) {
    for hit in hit_events.iter() {
        if let Ok((mut target, hurt, mut health)) = q.get_mut(hit.target) {
            if hurt.is_immune() {
                target.hit_event = None;
                continue;
            }

            health.hp -= hit.damage;
            target.hit_event = Some(*hit);
        }
    }
}

fn handle_dead_enemies(
    mut commands: Commands,
    enemies: Query<(Entity, &Health), With<Enemy>>
) {
    for (entity, health) in enemies.iter() {
        if health.hp <= 0 {
            commands.entity(entity).despawn();
        }
    }
}