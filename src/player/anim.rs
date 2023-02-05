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
use crate::player::abilities::autotarget::AttackDirection;
use crate::player::abilities::shoot::{shoot_ability_trigger, ShootAbility};
use crate::util::Facing;

pub fn player_setup_anim(app: &mut App) {
    app.add_system_set(
        SystemSet::on_update(GameState::Gameplay)
            .with_system(anim_run)
            .with_system(anim_idle)
            .with_system(anim_crouch)
            .with_system(anim_slash)
            .with_system(anim_dash)
            .with_system(anim_fall)
            .with_system(anim_jump)
            .with_system(anim_shoot.after(shoot_ability_trigger))
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

fn anim_fall(
    anims: Res<PlayerAssets>,
    mut q: Query<Entity, (With<Player>, Added<Fall>)>,
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

fn anim_jump(
    anims: Res<PlayerAssets>,
    mut q: Query<Entity, (With<Player>, Added<Jump>)>,
    mut evw: EventWriter<AnimationChangeEvent>
) {
    if q.is_empty() {
        return;
    }

    evw.send(AnimationChangeEvent {
        e: q.single(),
        new_anim: anims.anims["JUMP"].clone()
    });
}

fn anim_slash(
    anims: Res<PlayerAssets>,
    mut q: Query<Entity, (With<Player>, Added<Slash>)>,
    mut evw: EventWriter<AnimationChangeEvent>
) {
    if q.is_empty() {
        return;
    }

    evw.send(AnimationChangeEvent {
        e: q.single(),
        new_anim: anims.anims["HIT"].clone()
    })
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

fn anim_dash(
    anims: Res<PlayerAssets>,
    mut q: Query<Entity, (With<Player>, Added<Dash>)>,
    mut evw: EventWriter<AnimationChangeEvent>
) {
    if q.is_empty() {
        return;
    }

    evw.send(AnimationChangeEvent {
        e: q.single(),
        new_anim: anims.anims["DASH_INIT"].clone()
    });
}

fn anim_shoot(
    anims: Res<PlayerAssets>,
    q: Query<(Entity, &ShootAbility), (With<Player>, Added<Shoot>)>,
    mut evw: EventWriter<AnimationChangeEvent>
) {
    if q.is_empty() {
        return;
    }

    let (entity, shoot) = q.single();

    let name = match shoot.shoot_target {
        Some((_, dir)) => match dir {
            AttackDirection::Left | AttackDirection::Right => {
                "SHOOT_STRAIGHT"
            },
            AttackDirection::Up => {
                "SHOOT_UP"
            },
            AttackDirection::Down => {
                "SHOOT_DOWN"
            },
            AttackDirection::UpLeft | AttackDirection::UpRight => {
                "SHOOT_UP"
            },
            AttackDirection::DownLeft | AttackDirection::DownRight => {
                "SHOOT_DOWN"
            }
        },
        None => "SHOOT_STRAIGHT"
    };
    println!("{:?}", name);
    evw.send(AnimationChangeEvent {
        e: entity,
        new_anim: anims.anims[name].clone()
    });
}