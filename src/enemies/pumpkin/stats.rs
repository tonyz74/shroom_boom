use crate::enemies::stats::{CustomEnemyStats, EnemyStats, RangedStats};

pub const PUMPKIN_EASY: EnemyStats = EnemyStats {
    jump_speed: 8.0,
    patrol_speed: 1.0,
    speed: 2.0,
    attack_damage: 8,
    collision_damage: 1,
    health: 8,
    custom: CustomEnemyStats::Ranged(RangedStats {
        proj_speed: 7.0,
        atk_pause: 2.0,
        atk_cd: 2.0,
        max_shoot_dist: 240.0,
    })
};

pub const PUMPKIN_MEDIUM: EnemyStats = EnemyStats {
    jump_speed: 7.0,
    patrol_speed: 1.3,
    speed: 2.5,
    attack_damage: 12,
    collision_damage: 1,
    health: 10,
    custom: CustomEnemyStats::Ranged(RangedStats {
        proj_speed: 8.0,
        atk_pause: 1.0,
        atk_cd: 1.0,
        max_shoot_dist: 280.0,
    })
};

pub const PUMPKIN_HARD: EnemyStats = EnemyStats {
    jump_speed: 6.0,
    patrol_speed: 2.0,
    speed: 3.0,
    attack_damage: 12,
    collision_damage: 2,
    health: 16,
    custom: CustomEnemyStats::Ranged(RangedStats {
        proj_speed: 9.0,
        atk_pause: 0.5,
        atk_cd: 0.8,
        max_shoot_dist: 320.0,
    })
};