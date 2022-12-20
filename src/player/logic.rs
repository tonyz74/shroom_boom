use bevy::prelude::*;
use bevy::time::FixedTimestep;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
    input::InputAction,
    player::{
        Player,
        state_machine as ps,
        consts::{
            PLAYER_FALL_GRAVITY,
            PLAYER_TERMINAL_VELOCITY,
            PLAYER_RUN_SPEED,
        }
    },
    state::GameState,
    common::{UpdateStage, PHYSICS_STEPS_PER_SEC},
    level::consts::SOLIDS_INTERACTION_GROUP
};

pub fn player_setup_logic(app: &mut App) {
    use crate::player::abilities::{dash, slash, jump};

    app.add_system_set(
        SystemSet::on_update(GameState::Gameplay)
            .label(UpdateStage::GameLogic)
            .with_system(idle)
            .with_system(run_grounded)
            .with_system(run_air)
            .with_system(enter_fall)
            .with_system(crouch)
            .with_system(crouch_update)
    );

    dash::register_dash_ability(app);
    slash::register_slash_ability(app);
    jump::register_jump_ability(app);

    app.add_system_set(
        SystemSet::on_update(GameState::Gameplay)
            .label(UpdateStage::Physics)
            .after(UpdateStage::GameLogic)
            .with_run_criteria(FixedTimestep::steps_per_second(PHYSICS_STEPS_PER_SEC))
            .with_system(fall.before(physics_update))
            .with_system(physics_update)
            .with_system(update_grounded)
    );
}

pub fn idle(mut q: Query<&mut Player, With<ps::Idle>>) {
    if q.is_empty() {
        return;
    }

    let mut player = q.single_mut();
    player.vel.x = 0.0;
    player.vel.y = 0.0;
}

// HELPER
fn run_common(entity: Entity,
              action: &ActionState<InputAction>,
              player: &mut Player,
              tf: &GlobalTransform,
              ctx: &mut RapierContext) {

    use crate::player::consts::PLAYER_COLLIDER_CAPSULE;

    let vel_x = if action.pressed(InputAction::RunLeft) {
        -PLAYER_RUN_SPEED
    } else if action.pressed(InputAction::RunRight) {
        PLAYER_RUN_SPEED
    } else {
        0.0
    };

    let pos = tf.translation();

    let span = (PLAYER_COLLIDER_CAPSULE.segment.a.coords.xy().y).abs()
        + (PLAYER_COLLIDER_CAPSULE.segment.b.coords.xy().y).abs()
        + (PLAYER_COLLIDER_CAPSULE.radius);

    // Cast from both head and feet
    let raycast_origins = [
        Vect::new(pos.x, pos.y + (span / 2.0)),
        Vect::new(pos.x, pos.y + (0.00000000)),
        Vect::new(pos.x, pos.y - (span / 2.0)),
    ];

    for origin in raycast_origins.iter() {
        let rc = ctx.cast_ray(
            *origin,
            Vect::new(vel_x, 0.0).normalize(),
            PLAYER_COLLIDER_CAPSULE.radius + 1.0,
            true,
            QueryFilter {
                flags: QueryFilterFlags::EXCLUDE_SENSORS,
                exclude_collider: Some(entity),
                groups: Some(SOLIDS_INTERACTION_GROUP),
                ..default()
            }
        );

        if rc.is_some() {
            player.vel.x = 0.0;
            return;
        }
    }

    player.vel.x = vel_x;
}

pub fn run_grounded(
    mut q: Query<(Entity,
                  &ActionState<InputAction>,
                  &GlobalTransform,
                  &mut Player),
                  With<ps::Run>>,
    mut rapier: ResMut<RapierContext>
) {
    if q.is_empty() {
        return;
    }

    let (e, action, tf, mut player) = q.single_mut();
    player.vel.y = 0.0;

    run_common(e, &action, &mut player, tf, &mut rapier);
}

pub fn run_air(
    mut q: Query<(
        Entity,
        &ActionState<InputAction>,
        &GlobalTransform,
        &mut Player
    ), Or<(
        With<ps::Jump>,
        With<ps::Fall>,
        With<ps::Slash>,
        With<ps::Crouch>
    )>>,
    mut rapier: ResMut<RapierContext>
) {
    if q.is_empty() {
        return;
    }

    let (e, action, tf, mut player) = q.single_mut();
    run_common(e, &action, &mut player, tf, &mut rapier);
}

pub fn enter_fall(mut q: Query<&mut Player, Added<ps::Fall>>) {
    for mut p in q.iter_mut() {
        if p.vel.y > 0.0 {
            p.vel.y = 0.0;
        }
    }
}

pub fn fall(
    mut q: Query<
        (&mut Player, Option<&ps::Slash>),
        Or<(
            With<ps::Jump>,
            With<ps::Fall>,
            With<ps::Slash>,
            With<ps::Crouch>,
        )>
    >
) {
    if q.is_empty() {
        return;
    }

    let (mut player, _s) = q.single_mut();

    if player.grounded {
        return;
    }

    // Fixed timestep
    player.vel.y += 0.01667 * PLAYER_FALL_GRAVITY;

    if player.vel.y <= PLAYER_TERMINAL_VELOCITY {
        player.vel.y = PLAYER_TERMINAL_VELOCITY;
    }

}

pub fn physics_update(
    mut q: Query<(&mut KinematicCharacterController, &Player)>,
    state: Res<State<GameState>>
) {
    if *state.current() != GameState::Gameplay {
        return;
    }

    let (mut cc, p) = q.single_mut();
    cc.translation = Some(p.vel);
}

pub fn update_grounded(mut q: Query<(&mut Player, &KinematicCharacterControllerOutput)>) {
    for (mut player, out) in q.iter_mut() {
        player.grounded = out.grounded;
    }
}

pub fn crouch(q: Query<&Player, Added<ps::Crouch>>) {
    for _ in q.iter() {
    }
}

pub fn crouch_update(mut q: Query<&mut Player, With<ps::Crouch>>) {
    for mut player in q.iter_mut() {
        if player.grounded {
            player.vel.y = 0.0;
        }
    }
}