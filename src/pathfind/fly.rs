use rand::prelude::*;
use bevy::prelude::*;
use bevy::time::FixedTimestep;
use bevy_rapier2d::prelude::*;

use crate::{
    enemies::Enemy, level::{coord, LevelInfo},
    pathfind::{
        Pathfinder,
        util::GridRegion,
        grid::{PathfindingGrid, PathfindingResult},
        knockbacks as kb,
    },
    state::GameState,
    entity_states::*,
    util
};

use crate::combat::HurtAbility;
use crate::common::PHYSICS_STEPS_PER_SEC;
use crate::pathfind::Patrol;

#[derive(Component, Debug)]
pub struct FlyPathfinder {
    pub path: PathfindingResult,
    pub path_index: usize,
}

impl Default for FlyPathfinder {
    fn default() -> Self {
        Self {
            path: PathfindingResult::default(),
            path_index: 0,
        }
    }
}

pub fn register_fly_pathfinders(app: &mut App) {
    app
        .add_system_set(
            SystemSet::on_update(GameState::Gameplay)
                .with_system(fly_pathfinder_chase)
                .with_system(fly_pathfinder_patrol)
                .with_system(fly_pathfinder_lose_notice)
                .with_system(fly_pathfinder_got_hurt)
                .with_system(fly_pathfinder_just_died)
        )
        .add_system_set(
            SystemSet::on_update(GameState::Gameplay)
                .with_run_criteria(FixedTimestep::steps_per_second(PHYSICS_STEPS_PER_SEC))
                .with_system(fly_pathfinder_remove_kb)
        );
}

pub fn fly_pathfinder_follow_path(
    grid: &PathfindingGrid,
    lvl_info: &LevelInfo,
    pathfinder: &Pathfinder,
    fly: &mut FlyPathfinder,
    enemy: &mut Enemy,
    self_pos: Vec2,
    target: Vec2,
    speed: f32
) -> bool {
    let patrol_region_grid = pathfinder.region.to_grid_region(&lvl_info);

    let start = coord::world_to_grid(self_pos, lvl_info.grid_size);
    let end = coord::world_to_grid(target, lvl_info.grid_size);

    let pos_in_path = match fly.path.path.as_ref() {
        Some(path) => path.iter().position(|&x| x == start),
        None => None
    };

    let obj_size = pathfinder.bb.half_extents * 2.0;

    let grid_half_y_span = (grid.grid_span_for_size(obj_size).y as f32 / 2.0).ceil() as i32;

    if pos_in_path.is_none() || fly.path.end != end + IVec2::new(0, -grid_half_y_span) {
        let region = if pathfinder.within_region {
            Some(patrol_region_grid)
        } else {
            None
        };

        let new_path = grid.find_path(start, end, region, obj_size);

        if let Some(p) = &new_path.path {
            if let Some(q) = &fly.path.path {
                if p.len() == 2 && q.len() == 2 && p[0] == q[1] && p[1] == q[0] {
                    return true;
                }
            }
        }

        fly.path = new_path;
        fly.path_index = 0;
    }

    if let Some(pos) = pos_in_path {
        fly.path_index = pos;
    }

    if let Some(result) = &fly.path.path {
        // Already there
        if result.len() <= fly.path_index + 1 {
            return true;
        }

        let dir = (result[fly.path_index + 1] - start)
            .as_vec2()
            .normalize()
            * Vec2::new(1.0, -1.0);

        enemy.vel = dir * speed;
    } else {
        // Effectively the same as "If the pathfinder just exited"
        if !pathfinder.region.contains(self_pos) && pathfinder.within_region {
            enemy.vel *= -1.0;
        }
    }

    false
}

pub fn fly_pathfinder_chase(
    lvl_info: Res<LevelInfo>,
    grid: Res<PathfindingGrid>,
    mut fly: Query<(
        &GlobalTransform,
        &mut Enemy,
        &Collider,
        &mut Pathfinder,
        &mut FlyPathfinder,
        &mut Patrol,
    ), (Without<Hurt>, Without<Die>)>,
    rapier: Res<RapierContext>
) {
    for (pos, mut enemy, collider, mut pathfinder, mut fly, patrol) in fly.iter_mut() {
        if !pathfinder.active {
            continue;
        }

        let self_pos = Vec2::new(
            pos.translation().x,
            pos.translation().y
        );

        if pathfinder.target.is_none() {
            continue;
        }

        if let Some(target) = pathfinder.target {
            let x_dir = {
                let diff = target - self_pos;
                Vec2::new(diff.x, 0.0).normalize_or_zero().x
            };

            let adjusted = target
                + Vec2::new(0.0, pathfinder.bb.half_extents.y)
                + Vec2::new(-x_dir * pathfinder.bb.half_extents.x, 0.0);

            if adjusted.distance(self_pos) <= 2.0 || target.distance(self_pos) <= 2.0 {
                if patrol.lost_target {
                    pathfinder.target = None;
                }

                enemy.vel = Vec2::ZERO;
                continue;
            }

            // First check if the enemy can make a direct flying attempt towards the target
            if rapier.cast_shape(
                self_pos,
                Rot::default(),
                (adjusted - self_pos).normalize(),
                collider,
                adjusted.distance(self_pos),
                QueryFilter {
                    flags: QueryFilterFlags::ONLY_FIXED | QueryFilterFlags::EXCLUDE_SENSORS,
                    ..default()
                }
            ).is_none() {
                enemy.vel = (adjusted - self_pos).normalize() * pathfinder.speed;
                continue;
            }

            fly_pathfinder_follow_path(
                &grid,
                &lvl_info,
                &pathfinder,
                &mut fly,
                &mut enemy,
                self_pos,
                target,
                pathfinder.speed
            );
        }
    }
}

pub fn fly_pathfinder_lose_notice(
    time: Res<Time>,
    mut fly: Query<(&mut Pathfinder, &mut Patrol), Without<Die>>
) {
    for (pathfinder, mut patrol) in fly.iter_mut() {
        if pathfinder.target.is_none() {
            patrol.lose_notice_timer.tick(time.delta());

            if patrol.lose_notice_timer.finished() {
                patrol.patrol_timer.tick(time.delta());
                patrol.patrol_pause_timer.tick(time.delta());
            }
        }
    }
}

pub fn fly_pathfinder_pick_patrol_point(
    grid: &PathfindingGrid,
    lvl_info: &LevelInfo,
    grid_region: GridRegion,
    self_grid_pos: IVec2,
    obj_size: Vec2
) -> Vec2 {
    let mut rng = thread_rng();
    let mut tries = 0;
    let mut sel = IVec2::ZERO;

    const MAX_TRIES: usize = 50;

    while tries <= MAX_TRIES {
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

    coord::grid_coord_to_translation(sel, lvl_info.grid_size.as_ivec2())
}

pub fn fly_pathfinder_patrol(
    lvl_info: Res<LevelInfo>,
    grid: Res<PathfindingGrid>,
    mut fly: Query<(
        &GlobalTransform,
        &mut Enemy,
        &mut Pathfinder,
        &mut FlyPathfinder,
        &mut Patrol
    ), (Without<Die>, Without<Hurt>)>
) {
    let mut all_should_start_patrolling = false;

    for (tf, mut enemy, pathfinder, mut fly, mut patrol) in fly.iter_mut() {
        if !pathfinder.active {
            continue;
        }

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

        patrol.patrol(
            |p| {
                all_should_start_patrolling = true;

                p.patrol_timer.reset();
                util::timer_tick_to_finish(&mut p.patrol_pause_timer);

                p.target = fly_pathfinder_pick_patrol_point(
                    &grid,
                    &lvl_info,
                    grid_region,
                    self_grid_pos,
                    obj_size
                );
            },
            |p| {
                let target = p.target;
                let stop = fly_pathfinder_follow_path(
                    &grid,
                    &lvl_info,
                    &pathfinder,
                    &mut fly,
                    &mut enemy,
                    self_pos,
                    target,
                    pathfinder.patrol_speed
                );

                if self_pos.distance(p.target) <= 2.0 || p.patrol_timer.finished() || stop {
                    p.patrol_pause_timer.reset();
                    enemy.vel = Vec2::ZERO;

                    return;
                }
            },

            |_| {}
        );

    }

    if all_should_start_patrolling {
        for (_, _, mut pathfinder, _, mut patrol) in fly.iter_mut() {
            pathfinder.target = None;
            util::timer_tick_to_almost_finish(&mut patrol.lose_notice_timer);
        }
    }
}

pub fn fly_pathfinder_just_died(
    mut fly: Query<
        &mut Enemy,
        (With<FlyPathfinder>, Added<Die>)
    >
) {
    for mut enemy in fly.iter_mut() {
        enemy.vel = Vec2::ZERO;
    }
}

pub fn fly_pathfinder_got_hurt(
    mut fly: Query<(
        &mut Enemy,
        &mut HurtAbility
    ), (With<FlyPathfinder>, Added<Hurt>, Without<Die>)>
) {
    for (mut enemy, mut hurt) in fly.iter_mut() {
        if hurt.hit_event.is_none() {
            continue;
        }

        let hit_ev = hurt.hit_event.take().unwrap();
        enemy.vel = kb::randomize_knockback(kb::fly_pathfinder_knockback(hit_ev.kb));
    }
}

pub fn fly_pathfinder_remove_kb(mut fly: Query<&mut Enemy, (With<Hurt>, Without<Die>)>) {
    for mut enemy in fly.iter_mut() {
        enemy.vel *= 0.94;
    }
}
