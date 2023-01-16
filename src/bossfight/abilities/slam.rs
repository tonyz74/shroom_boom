use bevy::prelude::*;
use seldom_state::prelude::Done;
use crate::bossfight::{Boss, BossConfig};
use crate::bossfight::enraged::EnragedAttackMove;
use crate::bossfight::state_machine::{AbilityStartup, Slam};
use crate::combat::Immunity;
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
    mut q: Query<(
        &mut Immunity,
        &Boss
    ), Added<AbilityStartup>>
) {
    if q.is_empty() {
        return;
    }

    let (mut immunity, boss) = q.single_mut();

    if boss.current_move() != EnragedAttackMove::Slam {
        return;
    }

    immunity.is_immune = true;
}

fn slam_update(
    mut commands: Commands,
    mut q: Query<(
        Entity,
        &GlobalTransform,
        &mut Enemy,
        &mut Immunity,
        &BossConfig
    ), With<Slam>>
) {
    if q.is_empty() {
        return;
    }

    let (entity, tf, mut enemy, mut immunity, cfg) = q.single_mut();
    enemy.vel.y = -30.0;


    let y_level = tf.translation().y;
    println!("diff: {:?}", (y_level - cfg.slam_base.y).abs());
    if (y_level - cfg.slam_base.y).abs() <= 2.0 {
        immunity.is_immune = false;
        commands.entity(entity).insert(Done::Success);
    }
}