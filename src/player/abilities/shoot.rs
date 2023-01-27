use bevy::prelude::*;

use bevy_rapier2d::prelude::RapierContext;
use seldom_state::prelude::*;
use crate::assets::PlayerAssets;
use crate::combat::{AttackStrength, CombatLayerMask, ProjectileAttack, ProjectileAttackBundle};
use crate::entity_states::Shoot;
use crate::player::abilities::autotarget;
use crate::player::abilities::autotarget::{AttackDirection, change_facing_for_direction, direction_for_facing, direction_to_vec};
use crate::player::consts::{PLAYER_SHOOT_EXPIRATION_TIME, SHOOT_LEVELS};
use crate::player::Player;
use crate::state::GameState;
use crate::anim::Animator;
use crate::util::Facing;


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

            startup: Timer::from_seconds(0.1, TimerMode::Once),
            shoot_target: None
        }
    }
}

pub fn register_shoot_ability(app: &mut App) {
    app.add_system_set(
        SystemSet::on_update(GameState::Gameplay)
            .with_system(shoot_cooldown_tick)
            .with_system(start_shoot)
            .with_system(shoot_ability_update)
    );
}

fn start_shoot(
    mut q: Query<(Entity, &mut ShootAbility), Added<Shoot>>,
    transforms: Query<&GlobalTransform>,
    combat_layers: Query<&CombatLayerMask>,
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
        &transforms,
        &combat_layers,
        &rapier
    );

    info!("Player shooting at {:?}", shoot.shoot_target);
}

fn spawn_player_projectile(
    commands: &mut Commands,
    _player: &Player,
    player_pos: Vec2,
    shoot: &ShootAbility,
    facing: &Facing,
    assets: &PlayerAssets
) {
    let dir = match shoot.shoot_target {
        Some((enemy_pos, _)) => {
            enemy_pos - player_pos
        },
        None => direction_to_vec(direction_for_facing(*facing))
    };

    commands.spawn((
        PlayerProjectileAttack,
        ProjectileAttackBundle {
            anim: Animator::default(),

            sprite_sheet: SpriteSheetBundle {

                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::new(16., 16.)),
                    ..default()
                },

                texture_atlas: assets.anims["IDLE"].tex.clone(),

                transform: Transform::from_xyz(player_pos.x, player_pos.y, 0.0),

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

    if shoot.startup.just_finished() {
        if let Some((_pos, dir)) = shoot.shoot_target {
            change_facing_for_direction(&mut facing, dir);
        }

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