use bevy::prelude::*;
use seldom_state::prelude::*;

mod states;
mod triggers;

pub use states::*;
pub use triggers::*;

pub fn register_triggers(app: &mut App) {
    use TriggerPlugin as TP;

    app
        .add_plugin(TP::<FallTrigger>::default())
        .add_plugin(TP::<GroundedTrigger>::default())
        .add_plugin(TP::<NeedsJumpTrigger>::default())
        .add_plugin(TP::<StopHurtTrigger>::default())
        .add_plugin(TP::<HurtTrigger>::default())
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
}