use bevy::prelude::*;
use rand::prelude::*;
use crate::bossfight::{Boss, BossConfig};
use crate::bossfight::consts::{BOSS_BOOM_PARTITION_SIZE, BOSS_SUMMON_COUNT_EASY, BOSS_SUMMON_COUNT_HARD, BOSS_SUMMON_COUNT_MEDIUM};
use crate::bossfight::stage::BossStage;
use crate::bossfight::state_machine::Summon;
use crate::bossfight::util::pick_point_in_region;
use crate::combat::{ColliderAttack, Immunity};
use crate::enemies::spawner::{EnemyDifficulty, EnemyLocation, EnemySpawnEvent, EnemyType};
use crate::fx::indicator::Indicator;
use crate::level::consts::RENDERED_TILE_SIZE;
use crate::level::LevelInfo;
use crate::pathfind::Region;
use crate::state::GameState;



#[derive(Clone, Component)]
pub struct SummonAbility {
    pub target_count: usize,
    pub total_region: Region,
    pub enemies: Vec<EnemySpawnEvent>,
    pub difficulty: EnemyDifficulty,
    pub summon_lag: Timer,
    pub wait: Timer,
    pub extra_delay: Timer
}

impl Default for SummonAbility {
    fn default() -> Self {
        Self {
            enemies: vec![],
            target_count: 0,
            total_region: Region::default(),
            difficulty: EnemyDifficulty::Easy,
            summon_lag: Timer::from_seconds(0.1, TimerMode::Repeating),
            wait: Timer::from_seconds(0.8, TimerMode::Once),
            extra_delay: Timer::from_seconds(0.1, TimerMode::Once)
        }
    }
}


#[derive(Component, Debug, Clone, Copy)]
pub struct SummonedEnemy;

#[derive(Component, Debug, Clone, Copy)]
pub struct FinishedSummoning;


pub fn register_boss_summon(app: &mut App) {
    app.add_system_set(
        SystemSet::on_update(GameState::Gameplay)
            .with_system(enter_summon)
            .with_system(summon_update)
            .with_system(summon_enemies)
    );
}

fn enter_summon(
    mut collider_attacks: Query<&mut ColliderAttack>,
    mut q: Query<(
        &mut Immunity,
        &Children,
        &BossStage,
        &mut SummonAbility,
    ), (With<Boss>, Added<Summon>)>,

    lvl_info: Res<LevelInfo>,
) {
    if q.is_empty() {
        return;
    }

    let (mut immunity, children, stage, mut summon) = q.single_mut();

    summon.enemies.clear();
    summon.summon_lag.reset();
    summon.wait.reset();
    summon.extra_delay.reset();

    summon.total_region = Region {
        tl: Vec2::new(0.0, lvl_info.grid_size.y * RENDERED_TILE_SIZE),
        br: Vec2::new(lvl_info.grid_size.x * RENDERED_TILE_SIZE, 0.0),
    };

    let (count, difficulty) = match stage {
        BossStage::SummonEasy => (BOSS_SUMMON_COUNT_EASY, EnemyDifficulty::Easy),
        BossStage::SummonMedium => (BOSS_SUMMON_COUNT_MEDIUM, EnemyDifficulty::Medium),
        BossStage::SummonHard => (BOSS_SUMMON_COUNT_HARD, EnemyDifficulty::Hard),
        _ => panic!()
    };

    summon.target_count = count;
    summon.difficulty = difficulty;

    immunity.is_immune = true;

    for child in children {
        if let Ok(mut atk) = collider_attacks.get_mut(*child) {
            atk.enabled = false;
        }
    }
}

fn summon_update(
    time: Res<Time>,
    mut q: Query<(
        &BossConfig,
        &mut SummonAbility
    ), (With<Boss>, With<Summon>)>,

    mut indicators: EventWriter<Indicator>
) {
    if q.is_empty() {
        return;
    }

    let (cfg, mut summon) = q.single_mut();
    summon.summon_lag.tick(time.delta());

    if summon.enemies.len() < summon.target_count && summon.summon_lag.just_finished() {
        let mut p;
        loop {
            p = pick_point_in_region(cfg.summon_region, BOSS_BOOM_PARTITION_SIZE);
            if summon.enemies.iter().find(|i| i.location.pos == p).is_none() {
                break;
            }
        }

        let types = [EnemyType::Flower, EnemyType::Dandelion, EnemyType::Pumpkin];
        let rand_type = types[thread_rng().gen_range(0..types.len())];

        let len = summon.enemies.len();
        let wait = (0.1 * (summon.target_count - len) as f32) - 0.1;

        println!("picked {:?}", p);

        indicators.send(Indicator {
            region: Region {
                tl: p + Vec2::new(-36.0, 36.0),
                br: p + Vec2::new(36.0, -36.0),
            },
            wait_time: wait,
            expand_time: 0.4,
            ..Indicator::SPAWNER
        });

        let region = summon.total_region;
        let difficulty = summon.difficulty;

        summon.enemies.push(EnemySpawnEvent {
            ty: rand_type,
            coins: 0,
            difficulty,
            location: EnemyLocation {
                pos: p,
                patrol_region: region
            },
            rand_range: 0.9..1.1,
            extra_components: Some(|cmd, x| {
                println!("inserting summoned enemy");
                cmd.entity(x).insert(SummonedEnemy);
            })
        });
    }
}

fn summon_enemies(
    time: Res<Time>,
    mut commands: Commands,
    mut q: Query<(Entity, &mut SummonAbility), With<Summon>>,
    mut spawn_events: EventWriter<EnemySpawnEvent>
) {
    if q.is_empty() {
        return;
    }

    let (e, mut summon) = q.single_mut();

    if summon.enemies.len() < summon.target_count {
        return;
    }

    summon.wait.tick(time.delta());
    if summon.wait.just_finished() {
        for i in &summon.enemies {
            spawn_events.send(i.clone());
        }
    }

    if summon.wait.finished() {
        summon.extra_delay.tick(time.delta());
        if summon.extra_delay.just_finished() {
            commands.entity(e).insert(FinishedSummoning);
        }
    }

}