use bevy::prelude::*;
use seldom_state::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
    input::InputAction,
    state::GameState,
    common::UpdateStage,
    player::{
        Player,
        consts::{
            PLAYER_JUMP_SPEED
        },
        state_machine as s
    }
};

#[derive(Component)]
pub struct JumpAbility {
    pub jump_buffer: Timer,
    pub coyote_time: Timer,
}

impl Default for JumpAbility {
    fn default() -> Self {
        Self {
            jump_buffer: Timer::from_seconds(0.1, TimerMode::Once),
            coyote_time: Timer::from_seconds(0.2, TimerMode::Once),
        }
    }
}

pub fn register_jump_ability(app: &mut App) {
    app.add_system_set(
        SystemSet::on_update(GameState::Gameplay)
            .label(UpdateStage::GameLogic)
            .with_system(jump_ability_trigger)
            .with_system(jump_ability_request)
            .with_system(jump_ability_tick_buffer)
            .with_system(jump_ability_tick_coyote_time)
            .with_system(jump_ability_reset_coyote_time)
    );
}

pub fn jump_ability_request(
    mut q: Query<(&ActionState<InputAction>, &mut JumpAbility)>
) {
    for (input, mut jump) in q.iter_mut() {
        if !input.pressed(InputAction::Jump) {
            return;
        }

        jump.jump_buffer.reset();
    }
}

pub fn jump_ability_reset_coyote_time(mut q: Query<(&Player, &mut JumpAbility), Without<s::Jump>>) {
    for (player, mut jump) in q.iter_mut() {
        if player.grounded {
            jump.coyote_time.reset();
        }
    }
}

pub fn jump_ability_tick_buffer(
    time: Res<Time>,
    mut q: Query<&mut JumpAbility>
) {
    for mut jump in q.iter_mut() {
        jump.jump_buffer.tick(time.delta());
    }
}

pub fn jump_ability_tick_coyote_time(
    time: Res<Time>,
    mut q: Query<(&Player, &mut JumpAbility)>
) {
    for (player, mut jump) in q.iter_mut() {
        if player.grounded {
            return;
        }

        jump.coyote_time.tick(time.delta());
    }
}

pub fn jump_ability_trigger(
    mut q: Query<(
        &mut Player,
        &mut JumpAbility
    ), Added<s::Jump>>
) {
    for (mut player, mut jump) in q.iter_mut() {
        player.vel.y = PLAYER_JUMP_SPEED;

        let dur = jump.coyote_time.duration();
        jump.coyote_time.tick(dur + std::time::Duration::from_secs(1));
    }
}