use rand::prelude::*;
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

#[derive(Component, Debug)]
pub struct FlyPathfinder {
    pub regain_control_timer: Timer,
    pub path: PathfindingResult,

    pub patrol_timer: Timer,
    pub patrol_pause_timer: Timer,
    pub patrol_target: Vec2,
}

impl Default for FlyPathfinder {
    fn default() -> Self {
        Self {
            regain_control_timer: Timer::from_seconds(0.5, TimerMode::Once),
            path: PathfindingResult::default(),

            patrol_timer: Timer::from_seconds(12.0, TimerMode::Once),
            patrol_pause_timer: Timer::from_seconds(4.0, TimerMode::Once),
            patrol_target: Vec2::ZERO
        }
    }
}

pub fn register_fly_pathfinders(app: &mut App) {
    app.add_system_set(
        SystemSet::on_update(GameState::Gameplay)
            .with_system(fly_pathfinder_chase)
            .with_system(fly_pathfinder_patrol)
            .with_system(fly_pathfinder_lose_notice)
            .with_system(fly_pathfinder_hurt)
            .with_system(fly_pathfinder_got_hurt)
    );
}

pub fn fly_pathfinder_chase(
    lvl_info: Res<LevelInfo>,
    graph: Res<PathfindingGrid>,
    mut fly: Query<(
        &GlobalTransform,
        &mut Enemy,
        &Collider,
        &mut Pathfinder,
        &mut FlyPathfinder,
    ), Without<s::Hurt>>,
    rapier: Res<RapierContext>
) {
    for (pos, mut enemy, collider, mut pathfinder, mut fly) in fly.iter_mut() {
        let self_pos = Vec2::new(
            pos.translation().x,
            pos.translation().y
        );

        if pathfinder.target.is_none() {
            continue;
        }

        if let Some(target) = pathfinder.target {
            let slightly_above = target + Vec2::new(0.0, pathfinder.bb.half_extents.y);

            if slightly_above.distance(self_pos) <= 2.0 || target.distance(self_pos) <= 2.0 {
                if pathfinder.lost_target {
                    println!("i lost my target");
                    pathfinder.target = None;
                }

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

pub fn fly_pathfinder_lose_notice(
    time: Res<Time>,
    mut fly: Query<(&mut Pathfinder, &mut FlyPathfinder)>
) {
    for (mut pathfinder, mut fly) in fly.iter_mut() {
        if pathfinder.target.is_none() {
            pathfinder.lose_notice_timer.tick(time.delta());

            if pathfinder.lose_notice_timer.finished() {
                fly.patrol_timer.tick(time.delta());
                fly.patrol_pause_timer.tick(time.delta());
            }
        }
    }
}

pub fn fly_pathfinder_patrol(
    lvl_info: Res<LevelInfo>,
    grid: Res<PathfindingGrid>,
    mut fly: Query<(&GlobalTransform, &mut Enemy, &mut Pathfinder, &mut FlyPathfinder)>
) {
    let mut all_should_start_patrolling = false;

    for (tf, mut enemy, pathfinder, mut fly) in fly.iter_mut() {
        let self_pos = Vec2::new(
            tf.translation().x,
            tf.translation().y
        );

        if pathfinder.target.is_some() {
            continue;
        }

        let obj_size = pathfinder.bb.half_extents * 2.0;
        let grid_region = pathfinder.region.to_grid_region(&lvl_info);
        let self_grid_pos = coord::world_to_grid(self_pos, lvl_info.grid_size);

        if pathfinder.lose_notice_timer.just_finished()
            || fly.patrol_pause_timer.just_finished() {

            all_should_start_patrolling = true;

            fly.patrol_timer.reset();

            let dur = fly.patrol_pause_timer.duration() + std::time::Duration::from_secs(1);
            fly.patrol_pause_timer.set_elapsed(dur);

            let mut rng = thread_rng();
            let mut tries = 0;
            let mut sel = IVec2::ZERO;

            while tries <= 50 {
                tries += 1;

                sel = IVec2::new(
                    rng.gen_range(grid_region.tl.x..grid_region.br.x),
                    rng.gen_range(grid_region.tl.y..grid_region.br.y)
                );

                if !grid_region.contains(sel)
                    || grid.solids.contains(&sel)
                    || grid.find_path(self_grid_pos, sel, Some(grid_region), obj_size).path.is_none() {
                    continue;
                } else {
                    break;
                }
            }

            fly.patrol_target = coord::grid_coord_to_translation(sel, lvl_info.grid_size.as_ivec2());

        } else if fly.patrol_pause_timer.finished() && pathfinder.lose_notice_timer.finished() {
            let end = coord::world_to_grid(fly.patrol_target, lvl_info.grid_size);

            if self_pos.distance(fly.patrol_target) <= 2.0
                || fly.patrol_timer.finished()
                || (fly.path.start == self_grid_pos
                    && fly.path.end == end
                    && fly.path.path.is_some()
                    && fly.path.path.as_ref().unwrap().len() == 2
            ) {
                fly.patrol_pause_timer.reset();
                enemy.vel = Vec2::ZERO;
                continue;
            }


            // Calculate a path to get to the patrol target
            if fly.path.start != self_grid_pos || fly.path.end != end {
                fly.path = grid.find_path(self_grid_pos, end, Some(grid_region), obj_size);
            }

            if let Some(result) = &fly.path.path {
                if result.len() <= 1 {
                    continue;
                }

                let dir = (result[1] - self_grid_pos)
                    .as_vec2()
                    .normalize()
                    * Vec2::new(1.0, -1.0);

                enemy.vel = dir * pathfinder.patrol_speed;
            }
        }
    }

    if all_should_start_patrolling {
        for (_, _, mut pathfinder, _) in fly.iter_mut() {
            pathfinder.target = None;
            let dur = pathfinder.lose_notice_timer.duration();
            pathfinder.lose_notice_timer.tick(dur - std::time::Duration::from_nanos(1));
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
