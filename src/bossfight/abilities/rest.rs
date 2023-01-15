use bevy::prelude::*;
use seldom_state::prelude::*;
use crate::bossfight::Boss;
use crate::bossfight::enraged::EnragedAttackMove;
use crate::bossfight::stage::BossStage;
use crate::bossfight::state_machine::AbilityStartup;
use crate::combat::Immunity;
use crate::state::GameState;

#[derive(Debug, Component, Clone)]
pub struct RestAbility {
    pub timer: Timer
}


impl Default for RestAbility {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(4.0, TimerMode::Once)
        }
    }
}


pub fn register_rest_ability(app: &mut App) {
    app.add_system_set(
        SystemSet::on_update(GameState::Gameplay)
            .with_system(start_resting)
            .with_system(rest_update)
    );
}


fn start_resting(
    mut q: Query<(
        &mut Immunity,
        &mut RestAbility,
        &Boss
    ), Added<AbilityStartup>>
) {
    if q.is_empty() {
        return;
    }

    let (mut immunity, mut rest, boss) = q.single_mut();
    if boss.current_move() != EnragedAttackMove::Rest {
        return;
    }

    rest.timer.reset();
    immunity.is_immune = false;
}

fn rest_update(
    time: Res<Time>,
    mut commands: Commands,
    mut q: Query<(Entity, &mut RestAbility, &BossStage, &Boss)>
) {
    if q.is_empty() {
        return;
    }

    let (entity, mut rest, stage, boss) = q.single_mut();

    if stage != &BossStage::Enraged || boss.current_move() != EnragedAttackMove::Rest {
        return;
    }

    rest.timer.tick(time.delta());
    if rest.timer.just_finished() {
        commands.entity(entity).insert(Done::Success);
    }
}
