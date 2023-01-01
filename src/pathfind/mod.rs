pub mod fly;
pub mod walk;
pub mod melee;
pub mod ranged;

pub mod patrol;
pub mod state_machine;
pub mod grid;
pub mod knockbacks;
pub mod util;

pub use fly::*;
pub use walk::*;
pub use melee::*;
pub use ranged::*;
pub use patrol::*;
pub use util::*;

use bevy::prelude::*;
use crate::pathfind::grid::register_pathfinding_grid;

use crate::state::GameState;
use crate::player::Player;

pub struct PathfindingPlugin;

#[derive(Component, Debug, Default, Clone)]
pub struct Pathfinder {
    pub region: Region,
    pub within_region: bool,

    pub speed: f32,
    pub patrol_speed: f32,
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

        register_pathfinding_grid(app);

        register_walk_pathfinders(app);
        register_melee_pathfinders(app);
        register_ranged_pathfinders(app);
        register_fly_pathfinders(app);

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
            let slightly_larger_region = pathfinder.region.expanded_by(4.0);

            if slightly_larger_region.contains(enemy_pos) {
                pathfinder.within_region = true;

                if !pathfinder.region.contains(player_pos) {
                    if pathfinder.target.is_some() {
                        pathfinder.lose_notice_timer.reset();
                    }

                    pathfinder.lost_target = true;
                    continue;
                }
            } else {
                pathfinder.within_region = false;
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