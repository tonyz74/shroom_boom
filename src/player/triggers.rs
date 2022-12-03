use bevy::prelude::*;
use bevy_rapier2d::prelude::KinematicCharacterControllerOutput;
use seldom_state::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
    player::Player,
    input::InputAction
};

pub fn player_setup_triggers(app: &mut App) {
    use TriggerPlugin as TP;

    app

        // Input triggers
        .add_plugin(TP::<RunTrigger>::default())
        .add_plugin(TP::<JumpTrigger>::default())
        .add_plugin(TP::<DashTrigger>::default())
        .add_plugin(TP::<SlashTrigger>::default())

        // Environment triggers
        .add_plugin(TP::<FallTrigger>::default())
        .add_plugin(TP::<GroundedTrigger>::default());
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
    JumpTrigger,
    [InputAction::Jump]
);

action_trigger!(
    With<Player>,
    DashTrigger,
    [InputAction::Dash]
);

action_trigger!(
    With<Player>,
    SlashTrigger,
    [InputAction::Slash]
);

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
        return player.vel.y < 0.0;
    }
}

// HIT GROUND TRIGGER

#[derive(Copy, Clone, Reflect, FromReflect)]
pub struct GroundedTrigger;

impl Trigger for GroundedTrigger {
    type Param<'w, 's> = Query<'w, 's,
                               &'static KinematicCharacterControllerOutput,
                               With<Player>>;

    fn trigger(&self, _: Entity, player_q: &Self::Param<'_, '_>) -> bool {
        if player_q.is_empty() {
            return false;
        }

        let out = player_q.single();
        return out.grounded;
    }
}

