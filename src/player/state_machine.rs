use bevy::prelude::*;
use seldom_state::prelude::*;
use crate::combat::HurtTrigger;
use crate::entity_states::*;

// STATES

#[derive(Component, Reflect, Copy, Clone)]
pub struct Slash;

#[derive(Component, Reflect, Copy, Clone)]
pub struct Dash;

#[derive(Component, Reflect, Copy, Clone)]
pub struct Crouch;

// TRIGGERS

pub fn player_state_machine() -> StateMachine {
    use NotTrigger as Not;
    use crate::player::triggers as tg;

    StateMachine::new(Fall)
        // To Idling
        .trans::<Move>(Not(tg::RunTrigger), Idle)

        // To Running
        .trans::<Idle>(tg::RunTrigger, Move)
        .trans::<Fall>(tg::GroundedTrigger, Move)

        // To Crouching
        .trans::<Move>(tg::CrouchTrigger, Crouch)
        .trans::<Fall>(tg::CrouchTrigger, Crouch)
        .trans::<Idle>(tg::CrouchTrigger, Crouch)

        // To Falling
        .trans::<Move>(Not(tg::GroundedTrigger), Fall)
        .trans::<Crouch>(Not(tg::CrouchTrigger), Fall)
        .trans::<Idle>(Not(tg::GroundedTrigger), Fall)
        .trans::<Jump>(tg::FallTrigger, Fall)
        .trans::<Jump>(tg::HitHeadTrigger, Fall)
        .trans::<Slash>(DoneTrigger::Success, Fall)
        .trans::<Dash>(DoneTrigger::Success, Fall)

        // To Slashing
        .trans::<Idle>(tg::SlashTrigger, Slash)
        .trans::<Move>(tg::SlashTrigger, Slash)
        .trans::<Jump>(tg::SlashTrigger, Slash)
        .trans::<Fall>(tg::SlashTrigger, Slash)

        // To Jumping
        .trans::<Idle>(tg::JumpTrigger, Jump)
        .trans::<Move>(tg::JumpTrigger, Jump)
        .trans::<Crouch>(tg::JumpTrigger, Jump)
        .trans::<Slash>(tg::JumpTrigger, Jump)
        .trans::<Fall>(tg::JumpTrigger, Jump)

        // To Dashing
        .trans::<Idle>(tg::DashTrigger, Dash)
        .trans::<Move>(tg::DashTrigger, Dash)
        .trans::<Fall>(tg::DashTrigger, Dash)
        .trans::<Jump>(tg::DashTrigger, Dash)
        .trans::<Slash>(tg::DashTrigger, Dash)
        .trans::<Crouch>(tg::DashTrigger, Dash)

        .trans::<Hurt>(DoneTrigger::Success, Fall)
        .trans::<Hurt>(tg::HitHeadTrigger, Fall)
        .trans::<Hurt>(tg::StopHurtTrigger, Fall)
        .trans::<Hurt>(tg::HitWallTrigger, Fall)

        .trans::<Idle>(HurtTrigger, Hurt)
        .trans::<Fall>(HurtTrigger, Hurt)
        .trans::<Move>(HurtTrigger, Hurt)
        .trans::<Jump>(HurtTrigger, Hurt)
        .trans::<Slash>(HurtTrigger, Hurt)
        .trans::<Crouch>(HurtTrigger, Hurt)
}
