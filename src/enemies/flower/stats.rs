use crate::enemies::stats::{CustomEnemyStats, EnemyStats};

pub const FLOWER_MELLOW: EnemyStats = EnemyStats {
    jump_speed: 7.0,
    patrol_speed: 1.0,
    speed: 1.4,
    attack_damage: 12,
    collision_damage: 1,
    health: 5,
    custom: CustomEnemyStats::Melee
};

pub const FLOWER_EASY: EnemyStats = EnemyStats {
    jump_speed: 8.0,
    patrol_speed: 1.0,
    speed: 2.0,
    attack_damage: 16,
    collision_damage: 2,
    health: 8,
    custom: CustomEnemyStats::Melee
};

pub const FLOWER_MEDIUM: EnemyStats = EnemyStats {
    jump_speed: 7.0,
    patrol_speed: 1.3,
    speed: 2.5,
    attack_damage: 24,
    collision_damage: 3,
    health: 12,
    custom: CustomEnemyStats::Melee
};

pub const FLOWER_HARD: EnemyStats = EnemyStats {
    jump_speed: 6.0,
    patrol_speed: 2.0,
    speed: 3.0,
    attack_damage: 32,
    collision_damage: 4,
    health: 16,
    custom: CustomEnemyStats::Melee
};