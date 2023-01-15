use bevy::prelude::*;

#[derive(Component, Clone, Debug)]
pub struct BossConfig {
    pub relocate_point: Vec2,

    pub dash_right: Vec2,
    pub dash_left: Vec2,

    pub hover_base: Vec2,
    pub summon_base: Vec2,

    pub x_min: f32,
    pub x_max: f32,

    pub boom_box: Vec2
}