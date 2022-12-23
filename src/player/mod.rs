use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    state::GameState,
    input::InputAction,
    assets::PlayerAssets,
    common::AnimTimer
};

pub mod consts;
pub mod anim;
pub mod logic;
pub mod triggers;
pub mod state_machine;
pub mod abilities;

use abilities::dash::DashAbility;
use crate::level::consts::SOLIDS_INTERACTION_GROUP;
use crate::player::abilities::slash::SlashAbility;
use crate::player::abilities::jump::JumpAbility;

#[derive(Component)]
pub struct Player {
    pub vel: Vec2,
    pub health: u8,
    pub grounded: bool,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::LevelTransition).with_system(setup_player));

        anim::player_setup_anim(app);
        logic::player_setup_logic(app);
        triggers::player_setup_triggers(app);
    }
}

#[derive(Component)]
pub struct FollowMarker;

fn setup_player(
    mut commands: Commands,
    assets: Res<PlayerAssets>,
    exists: Query<&Player>
) {
    if !exists.is_empty() {
        return;
    }

    use consts::{PLAYER_COLLIDER_CAPSULE, PLAYER_SIZE_PX};
    let anim = &assets.anims["IDLE"];

    commands.spawn((
        SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                custom_size: Some(PLAYER_SIZE_PX),
                ..default()
            },
            texture_atlas: anim.tex.clone(),
            transform: Transform::from_xyz(20.0, 800.0, 5.0),
            ..default()
        },

        AnimTimer::from_seconds(anim.speed),

        Collider::capsule(PLAYER_COLLIDER_CAPSULE.segment.a.into(),
                          PLAYER_COLLIDER_CAPSULE.segment.b.into(),
                          PLAYER_COLLIDER_CAPSULE.radius),

        RigidBody::KinematicPositionBased,

        KinematicCharacterController {
            slide: true,
            snap_to_ground: Some(CharacterLength::Relative(0.2)),
            offset: CharacterLength::Relative(0.02),
            filter_flags: QueryFilterFlags::EXCLUDE_SENSORS,
            filter_groups: Some(SOLIDS_INTERACTION_GROUP),
            ..default()
        },

        Player {
            health: 10,
            grounded: false,
            vel: Vec2::splat(0.0),
        },

        Sensor,

        DashAbility::default(),
        SlashAbility::default(),
        JumpAbility::default(),

        InputAction::input_manager_bundle(),

        state_machine::player_state_machine(),
    ));
}
