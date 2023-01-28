use bevy::prelude::*;
use seldom_state::prelude::*;
use crate::combat::{DeathTrigger, HurtTrigger};
use crate::entity_states::*;
use crate::player::triggers::ShootTrigger;

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
        .trans::<Jump>(tg::CrouchTrigger, Crouch)
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

        // Exiting Hurting
        .trans::<Hurt>(DoneTrigger::Success, Fall)
        .trans::<Hurt>(tg::HitHeadTrigger, Fall)
        .trans::<Hurt>(tg::StopHurtTrigger, Fall)
        .trans::<Hurt>(tg::HitWallTrigger, Fall)

        // Into Hurting
        .trans::<Idle>(HurtTrigger, Hurt)
        .trans::<Fall>(HurtTrigger, Hurt)
        .trans::<Move>(HurtTrigger, Hurt)
        .trans::<Jump>(HurtTrigger, Hurt)
        .trans::<Slash>(HurtTrigger, Hurt)
        .trans::<Crouch>(HurtTrigger, Hurt)

        // Shooting
        .trans::<Idle>(ShootTrigger, Shoot)
        .trans::<Fall>(ShootTrigger, Shoot)
        .trans::<Move>(ShootTrigger, Shoot)
        .trans::<Jump>(ShootTrigger, Shoot)
        .trans::<Crouch>(ShootTrigger, Shoot)
        .trans::<Shoot>(DoneTrigger::Success, Fall)

        // Death
        .trans::<Idle>(DeathTrigger, Die::default())
        .trans::<Fall>(DeathTrigger, Die::default())
        .trans::<Move>(DeathTrigger, Die::default())
        .trans::<Jump>(DeathTrigger, Die::default())
        .trans::<Slash>(DeathTrigger, Die::default())
        .trans::<Crouch>(DeathTrigger, Die::default())
        .trans::<Dash>(DeathTrigger, Die::default())
        .trans::<Hurt>(DeathTrigger, Die::default())
        .trans::<Die>(Not(AlwaysTrigger), Hurt)
}
