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

#[derive(Component, Debug, Copy, Clone)]
pub struct KnockbackModifier {
    pub mod_fn: fn(Vec2) -> Vec2
}

impl Default for KnockbackModifier {
    fn default() -> Self {
        Self { mod_fn: |a| a }
    }
}

impl KnockbackModifier {
    pub fn new(f: fn(Vec2) -> Vec2) -> Self {
        KnockbackModifier {
            mod_fn: f
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
    pub hp: i32,
    pub max_hp: i32
}

impl Health {
    pub fn new(hp: i32) -> Self {
        Self::new_with_max(hp, hp)
    }

    pub fn new_with_max(hp: i32, max_hp: i32) -> Self {
        Health { hp, max_hp }
    }
}

#[derive(Component, Debug, Copy, Clone, Default)]
pub struct Immunity {
    pub is_immune: bool
}
