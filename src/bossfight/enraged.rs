use bevy::prelude::*;
use crate::bossfight::{Boss, BossStage};
use crate::bossfight::state_machine::BeginEnraged;
use crate::entity_states::Idle;
use crate::state::GameState;

#[derive(Copy, Clone, Component, Debug, PartialEq, Eq)]
pub enum EnragedAttackMove {
    Rest,
    Boom,
    RelocateRight,
    ChargeLeft,
    ChargeRight,
    Hover,
    Slam,
}

pub const ATTACK_SEQUENCE_LEN: usize = 21;
pub const ATTACK_SEQUENCE: &[EnragedAttackMove] = &[
    EnragedAttackMove::Rest,

    EnragedAttackMove::Boom,
    EnragedAttackMove::Rest,

    EnragedAttackMove::RelocateRight,

    EnragedAttackMove::Rest,
    EnragedAttackMove::ChargeLeft,
    EnragedAttackMove::Rest,
    EnragedAttackMove::ChargeRight,

    EnragedAttackMove::Rest,
    EnragedAttackMove::ChargeLeft,
    EnragedAttackMove::Rest,
    EnragedAttackMove::ChargeRight,

    EnragedAttackMove::Rest,
    EnragedAttackMove::Hover,
    EnragedAttackMove::Slam,

    EnragedAttackMove::Rest,
    EnragedAttackMove::Hover,
    EnragedAttackMove::Slam,

    EnragedAttackMove::Rest,
    EnragedAttackMove::Hover,
    EnragedAttackMove::Slam,
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
    idling: Query<&Idle>,
    start_idling: Query<Entity, Added<Idle>>,
    mut q: Query<(Entity, &BossStage, &mut Boss)>
) {
    for (entity, stage, mut boss) in q.iter_mut() {
        if stage != &BossStage::Enraged || start_idling.contains(entity) {
            return;
        }

        if idling.contains(entity) {
            boss.move_index = (boss.move_index + 1) % ATTACK_SEQUENCE_LEN;
        }
    }
}