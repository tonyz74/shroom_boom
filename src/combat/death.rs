use bevy::prelude::*;
use seldom_state::prelude::*;
use crate::combat::Health;

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