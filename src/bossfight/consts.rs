use bevy::prelude::*;

pub const BOSS_FULL_SIZE: Vec2 = Vec2::new(256.0, 512.0);
pub const BOSS_HALF_SIZE: Vec2 = Vec2::new(128.0, 256.0);
pub const BOSS_HEAD_HALF_SIZE: Vec2 = Vec2::new(48.0, BOSS_HALF_SIZE.x);

pub const BOSS_CHARGE_SPEED: f32 = 32.0;

pub const BOSS_HOVER_SPEED: f32 = 10.0;
pub const BOSS_HOVER_CMP_THRESHOLD: f32 = 8.0;

pub const BOSS_LEAP_SPEED: f32 = 24.0;
pub const BOSS_LEAP_ROTATE_SPEED: f32 = 360.0;
pub const BOSS_LEAP_CMP_THRESHOLD: f32 = 12.0;
pub const BOSS_LEAP_ROTATE_LAG: f32 = 0.1;

pub const BOSS_SLAM_SPEED: f32 = 30.0;
pub const BOSS_TAKEOFF_SPEED: f32 = 30.0;

pub const BOSS_RELOCATE_RETRACT_TIME: f32 = 0.8;
pub const BOSS_RELOCATE_EXTEND_TIME: f32 = 0.8;

pub const BOSS_BOOM_EXPLOSION_COUNT: usize = 16;
pub const BOSS_BOOM_SELECTION_TIME: f32 = 0.1;
pub const BOSS_BOOM_WAIT_TIME: f32 = 0.8;
pub const BOSS_BOOM_PARTITION_SIZE: f32 = 128.0;

pub const BOSS_HEALTH: i32 = 400;
pub const BOSS_EASY_HEALTH_THRESHOLD: i32 = 300;
pub const BOSS_MEDIUM_HEALTH_THRESHOLD: i32 = 200;
pub const BOSS_HARD_HEALTH_THRESHOLD: i32 = 100;

pub const BOSS_SUMMON_COUNT_EASY: usize = 6;
pub const BOSS_SUMMON_COUNT_MEDIUM: usize = 9;
pub const BOSS_SUMMON_COUNT_HARD: usize = 12;
