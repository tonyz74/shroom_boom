use bevy::prelude::*;
use seldom_state::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::math::Vec3Swizzles;
use crate::bossfight::{Boss, BossConfig};
use crate::bossfight::consts::{BOSS_CHARGE_SPEED, BOSS_HEAD_HALF_SIZE, BOSS_HALF_SIZE};
use crate::bossfight::enraged::EnragedAttackMove;
use crate::bossfight::state_machine::{AbilityStartup, Charge};
use crate::combat::{ColliderAttack, Immunity};
use crate::enemies::Enemy;
use crate::state::GameState;
use crate::util::{Facing, FacingX, FacingY};

#[derive(Component, Debug, Clone)]
pub struct ChargeAbility {
    pub dir: f32
}

impl Default for ChargeAbility {
    fn default() -> Self {
        Self {
            dir: 0.0
        }
    }
}

pub fn register_boom_ability(app: &mut App) {
    app.add_system_set(
        SystemSet::on_update(GameState::Gameplay)
            .with_system(start_charging)
            .with_system(charge_update)
    );
}

fn start_charging(
    mut p: Query<(&mut ColliderAttack, &mut Transform, &mut Collider)>,
    mut q: Query<(
        &Children,
        &mut Immunity,
        &mut ChargeAbility,
        &mut Boss,
        &mut Facing,
    ), Added<AbilityStartup>>,
) {
    if q.is_empty() {
        return;
    }

    let (children, mut immunity, mut charge, boss, mut facing) = q.single_mut();

    let (new_facing_y, dir) = match boss.current_move() {
        EnragedAttackMove::ChargeLeft => (FacingY::Up, -1.0),
        EnragedAttackMove::ChargeRight => (FacingY::Down, 1.0),
        _ => return
    };

    facing.y = new_facing_y;

    for child in children {
        if let Ok((mut atk, mut transform, mut collider)) = p.get_mut(*child) {
            atk.enabled = true;

            *collider = Collider::cuboid(BOSS_HEAD_HALF_SIZE.x, BOSS_HEAD_HALF_SIZE.y);
            transform.translation.y = -dir * (BOSS_HALF_SIZE.y - BOSS_HEAD_HALF_SIZE.x);
        }
    }

    charge.dir = dir;
    immunity.is_immune = true;
}

fn charge_update(
    mut commands: Commands,
    mut p: Query<(&mut ColliderAttack, &mut Transform, &mut Collider)>,
    mut q: Query<(
        Entity,
        &Children,
        &GlobalTransform,
        &mut Enemy,
        &ChargeAbility,
        &BossConfig,
        &Boss,
        &mut Facing
    ), With<Charge>>
) {
    if q.is_empty() {
        return;
    }

    let (entity, children, transform, mut enemy, charge, config, boss, mut facing) = q.single_mut();
    enemy.vel = Vec2::new(BOSS_CHARGE_SPEED * charge.dir, 0.0);

    let pos = transform.translation().xy();

    let target = match boss.current_move() {
        EnragedAttackMove::ChargeLeft => config.charge_left.x,
        EnragedAttackMove::ChargeRight => config.charge_right.x,
        _ => panic!()
    };

    if (target - pos.x).abs() <= 8.0 {
        commands.entity(entity).insert(Done::Success);

        for child in children {
            if let Ok((mut atk, mut transform, mut collider)) = p.get_mut(*child) {
                atk.enabled = false;
                *collider = Collider::cuboid(BOSS_HALF_SIZE.x, BOSS_HALF_SIZE.y);
                transform.translation = Vec3::ZERO;
            }
        }

        let new_facing = match boss.current_move() {
            EnragedAttackMove::ChargeLeft => Facing {
                x: FacingX::Left,
                y: FacingY::Down
            },
            EnragedAttackMove::ChargeRight => Facing {
                x: FacingX::Right,
                y: FacingY::Up
            },
            _ => panic!()
        };

        *facing = new_facing;
    }
}