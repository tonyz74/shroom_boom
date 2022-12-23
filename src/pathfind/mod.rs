pub mod crawl;

use bevy::prelude::*;
use bevy::math::Vec3Swizzles;
use bevy_ecs_ldtk::prelude::*;

use std::collections::HashSet;
use crate::level::consts::{SCALE_FACTOR, TILE_SIZE};
use crate::level::coord;
use crate::state::GameState;
use crate::player::Player;

/// PATHFINDING SUBSYSTEM:
/// Create a grid that holds all solid tiles, and create a path to the player from any
/// given one of them. This can be used by enemies to see if they should follow the
/// player or not.
///
/// For example, an enemy that can only handle vertical Y drops of 1 can
/// go back into its idle state once the code finds that the path to the player
/// requires a drop.
///
pub struct PathfindingPlugin;

#[derive(Component, Debug, Default, Copy, Clone)]
pub struct PatrolRegion {
    pub tl: GridCoords,
    pub br: GridCoords
}

#[derive(Component, Debug, Default, Copy, Clone)]
pub struct Pathfinder {
    pub region: PatrolRegion,
    pub start: GridCoords,
    pub target: Option<Entity>,
}

#[derive(Resource, Default)]
pub struct PathfindingGrid {
    pub cell_size: IVec2,
    pub grid_size: IVec2,
}

impl Plugin for PathfindingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PathfindingGrid>();

        app.add_system_set(
            SystemSet::on_update(GameState::LevelTransition)
                .with_system(build_pathfinding_grid)
        );

        app.add_system_set(
            SystemSet::on_update(GameState::Gameplay)
                .with_system(pathfind_track_player)
        );

        crawl::register_crawl_pathfinders(app);
    }
}

fn build_pathfinding_grid(
    sel: Res<LevelSelection>,
    level: Query<&Handle<LdtkAsset>>,
    assets: Res<Assets<LdtkAsset>>,

    mut grid: ResMut<PathfindingGrid>,
    // items: Query<&GridCoords, Added<GridCoords>>
) {
    if level.is_empty() || assets.is_empty() {
        return;
    }

    let lvl = assets.get(level.single()).unwrap().get_level(&sel).unwrap();

    grid.grid_size = IVec2::new(
        (lvl.px_wid as f32 / TILE_SIZE) as i32,
        (lvl.px_hei as f32 / TILE_SIZE) as i32
    );

    grid.cell_size = IVec2::splat(TILE_SIZE as i32);

    // for item in items.iter() {
    //     grid.colliders.insert(*item);
    // }
}

pub fn pathfind_track_player(
    player: Query<Entity, With<Player>>,
    mut pathfinders: Query<&mut Pathfinder>
) {
    for player in player.iter() {
        for mut pathfinder in pathfinders.iter_mut() {
            pathfinder.target = Some(player);
        }
    }
}

// pub fn pathfind_track_player(
//     grid: Res<PathfindingGrid>,
//     level: Query<&GlobalTransform, With<Handle<LdtkAsset>>>,
//     player: Query<&GlobalTransform, With<Player>>,
//     mut pathfinders: Query<&mut Pathfinder>
// ) {
//     if level.is_empty() || player.is_empty() {
//         return;
//     }
//
//     let lvl_pos = level.single().translation();
//     let player_pos = player.single().translation();
//
//     let grid_pos = coord::world_to_grid(
//         Vec2::new(player_pos.x, player_pos.y),
//         Vec2::new(lvl_pos.x, lvl_pos.y),
//         grid.grid_size
//     );
//
//     for mut pathfinder in pathfinders.iter_mut() {
//         let region = pathfinder.region;
//
//         if grid_pos.x >= region.tl.x && grid_pos.x <= region.br.x
//             && grid_pos.y >= region.tl.y && grid_pos.y <= region.br.y {
//             pathfinder.target = Some(grid_pos.into());
//         }
//     }
// }

// pub fn pathfind_update_self(
//     grid: Res<PathfindingGrid>,
//     level: Query<&GlobalTransform, With<Handle<LdtkAsset>>>,
//     mut pathfinders: Query<(&GlobalTransform, &mut Pathfinder)>,
// ) {
//     if level.is_empty() {
//         return;
//     }
//
//     let lvl_pos = level.single().translation();
//
//     for (pos, mut pathfinder) in pathfinders.iter_mut() {
//         let grid_pos = coord::world_to_grid(
//             pos.translation().xy(),
//             lvl_pos.xy(),
//             grid.grid_size
//         );
//
//         pathfinder.current = grid_pos.into();
//     }
// }