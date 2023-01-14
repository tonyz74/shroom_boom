use bevy::prelude::*;
use seldom_state::prelude::*;
use bevy::time::FixedTimestep;
use bevy_rapier2d::prelude::*;
use crate::entity_states::*;
use crate::assets::CoinAssets;
use crate::coin::pickup::CoinCollector;
use crate::coin::state_machine::{coin_state_machine, Follow, register_coin_state_machine};
use crate::common::{AnimTimer, PHYSICS_STEP_DELTA, PHYSICS_STEPS_PER_SEC};
use crate::state::GameState;


pub fn register_coin(app: &mut App) {
    register_coin_state_machine(app);

    app
        .add_system_set(
            SystemSet::on_update(GameState::Gameplay)
                .with_system(coin_move)
                .with_system(coin_slide)
                .with_system(coin_set_grounded)
                .with_system(set_coin_target)
                .with_system(coin_track_target)
                .with_system(coin_disable_collisions)
                .with_system(coin_wall_slide)
                .with_system(coin_added_die)
                .with_system(coin_dead_update)
        )
        .add_system_set(
            SystemSet::on_update(GameState::Gameplay)
                .with_run_criteria(FixedTimestep::steps_per_second(PHYSICS_STEPS_PER_SEC))
                .with_system(coin_fall)
                .with_system(coin_shrink)
        )
    ;
}

fn coin_fall(mut coins: Query<&mut CoinMovement, With<Idle>>) {
    for mut mover in coins.iter_mut() {
        if mover.grounded {
            continue;
        }

        mover.vel.y += -20.0 * PHYSICS_STEP_DELTA;

        if mover.vel.y < -20.0 {
            mover.vel.y = -20.0;
        }
    }
}

fn coin_added_die(mut coins: Query<&mut CoinMovement, Added<Die>>) {
    for mut coin in coins.iter_mut() {
        coin.vel = Vec2::ZERO;
    }
}

fn coin_shrink(mut coins: Query<&mut Transform, (With<Coin>, With<Die>)>) {
    for mut transform in coins.iter_mut() {
        transform.scale -= (1.0 / 0.1) * PHYSICS_STEP_DELTA;
    }
}

fn coin_wall_slide(
    mut coins: Query<(&GlobalTransform, &Collider, &mut CoinMovement), With<Idle>>,
    rapier: Res<RapierContext>
) {
    for (tf, collider, mut coin) in coins.iter_mut() {
        let pos = Vec2::new(
            tf.translation().x,
            tf.translation().y
        );

        let dirs = [Vec2::new(-1.0, 0.0), Vec2::new(1.0, 0.0)];

        for dir in dirs {
            if rapier.cast_shape(
                pos,
                Rot::default(),
                dir,
                collider,
                4.0,
                QueryFilter {
                    flags: QueryFilterFlags::ONLY_FIXED | QueryFilterFlags::EXCLUDE_SENSORS,
                    ..default()
                }
            ).is_some() {
                coin.vel.x = 0.0;
                continue;
            }
        }
    }
}

fn coin_move(mut coins: Query<(&mut KinematicCharacterController, &CoinMovement)>) {
    for (mut cc, mov) in coins.iter_mut() {
        cc.translation = Some(mov.vel);
    }
}

fn set_coin_target(
    idling: Query<&Idle>,
    mut coins: Query<(Entity, &GlobalTransform, &mut CoinMovement)>,
    mut collectors: Query<&GlobalTransform, With<CoinCollector>>,
) {
    for (ent, transform, mut coin) in coins.iter_mut() {
        let coin_pos = Vec2::new(
            transform.translation().x,
            transform.translation().y
        );

        let mut closest = None;
        let mut closest_dist = f32::MAX;

        for collector in collectors.iter_mut() {
            let target_pos = Vec2::new(
                collector.translation().x,
                collector.translation().y
            );

            let distance = target_pos.distance(coin_pos);

            if idling.contains(ent) && distance > 256.0 {
                continue;
            }

            if distance < closest_dist {
                closest = Some(target_pos);
                closest_dist = distance;
            }
        }

        if let Some(target) = closest {
            coin.target = Some(target);
        } else {
            coin.target = None;
        }
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
    }
}

pub fn coin_track_target(
    mut coins: Query<(&GlobalTransform, &mut CoinMovement), With<Follow>>
) {
    for (tf, mut coin) in coins.iter_mut() {
        let pos = Vec2::new(
            tf.translation().x,
            tf.translation().y
        );

        if let Some(target) = coin.target {
            coin.vel = (target - pos).normalize() * 8.0;
        }
    }
}

pub fn coin_dead_update(
    time: Res<Time>,
    mut coins: Query<(&mut CoinMovement, &mut Die)>
) {
    for (mut coin, mut die) in coins.iter_mut() {
        coin.pickup_timer.tick(time.delta());

        if coin.pickup_timer.just_finished() {
            die.should_despawn = true;
        }
    }
}

pub fn coin_disable_collisions(
    mut coins: Query<
        &mut KinematicCharacterController,
        (Added<Follow>, With<Coin>)
    >
) {
    for mut cc in coins.iter_mut() {
        cc.filter_flags = QueryFilterFlags::EXCLUDE_SENSORS | QueryFilterFlags::EXCLUDE_FIXED;
    }
}

pub fn coin_slide(
    mut coins: Query<&mut CoinMovement, With<Idle>>
) {
    for mut coin in coins.iter_mut() {
        if coin.grounded {
            coin.vel.x *= 0.8;
        }
    }
}




#[derive(Component, Copy, Clone, Debug, Default)]
pub struct Coin {
    pub value: i32,
    pub collected: bool
}

#[derive(Component, Clone, Debug)]
pub struct CoinMovement {
    pub vel: Vec2,
    pub grounded: bool,
    pub picked_up: bool,
    pub target: Option<Vec2>,
    pub pickup_timer: Timer
}

impl Default for CoinMovement {
    fn default() -> Self {
        Self {
            vel: Default::default(),
            grounded: false,
            picked_up: false,
            target: None,
            pickup_timer: Timer::from_seconds(0.1, TimerMode::Once)
        }
    }
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

    pub state_machine: StateMachine,

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

            state_machine: coin_state_machine()
        }
    }
}