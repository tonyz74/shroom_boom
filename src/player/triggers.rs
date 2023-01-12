use bevy::prelude::*;
use seldom_state::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
    player::{
        Player,
        consts::PLAYER_SIZE_PX,
        abilities::{
            dash::DashAbility,
            slash::SlashAbility,
            jump::JumpAbility
        }
    },
    input::InputAction
};
use crate::combat::HurtAbility;
use crate::level::consts::SOLIDS_INTERACTION_GROUP;
use crate::player::abilities::shoot::ShootAbility;
use crate::player::ammo::Ammo;
use crate::player::consts::PLAYER_COLLIDER_CAPSULE;

pub fn player_setup_triggers(app: &mut App) {
    use TriggerPlugin as TP;

    app
        // Input triggers
        .add_plugin(TP::<RunTrigger>::default())
        .add_plugin(TP::<JumpTrigger>::default())
        .add_plugin(TP::<DashTrigger>::default())
        .add_plugin(TP::<SlashTrigger>::default())
        .add_plugin(TP::<ShootTrigger>::default())
        .add_plugin(TP::<CrouchTrigger>::default())

        // Environment triggers
        .add_plugin(TP::<FallTrigger>::default())
        .add_plugin(TP::<HitWallTrigger>::default())
        .add_plugin(TP::<HitHeadTrigger>::default())
        .add_plugin(TP::<GroundedTrigger>::default())
        .add_plugin(TP::<StopHurtTrigger>::default());
}


// ACTION TRIGGERS

macro_rules! action_trigger {
    ($filter: ty, $trig_name:ident, $actions:expr) => {
        #[derive(Copy, Clone, Reflect, FromReflect)]
        pub struct $trig_name;

        impl Trigger for $trig_name {
            type Param<'w, 's> = Query<'w, 's, &'static ActionState<InputAction>, $filter>;

            fn trigger(&self, _: Entity, actions: &Self::Param<'_, '_>) -> bool {
                let action_state = actions.single();

                for i in $actions {
                    if action_state.pressed(i) {
                        return true;
                    }
                }

                return false;
            }
        }
    }
}

action_trigger!(
    With<Player>,
    RunTrigger,
    [InputAction::RunLeft, InputAction::RunRight]
);

action_trigger!(
    With<Player>,
    CrouchTrigger,
    [InputAction::Crouch]
);

#[derive(Copy, Clone, Reflect, FromReflect)]
pub struct DashTrigger;

impl Trigger for DashTrigger {
    type Param<'w, 's> = Query<'w, 's, (
        &'static ActionState<InputAction>,
        &'static DashAbility
    ), With<Player>>;

    fn trigger(&self, _: Entity, params: &Self::Param<'_, '_>) -> bool {
        for (input, dash) in params.iter() {
            let ok = input.pressed(InputAction::Dash) && dash.cd.finished();
            return ok;
        }
        false
    }
}
//
action_trigger!(
    With<Player>,
    JumpTrigger,
    [InputAction::Jump]
);

// #[derive(Copy, Clone, Reflect, FromReflect)]
// pub struct JumpTrigger;

// impl Trigger for JumpTrigger {
//     type Param<'w, 's> = Query<'w, 's, &'static JumpAbility, With<Player>>;
//
//     fn trigger(&self, _: Entity, player_q: &Self::Param<'_, '_>) -> bool {
//         if player_q.is_empty() {
//             return false;
//         }
//
//         let jump = player_q.single();
//         let ok = !jump.coyote_time.finished() && !jump.jump_buffer.finished();
//         ok
//     }
// }

#[derive(Copy, Clone, Reflect, FromReflect)]
pub struct SlashTrigger;

impl Trigger for SlashTrigger {
    type Param<'w, 's> = Query<'w, 's, (
        &'static ActionState<InputAction>,
        &'static SlashAbility
    )>;

    fn trigger(&self, _: Entity, actions: &Self::Param<'_, '_>) -> bool {
        let (action_state, slash) = actions.single();

        action_state.pressed(InputAction::Slash) && slash.cd.finished()
    }
}
// FALLING TRIGGER

#[derive(Copy, Clone, Reflect, FromReflect)]
pub struct FallTrigger;

impl Trigger for FallTrigger {
    type Param<'w, 's> = Query<'w, 's, &'static Player>;

    fn trigger(&self, _: Entity, player_q: &Self::Param<'_, '_>) -> bool {
        if player_q.is_empty() {
            return false;
        }

        let player = player_q.single();
        player.vel.y < 0.0
    }
}

#[derive(Copy, Clone, Reflect, FromReflect)]
pub struct HitHeadTrigger;

impl Trigger for HitHeadTrigger {
    type Param<'w, 's> = (
        Query<'w, 's, (Entity, &'static Player, &'static GlobalTransform)>,
        Res<'w, RapierContext>
    );

    fn trigger(&self, _: Entity, (q, ctx): &Self::Param<'_, '_>) -> bool {
        if q.is_empty() {
            return false;
        }

        let (entity, ply, tf) = q.single();
        let pos = Vec2::new(tf.translation().x, tf.translation().y);

        let span = PLAYER_COLLIDER_CAPSULE.radius;

        let origins = [
            Vect::new(pos.x - span, pos.y),
            Vect::new(pos.x + 0.00, pos.y),
            Vect::new(pos.x + span, pos.y),
        ];

        for origin in origins {
            let rc = ctx.cast_ray(
                origin,
                Vect::new(0.0, 1.0).normalize(),
                PLAYER_SIZE_PX.y / 2.0 + ply.vel.y + 1.0,
                true,
                QueryFilter {
                    flags: QueryFilterFlags::EXCLUDE_SENSORS,
                    exclude_collider: Some(entity),
                    groups: Some(SOLIDS_INTERACTION_GROUP),
                    ..default()
                }
            );

            if rc.is_some() {
                return true;
            }
        }

        false
    }
}

// HIT GROUND TRIGGER

#[derive(Copy, Clone, Reflect, FromReflect)]
pub struct GroundedTrigger;

impl Trigger for GroundedTrigger {
    type Param<'w, 's> = Query<'w, 's, &'static Player>;

    fn trigger(&self, _: Entity, player_q: &Self::Param<'_, '_>) -> bool {
        if player_q.is_empty() {
            return false;
        }

        let p = player_q.single();
        p.grounded
    }
}

#[derive(Copy, Clone, Reflect, FromReflect)]
pub struct StopHurtTrigger;

impl Trigger for StopHurtTrigger {
    type Param<'w, 's> = Query<'w, 's, (&'static Player, &'static HurtAbility)>;

    fn trigger(&self, _: Entity, player_q: &Self::Param<'_, '_>) -> bool {
        if player_q.is_empty() {
            return false;
        }

        let (player, hurt) = player_q.single();
        player.grounded && hurt.can_stop_hurting()
    }
}







#[derive(Copy, Clone, Reflect, FromReflect)]
pub struct HitWallTrigger;

impl Trigger for HitWallTrigger {
    type Param<'w, 's> = (
        Query<'w, 's, &'static Player>,
        Query<'w, 's, &'static KinematicCharacterControllerOutput>
    );

    fn trigger(
        &self,
        entity: Entity,
        (player, cc_outs): &Self::Param<'_, '_>
    ) -> bool {
        if player.is_empty() { return false; }

        let player = player.single();

        if cc_outs.contains(entity) {
            let out = cc_outs.get(entity).unwrap();
            if out.desired_translation != Vec2::ZERO && out.effective_translation == Vec2::ZERO {
                return true;
            }
        }

        let ok = player.vel.length() <= 1.0;
        ok
    }
}


#[derive(Copy, Clone, Reflect, FromReflect)]
pub struct ShootTrigger;

impl Trigger for ShootTrigger {
    type Param<'w, 's> = Query<'w, 's, (
        &'static ActionState<InputAction>,
        &'static ShootAbility,
        &'static Ammo
    ), With<Player>>;

    fn trigger(&self, _: Entity, q: &Self::Param<'_, '_>) -> bool {
        if q.is_empty() {
            return false;
        }

        let (input, shoot, ammo) = q.single();
        input.pressed(InputAction::Shoot) && shoot.cd.finished() && ammo.rounds_left > 0
    }
}