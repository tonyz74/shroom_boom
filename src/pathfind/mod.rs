pub mod walk;
pub mod state_machine;

use bevy::prelude::*;

use crate::state::GameState;
use crate::player::Player;

pub struct PathfindingPlugin;

#[derive(Component, Debug, Default, Copy, Clone)]
pub struct PatrolRegion {
    pub tl: Vec2,
    pub br: Vec2
}

impl PatrolRegion {
    pub fn contains(&self, p: Vec2) -> bool {
        p.x < self.br.x && p.x > self.tl.x && p.y < self.tl.y && p.y > self.br.y
    }

    pub fn expanded_by(&self, n: f32) -> Self {
        let mut dup = self.clone();
        dup.tl.x -= n;
        dup.tl.y += n;
        dup.br.x += n;
        dup.br.y -= n;

        dup
    }
}

#[derive(Component, Debug, Default, Copy, Clone)]
pub struct BoundingBox {
    pub half_extents: Vec2
}

impl BoundingBox {
    pub fn new(half_x: f32, half_y: f32) -> Self {
        Self {
            half_extents: Vec2::new(half_x, half_y)
        }
    }
}

#[derive(Component, Debug, Default, Clone)]
pub struct Pathfinder {
    pub region: PatrolRegion,
    pub start: Vec2,
    pub speed: f32,
    pub bb: BoundingBox,
    pub target: Option<Vec2>,

    pub lost_target: bool,
    pub lose_notice_timer: Timer,
}

#[derive(Copy, Clone)]
pub struct PathfinderStartChaseEvent {
    pub pathfinder: Entity
}

#[derive(Copy, Clone)]
pub struct PathfinderStopChaseEvent {
    pub pathfinder: Entity
}






impl Plugin for PathfindingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_update(GameState::Gameplay)
                    .with_system(pathfind_track_player)
            )
            .add_event::<PathfinderStartChaseEvent>()
            .add_event::<PathfinderStopChaseEvent>();

        walk::register_walk_pathfinders(app);

        state_machine::register_triggers(app);
    }
}

pub fn pathfind_track_player(
    player: Query<(Entity, &GlobalTransform), With<Player>>,
    mut pathfinders: Query<(Entity, &GlobalTransform, &mut Pathfinder)>,
    mut ev_start: EventWriter<PathfinderStartChaseEvent>,
) {
    for (_player, transform) in player.iter() {
        let player_pos = Vec2::new(
            transform.translation().x,
            transform.translation().y
        );

        for (entity, transform, mut pathfinder) in pathfinders.iter_mut() {
            let enemy_pos = Vec2::new(
                transform.translation().x,
                transform.translation().y,
            );

            // If the enemy is within its own region, do the whole patrolling business
            let slightly_larger_region = pathfinder.region.expanded_by(2.0);

            if slightly_larger_region.contains(enemy_pos)
                && !pathfinder.region.contains(player_pos) {

                if pathfinder.target.is_some() {
                    pathfinder.lose_notice_timer.reset();
                }

                pathfinder.lost_target = true;
                continue;
            }

            if pathfinder.target.is_none() {
                ev_start.send(PathfinderStartChaseEvent {
                    pathfinder: entity
                });

                pathfinder.lost_target = false;
            }

            pathfinder.target = Some(player_pos);
        }
    }
}