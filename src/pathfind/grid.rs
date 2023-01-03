use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use std::collections::HashSet;

use crate::{
    level::{LevelInfo, solid::SolidTileSpawnMarker},
    state::GameState,
    pathfind::util::GridRegion
};

use pathfinding::prelude::bfs;
use crate::level::consts::RENDERED_TILE_SIZE;

#[derive(Default, Debug)]
pub struct PathfindingResult {
    pub start: IVec2,
    pub end: IVec2,
    pub path: Option<Vec<IVec2>>
}

#[derive(Default, Resource)]
pub struct PathfindingGrid {
    pub solids: HashSet<IVec2>,
    pub lvl_info: LevelInfo
}

impl PathfindingGrid {
    pub fn fill(
        mut graph: ResMut<PathfindingGrid>,
        solids: Query<&GridCoords, Added<SolidTileSpawnMarker>>,
        lvl_info: Res<LevelInfo>
    ) {
        for coord in solids.iter() {
            graph.solids.insert(IVec2::new(coord.x, lvl_info.grid_size.y as i32 - coord.y - 1));
        }

        graph.lvl_info = *lvl_info;
    }

    pub fn grid_span_for_size(&self, obj_size: Vec2) -> IVec2 {
        (obj_size / RENDERED_TILE_SIZE)
            .ceil()
            .as_ivec2()
            .clamp(IVec2::new(0, 0), IVec2::splat(i32::MAX))
    }

    pub fn neighbors_for_grid_non_centered(
        pos: IVec2,
        span: IVec2
    ) -> Vec<IVec2> {
        if span == IVec2::ZERO {
            return vec![pos];
        }

        let tl = pos - (span / 2);

        let mut arr = vec![IVec2::ZERO; (span.x * span.y) as usize];

        for row in 0..span.x as usize {
            for col in 0..span.y as usize {
                arr[row * span.x as usize + col] = tl + IVec2::new(col as i32, row as i32);
            }
        }

        arr
    }

    pub fn find_path(
        &self,
        start: IVec2,
        mut end: IVec2,
        region: Option<GridRegion>,
        obj_size: Vec2
    ) -> PathfindingResult {
        let span = self.grid_span_for_size(obj_size);
        end += IVec2::new(0, -1) * (span.y as f32 / 2.0).ceil() as i32;

        let path = bfs(
            &start,
            |tile| {
                let &IVec2 { x, y } = tile;

                let successors = vec![
                    // Top, Down, Left, Right + Diagonals
                    (x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y),
                    (x + 1, y + 1), (x + 1, y - 1), (x - 1, y + 1), (x - 1, y - 1)
                ];

                successors
                    .into_iter()
                    .map(|i| IVec2::new(i.0, i.1))
                    .filter(|i| {
                        if let Some(reg) = region {
                            reg.contains(*i)
                        } else {
                            GridRegion {
                                tl: IVec2::new(0, 0),
                                br: IVec2::new(
                                    self.lvl_info.grid_size.x as i32,
                                    self.lvl_info.grid_size.y as i32
                                )
                            }.contains(*i)
                        }
                    })
                    .filter(|i| {
                        if *i == start || *i == end {
                            return true;
                        }

                        for j in Self::neighbors_for_grid_non_centered(*i, span + 2) {
                            if self.solids.contains(&j) {
                                return false;
                            }
                        }

                        true
                    })
                    .collect::<Vec<_>>()
            },
            |tile| *tile == end
        );

        PathfindingResult {
            start,
            end,
            path
        }
    }
}

pub fn register_pathfinding_grid(app: &mut App) {
    app
        .init_resource::<PathfindingGrid>()
        .add_system_set(
            SystemSet::on_update(GameState::LevelTransition)
                .with_system(PathfindingGrid::fill)
        );
}