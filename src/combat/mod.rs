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

pub use melee::*;
pub use projectile::*;
pub use components::*;
pub use events::*;
pub use hurt::*;
pub use collision::*;
pub use death::*;
pub use explosion::*;
pub use spore_cloud::*;

use crate::assets::ExplosionAssets;

use crate::camera::GameCamera;

use crate::combat::collision::register_collider_attacks;
use crate::combat::spore_cloud::SporeCloudAttackBundle;
use crate::entity_states::*;

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
    mut commands: Commands,
    camera: Query<&GlobalTransform, With<GameCamera>>,
    assets: Res<ExplosionAssets>
) {
    if !events.just_pressed(MouseButton::Left) || camera.is_empty() {
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

    // commands.spawn(ExplosionAttackBundle::from_pos(world_pos, &assets));
    commands.spawn(SporeCloudAttackBundle::from_pos(world_pos, Vec2::new(256.0, 128.0)));
}

fn handle_hits(
    immune: Query<&Immunity>,
    mut q: Query<(Entity, &mut HurtAbility, &mut Health), (Without<Hurt>, Without<Die>)>,
    mut hit_events: EventReader<CombatEvent>
) {
    for hit in hit_events.iter() {
        if let Ok((entity, mut hurt, mut health)) = q.get_mut(hit.target) {

            if immune.contains(entity) {
                hurt.hit_event = None;
                continue;
            }

            health.hp -= hit.damage;
            hurt.hit_event = Some(*hit);
        }
    }
}