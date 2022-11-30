#![allow(unused)]

use bevy::prelude::*;
use crate::{
    common::AnimTimer,
    state::GameState,
    assets::PlayerAssets,
    player::{
        Player,
        state_machine::{Idle, Run, Jump, Fall, Dash, Slash, Teleport}
    }
};

use std::time::Duration;

pub fn player_setup_anim(app: &mut App) {
    app.add_system_set(
        SystemSet::on_update(GameState::Gameplay)
            .with_system(animate_player)
            .with_system(anim_run)
            .with_system(anim_idle)
            .with_system(flip_sprite_on_direction)
    );
}

fn animate_player(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut AnimTimer,
                      &mut TextureAtlasSprite,
                      &Handle<TextureAtlas>),
                     With<Player>>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

// ANIMATION CHANGES BASED ON STATE MACHINE

fn anim_run(anims: Res<PlayerAssets>,
            mut q: Query<(&mut TextureAtlasSprite,
                          &mut Handle<TextureAtlas>,
                          &mut AnimTimer),
                         (With<Player>, Added<Run>)>
) {
    if q.is_empty() {
        return;
    }

    let run = &anims.sprite_sheets["RUN"];
    let (mut sprite, mut atlas, mut timer) = q.single_mut();

    sprite.index = 0;
    *atlas = run.0.clone();

    timer.set_duration(Duration::new(run.1 as u64, (run.1.fract() * 1_000_000_000.) as u32));
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

    let idle = &anims.sprite_sheets["IDLE"];
    let (mut sprite, mut atlas, mut timer) = q.single_mut();

    sprite.index = 0;
    *atlas = idle.0.clone();

    timer.set_duration(Duration::new(idle.1 as u64, (idle.1.fract() * 1_000_000_000.) as u32));
}


// GENERAL ANIMATIONS

fn flip_sprite_on_direction(mut q: Query<(&mut TextureAtlasSprite, &Player)>) {
    if q.is_empty() {
        return;
    }

    let (mut sprite, player) = q.single_mut();

    if player.vel.x < 0.0 {
        sprite.flip_x = true;
    } else if player.vel.x > 0.0 {
        sprite.flip_x = false;
    }
}
