use bevy::prelude::*;
use seldom_state::prelude::*;
use crate::coin::coin::CoinMovement;
use crate::entity_states::*;


pub fn register_coin_state_machine(app: &mut App) {
    app
        .add_plugin(TriggerPlugin::<CollectorInRangeTrigger>::default())
        .add_plugin(TriggerPlugin::<PickedUpTrigger>::default());
}



#[derive(Component, Copy, Clone, Reflect, FromReflect)]
pub struct CollectorInRangeTrigger;

impl Trigger for CollectorInRangeTrigger {
    type Param<'w, 's> = Query<'w, 's, &'static CoinMovement>;

    fn trigger(&self, ent: Entity, q: &Self::Param<'_, '_>) -> bool {
        if !q.contains(ent) {
            return false;
        }

        q.get(ent).unwrap().target.is_some() && q.get(ent).unwrap().grounded
    }
}

#[derive(Component, Copy, Clone, Reflect, FromReflect)]
pub struct PickedUpTrigger;

impl Trigger for PickedUpTrigger {
    type Param<'w, 's> = Query<'w, 's, &'static CoinMovement>;

    fn trigger(&self, ent: Entity, q: &Self::Param<'_, '_>) -> bool {
        if !q.contains(ent) {
            return false;
        }

        q.get(ent).unwrap().picked_up
    }
}



#[derive(Component, Copy, Clone, Reflect, Debug)]
pub struct Follow;

pub fn coin_state_machine() -> StateMachine {
    StateMachine::new(Idle)
        .trans::<Idle>(CollectorInRangeTrigger, Follow)
        .trans::<Follow>(PickedUpTrigger, Die::default())
        .trans::<Die>(NotTrigger(AlwaysTrigger), Die::default())
}