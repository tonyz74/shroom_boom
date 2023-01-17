use bevy::prelude::*;
use seldom_state::prelude::Done;
use crate::bossfight::{Boss, BossConfig};
use crate::bossfight::consts::BOSS_TAKEOFF_SPEED;
use crate::bossfight::enraged::EnragedAttackMove;
use crate::bossfight::state_machine::{AbilityStartup, Takeoff};
use crate::combat::Immunity;
use crate::enemies::Enemy;
use crate::state::GameState;

#[derive(Component, Debug, Clone)]
pub struct TakeoffAbility;

impl Default for TakeoffAbility  {
    fn default() -> Self {
        Self
    }
}

pub fn register_takeoff_ability(app: &mut App) {
    app.add_system_set(
        SystemSet::on_update(GameState::Gameplay)
            .with_system(start_takeoff)
            .with_system(takeoff_update)
    );
}

fn start_takeoff(
    mut q: Query<(
        &mut Immunity,
        &TakeoffAbility,
        &Boss
    ), Added<AbilityStartup>>
) {
    if q.is_empty() {
        return;
    }

    let (mut immunity, _takeoff, boss) = q.single_mut();
    if boss.current_move() != EnragedAttackMove::Takeoff {
        return;
    }

    immunity.is_immune = true;
}

fn takeoff_update(
    mut commands: Commands,
    mut q: Query<(
        Entity,
        &mut Transform,
        &GlobalTransform,
        &mut Immunity,
        &mut Enemy,
        &BossConfig
    ), With<Takeoff>>
) {
    if q.is_empty() {
        return;
    }

    let (entity, mut mov, transform, mut immunity, mut enemy, cfg) = q.single_mut();
    let y_level = transform.translation().y;

    if y_level > cfg.hover_base.y {
        immunity.is_immune = false;
        commands.entity(entity).insert(Done::Success);
        mov.translation.y = cfg.hover_base.y;
        enemy.vel = Vec2::ZERO;

        return;
    }

    enemy.vel.y = BOSS_TAKEOFF_SPEED;
}
