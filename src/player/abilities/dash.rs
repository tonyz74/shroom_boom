use bevy::prelude::*;
use seldom_state::prelude::*;

use crate::{
    state::GameState,
    player::{
        Player,
        consts::{
            PLAYER_DASH_LENGTH,
            PLAYER_DASH_COOLDOWN,
            PLAYER_DASH_SPEED
        },
        state_machine::*
    }
};
use crate::combat::{ColliderAttack, HurtAbility, Immunity};
use crate::entity_states::Die;
use crate::util::Facing;

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
            .with_system(dash_ability_trigger)
            .with_system(dash_ability_update)
            .with_system(dash_ability_cooldown_update)
    );
}

// Systems

fn dash_ability_trigger(
    mut q: Query<(
        &Children,
        &mut Player,
        &mut DashAbility
    ), (Added<Dash>, Without<Die>)>,
    mut collider_attacks: Query<&mut ColliderAttack>
) {
    if q.is_empty() {
        return;
    }

    let (children, mut player, mut dash) = q.single_mut();

    dash.dur.reset();
    player.vel.y = 0.0;


    for child in children.iter() {
        if let Ok(mut collider_attack) = collider_attacks.get_mut(*child) {
            collider_attack.enabled = true;
        }
    }


    let dir = match player.facing {
        Facing::Left => -1.0,
        Facing::Right => 1.0,
    };

    player.vel.x = dir * PLAYER_DASH_SPEED;
}

fn dash_ability_update(
    time: Res<Time>,
    mut commands: Commands,
    mut q: Query<(
        Entity,
        &Children,
        &mut Player,
        &mut DashAbility,
        &HurtAbility
    ), (With<Dash>, Without<Die>)>,
    mut collider_attacks: Query<&mut ColliderAttack>
) {
    if q.is_empty() {
        return;
    }

    let (e, children, player, mut dash, hurt) = q.single_mut();

    let _ = player;

    dash.dur.tick(time.delta());
    commands.entity(e).insert(Immunity);

    if dash.dur.just_finished() {
        // Transition out of the dashing state
        commands.entity(e)
            .insert(Done::Success);

        dash.cd.reset();

        if !hurt.is_immune() {
            commands.entity(e).remove::<Immunity>();
        }

        for child in children.iter() {
            if let Ok(mut collider_attack) = collider_attacks.get_mut(*child) {
                collider_attack.enabled = false;
            }
        }

    }
}

fn dash_ability_cooldown_update(
    time: Res<Time>,
    mut q: Query<&mut DashAbility, Without<Die>>,
) {
    if q.is_empty() {
        return;
    }

    let mut dash = q.single_mut();
    dash.cd.tick(time.delta());
}