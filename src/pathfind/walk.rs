use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    enemies::Enemy,
    state::GameState,
    attack::HurtAbility,
    level::consts::SCALE_FACTOR,
    pathfind::{Pathfinder, state_machine as s},
};
use crate::common::PHYSICS_STEP_DELTA;
use crate::pathfind::PathfinderStopChaseEvent;


#[derive(Copy, Clone, Debug)]
pub enum WalkPathfinderPatrolPoint {
    Left,
    Right
}

impl WalkPathfinderPatrolPoint {
    pub fn advance(&mut self) {
        match self {
            Self::Left => {
                *self = Self::Right;
            },
            Self::Right => {
                *self = Self::Left
            },
        }
    }
}

impl Default for WalkPathfinderPatrolPoint {
    fn default() -> Self {
        Self::Left
    }
}

#[derive(Component, Default)]
pub struct WalkPathfinder {
    pub jump_speed: f32,
    pub needs_jump: bool,
    pub grounded: bool,
    pub next_patrol_point: WalkPathfinderPatrolPoint,
}

pub fn register_walk_pathfinders(app: &mut App) {
    app.add_system_set(
        SystemSet::on_update(GameState::Gameplay)
            .with_system(walk_pathfinder_move)
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

fn jump_if_needed(
    pos: Vec2,
    dir: Vec2,
    collider: &Collider,
    enemy: &mut Enemy,
    pathfinder: &Pathfinder,
    walk: &mut WalkPathfinder,
    rapier: &Res<RapierContext>
) {
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

    if ix.is_some() {
        enemy.vel.x = 0.0;

        if walk.grounded {
            walk.needs_jump = true;
        } else {
            walk.needs_jump = false;
        }
    } else {
        walk.needs_jump = false;
    }
}

fn walk_pathfinder_got_hurt(
    mut pathfinders: Query<(&mut Enemy, &HurtAbility), Added<s::Hurt>>
) {
    for (mut enemy, _hurt) in pathfinders.iter_mut() {
        let hit_event = enemy.hit_event.take().unwrap();
        enemy.vel = hit_event.kb;
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
   for (transform, collider, mut enemy, mut pathfinder, mut walk) in walks.iter_mut() {
       let self_pos = Vec2::new(
           transform.translation().x,
           transform.translation().y
       );

       jump_if_needed(
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

fn walk_pathfinder_move(
    transforms: Query<&GlobalTransform>,
    mut pathfinders: Query<(
        Entity,
        &Collider,
        &mut Enemy,
        &mut Pathfinder,
        &mut WalkPathfinder
    ), Without<s::Hurt>>,
    rapier: Res<RapierContext>,
    mut ev_stop: EventWriter<PathfinderStopChaseEvent>
) {
    for (ent, collider, mut enemy, mut pathfinder, mut walk) in pathfinders.iter_mut() {
        let self_transform = transforms.get(ent).unwrap();

        let self_pos = Vec2::new(
            self_transform.translation().x,
            self_transform.translation().y,
        );

        if let Some(target_pos) = pathfinder.target {

            if (target_pos.x - self_pos.x).abs() <= 2.0 {
                if pathfinder.lost_target {
                    pathfinder.target = None;
                }

                enemy.vel.x = 0.0;
                return;
            }

            let dir = Vec2::new((target_pos - self_pos).x, 0.0).normalize();
            enemy.vel.x = dir.x * pathfinder.speed;

            jump_if_needed(
                Vec2::new(self_pos.x, self_pos.y),
                dir.into(),
                collider,
                &mut enemy,
                &pathfinder,
                &mut walk,
                &rapier
            );

        } else if pathfinder.lose_notice_timer.just_finished() {
            // Enter patrolling state

            // Decide the next patrol point to go to, which should be the
            // point that is furthest from the current position (it goes away
            // from the player)

            let targets = [
                (WalkPathfinderPatrolPoint::Left, pathfinder.region.tl.x),
                (WalkPathfinderPatrolPoint::Right, pathfinder.region.br.x),
            ];

            let mut max_dist = 0.0;

            for (point, target) in targets.iter() {
                if (target - self_pos.x).abs() > max_dist {
                    walk.next_patrol_point = *point;
                    max_dist = (target - self_pos.x).abs();
                }
            }

            ev_stop.send(PathfinderStopChaseEvent {
                pathfinder: ent
            });

        } else if pathfinder.lose_notice_timer.finished() {
            // Do the actual patrolling
            let target_x = match walk.next_patrol_point {
                WalkPathfinderPatrolPoint::Left => {
                    pathfinder.region.tl.x
                },
                WalkPathfinderPatrolPoint::Right => {
                    pathfinder.region.br.x
                }
            };

            if (self_pos.x - target_x).abs() <= 1.0 {
                walk.next_patrol_point.advance();
            }

            let dir = Vec2::new(target_x - self_pos.x, 0.0).normalize();
            enemy.vel.x = dir.x;

            jump_if_needed(
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
        }
    }
}

fn walk_pathfinder_lose_notice(
    time: Res<Time>,
    mut pathfinders: Query<&mut Pathfinder>
) {
    for mut pathfinder in pathfinders.iter_mut() {
        if pathfinder.target.is_none() {
            pathfinder.lose_notice_timer.tick(time.delta());
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