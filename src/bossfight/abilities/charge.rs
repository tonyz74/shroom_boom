use bevy::prelude::*;
use seldom_state::prelude::*;
use bevy::math::Vec3Swizzles;
use crate::bossfight::{Boss, BossConfig};
use crate::bossfight::enraged::EnragedAttackMove;
use crate::bossfight::state_machine::{AbilityStartup, Charge};
use crate::combat::{ColliderAttack, Immunity};
use crate::enemies::Enemy;
use crate::state::GameState;
use crate::util::Facing;

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
    mut p: Query<&mut ColliderAttack>,
    mut q: Query<(
        &Children,
        &mut Immunity,
        &mut ChargeAbility,
        &mut Boss
    ), Added<AbilityStartup>>
) {
    if q.is_empty() {
        return;
    }

    let (children, mut immunity, mut charge, mut boss) = q.single_mut();


    let (facing, dir) = match boss.current_move() {
        EnragedAttackMove::ChargeLeft => (Facing::Left, -1.0),
        EnragedAttackMove::ChargeRight => (Facing::Right, 1.0),
        _ => return
    };

    boss.facing = facing;
    println!("setting facing to {:?}", boss.facing);

    for child in children {
        if let Ok(mut atk) = p.get_mut(*child) {
            atk.enabled = false;
        }
    }

    charge.dir = dir;
    immunity.is_immune = true;
}

fn charge_update(
    mut commands: Commands,
    mut q: Query<(
        Entity,
        &GlobalTransform,
        &mut Enemy,
        &ChargeAbility,
        &BossConfig,
        &Boss
    ), With<Charge>>
) {
    if q.is_empty() {
        return;
    }

    let (entity, transform, mut enemy, charge, config, boss) = q.single_mut();
    enemy.vel = Vec2::new(30.0 * charge.dir, 0.0);

    let pos = transform.translation().xy();

    let target = match boss.facing {
        Facing::Left => config.charge_left.x,
        Facing::Right => config.charge_right.x
    };

    if (target - pos.x).abs() <= 4.0 {
        commands.entity(entity).insert(Done::Success);
    }
}