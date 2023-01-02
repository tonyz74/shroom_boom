use rand::prelude::*;
use bevy::prelude::*;
use bevy::ecs::query::ReadOnlyWorldQuery;
use bevy_rapier2d::prelude::*;

use crate::{enemies::Enemy, state::GameState, level::consts::SCALE_FACTOR, pathfind::{
    Pathfinder,
    PathfinderStopChaseEvent,
    knockbacks as kb,
    state_machine as s,
}, common::PHYSICS_STEP_DELTA, util};

#[derive(Component)]
pub struct WalkPathfinder {
    pub jump_speed: f32,
    pub needs_jump: bool,
    pub grounded: bool,

    pub can_patrol: bool,
    pub patrol_timer: Timer,
    pub patrol_pause_timer: Timer,
    pub patrol_target_x: f32,
}

impl Default for WalkPathfinder {
    fn default() -> Self {
        Self {
            jump_speed: 0.0,
            needs_jump: false,
            grounded: false,

            can_patrol: true,
            patrol_timer: Timer::from_seconds(12.0, TimerMode::Once),
            patrol_pause_timer: Timer::from_seconds(2.0, TimerMode::Once),
            patrol_target_x: 0.0
        }
    }
}

pub fn register_walk_pathfinders(app: &mut App) {
    app.add_system_set(
        SystemSet::on_update(GameState::Gameplay)
            .with_system(walk_pathfinder_patrol)
            .with_system(walk_pathfinder_fall)
            .with_system(walk_pathfinder_jump)
            .with_system(walk_pathfinder_hurt)
            .with_system(walk_pathfinder_got_hurt)
            .with_system(walk_pathfinder_hit_ground)
            .with_system(walk_pathfinder_lose_notice)
            .with_system(walk_pathfinder_set_grounded)
    );
}

fn walk_pathfinder_fall(
    mut q: Query<(
        &mut Enemy,
        &WalkPathfinder
    )>
) {
    for (mut enemy, walk) in q.iter_mut() {
        if walk.grounded {
            continue;
        }

        enemy.vel.y += PHYSICS_STEP_DELTA * -40.0;

        if enemy.vel.y <= -20.0 {
            enemy.vel.y = -20.0;
        }
    }
}

fn walk_pathfinder_jump(
   mut q: Query<(&mut Enemy, &WalkPathfinder), Added<s::Jump>>
) {
    for (mut enemy, walk) in q.iter_mut() {
        enemy.vel.y = walk.jump_speed;
    }
}

fn walk_pathfinder_hit_ground(
    mut q: Query<&mut Enemy, (With<WalkPathfinder>, Added<s::Move>)>
) {
    for mut enemy in q.iter_mut() {
        enemy.vel.x = 0.0;
        enemy.vel.y = 0.0;
    }
}

pub fn walk_pathfinder_needs_jump(
    pos: Vec2,
    dir: Vec2,
    collider: &Collider,
    pathfinder: &Pathfinder,
    rapier: &Res<RapierContext>
) -> bool {
    let ix = rapier.cast_shape(
        pos.into(),
        Rot::default(),
        dir.into(),
        collider,
        pathfinder.bb.half_extents.x / SCALE_FACTOR,
        QueryFilter {
            flags: QueryFilterFlags::ONLY_FIXED | QueryFilterFlags::EXCLUDE_SENSORS,
            ..default()
        }
    );

    ix.is_some()
}

pub fn walk_pathfinder_jump_if_needed(
    pos: Vec2,
    dir: Vec2,
    collider: &Collider,
    enemy: &mut Enemy,
    pathfinder: &Pathfinder,
    walk: &mut WalkPathfinder,
    rapier: &Res<RapierContext>
) -> bool {
    let needs_jump = walk_pathfinder_needs_jump(pos, dir, collider, pathfinder, rapier);

    if needs_jump {
        enemy.vel.x = 0.0;
        walk.needs_jump = walk.grounded;
    } else {
        walk.needs_jump = false;
    }

    needs_jump
}

fn walk_pathfinder_got_hurt(
    mut pathfinders: Query<&mut Enemy, (With<WalkPathfinder>, Added<s::Hurt>)>
) {
    for mut enemy in pathfinders.iter_mut() {
        if enemy.hit_event.is_none() {
            continue;
        }

        let hit_ev = enemy.hit_event.take().unwrap();
        let kb = kb::randomize_knockback(kb::walk_pathfinder_knockback(hit_ev.kb));

        enemy.vel = kb;
    }
}

fn walk_pathfinder_hurt(
    mut walks: Query<(
        &GlobalTransform,
        &Collider,
        &mut Enemy,
        &mut Pathfinder,
        &mut WalkPathfinder
    ), With<s::Hurt>>,
    rapier: Res<RapierContext>
) {
   for (transform, collider, mut enemy, pathfinder, mut walk) in walks.iter_mut() {
       let self_pos = Vec2::new(
           transform.translation().x,
           transform.translation().y
       );

       walk_pathfinder_jump_if_needed(
           Vec2::new(self_pos.x, self_pos.y),
           Vec2::new(enemy.vel.x, 0.0).normalize(),
           collider,
           &mut enemy,
           &pathfinder,
           &mut walk,
           &rapier
       );
   }
}

pub fn walk_pathfinder_stop_if_colliding_enemy_stopped<T: ReadOnlyWorldQuery>(
    e1: Entity,
    e2: Entity,
    q: &mut Query<(
        Entity,
        &Collider,
        &mut Enemy,
        &mut Pathfinder,
        &mut WalkPathfinder
    ), T>
) {
    if !q.contains(e1) || !q.contains(e2) {
        return;
    }

    let e1_stopped = {
        let v = q.get(e1).unwrap().2.vel;
        v.x == 0.0 && v.y == 0.0
    };

    let e2_stopped = {
        let v = q.get(e2).unwrap().2.vel;
        v.x == 0.0 && v.y == 0.0
    };

    if e1_stopped {
        q.get_mut(e2).unwrap().2.vel.x = 0.0;
    }

    if e2_stopped {
        q.get_mut(e1).unwrap().2.vel.x = 0.0;
    }
}

fn walk_pathfinder_patrol(
    mut pathfinders: Query<(
        Entity,
        &GlobalTransform,
        &Collider,
        &mut Enemy,
        &mut Pathfinder,
        &mut WalkPathfinder
    ), Without<s::Hurt>>,
    rapier: Res<RapierContext>,
    mut ev_stop: EventWriter<PathfinderStopChaseEvent>
) {
    let mut all_should_start_patrolling = false;

    for (ent, tf, collider, mut enemy, pathfinder, mut walk) in pathfinders.iter_mut() {
        let self_pos = tf.translation();

        if pathfinder.target.is_some() || !walk.can_patrol {
            continue;
        }

        if pathfinder.lose_notice_timer.just_finished()
            || walk.patrol_pause_timer.just_finished() {

            all_should_start_patrolling = true;

            // Enter patrolling state

            let range = {
                let dist_left = (self_pos.x - pathfinder.region.tl.x).abs();
                let dist_right = (self_pos.x - pathfinder.region.br.x).abs();

                let furthest = if dist_left < dist_right {
                    pathfinder.region.br.x
                } else {
                    pathfinder.region.tl.x
                };

                if furthest < self_pos.x {
                    furthest..self_pos.x
                } else {
                    self_pos.x..furthest
                }
            };

            let mut rng = thread_rng();
            walk.patrol_target_x = rng.gen_range(range);
            walk.patrol_timer.reset();

            ev_stop.send(PathfinderStopChaseEvent {
                pathfinder: ent
            });

        } else if walk.patrol_pause_timer.finished() && pathfinder.lose_notice_timer.finished() {
            // Do the actual patrolling

            if (self_pos.x - walk.patrol_target_x).abs() <= 2.0
                || walk.patrol_timer.finished() {
                // Wait on site for a short bit
                walk.patrol_pause_timer.reset();
                enemy.vel.x = 0.0;
                continue;
            }

            let dir = Vec2::new(walk.patrol_target_x - self_pos.x, 0.0).normalize();
            enemy.vel.x = dir.x * pathfinder.patrol_speed;

            walk_pathfinder_jump_if_needed(
                Vec2::new(self_pos.x, self_pos.y),
                dir.into(),
                collider,
                &mut enemy,
                &pathfinder,
                &mut walk,
                &rapier
            );
        } else {
            enemy.vel.x = 0.0;
            continue;
        }
    }

    if all_should_start_patrolling {
        for (_, _, _, _, mut pathfinder, _) in pathfinders.iter_mut() {
            pathfinder.target = None;
            util::timer_tick_to_almost_finish(&mut pathfinder.lose_notice_timer);
        }
    }
}

fn walk_pathfinder_lose_notice(
    time: Res<Time>,
    mut pathfinders: Query<(&mut Pathfinder, &mut WalkPathfinder)>
) {
    for (mut pathfinder, mut walk) in pathfinders.iter_mut() {
        if pathfinder.target.is_none() {
            pathfinder.lose_notice_timer.tick(time.delta());

            if pathfinder.lose_notice_timer.finished() {
                walk.patrol_timer.tick(time.delta());
                walk.patrol_pause_timer.tick(time.delta());
            }
        }
    }
}


fn walk_pathfinder_set_grounded(
    mut walk_pathfinders: Query<(
        &mut WalkPathfinder,
        &KinematicCharacterControllerOutput
    )>,
) {
   for (mut walk, out) in walk_pathfinders.iter_mut() {
        walk.grounded = out.grounded;
   }
}