use bevy::prelude::*;
use seldom_state::prelude::*;
use crate::pathfind::state_machine::Hurt;
use crate::util;

#[derive(Component, Clone, Debug)]
pub struct HurtAbility {
    pub immunity_timer: Timer,
    pub initial_stun_timer: Timer
}

impl HurtAbility {
    pub fn new(timer_secs: f32) -> Self {
        let mut timer = Timer::from_seconds(timer_secs, TimerMode::Once);
        util::timer_tick_to_finish(&mut timer);

        Self {
            immunity_timer: timer,
            initial_stun_timer: Timer::from_seconds(0.1, TimerMode::Once)
        }
    }

    pub fn is_immune(&self) -> bool {
        !self.immunity_timer.finished()
    }

    pub fn can_stop_hurting(&self) -> bool {
        self.initial_stun_timer.finished()
    }
}

pub fn hurt_ability_trigger(
    mut hurts: Query<&mut HurtAbility, Added<Hurt>>
) {
    for mut hurt in hurts.iter_mut() {
        hurt.immunity_timer.reset();
        hurt.initial_stun_timer.reset();
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
    }
}

pub fn stop_hurting(
    mut commands: Commands,
    hurts: Query<(Entity, &HurtAbility), With<Hurt>>
) {
    for (entity, hurt) in hurts.iter() {
        if hurt.immunity_timer.just_finished() {
            commands.entity(entity).insert(Done::Success);
        }
    }
}