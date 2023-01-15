use bevy::prelude::*;
use seldom_state::prelude::*;
use crate::bossfight::{Boss, BossConfig};
use crate::bossfight::enraged::EnragedAttackMove;
use crate::bossfight::state_machine::{AbilityStartup, Relocate};
use crate::combat::{ColliderAttack, Immunity};
use crate::state::GameState;


#[derive(Component, Clone, Debug)]
pub struct RelocateAbility {
    pub retract: Timer,
    pub extend: Timer
}

impl Default for RelocateAbility {
    fn default() -> Self {
        Self {
            retract: Timer::from_seconds(0.8, TimerMode::Once),
            extend: Timer::from_seconds(0.8, TimerMode::Once)
        }
    }
}

pub fn register_relocate_ability(app: &mut App) {
    app.add_system_set(
        SystemSet::on_update(GameState::Gameplay)
            .with_system(start_relocation)
            .with_system(relocate_update)
    );
}

fn start_relocation(
    mut q: Query<(
        &mut Immunity,
        &mut ColliderAttack,
        &mut RelocateAbility,
        &Boss,
    ), Added<AbilityStartup>>
) {
    if q.is_empty() {
        return;
    }

    let (mut immunity, mut atk, mut relocate, boss) = q.single_mut();
    if boss.current_move() != EnragedAttackMove::RelocateRight {
        return;
    }

    atk.enabled = false;
    immunity.is_immune = true;

    relocate.retract.reset();
    relocate.extend.reset();
}

fn relocate_update(
    time: Res<Time>,
    mut commands: Commands,
    mut q: Query<(
        Entity,
        &mut RelocateAbility,
        &mut Transform,
        &BossConfig,
        &mut Immunity
    ), With<Relocate>>
) {
    if q.is_empty() {
        return;
    }

    let (entity, mut relocate, mut transform, cfg, mut immunity) = q.single_mut();
    relocate.retract.tick(time.delta());

    if relocate.retract.just_finished() {

        // Move the boss
        transform.translation.x = cfg.charge_right.x;
        transform.translation.y = cfg.charge_right.y;

        let tl = transform.translation;
        transform.rotate_around(tl, Quat::from_rotation_z((3.14 / 180.0) * 90.0));

    }

    if relocate.retract.finished() {
        relocate.extend.tick(time.delta());

        if relocate.extend.just_finished() {
            commands.entity(entity)
                .insert(Done::Success);

            immunity.is_immune = false;
        }
    }
}