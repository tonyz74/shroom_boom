use seldom_state::prelude::*;
use crate::pathfind::state_machine::ranged_pathfinder_state_machine;

pub fn pumpkin_enemy_state_machine() -> StateMachine {
    ranged_pathfinder_state_machine()
}