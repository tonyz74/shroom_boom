use bevy::prelude::*;
use seldom_state::prelude::*;

#[derive(Copy, Clone, Component, Reflect)]
pub struct Idle;

#[derive(Copy, Clone, Component, Reflect)]
pub struct Attack;

pub fn snake_enemy_state_machine() -> StateMachine {
    StateMachine::new(Idle)
        .trans::<Idle>(NotTrigger(AlwaysTrigger), Attack)
}