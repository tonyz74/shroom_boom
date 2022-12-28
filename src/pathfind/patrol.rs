use bevy::prelude::*;

#[derive(Copy, Clone, Debug)]
pub enum WalkPathfinderPatrolPoint {
    Left,
    Right
}

impl Default for WalkPathfinderPatrolPoint {
    fn default() -> Self {
        Self::Left
    }
}

impl WalkPathfinderPatrolPoint {
    pub fn advance(&mut self) {
        match self {
            Self::Left => {
                *self = Self::Right;
            },
            Self::Right => {
                *self = Self::Left
            },
        }
    }
}

#[derive(Component, Debug, Default, Copy, Clone)]
pub struct PatrolRegion {
    pub tl: Vec2,
    pub br: Vec2
}

impl PatrolRegion {
    pub fn contains(&self, p: Vec2) -> bool {
        p.x < self.br.x && p.x > self.tl.x && p.y < self.tl.y && p.y > self.br.y
    }

    pub fn expanded_by(&self, n: f32) -> Self {
        let mut dup = self.clone();
        dup.tl.x -= n;
        dup.tl.y += n;
        dup.br.x += n;
        dup.br.y -= n;

        dup
    }

    pub fn extents(&self) -> Vec2 {
        Vec2::new(
            self.br.x - self.tl.x,
            self.tl.y - self.br.y
        )
    }
}

#[derive(Component, Debug, Default, Copy, Clone)]
pub struct BoundingBox {
    pub half_extents: Vec2
}

impl BoundingBox {
    pub fn new(half_x: f32, half_y: f32) -> Self {
        Self {
            half_extents: Vec2::new(half_x, half_y)
        }
    }
}