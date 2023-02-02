use bevy::prelude::*;
use seldom_state::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::anim::{AnimationChangeEvent, Animator};
use crate::anim::map::AnimationMap;
use crate::coin::drops::CoinHolder;
use crate::combat::{ColliderAttack, CombatLayerMask, Health, HurtAbility, Immunity};
use crate::enemies::anim::register_enemy_animations;
use crate::enemies::flower::register_flower_enemy;
use crate::enemies::pumpkin::register_pumpkin_enemy;

use crate::enemies::spawner::register_enemy_spawner;
use crate::entity_states::Die;
use crate::fx::smoke::SmokeEvent;
use crate::pathfind::PathfinderBundle;
use crate::state::GameState;
use crate::util::Facing;

pub mod flower;
pub mod pumpkin;
pub mod dandelion;
pub mod tumbleweed;

pub mod stats;
pub mod spawner;
mod anim;

#[derive(Default, Component, Clone, Copy)]
pub struct Enemy {
    pub vel: Vec2,
}

#[derive(Bundle)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub facing: Facing,
    pub sensor: Sensor,

    pub anim: Animator,
    pub anim_map: AnimationMap,

    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub state_machine: StateMachine,
    pub character_controller: KinematicCharacterController,

    pub hurt_ability: HurtAbility,

    pub health: Health,
    pub immunity: Immunity,
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
            .add_system_set(
                SystemSet::on_update(GameState::Gameplay)
                    .with_system(move_enemies)
                    .with_system(enemies_died)
                    .with_system(enemies_despawn)
            );

        register_enemy_spawner(app);
        register_enemy_animations(app);

        register_flower_enemy(app);
        register_pumpkin_enemy(app);
    }
}

fn move_enemies(mut q: Query<(&Enemy, &mut KinematicCharacterController)>) {
    for (enemy, mut cc) in q.iter_mut() {
        cc.translation = Some(enemy.vel);
    }
}

fn enemies_died(
    mut collider_attacks: Query<&mut ColliderAttack>,
    mut enemies: Query<(Entity, &GlobalTransform, &Children, &AnimationMap), (With<Enemy>, Added<Die>)>,
    mut change_events: EventWriter<AnimationChangeEvent>,
    mut smoke: EventWriter<SmokeEvent>
) {
    for (entity, tf, children, anims) in enemies.iter_mut() {
        for child in children.iter() {
            if let Ok(mut collider_attacks) = collider_attacks.get_mut(*child) {
                collider_attacks.enabled = false;
            }
        }

        change_events.send(AnimationChangeEvent {
            e: entity,
            new_anim: anims["DEATH"].clone()
        });

        smoke.send(SmokeEvent {
            pos: Vec2::new(tf.translation().x, tf.translation().y)
        });
    }
}

fn enemies_despawn(
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut enemies: Query<(&Animator, &mut Die, &AnimationMap), (With<Enemy>, With<Die>)>,
) {
    for (animator, mut die, anims) in enemies.iter_mut() {
        if animator.anim.name != "DEATH" {
            continue;
        }

        let frame_count = texture_atlases.get(&anims["DEATH"].tex).unwrap().textures.len();
        if animator.total_frames == frame_count as u32 - 1 || animator.total_looped >= 1 {
            die.should_despawn = true;
        }
    }
}