use bevy::prelude::*;
use bevy_ecs_ldtk::{EntityInstance, GridCoords};
use crate::level::consts::{RENDERED_TILE_SIZE, TILE_SIZE};
use crate::pathfind::Region;


pub fn grid_coords_to_region(
    inst: &EntityInstance,
    grid_size: Vec2
) -> Region {
    let reg_dim = IVec2::new(
        (inst.width as f32 / TILE_SIZE) as i32,
        (inst.height as f32 / TILE_SIZE) as i32
    );

    let tl = GridCoords::new(inst.grid.x, inst.grid.y);
    let br = GridCoords::new(inst.grid.x + reg_dim.x, inst.grid.y + reg_dim.y);

    let region = Region {
        tl: grid_coord_to_translation(tl.into(), grid_size.as_ivec2()),
        br: grid_coord_to_translation(br.into(), grid_size.as_ivec2()),
    };

    region
}

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