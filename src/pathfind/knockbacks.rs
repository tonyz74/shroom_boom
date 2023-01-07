use rand::prelude::*;
use bevy::prelude::*;

pub fn fly_pathfinder_knockback(kb: Vec2) -> Vec2 {
    let mut v = kb * 4.4;
    v.y = Vec2::new(0.0, v.y).normalize_or_zero().y * v.y.abs().clamp(2.0, 6.0);
    v
}

pub fn walk_pathfinder_knockback(kb: Vec2) -> Vec2 {
    Vec2::new(kb.x * 4.0, 4.0)
}

pub fn randomize_knockback(kb: Vec2) -> Vec2 {
    let mut rng = thread_rng();
    kb * Vec2::new(rng.gen_range(0.8..1.2), rng.gen_range(0.8..1.2))
}