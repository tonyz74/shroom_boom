use crate::enemies::stats::{CustomEnemyStats, EnemyStats};

pub const DANDELION_MELLOW: EnemyStats = EnemyStats {
    jump_speed: 0.0,
    patrol_speed: 1.0,
    speed: 1.6,
    attack_damage: 0,
    collision_damage: 2,
    health: 5,
    custom: CustomEnemyStats::Fly
};

pub const DANDELION_EASY: EnemyStats = EnemyStats {
    jump_speed: 0.0,
    patrol_speed: 1.0,
    speed: 2.0,
    attack_damage: 0,
    collision_damage: 2,
    health: 8,
    custom: CustomEnemyStats::Fly
};

pub const DANDELION_MEDIUM: EnemyStats = EnemyStats {
    jump_speed: 0.0,
    patrol_speed: 1.3,
    speed: 2.5,
    attack_damage: 0,
    collision_damage: 4,
    health: 12,
    custom: CustomEnemyStats::Fly
};

pub const DANDELION_HARD: EnemyStats = EnemyStats {
    jump_speed: 0.0,
    patrol_speed: 2.0,
    speed: 3.0,
    attack_damage: 0,
    collision_damage: 6,
    health: 16,
    custom: CustomEnemyStats::Fly
};