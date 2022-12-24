use seldom_state::prelude::*;

mod states;
mod triggers;

pub use states::*;
pub use triggers::*;


pub fn walk_pathfinder_state_machine() -> StateMachine {
    use NotTrigger as Not;

    StateMachine::new(Fall)
        .trans::<Move>(Not(GroundedTrigger), Fall)
        .trans::<Fall>(GroundedTrigger, Move)
        .trans::<Move>(NeedsJumpTrigger, Jump)
        .trans::<Jump>(FallTrigger, Fall)
}