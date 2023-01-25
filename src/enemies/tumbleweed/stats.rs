use crate::enemies::stats::{CustomEnemyStats, EnemyStats};

pub const TUMBLEWEED_EASY: EnemyStats = EnemyStats {
    jump_speed: 8.0,
    patrol_speed: 1.0,
    speed: 2.0,
    attack_damage: 0,
    collision_damage: 4,
    health: 8,
    custom: CustomEnemyStats::Melee
};

pub const TUMBLEWEED_MEDIUM: EnemyStats = EnemyStats {
    jump_speed: 7.0,
    patrol_speed: 1.3,
    speed: 2.5,
    attack_damage: 0,
    collision_damage: 6,
    health: 12,
    custom: CustomEnemyStats::Melee
};

pub const TUMBLEWEED_HARD: EnemyStats = EnemyStats {
    jump_speed: 6.0,
    patrol_speed: 2.0,
    speed: 3.0,
    attack_damage: 0,
    collision_damage: 8,
    health: 16,
    custom: CustomEnemyStats::Melee
};