use rand::prelude::*;
use bevy::prelude::*;
use crate::pathfind::Region;

pub fn pick_point_in_region(reg: Region, partition_size: f32) -> Vec2 {
    let mut rng = thread_rng();

    let x_min = reg.tl.x;
    let x_max = reg.br.x;
    let y_min = reg.br.y;
    let y_max = reg.tl.y;

    let x_range = [
        (x_min / partition_size).ceil() as i32,
        (x_max / partition_size).ceil() as i32
    ];

    let y_range = [
        (y_min / partition_size).ceil() as i32,
        (y_max / partition_size).ceil() as i32
    ];

    let coords = IVec2::new(
        rng.gen_range(x_range[0]..x_range[1]),
        rng.gen_range(y_range[0]..y_range[1])
    );

    coords.as_vec2() * partition_size
}