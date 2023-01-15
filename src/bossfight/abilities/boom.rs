use bevy::prelude::*;
use rand::prelude::*;
use seldom_state::prelude::*;
use crate::assets::ExplosionAssets;
use crate::bossfight::Boss;
use crate::bossfight::state_machine::{AbilityStartup, Boom};
use crate::combat::{ExplosionAttackBundle, Immunity};
use crate::level::coord::grid_coord_to_translation;
use crate::pathfind::grid::PathfindingGrid;
use crate::state::GameState;

const N_EXPLOSIONS: usize = 24;

#[derive(Component, Debug, Clone)]
pub struct BoomAbility {
    pub sel_timer: Timer,
    pub explosion_points: Vec<Vec2>,
    pub wait_timer: Timer,
}

impl Default for BoomAbility {
    fn default() -> Self {
        Self {
            sel_timer: Timer::from_seconds(0.15, TimerMode::Repeating),
            explosion_points: vec![],
            wait_timer: Timer::from_seconds(0.8, TimerMode::Once)
        }
    }
}

pub fn register_boom_ability(app: &mut App) {
    app.add_system_set(
        SystemSet::on_update(GameState::Gameplay)
            .with_system(start_booming)
            .with_system(boom_update)
    );
}

fn start_booming(
    mut commands: Commands,
    mut q: Query<(Entity, &mut BoomAbility), (With<Boss>, Added<AbilityStartup>)>
) {
    if q.is_empty() {
        return;
    }

    let (entity, mut boom) = q.single_mut();

    commands.entity(entity).insert(Immunity);

    boom.wait_timer.reset();
    boom.sel_timer.reset();
    boom.explosion_points.clear();
}



fn pick_explosion_point(
    grid: &PathfindingGrid,
) -> Vec2 {
    let mut rng = thread_rng();
    let mut coords = IVec2::ZERO;

    let max = grid.lvl_info.grid_size.as_ivec2();

    loop {
        coords = IVec2::new(
            rng.gen_range(0..max.x),
            rng.gen_range(0..max.y)
        );

        if !grid.solids.contains(&coords) {
            break;
        }
    }

    grid_coord_to_translation(coords, grid.lvl_info.grid_size.as_ivec2())
}

fn boom_update(
    time: Res<Time>,
    grid: Res<PathfindingGrid>,
    mut commands: Commands,
    mut q: Query<(Entity, &mut BoomAbility), (With<Boss>, With<Boom>)>,
    assets: Res<ExplosionAssets>
) {
    if q.is_empty() {
        return;
    }

    let (entity, mut boom) = q.single_mut();

    if boom.explosion_points.len() < N_EXPLOSIONS {
        boom.sel_timer.tick(time.delta());

        if boom.sel_timer.just_finished() {
            boom.explosion_points.push(pick_explosion_point(&grid));
        }

    } else {
        boom.wait_timer.tick(time.delta());

        if boom.wait_timer.just_finished() {
            // Spawn explosions
            boom_spawn_explosions(&mut commands, &boom.explosion_points, &assets);

            commands.entity(entity).remove::<Immunity>().insert(Done::Success);
        }
    }
}

fn boom_spawn_explosions(
    commands: &mut Commands,
    points: &[Vec2],
    assets: &ExplosionAssets
) {
    for point in points {
        commands.spawn(ExplosionAttackBundle::new(*point, assets));
    }
}