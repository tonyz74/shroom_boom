use bevy::prelude::*;

mod melee;
mod projectile;

mod components;
mod events;
mod hurt;
mod collision;
mod death;
mod explosion;
mod knockbacks;
mod spore_cloud;
mod consts;

pub use melee::*;
pub use projectile::*;
pub use components::*;
pub use events::*;
pub use hurt::*;
pub use collision::*;
pub use death::*;
pub use explosion::*;
pub use spore_cloud::*;

use crate::camera::GameCamera;
use crate::combat::collision::register_collider_attacks;
use crate::combat::consts::EXPLOSION_RADIUS;
use crate::combat::spore_cloud::SporeCloudAttackBundle;
use crate::entity_states::*;
use crate::fx::shake::ScreenShakeEvent;

use crate::state::GameState;
pub struct AttackPlugin;


impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {

        app
            .add_system_set(
                SystemSet::on_update(GameState::Gameplay)
                    .with_system(resolve_melee_attacks)
                    .with_system(handle_hits)
                    .with_system(temp_explosion)
            )

            .add_event::<CombatEvent>()
            .register_type::<HurtAbility>();

        register_death(app);
        register_projectile_attacks(app);
        register_hurt_ability(app);
        register_collider_attacks(app);
        register_explosion_attacks(app);
        register_spore_cloud_attacks(app);
    }
}

fn temp_explosion(
    events: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    camera: Query<&GlobalTransform, With<GameCamera>>,
    mut explosions: EventWriter<ExplosionEvent>,
    mut shakes: EventWriter<ScreenShakeEvent>
) {
    if camera.is_empty() {
        return;
    }

    let cam = Vec2::new(
        camera.single().translation().x,
        camera.single().translation().y
    );

    let win = windows.primary();

    if win.cursor_position().is_none() {
        return;
    }

    let cpos = win.cursor_position().unwrap();
    let world_pos = cpos + (cam - 0.5 * Vec2::new(win.width(), win.height()));

    if events.just_pressed(MouseButton::Left) {
        shakes.send(ScreenShakeEvent::LARGE);
    }

    if events.just_pressed(MouseButton::Right) {
        explosions.send(
            ExplosionEvent {
                pos: world_pos,
                radius: EXPLOSION_RADIUS * 12.0,
                max_damage: 50,
                combat_layer: CombatLayerMask::PLAYER
            }
        );

        let _ = SporeCloudAttackBundle::new(Vec2::ZERO, Vec2::ZERO);
    }
}

fn handle_hits(
    immune: Query<&Immunity>,
    mut q: Query<(Entity, &mut HurtAbility, &mut Health), (Without<Hurt>, Without<Die>)>,
    mut hit_events: EventReader<CombatEvent>
) {
    for hit in hit_events.iter() {
        if let Ok((entity, mut hurt, mut health)) = q.get_mut(hit.target) {
            if immune.contains(entity) && immune.get(entity).unwrap().is_immune {
                hurt.hit_event = None;
                continue;
            }

            if hurt.hit_event.is_some() {
                continue;
            }

            health.hp -= hit.damage.abs();
            hurt.hit_event = Some(*hit);
        }
    }
}