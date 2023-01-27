
use bevy::prelude::*;
use crate::anim::{AnimationChangeEvent};
use crate::anim::map::AnimationMap;


use crate::enemies::flower::FlowerEnemy;
use crate::enemies::flower::state_machine::Detonate;
use crate::state::GameState;
use crate::entity_states::*;


pub fn register_flower_enemy_animations(app: &mut App) {
    app.add_system_set(
        SystemSet::on_update(GameState::Gameplay)
            .with_system(flower_enemy_detonate)
    );
}


fn flower_enemy_detonate(
    q: Query<(&AnimationMap, Entity), (Added<Detonate>, With<FlowerEnemy>, Without<Die>)>,
    mut ev: EventWriter<AnimationChangeEvent>
) {
    for (anims, flower) in q.iter() {
        ev.send(AnimationChangeEvent {
            e: flower,
            new_anim: anims["DETONATE"].clone()
        });
    }
}