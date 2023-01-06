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
}

#[derive(Bundle, Debug, Default, Clone)]
pub struct PathfinderBundle {
    pub pathfinder: Pathfinder,
    pub patrol: Patrol
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
    player: Query<&GlobalTransform, With<Player>>,
    mut pathfinders: Query<(Entity, &GlobalTransform, &mut Pathfinder, &mut Patrol)>,
    mut ev_start: EventWriter<PathfinderStartChaseEvent>,
) {
    if player.is_empty() {
        return;
    }

    let player_tf = player.single();
    let player_pos = Vec2::new(player_tf.translation().x, player_tf.translation().y);

    for (entity, transform, mut pathfinder, mut patrol) in pathfinders.iter_mut() {
        let enemy_pos = Vec2::new(
            transform.translation().x,
            transform.translation().y,
        );

        // If the enemy is within its own region, do the whole patrolling business
        let slightly_larger_region = pathfinder.region.expanded_by(4.0);

        if slightly_larger_region.contains(enemy_pos) {
            pathfinder.within_region = true;

            let player_within_region = pathfinder.region.contains(player_pos);
            patrol.lost_target = !player_within_region;

            if !player_within_region {
                if pathfinder.target.is_some() {
                    patrol.lose_notice_timer.reset();
                }
                patrol.lost_target = true;

                continue;
            } else {
                patrol.lost_target = false;
            }

        } else {
            pathfinder.within_region = false;
        }

        if pathfinder.target.is_none() {
            ev_start.send(PathfinderStartChaseEvent {
                pathfinder: entity
            });

            patrol.lost_target = false;
        }

        pathfinder.target = Some(player_pos);
    }
}