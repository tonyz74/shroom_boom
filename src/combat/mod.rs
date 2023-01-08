use bevy::prelude::*;

mod melee;
mod projectile;

mod components;
mod events;
mod hurt;
mod collision;
mod death;

pub use melee::*;
pub use projectile::*;
pub use components::*;
pub use events::*;
pub use hurt::*;
pub use collision::*;
pub use death::*;

use crate::combat::collision::register_collider_attacks;
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
            )

            .add_event::<CombatEvent>()
            .register_type::<HurtAbility>();

        register_death(app);
        register_projectile_attacks(app);
        register_hurt_ability(app);
        register_collider_attacks(app);
    }
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