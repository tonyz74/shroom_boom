use bevy::prelude::*;
use bitflags::bitflags;

bitflags! {
    #[derive(Component, Default)]
    pub struct CombatLayerMask: u8 {
        const PLAYER    = 0b00000001;
        const ENEMY     = 0b00000010;
    }
}

#[allow(dead_code)]
impl CombatLayerMask {
    pub fn is_ally_with(self, other: Self) -> bool {
        self.intersects(other)
    }

    const HOSTILE_TO_ALL: Self = Self::empty();
    const FRIENDLY_TO_ALL: Self = Self::all();
}

#[derive(Component, Debug, Copy, Clone, Default)]
pub struct KnockbackResistance {
    pub resistance: f32
}

impl KnockbackResistance {
    pub fn new(res: f32) -> Self {
        KnockbackResistance {
            resistance: res
        }
    }
}

#[derive(Component, Debug, Copy, Clone, Default)]
pub struct KnockbackStrength {
    pub multiplier: f32
}

impl KnockbackStrength {
    pub fn new(mul: f32) -> Self {
        KnockbackStrength {
            multiplier: mul
        }
    }
}

#[derive(Component, Debug, Copy, Clone, Default)]
pub struct AttackStrength {
    pub power: i32,
}

impl AttackStrength {
    pub fn new(n: i32) -> Self {
        Self { power: n }
    }
}

#[derive(Component, Debug, Copy, Clone, Default)]
pub struct Health {
    pub hp: i32
}

impl Health {
    pub fn new(hp: i32) -> Self {
        Health { hp }
    }
}
