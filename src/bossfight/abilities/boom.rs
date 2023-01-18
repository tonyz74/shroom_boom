use bevy::prelude::*;
use rand::prelude::*;
use seldom_state::prelude::*;
use crate::assets::{ExplosionAssets, IndicatorAssets};
use crate::bossfight::{Boss, BossConfig};
use crate::bossfight::consts::{BOSS_BOOM_EXPLOSION_COUNT, BOSS_BOOM_EXPLOSION_SCALE, BOSS_BOOM_PARTITION_SIZE, BOSS_BOOM_SELECTION_TIME, BOSS_BOOM_WAIT_TIME};
use crate::bossfight::enraged::EnragedAttackMove;
use crate::bossfight::stage::BossStage;
use crate::bossfight::state_machine::{AbilityStartup, Boom};
use crate::combat::{ExplosionAttackBundle, Immunity};
use crate::fx::indicator::Indicator;
use crate::pathfind::Region;
use crate::state::GameState;

#[derive(Component, Debug, Clone)]
pub struct BoomAbility {
    pub sel_timer: Timer,
    pub explosion_points: Vec<Vec2>,
    pub wait_timer: Timer,
}

impl Default for BoomAbility {
    fn default() -> Self {
        Self {
            sel_timer: Timer::from_seconds(BOSS_BOOM_SELECTION_TIME, TimerMode::Repeating),
            explosion_points: vec![],
            wait_timer: Timer::from_seconds(BOSS_BOOM_WAIT_TIME, TimerMode::Once)
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

    let x_range = [
        (x_min / BOSS_BOOM_PARTITION_SIZE) as i32,
        (x_max / BOSS_BOOM_PARTITION_SIZE) as i32];

    let y_range = [
        (y_min / BOSS_BOOM_PARTITION_SIZE) as i32,
        (y_max / BOSS_BOOM_PARTITION_SIZE) as i32
    ];

    let coords = IVec2::new(
        rng.gen_range(x_range[0]..x_range[1]),
        rng.gen_range(y_range[0]..y_range[1])
    );

    coords.as_vec2() * BOSS_BOOM_PARTITION_SIZE
}

fn boom_update(
    time: Res<Time>,
    mut commands: Commands,
    booming: Query<&Boom>,
    mut q: Query<(
        Entity,
        &mut BoomAbility,
        &mut Immunity,
        &Boss,
        &BossStage,
        &BossConfig
    )>,

    assets: Res<ExplosionAssets>,
    mut indicators: EventWriter<Indicator>
) {
    if q.is_empty() {
        return;
    }

    let (entity, mut boom, mut immunity, boss, stage, cfg) = q.single_mut();
    if stage != &BossStage::Enraged || boss.current_move() != EnragedAttackMove::Boom {
        return;
    }

    if boom.explosion_points.len() < BOSS_BOOM_EXPLOSION_COUNT {
        boom.sel_timer.tick(time.delta());

        if boom.sel_timer.just_finished() {
            let mut point;

            loop {
                point = pick_explosion_point(&cfg);

                if !boom.explosion_points.contains(&point) {
                    break;
                }
            }

            let len = boom.explosion_points.len();
            let wait = (BOSS_BOOM_SELECTION_TIME * (BOSS_BOOM_EXPLOSION_COUNT - len) as f32) - 0.1;

            indicators.send(
                Indicator {
                    region: Region {
                        tl: point + Vec2::new(-40.0, 40.0),
                        br: point + Vec2::new(40.0, -40.0),
                    },
                    wait_time: wait,
                    expand_time: 0.4,
                    ..Indicator::EXPLOSION
                },
            );

            boom.explosion_points.push(point);
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

        let transform = &mut explosion.sprite_sheet.transform;
        transform.scale = Vec2::splat(BOSS_BOOM_EXPLOSION_SCALE).extend(1.0);

        commands.spawn(explosion);
    }
}