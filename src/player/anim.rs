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
            .with_system(swing)
            .with_system(crate::attack::animate_melee)
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
        Added<Run>
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


// TEMP

use leafwing_input_manager::prelude::*;
use crate::input::InputAction;

use crate::attack::MeleeAttack;

fn swing(
    assets: Res<PlayerAssets>,
    mut commands: Commands,
    q: Query<(
        Entity,
        &GlobalTransform,
        &ActionState<InputAction>
    ), With<Player>>
) {
    if q.is_empty() {
        return;
    }

    let (ent, pos, input) = q.single();
    let pos = pos.translation();

    if input.just_pressed(InputAction::Slash) {
        MeleeAttack::spawn(
            commands,
            MeleeAttack { source: ent, damage: 12 },
            Vec2::new(pos.x, pos.y),
            Vec2::new(72.0, 48.0),
            assets.slash_anim.clone()
        );
    }
}


