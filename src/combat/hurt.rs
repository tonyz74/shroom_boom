use bevy::prelude::*;
use seldom_state::prelude::*;
use crate::bossfight::Boss;
use crate::combat::{ColliderAttack, CombatEvent, Immunity};
use crate::enemies::Enemy;
use crate::entity_states::*;
use crate::state::GameState;
use crate::util;




#[derive(Copy, Clone, Reflect, FromReflect)]
pub struct HurtTrigger;

impl Trigger for HurtTrigger {
    type Param<'w, 's> = Query<'w, 's, &'static HurtAbility>;

    fn trigger(&self, entity: Entity, q: &Self::Param<'_, '_>) -> bool {
        if !q.contains(entity) {
            return false;
        }

        let hurt = q.get(entity).unwrap();
        let ok = hurt.hit_event.is_some();

        ok
    }
}

pub fn register_hurt_ability(app: &mut App) {
    app.add_plugin(TriggerPlugin::<HurtTrigger>::default());

    app.add_system_set(
        SystemSet::on_update(GameState::Gameplay)
            .with_system(hurt_ability_trigger)
            .with_system(hurt_ability_update)
            .with_system(hurt_ability_flash)
            .with_system(hurt_ability_enemy_disable_collider)
            .with_system(hurt_ability_enemy_enable_collider.after(hurt_ability_update))
    );
}



#[derive(Component, Clone, Debug, Reflect)]
pub struct HurtAbility {
    pub immunity_timer: Timer,
    pub initial_stun_timer: Timer,
    pub regain_control_timer: Option<Timer>,
    pub hit_event: Option<CombatEvent>,

    pub flash_timer: Timer,
    pub should_disable_immunity: bool
}

impl HurtAbility {
    pub fn new(immunity_len: f32, regain_control_len: Option<f32>) -> Self {
        let mut immunity_timer = Timer::from_seconds(immunity_len, TimerMode::Once);
        util::timer_tick_to_finish(&mut immunity_timer);

        let regain_control_timer = match regain_control_len {
            Some(len) => {
                let mut timer = Timer::from_seconds(len, TimerMode::Once);
                util::timer_tick_to_finish(&mut timer);
                Some(timer)
            },
            None => None
        };

        let mut initial_stun_timer = Timer::from_seconds(0.1, TimerMode::Once);
        util::timer_tick_to_finish(&mut initial_stun_timer);

        Self {
            immunity_timer,
            regain_control_timer,

            initial_stun_timer: Timer::from_seconds(0.1, TimerMode::Once),
            hit_event: None,

            flash_timer: Timer::from_seconds(0.14, TimerMode::Repeating),
            should_disable_immunity: true
        }
    }

    pub fn is_immune(&self) -> bool {
        !self.immunity_timer.finished()
    }

    pub fn can_stop_hurting(&self) -> bool {
        self.initial_stun_timer.finished()
    }
}


fn hurt_ability_enemy_disable_collider(
    mut p: Query<&mut ColliderAttack>,
    q: Query<&Children, (Added<Hurt>, (With<Enemy>, Without<Boss>))>
) {
    for children in q.iter() {
        for child in children {
            if let Ok(mut atk) = p.get_mut(*child) {
                atk.enabled = false;
            }
        }
    }
}

fn hurt_ability_enemy_enable_collider(
    mut p: Query<&mut ColliderAttack>,
    q: Query<(&Children, &HurtAbility), (With<Enemy>, Without<Boss>)>
) {
    for (children, hurt) in q.iter() {
        if hurt.immunity_timer.just_finished() {
            for child in children {
                if let Ok(mut atk) = p.get_mut(*child) {
                    atk.enabled = true;
                }
            }
        }
    }
}


pub fn hurt_ability_trigger(
    mut hurts: Query<(&mut Immunity, &mut HurtAbility), (Added<Hurt>, Without<Die>)>
) {
    for (mut immunity, mut hurt) in hurts.iter_mut() {
        hurt.immunity_timer.reset();
        hurt.initial_stun_timer.reset();

        if let Some(timer) = &mut hurt.regain_control_timer {
            timer.reset();
        }

        hurt.flash_timer.reset();
        immunity.is_immune = true;
    }
}

pub fn hurt_ability_flash(
    time: Res<Time>,
    mut q: Query<(&mut TextureAtlasSprite, &mut HurtAbility, Option<&Die>)>
) {
    for (mut spr, mut hurt, maybe_dead) in q.iter_mut() {
        if hurt.is_immune() && maybe_dead.is_none() {
            hurt.flash_timer.tick(time.delta());

            if hurt.flash_timer.just_finished() {
                let flash_color = Color::rgb(f32::MAX, f32::MAX, f32::MAX);

                spr.color = if spr.color != flash_color {
                    flash_color
                } else {
                    Color::WHITE
                };
            }
        } else {
            spr.color = Color::WHITE;
        }
    }
}


pub fn hurt_ability_update(
    time: Res<Time>,
    mut commands: Commands,
    hurting: Query<Entity, With<Hurt>>,
    mut hurts: Query<(Entity, &mut Immunity, &mut HurtAbility), Without<Die>>
) {
    for (entity, mut immunity, mut hurt) in hurts.iter_mut() {
        let dt = time.delta();

        hurt.immunity_timer.tick(dt);
        hurt.initial_stun_timer.tick(dt);

        if let Some(regain_control_timer) = &mut hurt.regain_control_timer {
            regain_control_timer.tick(dt);

            if regain_control_timer.just_finished() && hurting.contains(entity) {
                commands.entity(entity).insert(Done::Success);
            }
        }

        if hurt.immunity_timer.just_finished() {
            if hurt.should_disable_immunity {
                immunity.is_immune = false;
            }

            hurt.should_disable_immunity = true;
        }
    }
}