use bevy::prelude::*;
use seldom_state::prelude::*;
use crate::combat::{CombatEvent, Immunity};
use crate::pathfind::state_machine::Hurt;
use crate::util;

#[derive(Component, Clone, Debug)]
pub struct HurtAbility {
    pub immunity_timer: Timer,
    pub initial_stun_timer: Timer,
    pub regain_control_timer: Option<Timer>,
    pub hit_event: Option<CombatEvent>
}

impl HurtAbility {
    pub fn new(immunity_len: f32, regain_control_len: Option<f32>) -> Self {
        let mut immunity_timer = Timer::from_seconds(immunity_len, TimerMode::Once);
        util::timer_tick_to_finish(&mut immunity_timer);

        let regain_control_timer = match regain_control_len {
            Some(len) => {
                let mut timer = Timer::from_seconds(len, TimerMode::Once);
                util::timer_tick_to_finish(&mut timer);
                Some(timer)
            },
            None => None
        };

        let mut initial_stun_timer = Timer::from_seconds(0.1, TimerMode::Once);
        util::timer_tick_to_finish(&mut initial_stun_timer);

        Self {
            immunity_timer,
            regain_control_timer,

            initial_stun_timer: Timer::from_seconds(0.1, TimerMode::Once),
            hit_event: None
        }
    }

    pub fn is_immune(&self) -> bool {
        !self.immunity_timer.finished()
    }

    pub fn can_stop_hurting(&self) -> bool {
        self.initial_stun_timer.finished()
    }
}

pub fn hurt_ability_trigger(mut hurts: Query<&mut HurtAbility, Added<Hurt>>) {
    for mut hurt in hurts.iter_mut() {
        hurt.immunity_timer.reset();
        hurt.initial_stun_timer.reset();

        if let Some(timer) = &mut hurt.regain_control_timer {
            timer.reset();
        }
    }
}

pub fn add_immunity_while_hurting(
    mut commands: Commands,
    hurts: Query<(Entity, &HurtAbility)>
) {
    for (entity, hurt) in hurts.iter() {
        if hurt.is_immune() {
            if let Some(mut e_cmd) = commands.get_entity(entity) {
                e_cmd.insert(Immunity);
            }
        }
    }
}

pub fn hurt_ability_tick_immunity(
    time: Res<Time>,
    mut hurts: Query<&mut HurtAbility>
) {
    for mut hurt in hurts.iter_mut() {
        let dt = time.delta();

        hurt.immunity_timer.tick(dt);
        hurt.initial_stun_timer.tick(dt);

        if let Some(regain_control_timer) = &mut hurt.regain_control_timer {
            regain_control_timer.tick(dt);
        }
    }
}

pub fn stop_hurting(
    mut commands: Commands,
    hurts: Query<(Entity, &HurtAbility), With<Hurt>>
) {
    for (entity, hurt) in hurts.iter() {
        if let Some(regain_control_timer) = &hurt.regain_control_timer {
            if regain_control_timer.just_finished() {
                commands.entity(entity).insert(Done::Success);
            }
        }

        if hurt.immunity_timer.just_finished() {
            commands.entity(entity).insert(Done::Success);
        }
    }
}

pub fn remove_immunity(
    mut commands: Commands,
    hurts: Query<(Entity, &HurtAbility)>
) {
    for (entity, hurt) in hurts.iter() {
        if hurt.immunity_timer.just_finished() {
            commands.entity(entity).remove::<Immunity>();
        }
    }
}