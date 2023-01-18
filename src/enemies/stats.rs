use std::ops::Range;
use bevy::prelude::*;
use rand::prelude::*;

#[derive(Component, Debug, Clone, Copy)]
pub struct EnemyStats {
    pub jump_speed: f32,
    pub patrol_speed: f32,
    pub speed: f32,
    pub attack_damage: i32,
    pub collision_damage: i32,
    pub health: i32,
    pub custom: CustomEnemyStats
}

impl EnemyStats {
    pub fn randomized(mut self, range: Range<f32>) -> Self {
        let mut rng = thread_rng();

        self.speed *= rng.gen_range(range.clone());
        self.patrol_speed *= rng.gen_range(range.clone());
        self.jump_speed *= rng.gen_range(range.clone());

        self

    }
}

#[derive(Component, Debug, Clone, Copy)]
pub enum CustomEnemyStats {
    Fly,
    Ranged(RangedStats),
    Melee
}

#[derive(Component, Debug, Clone, Copy)]
pub struct RangedStats {
    pub proj_speed: f32,
    pub atk_freq: f32,
    pub max_shoot_dist: f32,
}