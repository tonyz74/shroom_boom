use bevy::prelude::*;
use seldom_state::prelude::*;
use crate::combat::Health;
use crate::entity_states::Die;
use crate::state::GameState;

#[derive(Component, Debug, Reflect, FromReflect, Clone)]
pub struct DeathTrigger;

impl Trigger for DeathTrigger {
    type Param<'w, 's> = Query<'w, 's, &'static Health>;

    fn trigger(&self, entity: Entity, q: &Self::Param<'_, '_>) -> bool {
        if !q.contains(entity) {
            return false;
        }

        let health = q.get(entity).unwrap();
        health.hp <= 0
    }
}

pub fn register_death(app: &mut App) {
    app.add_plugin(TriggerPlugin::<DeathTrigger>::default());
    app.add_system_set(
        SystemSet::on_update(GameState::Gameplay)
            .with_system(despawn_dead_entities)
    );
}

pub fn despawn_dead_entities(
    mut commands: Commands,
    deaths: Query<(Entity, &Die)>
) {
    for (entity, death) in deaths.iter() {
        if death.should_despawn {
            if let Some(cmd) = commands.get_entity(entity) {
                cmd.despawn_recursive();
            }
        }
    }
}