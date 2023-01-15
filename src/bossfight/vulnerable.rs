use bevy::prelude::*;
use crate::bossfight::{Boss, BossStage};
use crate::bossfight::state_machine::Vulnerable;
use crate::bossfight::summon::{FinishedSummoning, SummonedEnemy};
use crate::combat::{Health, Immunity};
use crate::state::GameState;

pub fn register_boss_vulnerable(app: &mut App) {
    app.add_system_set(
        SystemSet::on_update(GameState::Gameplay)
            .with_system(boss_enter_vulnerable)
            .with_system(boss_vulnerable_update)
            .with_system(boss_tick_vulnerable_timer)
    );
}


fn boss_enter_vulnerable(
    mut commands: Commands,
    summoned: Query<&SummonedEnemy>,
    mut bosses: Query<(Entity, &mut BossStage, &mut Boss, &mut Immunity), With<FinishedSummoning>>,
) {
    for (ent, mut stage, mut boss, mut immunity) in bosses.iter_mut() {
        if !summoned.is_empty() || !stage.is_summon_stage() {
            continue;
        }

        boss.vulnerability_timer.reset();

        commands
            .entity(ent)
            .remove::<FinishedSummoning>();

        immunity.is_immune = false;

        stage.advance();
    }
}


fn boss_advance_stage(
    health: &Health,
    stage: &mut BossStage
) {
    if health.hp <= 0 {
        return;
    }

    *stage = BossStage::from_health(health.hp);
}


fn boss_vulnerable_update(mut q: Query<(&Health, &mut BossStage), With<Vulnerable>>) {
    for (health, mut stage) in q.iter_mut() {

        if health.hp < stage.health_threshold() {
            boss_advance_stage(&health, &mut stage);
            continue;
        }

    }
}

fn boss_tick_vulnerable_timer(
    time: Res<Time>,
    mut q: Query<(&mut Boss, &Health, &mut BossStage)>
) {
    for (mut boss, health, mut stage) in q.iter_mut() {
        boss.vulnerability_timer.tick(time.delta());

        if boss.vulnerability_timer.just_finished() && stage.is_vulnerable_stage() {
            boss_advance_stage(&health, &mut stage);
            continue;
        }
    }
}