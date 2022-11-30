use bevy::prelude::*;
use seldom_state::prelude::*;

// STATES

#[derive(Component, Reflect, Copy, Clone)]
pub struct Idle;

#[derive(Component, Reflect, Copy, Clone)]
pub struct Run;

#[derive(Component, Reflect, Copy, Clone)]
pub struct Jump;

#[derive(Component, Reflect, Copy, Clone)]
pub struct Fall;

#[derive(Component, Reflect, Copy, Clone)]
pub struct Teleport;

#[derive(Component, Reflect, Copy, Clone)]
pub struct Slash;

#[derive(Component, Reflect, Copy, Clone)]
pub struct Dash;


// TRIGGERS

pub fn player_state_machine() -> StateMachine {
    use NotTrigger as Not;
    use crate::player::triggers as tg;

    StateMachine::new(Fall)
        // To Idling
        .trans::<Fall>(tg::GroundedTrigger, Run)
        .trans::<Run>(Not(tg::RunTrigger), Idle)

        // To Running
        .trans::<Idle>(tg::RunTrigger, Run)

        // To Jumping
        .trans::<Idle>(tg::JumpTrigger, Jump)
        .trans::<Run>(tg::JumpTrigger, Jump)

        // To Falling
        .trans::<Run>(Not(tg::GroundedTrigger), Fall)
        .trans::<Idle>(Not(tg::GroundedTrigger), Fall)
        .trans::<Jump>(tg::FallTrigger, Fall)
        .trans::<Fall>(Not(AlwaysTrigger), Jump)
}
