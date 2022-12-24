use bevy::prelude::*;

#[derive(Component, Debug, Copy, Clone)]
pub struct KnockbackResistance {
    pub multiplier: f32
}

#[derive(Component, Debug, Copy, Clone)]
pub struct Knockback {
    pub kb: Vec2
}

#[derive(Component, Debug, Copy, Clone)]
pub struct Damage {
    pub power: i32
}

#[derive(Component, Debug, Copy, Clone)]
pub struct Health {
    pub hp: i32
}
