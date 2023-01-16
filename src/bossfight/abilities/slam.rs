use bevy::prelude::*;
use seldom_state::prelude::Done;
use crate::bossfight::{Boss, BossConfig};
use crate::bossfight::enraged::EnragedAttackMove;
use crate::bossfight::state_machine::{AbilityStartup, Slam};
use crate::combat::{ColliderAttack, Immunity};
use crate::enemies::Enemy;
use crate::state::GameState;

#[derive(Component, Debug, Clone)]
pub struct SlamAbility;

impl Default for SlamAbility {
    fn default() -> Self {
        Self
    }
}

pub fn register_slam_ability(app: &mut App) {
    app.add_system_set(
        SystemSet::on_update(GameState::Gameplay)
            .with_system(start_slam)
            .with_system(slam_update)
    );
}

fn start_slam(
    mut colliders: Query<&mut ColliderAttack>,
    mut q: Query<(
        &Children,
        &mut Immunity,
        &Boss
    ), Added<AbilityStartup>>
) {
    if q.is_empty() {
        return;
    }

    let (children, mut immunity, boss) = q.single_mut();

    if boss.current_move() != EnragedAttackMove::Slam {
        return;
    }

    immunity.is_immune = true;

    for child in children {
        if let Ok(mut atk) = colliders.get_mut(*child) {
            atk.enabled = true;
        }
    }
}

fn slam_update(
    mut commands: Commands,
    mut colliders: Query<&mut ColliderAttack>,

    mut q: Query<(
        Entity,
        &Children,
        &GlobalTransform,
        &mut Enemy,
        &mut Immunity,
        &BossConfig
    ), With<Slam>>
) {
    if q.is_empty() {
        return;
    }

    let (entity, children, tf, mut enemy, mut immunity, cfg) = q.single_mut();
    enemy.vel.y = -30.0;



    let y_level = tf.translation().y;
    if (y_level - cfg.slam_base.y).abs() <= 2.0 {
        immunity.is_immune = false;
        commands.entity(entity).insert(Done::Success);

        for child in children {
            if let Ok(mut atk) = colliders.get_mut(*child) {
                atk.enabled = false;
            }
        }
    }
}