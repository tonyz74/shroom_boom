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
use crate::player::abilities::slash::SlashAbility;
use crate::player::abilities::jump::JumpAbility;

#[derive(Component)]
pub struct Player {
    pub vel: Vec2,
    pub health: u8,
    pub grounded: bool,
    pub attack_cooldown: Timer
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

        app.add_system(follow_player);

        anim::player_setup_anim(app);
        logic::player_setup_logic(app);
        triggers::player_setup_triggers(app);
    }
}

#[derive(Component)]
pub struct FollowMarker;

fn setup_player(mut commands: Commands, assets: Res<PlayerAssets>) {
    use consts::{PLAYER_COLLIDER_CAPSULE, PLAYER_SIZE_PX};
    let anim = &assets.anims["IDLE"];

    commands.spawn((
        SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                custom_size: Some(PLAYER_SIZE_PX),
                ..default()
            },
            texture_atlas: anim.tex.clone(),
            transform: Transform::from_xyz(0.0, 1000.0, 5.0),
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
            apply_impulse_to_dynamic_bodies: true,
            filter_flags: QueryFilterFlags::EXCLUDE_SENSORS,
            ..default()
        },

        Player {
            health: 10,
            grounded: false,
            vel: Vec2::splat(0.0),
            attack_cooldown: Timer::from_seconds(
                consts::PLAYER_ATTACK_COOLDOWN,
                TimerMode::Once
            ),
        },

        DashAbility::default(),
        SlashAbility::default(),
        JumpAbility::default(),

        InputAction::input_manager_bundle(),

        state_machine::player_state_machine(),
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(8.0, 8.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 20.0),
            ..default()
        },

        // FollowMarker
    ));
}

fn follow_player(
    q: Query<&GlobalTransform, With<Player>>,
    mut followers: Query<&mut Transform, With<FollowMarker>>
) {
    if q.is_empty() {
        return;
    }

    let pos = q.single();
    let pos = pos.translation();

    for mut follower in followers.iter_mut() {
        follower.translation.x = pos.x + 16.0;
        follower.translation.y = pos.y;
    }
}
