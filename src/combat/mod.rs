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

use crate::assets::{ExplosionAssets, IndicatorAssets};

use crate::camera::GameCamera;

use crate::combat::collision::register_collider_attacks;
use crate::combat::consts::EXPLOSION_RADIUS;
use crate::combat::spore_cloud::SporeCloudAttackBundle;
use crate::entity_states::*;
use crate::fx::indicator::Indicator;
use crate::pathfind::Region;

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
    input: Res<Input<KeyCode>>,
    windows: Res<Windows>,
    mut commands: Commands,
    camera: Query<&GlobalTransform, With<GameCamera>>,
    assets: Res<ExplosionAssets>,
    ind_assets: Res<IndicatorAssets>,
    mut explosions: EventWriter<ExplosionEvent>
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


    if events.just_pressed(MouseButton::Right) {
        explosions.send(
            ExplosionEvent {
                pos: world_pos,
                radius: EXPLOSION_RADIUS,
                max_damage: 20
            }
        );
    }

    if input.just_pressed(KeyCode::M) {
        Indicator::spawn(
            &ind_assets,
            &mut commands,
            Indicator {
                region: Region {
                    tl: world_pos,
                    br: world_pos + Vec2::new(400.0, -100.0),
                },
                wait_time: 1.0,
                expand_time: 0.4,
                ..Indicator::SPAWNER
            },
        );
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

            health.hp -= hit.damage.abs();
            hurt.hit_event = Some(*hit);
        }
    }
}