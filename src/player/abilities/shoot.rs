use bevy::prelude::*;
use seldom_state::prelude::*;
use crate::assets::PlayerAssets;
use crate::combat::{AttackStrength, CombatLayerMask, ProjectileAttack, ProjectileAttackBundle};
use crate::common::AnimTimer;
use crate::entity_states::Shoot;
use crate::state::GameState;


#[derive(Component, Default, Debug)]
pub struct PlayerProjectileAttack;

#[derive(Component, Debug)]
pub struct ShootAbility {
    pub damage: u32,
    pub startup: Timer,
    pub cd: Timer,
    pub shoot_target_pos: Vec2
}

impl Default for ShootAbility {
    fn default() -> Self {
        Self {
            damage: 10,
            startup: Timer::from_seconds(0.1, TimerMode::Once),
            cd: Timer::from_seconds(0.5, TimerMode::Once),
            shoot_target_pos: Vec2::ZERO
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

fn start_shoot(mut q: Query<&mut ShootAbility, Added<Shoot>>) {
    if q.is_empty() {
        return;
    }

    let mut shoot = q.single_mut();
    shoot.startup.reset();
}

fn shoot_ability_update(
    time: Res<Time>,
    assets: Res<PlayerAssets>,
    mut commands: Commands,
    mut q: Query<(Entity, &GlobalTransform, &mut ShootAbility), With<Shoot>>
) {
    if q.is_empty() {
        return;
    }

    let (entity, transform, mut shoot) = q.single_mut();

    let pos = transform.translation();
    shoot.startup.tick(time.delta());

    if shoot.startup.just_finished() {
        commands.spawn((
            PlayerProjectileAttack,
            ProjectileAttackBundle {
                anim_timer: AnimTimer::from_seconds(0.4),

                sprite_sheet: SpriteSheetBundle {

                    sprite: TextureAtlasSprite {
                        custom_size: Some(Vec2::new(16., 16.)),
                        ..default()
                    },

                    texture_atlas: assets.anims["IDLE"].tex.clone(),

                    transform: Transform::from_xyz(pos.x, pos.y, 0.0),

                    ..default()
                },

                attack: ProjectileAttack {
                    vel: Vec2::new(12.0, 0.0),
                    speed: 12.0,
                    expiration: Some(Timer::from_seconds(0.5, TimerMode::Once)),
                    ..default()
                },

                strength: AttackStrength::new(shoot.damage as i32),
                combat_layer: CombatLayerMask::PLAYER,

                ..ProjectileAttackBundle::from_size(Vec2::new(16., 16.))
            }
        ));

        shoot.cd.reset();

        commands.entity(entity).insert(Done::Success);
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