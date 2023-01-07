use bevy::prelude::*;
use seldom_state::prelude::*;

mod triggers;
use crate::entity_states::*;
use triggers::*;
use crate::combat::{DeathTrigger, HurtTrigger};

pub fn register_triggers(app: &mut App) {
    use TriggerPlugin as TP;

    app
        .add_plugin(TP::<FallTrigger>::default())
        .add_plugin(TP::<GroundedTrigger>::default())
        .add_plugin(TP::<NeedsJumpTrigger>::default())
        .add_plugin(TP::<StopHurtTrigger>::default())
        .add_plugin(TP::<HitWallTrigger>::default())
        .add_plugin(TP::<ShootTrigger>::default());
}

pub fn walk_pathfinder_state_machine() -> StateMachine {
    use NotTrigger as Not;

    StateMachine::new(Fall)
        .trans::<Move>(Not(GroundedTrigger), Fall)
        .trans::<Fall>(GroundedTrigger, Move)
        .trans::<Move>(NeedsJumpTrigger, Jump)
        .trans::<Jump>(FallTrigger, Fall)

        .trans::<Jump>(HurtTrigger, Hurt)
        .trans::<Move>(HurtTrigger, Hurt)
        .trans::<Fall>(HurtTrigger, Hurt)

        .trans::<Hurt>(StopHurtTrigger, Fall)
        .trans::<Hurt>(DoneTrigger::Success, Fall)

        .trans::<Move>(DeathTrigger, Die::default())
        .trans::<Jump>(DeathTrigger, Die::default())
        .trans::<Fall>(DeathTrigger, Die::default())
        .trans::<Hurt>(DeathTrigger, Die::default())
        .trans::<Die>(NotTrigger(AlwaysTrigger), Hurt)
}

pub fn melee_pathfinder_state_machine() -> StateMachine {
    walk_pathfinder_state_machine()
}

pub fn ranged_pathfinder_state_machine() -> StateMachine {
    walk_pathfinder_state_machine()
        .trans::<Shoot>(HurtTrigger, Hurt)

        .trans::<Move>(ShootTrigger, Shoot)
        .trans::<Fall>(ShootTrigger, Shoot)

        .trans::<Shoot>(DoneTrigger::Success, Fall)
        .trans::<Shoot>(DeathTrigger, Die::default())
}

pub fn fly_pathfinder_state_machine() -> StateMachine {
    StateMachine::new(Move)
        .trans::<Move>(HurtTrigger, Hurt)

        .trans::<Hurt>(HitWallTrigger, Move)
        .trans::<Hurt>(GroundedTrigger, Move)
        .trans::<Hurt>(DoneTrigger::Success, Move)

        .trans::<Move>(DeathTrigger, Die::default())
        .trans::<Hurt>(DeathTrigger, Die::default())
        .trans::<Die>(NotTrigger(AlwaysTrigger), Hurt)
}