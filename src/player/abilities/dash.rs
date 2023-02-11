use bevy::prelude::*;
use bevy_rapier2d::prelude::RapierContext;
use seldom_state::prelude::*;

use crate::{
    state::GameState,
    player::{
        Player,
        consts::PLAYER_DASH_LENGTH,
        state_machine::*
    }
};
use crate::combat::{AttackStrength, ColliderAttack, CombatLayerMask, HurtAbility, Immunity, ProjectileAttack};
use crate::entity_states::Die;
use crate::player::abilities::autotarget;
use crate::player::abilities::autotarget::{AttackDirection, change_facing_for_direction, direction_for_facing, direction_to_vec, Untargetable};
use crate::player::consts::DASH_LEVELS;
use crate::util::Facing;

// Ability

#[derive(Component)]
pub struct DashAbility {
    pub dur: Timer,
    pub cd: Timer,
    pub speed: f32,
    pub damage: i32
}

impl Default for DashAbility {
    fn default() -> Self {
        Self {
            dur: Timer::from_seconds(PLAYER_DASH_LENGTH, TimerMode::Once),

            speed: DASH_LEVELS[0].0,
            cd: Timer::from_seconds(DASH_LEVELS[0].1, TimerMode::Once),
            damage: DASH_LEVELS[0].2,
        }
    }
}

pub fn register_dash_ability(app: &mut App) {
    app.add_system_set(
        SystemSet::on_update(GameState::Gameplay)
            .with_system(dash_ability_trigger)
            .with_system(dash_ability_update)
            .with_system(dash_ability_cooldown_update)
            .with_system(dash_ability_remove_collider)
    );
}


fn dash_ability_trigger(
    mut commands: Commands,

    mut q: Query<(
        Entity,
        &Children,
        &mut Player,
        &mut Facing,
        &mut DashAbility
    ), (Added<Dash>, Without<Die>)>,

    transforms: Query<&GlobalTransform>,
    combat_layers: Query<&CombatLayerMask>,
    untargetable: Query<&Untargetable>,
    projectiles: Query<&ProjectileAttack>,

    mut collider_attacks: Query<&mut ColliderAttack>,
    rapier: Res<RapierContext>
) {
    if q.is_empty() {
        return;
    }

    let (entity, children, mut player, mut facing, mut dash) = q.single_mut();

    dash.dur.reset();
    player.vel.y = 0.0;


    for child in children.iter() {
        if let Ok(mut collider_attack) = collider_attacks.get_mut(*child) {
            collider_attack.enabled = true;
        }
    }

    let combat_layer = combat_layers.get(entity).unwrap();

    if player.vel.x == 0.0 {
        let dir = if let Some((_, dir)) = autotarget::get_closest_target(
            &mut commands,
            entity,
            *combat_layer,
            256.0,
            false,
            &transforms,
            &combat_layers,
            &untargetable,
            &projectiles,
            false,
            &rapier
        ) {
            match dir {
                AttackDirection::Up | AttackDirection::Down => direction_for_facing(*facing),
                _ => dir,
            }
        } else {
            direction_for_facing(*facing)
        };

        change_facing_for_direction(&mut facing, dir);
        let x = direction_to_vec(dir).x;

        player.vel.x = x * dash.speed;
    } else {
        // Player already has requested direction
        player.vel.x = Vec2::new(player.vel.x, 0.0).normalize().x * dash.speed;
    }
}

fn dash_ability_remove_collider(
    mut p: Query<&mut ColliderAttack>,
    q: Query<&Children, (With<Player>, Without<Dash>, Without<Die>)>
) {
    if q.is_empty() {
        return;
    }

    let children = q.single();
    for child in children.iter() {
        if let Ok(mut atk) = p.get_mut(*child) {
            atk.enabled = false;
        }
    }
}

fn dash_ability_update(
    time: Res<Time>,
    mut commands: Commands,
    mut q: Query<(
        Entity,
        &Children,
        &mut Player,
        &mut DashAbility,
        &mut Immunity,
        &HurtAbility
    ), (With<Dash>, Without<Die>)>,
    mut collider_attacks: Query<(&mut ColliderAttack, &mut AttackStrength)>
) {
    if q.is_empty() {
        return;
    }

    let (e, children, player, mut dash, mut immunity, hurt) = q.single_mut();

    let _ = player;

    dash.dur.tick(time.delta());
    immunity.is_immune = true;

    if dash.dur.just_finished() {
        // Transition out of the dashing state
        commands.entity(e)
            .insert(Done::Success);

        dash.cd.reset();

        if !hurt.is_immune() {
            immunity.is_immune = false;
        }

        for child in children.iter() {
            if let Ok((mut collider_attack, mut strength)) = collider_attacks.get_mut(*child) {
                collider_attack.enabled = false;
                strength.power = dash.damage;
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