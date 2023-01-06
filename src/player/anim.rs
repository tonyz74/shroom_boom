#![allow(unused)]

use bevy::prelude::*;
use crate::{
    common::AnimTimer,
    state::GameState,
    assets::PlayerAssets,
    player::{
        Player,
        state_machine::{Dash, Slash, Crouch}
    },
    entity_states::*
};

use std::time::Duration;
use crate::util::Facing;

pub fn player_setup_anim(app: &mut App) {
    app.add_system_set(
        SystemSet::on_update(GameState::Gameplay)
            .with_system(animate_player)
            .with_system(anim_run)
            .with_system(anim_idle)
            .with_system(anim_crouch)
            .with_system(flip_sprite_on_direction)
            .with_system(crate::combat::animate_melee)
    );
}

fn animate_player(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>
    ), With<Player>>
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

fn reset_anim_to(
    anims: Res<PlayerAssets>,
    name: &str,
    (mut sprite, mut atlas, mut timer): (
        Mut<'_, TextureAtlasSprite>,
        Mut<'_, Handle<TextureAtlas>>,
        Mut<'_, AnimTimer>
    )
) {
    let new = &anims.anims[name];

    sprite.index = 0;
    *atlas = new.tex.clone();

    timer.set_duration(Duration::new(
        new.speed as u64,
        (new.speed.fract() * 1_000_000_000.) as u32
    ));
}

// ANIMATION CHANGES BASED ON STATE MACHINE

fn anim_run(
    anims: Res<PlayerAssets>,
    mut q: Query<(
        &mut TextureAtlasSprite,
        &mut Handle<TextureAtlas>,
        &mut AnimTimer
    ), (
        With<Player>,
        Added<Move>
    )>
) {
    if q.is_empty() {
        return;
    }

    reset_anim_to(anims, "RUN", q.single_mut());
}

fn anim_idle(anims: Res<PlayerAssets>,
            mut q: Query<(&mut TextureAtlasSprite,
                          &mut Handle<TextureAtlas>,
                          &mut AnimTimer),
                         (With<Player>, Added<Idle>)>
) {
    if q.is_empty() {
        return;
    }

    reset_anim_to(anims, "IDLE", q.single_mut());
}

fn anim_crouch(
    anims: Res<PlayerAssets>,
     mut q: Query<(
         &mut TextureAtlasSprite,
         &mut Handle<TextureAtlas>,
         &mut AnimTimer
     ), (
         With<Player>,
         Added<Crouch>
     )>
) {
    if q.is_empty() {
        return;
    }

    reset_anim_to(anims, "CROUCH", q.single_mut());
}

// GENERAL ANIMATIONS

fn flip_sprite_on_direction(mut q: Query<
    (&mut TextureAtlasSprite, &Player),
>) {
    if q.is_empty() {
        return;
    }

    let (mut sprite, player) = q.single_mut();

    match player.facing {
        Facing::Left => sprite.flip_x = true,
        Facing::Right => sprite.flip_x = false
    };
}