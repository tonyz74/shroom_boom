use bevy::prelude::*;

pub fn melee_knockback(dir: Vec2) -> Vec2 {
    dir
}

pub fn projectile_knockback(mut dir: Vec2, vel: Vec2) -> Vec2 {
    if (dir.x < 0.0 && vel.x > 0.0) || (dir.x > 0.0 && vel.x < 0.0) {
        dir.x *= -1.0;
    }

    if (dir.y < 0.0 && vel.y > 0.0) || (dir.y > 0.0 && vel.y < 0.0) {
        dir.y *= -1.0;
    }

    dir
}

pub fn explosion_knockback(dir: Vec2, radius: f32) -> Vec2 {
    let y_dir = Vec2::new(0.0, dir.y).normalize_or_zero().y;
    let percent_from_center = 1.0 - dir.length() / radius;

    dir.normalize_or_zero()
        * percent_from_center
        * Vec2::new(3.0, 4.0)
        + Vec2::new(0.0, 4.0 * y_dir)
}

pub fn spore_cloud_knockback() -> Vec2 {
    Vec2::new(0.0, 0.2)
}