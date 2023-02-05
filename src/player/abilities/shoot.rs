use bevy::prelude::*;

use bevy_rapier2d::prelude::RapierContext;
use seldom_state::prelude::*;
use crate::assets::PlayerAssets;
use crate::combat::{AttackStrength, CombatLayerMask, ProjectileAttack, ProjectileAttackBundle};
use crate::entity_states::Shoot;
use crate::player::abilities::autotarget;
use crate::player::abilities::autotarget::{attack_direction_between, AttackDirection, change_facing_for_direction, direction_for_facing, direction_to_vec, Untargetable};
use crate::player::consts::{PLAYER_SHOOT_EXPIRATION_TIME, SHOOT_LEVELS};
use crate::player::Player;
use crate::state::GameState;
use crate::anim::Animator;
use crate::util::{Facing, quat_rot2d_rad};


#[derive(Component, Default, Debug)]
pub struct PlayerProjectileAttack;

#[derive(Component, Debug)]
pub struct ShootAbility {
    pub damage: i32,
    pub proj_speed: f32,
    pub cd: Timer,
    pub startup: Timer,
    pub shoot_target: Option<(Vec2, AttackDirection)>
}

impl Default for ShootAbility {
    fn default() -> Self {
        Self {
            cd: Timer::from_seconds(SHOOT_LEVELS[0].0, TimerMode::Once),
            proj_speed: SHOOT_LEVELS[0].1,
            damage: SHOOT_LEVELS[0].2,

            startup: Timer::from_seconds(0.3, TimerMode::Once),
            shoot_target: None
        }
    }
}

pub fn register_shoot_ability(app: &mut App) {
    app.add_system_set(
        SystemSet::on_update(GameState::Gameplay)
            .with_system(shoot_cooldown_tick)
            .with_system(shoot_ability_trigger)
            .with_system(shoot_ability_update)
    );
}

pub fn shoot_ability_trigger(
    mut q: Query<(Entity, &mut ShootAbility), Added<Shoot>>,
    transforms: Query<&GlobalTransform>,
    combat_layers: Query<&CombatLayerMask>,
    untargetable: Query<&Untargetable>,
    projectiles: Query<&ProjectileAttack>,
    rapier: Res<RapierContext>
) {
    if q.is_empty() {
        return;
    }

    let (entity, mut shoot) = q.single_mut();
    shoot.startup.reset();

    shoot.shoot_target = autotarget::get_closest_target(
        entity,
        CombatLayerMask::PLAYER,
        512.0,
        true,
        &transforms,
        &combat_layers,
        &untargetable,
        &projectiles,
        &rapier
    );

    info!("Player shooting at {:?}", shoot.shoot_target);
}

fn off_for_direction(atk_dir: AttackDirection) -> Vec2 {
    const Y_OFF: f32 = 48.0;
    const X_OFF: f32 = 64.0;
    const DIAG_OFF: f32 = 24.0;

    match atk_dir {
        AttackDirection::Up => { Vec2::new(0.0, Y_OFF) },
        AttackDirection::Down => { Vec2::new(0.0, -Y_OFF) },
        AttackDirection::UpRight => { Vec2::new(DIAG_OFF, DIAG_OFF) },
        AttackDirection::DownRight => { Vec2::new(DIAG_OFF, -DIAG_OFF) },
        AttackDirection::UpLeft => { Vec2::new(-DIAG_OFF, DIAG_OFF) },
        AttackDirection::DownLeft => { Vec2::new(-DIAG_OFF, -DIAG_OFF) },
        AttackDirection::Left => { Vec2::new(-X_OFF, 0.0) },
        AttackDirection::Right => { Vec2::new(X_OFF, 0.0) }
    }
}

fn spawn_player_projectile(
    commands: &mut Commands,
    _player: &Player,
    player_pos: Vec2,
    shoot: &ShootAbility,
    facing: &Facing,
    assets: &PlayerAssets
) {
    let (dir, off) = match shoot.shoot_target {
        Some((enemy_pos, atk_dir)) => {
            let off = off_for_direction(atk_dir);
            (enemy_pos - player_pos, off)
        },
        None => {
            let dir = direction_for_facing(*facing);
            (direction_to_vec(dir), off_for_direction(dir))
        }
    };

    commands.spawn((
        PlayerProjectileAttack,
        ProjectileAttackBundle {
            anim: Animator::default(),

            sprite_sheet: SpriteSheetBundle {

                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::new(32., 32.)),
                    ..default()
                },

                texture_atlas: assets.bullet.tex.clone(),

                transform: Transform::from_xyz(player_pos.x + off.x, player_pos.y + off.y, 10.0)
                    .with_rotation(quat_rot2d_rad(-dir.angle_between(Vec2::X))),

                ..default()
            },

            attack: ProjectileAttack {
                vel: dir.normalize() * shoot.proj_speed,
                speed: shoot.proj_speed,
                expiration: Some(Timer::from_seconds(PLAYER_SHOOT_EXPIRATION_TIME, TimerMode::Once)),
                ..default()
            },

            strength: AttackStrength::new(shoot.damage as i32),
            combat_layer: CombatLayerMask::PLAYER,

            ..ProjectileAttackBundle::from_size(Vec2::new(16., 16.))
        }
    ));

}

fn shoot_ability_update(
    time: Res<Time>,
    assets: Res<PlayerAssets>,
    mut commands: Commands,
    mut q: Query<(
        Entity,
        &mut Player,
        &mut Facing,
        &GlobalTransform,
        &mut ShootAbility
    ), With<Shoot>>
) {
    if q.is_empty() {
        return;
    }

    let (entity, player, mut facing, transform, mut shoot) = q.single_mut();
    let pos = transform.translation();

    shoot.startup.tick(time.delta());

    // match &mut shoot.shoot_target {
    //     Some((enemy_pos, dir)) => {
    //         *dir = attack_direction_between(Vec2::new(pos.x, pos.y), *enemy_pos);
    //     },
    //     None => {}
    // };

    if let Some((_pos, dir)) = shoot.shoot_target {
        change_facing_for_direction(&mut facing, dir);
    }

    if shoot.startup.just_finished() {
        spawn_player_projectile(
            &mut commands,
            &player,
            Vec2::new(pos.x, pos.y),
            &shoot,
            &facing,
            &assets
        );

        shoot.cd.reset();
        commands.entity(entity).insert(Done::Success);

        shoot.shoot_target = None;
    }
}

fn shoot_cooldown_tick(
    time: Res<Time>,
    mut q: Query<&mut ShootAbility>
) {
    if q.is_empty() {
        return;
    }

    let mut shoot = q.single_mut();
    shoot.cd.tick(time.delta());
}