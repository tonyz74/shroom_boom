use bevy::prelude::*;
use crate::level::consts::RENDERED_TILE_SIZE;

pub fn top_left_to_center(
    top_left: Vec2,
    extents: Vec2
) -> Vec2 {
    top_left + extents / 2.0
}

pub fn grid_coord_to_translation(
    grid: IVec2,
    world_grid_size: IVec2,
) -> Vec2 {
    Vec2::new(
        grid.x as f32,
        (world_grid_size.y - grid.y) as f32
    ) * RENDERED_TILE_SIZE
}

pub fn world_to_grid(
    world: Vec2,
    lvl_grid_size: Vec2,
) -> IVec2 {
    let grid_x = (world.x / RENDERED_TILE_SIZE) as i32;
    let grid_y = (world.y / RENDERED_TILE_SIZE) as i32;

    IVec2::new(grid_x, lvl_grid_size.y as i32 - grid_y - 1)
}