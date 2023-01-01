use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_debug_text_overlay::screen_print;

use crate::{
    enemies::Enemy,
    level::{coord, LevelInfo},
    pathfind::{
        Pathfinder,
        util::GridRegion,
        grid::{PathfindingGrid, PathfindingResult},
        knockbacks as kb,
        state_machine as s
    },
    state::GameState
};

#[derive(Default, Component, Debug)]
pub struct FlyPathfinder {
    pub regain_control_timer: Timer,
    pub path: PathfindingResult
}

pub fn register_fly_pathfinders(app: &mut App) {
    app.add_system_set(
        SystemSet::on_update(GameState::Gameplay)
            .with_system(fly_pathfinder_move)
            .with_system(fly_pathfinder_hurt)
            .with_system(fly_pathfinder_got_hurt)
    );
}

pub fn fly_pathfinder_move(
    lvl_info: Res<LevelInfo>,
    graph: Res<PathfindingGrid>,
    mut fly: Query<(
        &GlobalTransform,
        &mut Enemy,
        &Collider,
        &Pathfinder,
        &mut FlyPathfinder,
    ), Without<s::Hurt>>,
    rapier: Res<RapierContext>
) {
    for (pos, mut enemy, collider, pathfinder, mut fly) in fly.iter_mut() {
        let self_pos = Vec2::new(
            pos.translation().x,
            pos.translation().y
        );

        if let Some(target) = pathfinder.target {
            let slightly_above = target + Vec2::new(0.0, pathfinder.bb.half_extents.y);

            if slightly_above.distance(self_pos) <= 2.0 {
                enemy.vel = Vec2::ZERO;
                continue;
            }

            // First check if the enemy can make a direct flying attempt towards the target
            if rapier.cast_shape(
                self_pos,
                Rot::default(),
                (slightly_above - self_pos).normalize(),
                collider,
                slightly_above.distance(self_pos),
                QueryFilter {
                    flags: QueryFilterFlags::ONLY_FIXED | QueryFilterFlags::EXCLUDE_SENSORS,
                    ..default()
                }
            ).is_none() {
                enemy.vel = (slightly_above - self_pos).normalize() * pathfinder.speed;
                continue;
            }

            let patrol_region_grid = pathfinder.region.to_grid_region(&lvl_info);

            let start = coord::world_to_grid(self_pos, lvl_info.grid_size);
            let end = coord::world_to_grid(target, lvl_info.grid_size);

            // Recalculate!
            if fly.path.start != start || fly.path.end != end {
                let region = if pathfinder.within_region {
                    Some(patrol_region_grid)
                } else {
                    None
                };

                fly.path = graph.find_path(start, end, region, pathfinder.bb.half_extents * 2.0);
            }

            if let Some(result) = &fly.path.path {
                if result.len() <= 1 {
                    continue;
                }

                let dir = (result[1] - start)
                    .as_vec2()
                    .normalize()
                    * Vec2::new(1.0, -1.0);

                enemy.vel = dir * pathfinder.speed;
            } else {
                // Effectively the same as "If the pathfinder just exited"
                if !pathfinder.region.contains(self_pos) && pathfinder.within_region {
                    enemy.vel *= -1.0;
                }
            }
        }
    }
}

pub fn fly_pathfinder_got_hurt(
    mut fly: Query<(&mut Enemy, &mut FlyPathfinder), Added<s::Hurt>>
) {
    for (mut enemy, mut fly) in fly.iter_mut() {
        if enemy.hit_event.is_none() {
            continue;
        }

        let hit_ev = enemy.hit_event.take().unwrap();
        enemy.vel = kb::randomize_knockback(kb::fly_pathfinder_knockback(hit_ev.kb));

        fly.regain_control_timer.reset();
    }
}

pub fn fly_pathfinder_hurt(
    time: Res<Time>,
    mut fly: Query<(&mut Enemy, &mut FlyPathfinder), With<s::Hurt>>,
) {
    for (mut enemy, mut fly) in fly.iter_mut() {
        fly.regain_control_timer.tick(time.delta());

        let opp = {
            let opp_x = -Vec2::new(enemy.vel.x, 0.0).normalize_or_zero().x;
            let opp_y = -Vec2::new(0.0, enemy.vel.y).normalize_or_zero().y;
            Vec2::new(opp_x, opp_y)
        };

        enemy.vel += opp * 0.15;
    }
}
