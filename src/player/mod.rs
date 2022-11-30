use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    state::GameState,
    input::InputAction,
    assets::PlayerAssets,
    common::AnimTimer
};

mod consts;

mod anim;
mod logic;
mod triggers;
mod state_machine;

#[derive(Component)]
pub struct Player {
    pub vel: Vec2,
    pub health: u8,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Gameplay)
                .with_system(setup_player)
        );

        anim::player_setup_anim(app);
        logic::player_setup_logic(app);
        triggers::player_setup_triggers(app);
    }
}

fn setup_player(mut commands: Commands, assets: Res<PlayerAssets>) {
    use consts::{PLAYER_COLLIDER_CAPSULE, PLAYER_SIZE_PX};
    let anim = &assets.sprite_sheets["IDLE"];

    commands.spawn((
        SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                custom_size: Some(PLAYER_SIZE_PX),
                ..default()
            },
            texture_atlas: anim.0.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },

        AnimTimer::from_seconds(anim.1),

        Collider::capsule(PLAYER_COLLIDER_CAPSULE.segment.a.into(),
                          PLAYER_COLLIDER_CAPSULE.segment.b.into(),
                          PLAYER_COLLIDER_CAPSULE.radius),

        RigidBody::KinematicPositionBased,

        KinematicCharacterController {
            slide: true,
            snap_to_ground: Some(CharacterLength::Relative(0.2)),
            offset: CharacterLength::Relative(0.02),
            apply_impulse_to_dynamic_bodies: true,
            ..default()
        },

        Player { health: 10, vel: Vec2::splat(0.0) },

        InputAction::input_manager_bundle(),

        state_machine::player_state_machine(),
    ));
}

