use seldom_state::prelude::*;
pub use crate::pathfind::state_machine::*;

pub fn flower_enemy_state_machine() -> StateMachine {
    walk_pathfinder_state_machine()
}