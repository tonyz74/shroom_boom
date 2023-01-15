use bevy::prelude::*;
use seldom_state::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::bossfight::{Boss, BossConfig};
use crate::bossfight::state_machine::{AbilityStartup, Relocate};
use crate::combat::{ColliderAttack, Immunity};
use crate::state::GameState;


#[derive(Component, Clone, Debug)]
pub struct RelocateAbility {
    pub retract: Timer,
    pub extend: Timer
}

impl Default for RelocateAbility {
    fn default() -> Self {
        Self {
            retract: Timer::from_seconds(0.8, TimerMode::Once),
            extend: Timer::from_seconds(0.8, TimerMode::Once)
        }
    }
}

pub fn register_relocate_ability(app: &mut App) {
    app.add_system_set(
        SystemSet::on_update(GameState::Gameplay)
            .with_system(start_relocation)
            .with_system(relocate_update)
    );
}

fn start_relocation(
    mut commands: Commands,
    mut q: Query<(
        Entity,
        &mut ColliderAttack,
        &mut RelocateAbility
    ), (With<Boss>, Added<AbilityStartup>)>
) {
    if q.is_empty() {
        return;
    }

    let (entity, mut atk, mut relocate) = q.single_mut();

    atk.enabled = false;
    commands.entity(entity).insert(Immunity);

    relocate.retract.reset();
    relocate.extend.reset();
}

fn relocate_update(
    time: Res<Time>,
    mut commands: Commands,
    mut q: Query<(
        Entity,
        &mut RelocateAbility,
        &mut Transform,
        &BossConfig
    ), With<Relocate>>
) {
    if q.is_empty() {
        return;
    }

    let (entity, mut relocate, mut transform, cfg) = q.single_mut();
    relocate.retract.tick(time.delta());

    if relocate.retract.just_finished() {

        // Move the boss
        transform.translation.x = cfg.charge_right.x;
        transform.translation.y = cfg.charge_right.y;

        let tl = transform.translation;
        transform.rotate_around(tl, Quat::from_rotation_z((3.14 / 180.0) * 90.0));

    }

    if relocate.retract.finished() {
        relocate.extend.tick(time.delta());

        if relocate.extend.just_finished() {
            commands.entity(entity)
                .remove::<Immunity>()
                .insert(Done::Success);
        }
    }
}