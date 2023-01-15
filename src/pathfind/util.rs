use bevy::prelude::*;
use crate::level::{coord, LevelInfo};

#[derive(Default, Copy, Clone, Debug)]
pub struct GridRegion {
    pub tl: IVec2,
    pub br: IVec2
}

impl GridRegion {
    pub fn contains(&self, p: IVec2) -> bool {
        let ok = p.x <= self.br.x && p.x >= self.tl.x && p.y >= self.tl.y && p.y <= self.br.y;
        ok
    }
}

#[derive(Component, Debug, Default, Copy, Clone, Reflect)]
pub struct Region {
    pub tl: Vec2,
    pub br: Vec2
}

impl Region {
    pub fn contains(&self, p: Vec2) -> bool {
        p.x <= self.br.x && p.x >= self.tl.x && p.y <= self.tl.y && p.y >= self.br.y
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

    pub fn to_grid_region(&self, lvl_info: &LevelInfo) -> GridRegion {
        GridRegion {
            tl: coord::world_to_grid(self.tl, lvl_info.grid_size) + IVec2::Y,
            br: coord::world_to_grid(self.br, lvl_info.grid_size) + IVec2::NEG_X,
        }
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