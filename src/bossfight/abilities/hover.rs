use bevy::prelude::*;
use seldom_state::prelude::*;
use crate::bossfight::{Boss, BossConfig};
use crate::bossfight::consts::{BOSS_HALF_SIZE, BOSS_HOVER_CMP_THRESHOLD, BOSS_HOVER_SPEED};
use crate::bossfight::enraged::EnragedAttackMove;
use crate::bossfight::state_machine::AbilityStartup;
use crate::combat::Immunity;
use crate::enemies::Enemy;
use crate::player::Player;
use crate::state::GameState;
use crate::util::{Facing, FacingX};

#[derive(Component, Debug, Clone)]
pub struct HoverAbility;

impl Default for HoverAbility {
    fn default() -> Self {
        Self
    }
}

pub fn register_hover_ability(app: &mut App) {
    app.add_system_set(
        SystemSet::on_update(GameState::Gameplay)
            .with_system(start_hover)
            .with_system(hover_update)
    );
}

fn start_hover(
    mut q: Query<(
        &mut Immunity,
        &Boss
    ), Added<AbilityStartup>>
) {
    if q.is_empty() {
        return;
    }

    let (mut immunity, boss) = q.single_mut();
    if boss.current_move() != EnragedAttackMove::Hover {
        return;
    }

    immunity.is_immune = true;
}

fn hover_update(
    mut commands: Commands,
    p: Query<&GlobalTransform, With<Player>>,
    mut q: Query<(
        Entity,
        &GlobalTransform,
        &Boss,
        &mut Immunity,
        &mut Enemy,
        &mut Facing,
        &BossConfig
    )>
) {
    if p.is_empty() || q.is_empty() {
        return;
    }


    let player_pos = p.single().translation();
    let (entity, boss_tf, boss, mut immunity, mut enemy, mut facing, cfg) = q.single_mut();
    let boss_pos = boss_tf.translation();

    if boss.current_move() != EnragedAttackMove::Hover {
        return;
    }

    let mut threshold = BOSS_HOVER_CMP_THRESHOLD;

    let diff = player_pos.x - boss_pos.x;
    enemy.vel.x = Vec2::new(diff, 0.0).normalize_or_zero().x * BOSS_HOVER_SPEED;

    if enemy.vel.x < 0.0 {
        facing.x = FacingX::Left;
    } else if enemy.vel.x > 0.0 {
        facing.x = FacingX::Right;
    }

    if player_pos.x < cfg.charge_left.x
        || player_pos.x > cfg.charge_right.x {
        threshold += BOSS_HALF_SIZE.x;
    }

    if diff.abs() <= threshold {
        enemy.vel = Vec2::ZERO;
        commands.entity(entity).insert(Done::Success);
        immunity.is_immune = false;
    }
}