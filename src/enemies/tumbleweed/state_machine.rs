use seldom_state::prelude::*;
pub use crate::pathfind::state_machine::*;

pub fn tumbleweed_enemy_state_machine() -> StateMachine {
    melee_pathfinder_state_machine()
}
