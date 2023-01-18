use bevy::prelude::*;
use crate::bossfight::{Boss, BossStage};
use crate::bossfight::state_machine::{BeginEnraged, PickNextMove};
use crate::state::GameState;

#[derive(Copy, Clone, Component, Debug, PartialEq)]
pub enum EnragedAttackMove {
    Rest(f32),
    Boom,
    RelocateRight,
    ChargeLeft,
    ChargeRight,
    Leap,
    Takeoff,
    Hover,
    Slam,
}

pub const ATTACK_SEQUENCE: &[EnragedAttackMove] = &[
    EnragedAttackMove::Boom,
    EnragedAttackMove::Rest(1.0),

    EnragedAttackMove::RelocateRight,
    EnragedAttackMove::Rest(1.0),
    EnragedAttackMove::ChargeLeft,
    EnragedAttackMove::Rest(1.0),
    EnragedAttackMove::ChargeRight,

    EnragedAttackMove::Rest(1.0),
    EnragedAttackMove::ChargeLeft,
    EnragedAttackMove::Rest(1.0),
    EnragedAttackMove::ChargeRight,
    EnragedAttackMove::Rest(1.0),

    EnragedAttackMove::Leap,
    EnragedAttackMove::Hover,
    EnragedAttackMove::Rest(0.25),
    EnragedAttackMove::Slam,
    EnragedAttackMove::Rest(1.0),

    EnragedAttackMove::Takeoff,
    EnragedAttackMove::Hover,
    EnragedAttackMove::Rest(0.25),
    EnragedAttackMove::Slam,
    EnragedAttackMove::Rest(1.0),

    EnragedAttackMove::Takeoff,
    EnragedAttackMove::Hover,
    EnragedAttackMove::Rest(0.25),
    EnragedAttackMove::Slam,
    EnragedAttackMove::Rest(1.0),

    EnragedAttackMove::Rest(1.0),
];

pub fn register_boss_enraged(app: &mut App) {
    app.add_system_set(
        SystemSet::on_update(GameState::Gameplay)
            .with_system(boss_enter_enraged)
            .with_system(boss_enraged_update)
    );
}

pub fn boss_enter_enraged(
    mut q: Query<&mut Boss, Added<BeginEnraged>>
) {

    for mut boss in q.iter_mut() {
        boss.move_index = 0;
    }

}

pub fn boss_enraged_update(
    mut q: Query<(&BossStage, &mut Boss), With<PickNextMove>>
) {
    if q.is_empty() {
        return;
    }

    let (stage, mut boss) = q.single_mut();

    if stage != &BossStage::Enraged {
        return;
    }

    boss.move_index = (boss.move_index + 1) % ATTACK_SEQUENCE.len();
}