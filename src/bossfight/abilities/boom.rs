use bevy::prelude::*;
use rand::prelude::*;
use seldom_state::prelude::*;
use crate::assets::ExplosionAssets;
use crate::bossfight::{Boss, BossConfig};
use crate::bossfight::enraged::EnragedAttackMove;
use crate::bossfight::stage::BossStage;
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
            sel_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
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
    mut q: Query<(
        &mut Immunity,
        &mut BoomAbility,
        &Boss
    ), Added<AbilityStartup>>
) {
    if q.is_empty() {
        return;
    }

    let (mut immunity, mut boom, boss) = q.single_mut();
    if boss.current_move() != EnragedAttackMove::Boom {
        return;
    }

    immunity.is_immune = false;

    boom.wait_timer.reset();
    boom.sel_timer.reset();
    boom.explosion_points.clear();
}



fn pick_explosion_point(cfg: &BossConfig) -> Vec2 {
    let mut rng = thread_rng();

    let x_min = cfg.boom_region.tl.x;
    let x_max = cfg.boom_region.br.x;
    let y_min = cfg.boom_region.br.y;
    let y_max = cfg.boom_region.tl.y;

    let x_range = [(x_min / 128.0) as i32, (x_max / 128.0) as i32];
    let y_range = [(y_min / 128.0) as i32, (y_max / 128.0) as i32];

    let coords = IVec2::new(
        rng.gen_range(x_range[0]..x_range[1]),
        rng.gen_range(y_range[0]..y_range[1])
    );

    coords.as_vec2() * 128.0
}

fn boom_update(
    time: Res<Time>,
    mut commands: Commands,
    booming: Query<&Boom>,
    mut q: Query<(Entity, &mut BoomAbility, &mut Immunity, &Boss, &BossStage, &BossConfig)>,
    assets: Res<ExplosionAssets>
) {
    if q.is_empty() {
        return;
    }

    let (entity, mut boom, mut immunity, boss, stage, cfg) = q.single_mut();
    if stage != &BossStage::Enraged || boss.current_move() != EnragedAttackMove::Boom {
        return;
    }

    if boom.explosion_points.len() < N_EXPLOSIONS {
        boom.sel_timer.tick(time.delta());

        if boom.sel_timer.just_finished() {
            boom.explosion_points.push(pick_explosion_point(&cfg));
        }

    } else {
        boom.wait_timer.tick(time.delta());

        if boom.wait_timer.finished() && booming.contains(entity) {
            boom_spawn_explosions(&mut commands, &boom.explosion_points, &assets);
            immunity.is_immune = true;
            commands.entity(entity).insert(Done::Success);
        }
    }
}

fn boom_spawn_explosions(
    commands: &mut Commands,
    points: &[Vec2],
    assets: &ExplosionAssets
) {
    for point in points {
        let mut explosion = ExplosionAttackBundle::new(*point, assets);
        explosion.sprite_sheet.transform.scale = Vec2::splat(1.25).extend(1.0);

        commands.spawn(explosion);
    }
}