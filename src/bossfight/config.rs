use bevy::prelude::*;
use crate::pathfind::Region;

#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct BossConfig {
    pub relocate_point: Vec2,

    pub charge_right: Vec2,
    pub charge_left: Vec2,

    pub hover_base: Vec2,
    pub summon_base: Vec2,

    pub slam_base: Vec2,

    pub x_min: f32,
    pub x_max: f32,

    pub boom_region: Region
}