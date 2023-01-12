use bevy::prelude::*;
use bevy::time::FixedTimestep;
use bevy_debug_text_overlay::screen_print;
use bevy_rapier2d::prelude::*;
use crate::assets::CoinAssets;
use crate::common::{AnimTimer, PHYSICS_STEP_DELTA, PHYSICS_STEPS_PER_SEC};
use crate::state::GameState;


pub fn register_coin(app: &mut App) {
    app
        .add_system_set(
            SystemSet::on_update(GameState::Gameplay)
                .with_system(coin_move)
                .with_system(coin_set_grounded)
        )
        .add_system_set(
            SystemSet::on_update(GameState::Gameplay)
                .with_run_criteria(FixedTimestep::steps_per_second(PHYSICS_STEPS_PER_SEC))
                .with_system(coin_fall)
        )
    ;
}

pub fn coin_fall(mut coins: Query<&mut CoinMovement>) {
    for mut mover in coins.iter_mut() {
        mover.vel.y += -20.0 * PHYSICS_STEP_DELTA;

        if mover.vel.y < -20.0 {
            mover.vel.y = -20.0;
        }
    }
}

pub fn coin_move(mut coins: Query<(&mut KinematicCharacterController, &CoinMovement)>) {
    for (mut cc, mov) in coins.iter_mut() {
        cc.translation = Some(mov.vel);
        screen_print!("{:?}", mov.vel);
    }
}


pub fn coin_set_grounded(
    mut coins: Query<(
        &KinematicCharacterControllerOutput,
        &mut CoinMovement
    )>
) {
    for (out, mut mov) in coins.iter_mut() {
        mov.grounded = out.grounded;

        if mov.grounded {
            mov.vel.x *= 0.8;
        }
    }
}




#[derive(Component, Copy, Clone, Debug, Default)]
pub struct Coin {
    pub value: i32
}

#[derive(Component, Copy, Clone, Debug, Default)]
pub struct CoinMovement {
    pub vel: Vec2,
    pub grounded: bool
}

#[derive(Bundle)]
pub struct CoinBundle {
    pub coin: Coin,
    pub coin_movement: CoinMovement,
    pub anim_timer: AnimTimer,

    pub sensor: Sensor,
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub controller: KinematicCharacterController,

    #[bundle]
    pub sprite_sheet: SpriteSheetBundle
}

impl CoinBundle {
    pub fn new(pos: Vec2, assets: &CoinAssets) -> Self {
        let anim = &assets.anims["SPIN"];

        Self {
            coin: Coin::default(),
            coin_movement: CoinMovement::default(),

            collider: Collider::ball(16.0),

            rigid_body: RigidBody::KinematicPositionBased,

            controller: KinematicCharacterController {
                filter_flags: QueryFilterFlags::EXCLUDE_SENSORS,
                ..default()
            },

            sensor: Sensor,

            anim_timer: AnimTimer::from_seconds(anim.speed),

            sprite_sheet: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::new(32.0, 32.0)),
                    ..default()
                },

                texture_atlas: anim.tex.clone(),

                transform: Transform::from_xyz(pos.x, pos.y, 10.0),
                ..default()
            },
        }
    }
}