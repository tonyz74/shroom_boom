use bevy::prelude::*;
use seldom_state::prelude::*;

use crate::{
    state::GameState,
    common::UpdateStage,
    player::{
        Player,
        consts::{
            PLAYER_DASH_LENGTH,
            PLAYER_DASH_COOLDOWN,
            PLAYER_DASH_SPEED
        },
        state_machine as s
    }
};

// Ability

#[derive(Component)]
pub struct DashAbility {
    pub dur: Timer,
    pub cd: Timer
}

impl Default for DashAbility {
    fn default() -> Self {
        Self {
            dur: Timer::from_seconds(PLAYER_DASH_LENGTH, TimerMode::Once),
            cd: Timer::from_seconds(PLAYER_DASH_COOLDOWN, TimerMode::Once)
        }
    }
}

pub fn register_dash_ability(app: &mut App) {
    app.add_system_set(
        SystemSet::on_update(GameState::Gameplay)
            .label(UpdateStage::GameLogic)
            .with_system(dash_ability_trigger)
            .with_system(dash_ability_update)
            .with_system(dash_ability_cooldown_update)
    );
}

// Systems

fn dash_ability_trigger(
    mut q: Query<(
        &mut Player,
        &TextureAtlasSprite,
        &mut DashAbility
    ), Added<s::Dash>>
) {
    for (mut player, spr, mut dash) in q.iter_mut() {
        dash.dur.reset();
        player.vel.y = 0.0;
        player.vel.x = PLAYER_DASH_SPEED * (if spr.flip_x { -1.0 } else { 1.0 });
    }
}

fn dash_ability_update(
    time: Res<Time>,
    mut commands: Commands,
    mut q: Query<(
        Entity,
        &mut Player,
        &mut DashAbility
    ), With<s::Dash>>
) {
    for (e, player, mut dash) in q.iter_mut() {
        let _ = player;
        dash.dur.tick(time.delta());

        // If the dash naturally ends (timer runs out):
        if dash.dur.just_finished() {
            // Transition out of the dashing state
            commands.entity(e)
                .insert(Done::Success);

            dash.cd.reset();
        }
    }
}

fn dash_ability_cooldown_update(
    time: Res<Time>,
    mut q: Query<&mut DashAbility>
) {
    for mut dash in q.iter_mut() {
        dash.cd.tick(time.delta());
    }
}

