use seldom_state::prelude::*;
pub use crate::pathfind::state_machine::*;

pub fn dandelion_enemy_state_machine() -> StateMachine {
    fly_pathfinder_state_machine()
}