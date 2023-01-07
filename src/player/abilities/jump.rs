use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use std::time::Duration;

use crate::{
    input::InputAction,
    state::GameState,
    player::{
        Player,
        consts::{
            PLAYER_JUMP_SPEED,
            PLAYER_COYOTE_TIME,
            PLAYER_JUMP_BUFFER_TIME
        },
    },
    entity_states::Jump,
    util
};

#[derive(Component)]
pub struct JumpAbility {
    pub jump_buffer: Timer,
    pub coyote_time: Timer,
}

impl Default for JumpAbility {
    fn default() -> Self {
        let mut me = Self {
            coyote_time: Timer::from_seconds(PLAYER_COYOTE_TIME, TimerMode::Once),
            jump_buffer: Timer::from_seconds(PLAYER_JUMP_BUFFER_TIME, TimerMode::Once),
        };

        me.coyote_time.tick(me.coyote_time.duration() + Duration::from_secs(1));
        me.jump_buffer.tick(me.jump_buffer.duration() + Duration::from_secs(1));

        me
    }
}

pub fn register_jump_ability(app: &mut App) {
    app.add_system_set(
        SystemSet::on_update(GameState::Gameplay)
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

pub fn jump_ability_reset_coyote_time(
    mut q: Query<(
        &Player,
        &mut JumpAbility
    ), Without<Jump>>
) {
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
    ), Added<Jump>>
) {
    for (mut player, mut jump) in q.iter_mut() {
        player.vel.y = PLAYER_JUMP_SPEED;
        util::timer_tick_to_finish(&mut jump.coyote_time);
    }
}