use seldom_state::prelude::*;
use crate::combat::{DeathTrigger, HurtTrigger};
use crate::entity_states::*;

pub fn boss_state_machine() -> StateMachine {
    StateMachine::new(Idle)
        .trans::<Idle>(HurtTrigger, Hurt)
        .trans::<Hurt>(DoneTrigger::Success, Idle)

        .trans::<Idle>(DeathTrigger, Die::default())
}