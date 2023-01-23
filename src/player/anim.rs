#![allow(unused)]

use bevy::prelude::*;
use crate::{
    state::GameState,
    assets::PlayerAssets,
    player::{
        Player,
        state_machine::{Dash, Slash, Crouch}
    },
    entity_states::*
};

use std::time::Duration;
use crate::anim::AnimationChangeEvent;
use crate::util::Facing;

pub fn player_setup_anim(app: &mut App) {
    app.add_system_set(
        SystemSet::on_update(GameState::Gameplay)
            .with_system(anim_run)
            .with_system(anim_idle)
            .with_system(anim_crouch)
            // .with_system(flip_sprite_on_direction)
    );
}

// ANIMATION CHANGES BASED ON STATE MACHINE

fn anim_run(
    anims: Res<PlayerAssets>,
    mut q: Query<Entity, (With<Player>, Added<Move>)>,
    mut evw: EventWriter<AnimationChangeEvent>
) {
    if q.is_empty() {
        return;
    }


    evw.send(AnimationChangeEvent {
        e: q.single(),
        new_anim: anims.anims["RUN"].clone()
    });
}

fn anim_idle(
    anims: Res<PlayerAssets>,
    mut q: Query<Entity, (With<Player>, Added<Idle>)>,
    mut evw: EventWriter<AnimationChangeEvent>
) {
    if q.is_empty() {
        return;
    }

    evw.send(AnimationChangeEvent {
        e: q.single(),
        new_anim: anims.anims["IDLE"].clone()
    });
}

fn anim_crouch(
    anims: Res<PlayerAssets>,
     mut q: Query<Entity, (With<Player>, Added<Crouch>)>,
    mut evw: EventWriter<AnimationChangeEvent>
) {
    if q.is_empty() {
        return;
    }

    evw.send(AnimationChangeEvent {
        e: q.single(),
        new_anim: anims.anims["CROUCH"].clone()
    });
}