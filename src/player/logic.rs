use bevy::prelude::*;
use bevy::time::FixedTimestep;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
    input::InputAction,
    player::{Player, state_machine as ps},
    state::GameState,
    common::{UpdateStage, PHYSICS_STEPS_PER_SEC}
};

pub fn player_setup_logic(app: &mut App) {
    app.add_system_set(
        SystemSet::on_update(GameState::Gameplay)
            .label(UpdateStage::GameLogic)
            .after(UpdateStage::Physics)
            .with_system(idle)
            .with_system(run_grounded)
            .with_system(run_air)
            .with_system(jump)
    );

    app.add_system_set(
        SystemSet::on_update(GameState::Gameplay)
            .label(UpdateStage::Physics)
            .with_run_criteria(FixedTimestep::steps_per_second(PHYSICS_STEPS_PER_SEC))
            .with_system(fall.before(physics_update))
            .with_system(physics_update)
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
        -5.0
    } else if action.pressed(InputAction::RunRight) {
        5.0
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
        Vect::new(pos.x, pos.y - (span / 2.0)),
    ];

    for origin in raycast_origins.iter() {
        let rc = ctx.cast_ray(*origin,
                              Vect::new(vel_x, 0.0).normalize(),
                              PLAYER_COLLIDER_CAPSULE.radius + 1.0,
                              true,
                              QueryFilter {
                                  exclude_collider: Some(entity),
                                  ..default()
                              });

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
    mut q: Query<(Entity,
                  &ActionState<InputAction>,
                  &GlobalTransform,
                  &mut Player),
                 Or<(With<ps::Jump>, With<ps::Fall>)>>,
    mut rapier: ResMut<RapierContext>
) {
    if q.is_empty() {
        return;
    }

    let (e, action, tf, mut player) = q.single_mut();
    run_common(e, &action, &mut player, tf, &mut rapier);
}


pub fn jump(mut q: Query<&mut Player, Added<ps::Jump>>) {
    if q.is_empty() {
        return;
    }

    let mut player = q.single_mut();
    player.vel.y = 12.0;
}

pub fn fall(mut q: Query<&mut Player, Or<(With<ps::Jump>, With<ps::Fall>)>>) {
    if q.is_empty() {
        return;
    }

    let mut player = q.single_mut();
    player.vel.y += 0.01667 * -40.0;

    if player.vel.y <= -20.0 {
        player.vel.y = -20.0;
    }
}

pub fn physics_update(
    mut q: Query<(&mut KinematicCharacterController, &Player)>,
) {
    let (mut cc, p) = q.single_mut();
    cc.translation = Some(p.vel);
}

