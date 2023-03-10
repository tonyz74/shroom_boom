use bevy::prelude::*;
use bevy::time::FixedTimestep;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
    input::InputAction,
    player::{
        Player,
        state_machine::*,
        consts::{
            PLAYER_FALL_GRAVITY,
            PLAYER_TERMINAL_VELOCITY,
            PLAYER_RUN_SPEED,
        }
    },
    state::GameState,
    common::PHYSICS_STEPS_PER_SEC,
    level::consts::SOLIDS_INTERACTION_GROUP,
    entity_states::*
};
use crate::anim::{AnimationChangeEvent, Animator};
use crate::assets::PlayerAssets;
use crate::coin::drops::CoinHolder;
use crate::combat::{CombatLayerMask, ExplosionEvent, HurtAbility};
use crate::common::PHYSICS_STEP_DELTA;
use crate::fx::smoke::SmokeEvent;
use crate::player::abilities::shoot;
use crate::ui::menu::GotoMenuEvent;
use crate::util::{Facing, FacingX};



#[derive(Resource, Copy, Clone, Default)]
pub struct PlayerScore {
    pub score: u32
}


pub fn player_setup_logic(app: &mut App) {
    use crate::player::abilities::{dash, slash, jump};

    app.add_system_set(
        SystemSet::on_update(GameState::Gameplay)
            .with_system(idle)
            .with_system(run)
            .with_system(enter_fall)
            .with_system(hit_ground)
            .with_system(got_hurt)
            .with_system(start_crouch)
            .with_system(crouch)
            .with_system(player_died)
            .with_system(player_despawn)
            .with_system(player_sync_score)
    );

    dash::register_dash_ability(app);
    slash::register_slash_ability(app);
    jump::register_jump_ability(app);
    shoot::register_shoot_ability(app);

    app.init_resource::<PlayerScore>();

    app.add_system_set(
        SystemSet::on_update(GameState::Gameplay)
            .with_run_criteria(FixedTimestep::steps_per_second(PHYSICS_STEPS_PER_SEC))
            .with_system(fall.before(physics_update))
            .with_system(physics_update)
            .with_system(update_grounded)
    );
}



pub fn player_sync_score(
    mut score: ResMut<PlayerScore>,
    q: Query<&CoinHolder, With<Player>>
) {
    if q.is_empty() {
        return;
    }

    score.score = q.single().total_value as u32;
}

pub fn player_died(
    mut commands: Commands,
    mut q: Query<(Entity, &mut Player, &GlobalTransform), Added<Die>>,
    mut anim: EventWriter<AnimationChangeEvent>,
    anims: Res<PlayerAssets>,
    mut explosions: EventWriter<ExplosionEvent>,
    mut smoke: EventWriter<SmokeEvent>
) {
    if q.is_empty() {
        return;
    }

    let (e, mut player, tf) = q.single_mut();
    player.vel.x = 0.0;

    commands.entity(e).despawn_descendants();
    anim.send(AnimationChangeEvent {
        e, new_anim: anims.anims["DEATH"].clone()
    });

    let pos = Vec2::new(tf.translation().x, tf.translation().y);
    explosions.send(ExplosionEvent {
        pos,
        max_damage: 1,
        radius: 48.0,
        combat_layer: CombatLayerMask::PLAYER
    });

    smoke.send(SmokeEvent {
        pos
    });
}

pub fn player_despawn(
    mut q: Query<(Entity, &mut Die, &Animator), With<Player>>,
    mut state: ResMut<State<GameState>>,
    trans: Res<GotoMenuEvent>
) {
    if q.is_empty() {
        return;
    }

    let (e, mut die, animator) = q.single_mut();

    if animator.anim.name == "DEATH" && animator.total_looped == 1 {
        if !trans.attempt {
            die.should_despawn = true;
            state.push(GameState::GameLostMenu).unwrap();
        }
    }
}

pub fn idle(mut q: Query<&mut Player, (With<Idle>, Without<Die>)>) {
    if q.is_empty() {
        return;
    }

    let mut player = q.single_mut();
    player.vel.x = 0.0;
    player.vel.y = 0.0;
}

// HELPER
fn run_common(
    entity: Entity,
    action: &ActionState<InputAction>,
    player: &mut Player,
    keep_facing: bool,
    facing: &mut Facing,
    tf: &GlobalTransform,
    ctx: &mut RapierContext
) {
    use crate::player::consts::PLAYER_COLLIDER_CAPSULE;

    let vel_x = if action.pressed(InputAction::RunLeft) {
        if !keep_facing {
            facing.x = FacingX::Left;
        }
        -PLAYER_RUN_SPEED
    } else if action.pressed(InputAction::RunRight) {
        if !keep_facing {
            facing.x = FacingX::Right;
        }
        PLAYER_RUN_SPEED
    } else {
        0.0
    };

    let pos = tf.translation();

    let span_x = (PLAYER_COLLIDER_CAPSULE.segment.a.coords.xy().x).abs()
        + (PLAYER_COLLIDER_CAPSULE.segment.b.coords.xy().x).abs()
        + (PLAYER_COLLIDER_CAPSULE.radius);

    let span_y = (PLAYER_COLLIDER_CAPSULE.segment.a.coords.xy().y).abs()
        + (PLAYER_COLLIDER_CAPSULE.segment.b.coords.xy().y).abs()
        + (PLAYER_COLLIDER_CAPSULE.radius);

    // Cast from both head and feet
    let raycast_origins = [
        Vect::new(pos.x, pos.y + span_y / 2.0),
        Vect::new(pos.x, pos.y + 0.0000000000),
        Vect::new(pos.x, pos.y - span_y / 2.0),
    ];

    for origin in raycast_origins.iter() {
        let rc = ctx.cast_ray(
            *origin,
            Vect::new(vel_x, 0.0).normalize(),
            PLAYER_COLLIDER_CAPSULE.radius + span_x / 2.0 + 1.0,
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

pub fn got_hurt(mut q: Query<(&mut Player, &mut HurtAbility), (Added<Hurt>, Without<Die>)>) {
    if q.is_empty() {
        return;
    }

    let (mut player, mut hurt) = q.single_mut();

    if hurt.hit_event.is_none() {
        return;
    }

    let hit_event = hurt.hit_event.take().unwrap();

    let mut kb = Vec2::new(hit_event.kb.x * 4.0, hit_event.kb.y + 4.0);

    if hit_event.kb.x.abs() <= 0.1 {
        kb.x = 0.0;
    }

    player.vel = kb;
}

pub fn hit_ground(mut q: Query<&mut Player, (Added<Move>, Without<Die>)>) {
    if q.is_empty() {
        return;
    }

    let mut player = q.single_mut();
    player.vel.y = 0.0;
}

pub fn run(
    mut q: Query<(
        Entity,
        &ActionState<InputAction>,
        &GlobalTransform,
        &mut Player,
        &mut Facing,
        Option<&Shoot>
    ), (Without<Hurt>, Without<Dash>, Without<Die>, Without<Crouch>)>,
    mut rapier: ResMut<RapierContext>
) {
    if q.is_empty() {
        return;
    }

    let (e, action, tf, mut player, mut facing, is_shooting) = q.single_mut();
    run_common(e, &action, &mut player, is_shooting.is_some(), &mut facing, tf, &mut rapier);
}

pub fn enter_fall(mut q: Query<&mut Player, Added<Fall>>) {
    for mut p in q.iter_mut() {
        if p.vel.y > 0.0 {
            p.vel.y = 0.0;
        }
    }
}

pub fn fall(mut q: Query<&mut Player, Without<Dash>>) {
    if q.is_empty() {
        return;
    }

    let mut player = q.single_mut();

    if player.grounded {
        return;
    }

    // Fixed timestep
    player.vel.y += PHYSICS_STEP_DELTA * PLAYER_FALL_GRAVITY;

    if player.vel.y <= PLAYER_TERMINAL_VELOCITY {
        player.vel.y = PLAYER_TERMINAL_VELOCITY;
    }

}

pub fn start_crouch(mut q: Query<&mut Player, Added<Crouch>>) {
    if q.is_empty() {
        return;
    }

    let mut player = q.single_mut();
    player.vel.x = 0.0;
    if player.vel.y > 2.0 {
        player.vel.y = 0.0;
    } else if player.vel.y > -3.0 {
        player.vel.y = player.vel.y.abs() * -2.0;
    }
}

pub fn crouch(mut q: Query<&mut Player, With<Crouch>>) {
    if q.is_empty() {
        return;
    }

    let mut player = q.single_mut();
    if player.grounded {
        player.vel.y = 0.0;
    }
}

pub fn physics_update(
    mut q: Query<(&mut KinematicCharacterController, &Player)>,
    state: Res<State<GameState>>
) {
    if *state.current() != GameState::Gameplay || q.is_empty() {
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